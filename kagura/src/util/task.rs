use async_std::sync::{Arc, Mutex};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

pub struct Task<T: 'static> {
    state: Arc<Mutex<State<T>>>,
}

struct State<T> {
    waker: Option<Waker>,
    result: Option<T>,
}

pub type TaskResolver<T> = Box<dyn FnOnce(T)>;

impl<T> Task<T> {
    pub fn new(task: impl FnOnce(TaskResolver<T>)) -> Self {
        let state = Arc::new(Mutex::new(State {
            waker: None,
            result: None,
        }));
        let task_resolver: TaskResolver<T> = Box::new({
            let state = Arc::clone(&state);
            move |result| {
                if let Some(mut state) = state.try_lock_arc() {
                    state.result = Some(result);
                    if let Some(waker) = state.waker.take() {
                        waker.wake();
                    }
                }
            }
        });

        task(task_resolver);

        Self { state }
    }
}

impl<T> Future for Task<T> {
    type Output = T;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(mut state) = self.as_mut().state.try_lock_arc() {
            if let Some(result) = state.result.take() {
                return Poll::Ready(result);
            } else {
                state.waker = Some(cx.waker().clone())
            }
        }
        Poll::Pending
    }
}
