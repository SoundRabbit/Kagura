use crate::component::Msg;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

pub struct Task {
    tasks: Arc<Mutex<VecDeque<Pin<Box<dyn Future<Output = Msg>>>>>>,
}

struct TaskPoller {
    tasks: Arc<Mutex<VecDeque<Pin<Box<dyn Future<Output = Msg>>>>>>,
}

impl Task {
    pub fn listen(&self) -> impl Future<Output = Vec<Msg>> {
        TaskPoller {
            tasks: Arc::clone(&self.tasks),
        }
    }
}

impl Future for TaskPoller {
    type Output = Vec<Msg>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut tasks = if let Ok(mut tasks) = self.as_mut().tasks.lock() {
            tasks.drain(..).collect::<VecDeque<_>>()
        } else {
            return Poll::Pending;
        };

        let mut new_tasks = VecDeque::new();
        let mut output = vec![];

        while let Some(mut task) = tasks.pop_front() {
            match task.as_mut().poll(cx) {
                Poll::Pending => {
                    new_tasks.push_back(task);
                }
                Poll::Ready(msg) => {
                    output.push(msg);
                }
            }
        }

        if output.is_empty() {
            Poll::Pending
        } else {
            Poll::Ready(output)
        }
    }
}
