use crate::node::msg::{FutureMsg, Msg};
use async_std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct Task {
    tasks: Arc<Mutex<VecDeque<FutureMsg>>>,
}

struct TaskPoller {
    tasks: Arc<Mutex<VecDeque<FutureMsg>>>,
}

impl Task {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn listen(&self) -> impl Future<Output = Vec<Msg>> {
        TaskPoller {
            tasks: Arc::clone(&self.tasks),
        }
    }

    pub async fn append(&mut self, new_tasks: &mut VecDeque<FutureMsg>) {
        let mut tasks = self.tasks.lock_arc().await;
        tasks.append(new_tasks);
    }
}

impl Future for TaskPoller {
    type Output = Vec<Msg>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(mut tasks) = self.as_mut().tasks.try_lock_arc() {
            let mut new_tasks = VecDeque::new();
            let mut output = vec![];

            while let Some(mut task) = tasks.pop_front() {
                match task.as_mut().poll(cx) {
                    Poll::Pending => {
                        new_tasks.push_back(task);
                    }
                    Poll::Ready(mut msg) => {
                        output.append(&mut msg);
                    }
                }
            }

            tasks.append(&mut new_tasks);

            if !output.is_empty() {
                return Poll::Ready(output);
            }
        }

        Poll::Pending
    }
}
