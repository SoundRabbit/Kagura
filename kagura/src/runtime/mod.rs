use crate::node::{NodeCmd, RenderNode, UpdateNode};
use std::collections::VecDeque;

mod schedule;

use schedule::Scedule;

pub struct Runtime<Node: UpdateNode + RenderNode<NodeCmd> + 'static> {
    schedule: Scedule,
    node: Node,
}

impl<Node: UpdateNode + RenderNode<NodeCmd> + 'static> Runtime<Node> {
    pub async fn run(node: Node) {
        let mut runtime = Self {
            schedule: Scedule::new(),
            node: node,
        };

        let mut is_busy = true;
        loop {
            is_busy = runtime.event_loop(is_busy).await;
        }
    }

    async fn event_loop(&mut self, is_busy: bool) -> bool {
        let mut is_busy = if is_busy {
            let mut schedules = self.node.render();
            self.schedule.append(&mut schedules).await;
            !schedules.is_lazy()
        } else {
            false
        };
        let msgs = self.schedule.listen().await;
        let mut schedules = VecDeque::new();
        for msg in msgs {
            let msg_is_busy = !msg.is_lazy();
            let mut node_cmd = self.node.update(msg);
            let cmd_is_busy = !node_cmd.is_lazy();
            schedules.append(&mut node_cmd);
            if msg_is_busy && cmd_is_busy {
                is_busy = true;
            }
        }
        self.schedule.append(&mut schedules).await;
        is_busy
    }
}
