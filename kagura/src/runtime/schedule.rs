use crate::future_msg::{Batch, Task};
use crate::node::msg::Msg;
use crate::FutureMsg;
use async_std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct Scedule {
    scedules: Arc<Mutex<VecDeque<FutureScedule>>>,
}

struct ScedulePoller {
    scedules: Arc<Mutex<VecDeque<FutureScedule>>>,
}

struct FutureScedule {
    batch: Option<Box<dyn Batch>>,
    task: Task,
}

impl Scedule {
    pub fn new() -> Self {
        Self {
            scedules: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn listen(&self) -> impl Future<Output = Vec<Msg>> {
        ScedulePoller {
            scedules: Arc::clone(&self.scedules),
        }
    }

    pub async fn append(&mut self, new_scedules: &mut VecDeque<FutureMsg>) {
        let mut scedules = self.scedules.lock_arc().await;
        for new_scedule in new_scedules.drain(..) {
            match new_scedule {
                FutureMsg::Task(task) => {
                    scedules.push_back(FutureScedule {
                        batch: None,
                        task: task,
                    });
                }
                FutureMsg::Batch(mut batch) => {
                    if let Some(task) = batch.poll() {
                        scedules.push_back(FutureScedule {
                            batch: Some(batch),
                            task: task,
                        });
                    }
                }
            }
        }
    }
}

impl Future for ScedulePoller {
    type Output = Vec<Msg>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(mut scedules) = self.as_mut().scedules.try_lock_arc() {
            let mut new_scedules = VecDeque::new();
            let mut output = vec![];
            let mut is_ready = false;

            while let Some(mut scedule) = scedules.pop_front() {
                match scedule.task.as_mut().poll(cx) {
                    Poll::Pending => {
                        new_scedules.push_back(scedule);
                    }
                    Poll::Ready(mut msg) => {
                        is_ready = true;
                        output.append(&mut msg);
                        if let Some(task) = scedule.batch.as_mut().and_then(|batch| batch.poll()) {
                            scedule.task = task;
                            new_scedules.push_back(scedule);
                        }
                    }
                }
            }

            scedules.append(&mut new_scedules);

            if is_ready {
                return Poll::Ready(output);
            }
        }

        Poll::Pending
    }
}
