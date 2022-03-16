mod task;

use crate::node::{FutureMsg, RenderNode, UpdateNode};
use std::collections::VecDeque;
use task::Task;

pub struct Runtime<Node: UpdateNode + RenderNode<VecDeque<FutureMsg>> + 'static> {
    task: Task,
    node: Node,
}

impl<Node: UpdateNode + RenderNode<VecDeque<FutureMsg>> + 'static> Runtime<Node> {
    pub fn run(node: Node) {
        let mut runtime = Self {
            task: Task::new(),
            node: node,
        };

        wasm_bindgen_futures::spawn_local(async move {
            loop {
                runtime.event_loop().await;
            }
        });
    }

    async fn event_loop(&mut self) {
        self.task.append(&mut self.node.render()).await;
        let msgs = self.task.listen().await;
        let mut tasks = VecDeque::new();
        for msg in msgs {
            tasks.append(&mut self.node.update(msg));
        }
        self.task.append(&mut tasks).await;
    }
}
