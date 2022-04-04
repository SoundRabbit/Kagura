use crate::component::{BatchProcess, Cmd};
use crate::Component;
use async_std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

pub struct Batch<T: 'static> {
    state: Arc<Mutex<State<T>>>,
}

pub struct BatchPoller<T: 'static> {
    state: Arc<Mutex<State<T>>>,
}

struct State<T> {
    waker: Option<Waker>,
    result_queue: VecDeque<T>,
}

pub type BatchResolver<T> = Box<dyn FnMut(T)>;

impl<T> Batch<T> {
    pub fn new(batch: impl FnOnce(BatchResolver<T>)) -> Self {
        let state = Arc::new(Mutex::new(State {
            waker: None,
            result_queue: VecDeque::new(),
        }));
        let batch_resolver: BatchResolver<T> = Box::new({
            let state = Arc::clone(&state);
            move |result| {
                if let Some(mut state) = state.try_lock_arc() {
                    state.result_queue.push_back(result);
                    if let Some(waker) = state.waker.take() {
                        waker.wake();
                    }
                }
            }
        });

        batch(batch_resolver);

        Self { state }
    }

    pub fn poll(&self) -> impl Future<Output = T> {
        BatchPoller {
            state: Arc::clone(&self.state),
        }
    }
}

impl<C: Component> BatchProcess<C> for Batch<Cmd<C>> {
    fn poll(&mut self) -> Pin<Box<dyn Future<Output = Cmd<C>>>> {
        Box::pin(Batch::poll(self))
    }
}

impl<T> Future for BatchPoller<T> {
    type Output = T;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(mut state) = self.as_mut().state.try_lock_arc() {
            if let Some(result) = state.result_queue.pop_front() {
                return Poll::Ready(result);
            } else {
                state.waker = Some(cx.waker().clone())
            }
        }
        Poll::Pending
    }
}
