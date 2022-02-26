mod task;

use task::Task;

pub struct Runtime {
    task: Task,
}

impl Runtime {
    pub fn run() {}

    async fn event_loop(&mut self) {
        let msg = self.task.listen().await;
    }
}
