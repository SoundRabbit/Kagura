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
        if is_busy {
            let mut schedules = self.node.render();
            self.schedule.append(&mut schedules).await;
        }
        let msgs = self.schedule.listen().await;
        let mut schedules = VecDeque::new();
        let mut is_busy = false;
        for msg in msgs {
            let mut node_cmd = self.node.update(msg);
            schedules.append(&mut node_cmd);
            is_busy = true;
        }
        self.schedule.append(&mut schedules).await;
        is_busy
    }
}
