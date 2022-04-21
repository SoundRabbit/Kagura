use async_std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use wasm_bindgen::{prelude::*, JsCast};

pub struct DomEvent {
    state: Arc<Mutex<State>>,
}

pub struct DomEventPoller {
    state: Arc<Mutex<State>>,
}

struct State {
    event_queue: VecDeque<web_sys::Event>,
    waker: Option<Waker>,
}

impl DomEvent {
    pub fn new(target: &web_sys::EventTarget, event_type: &str) -> Self {
        let state = Arc::new(Mutex::new(State {
            event_queue: VecDeque::new(),
            waker: None,
        }));

        let a = Closure::wrap(Box::new({
            let state = Arc::clone(&state);
            move |e| {
                let state = Arc::clone(&state);
                wasm_bindgen_futures::spawn_local(async move {
                    let mut state = state.lock_arc().await;
                    state.event_queue.push_back(e);
                    if let Some(waker) = state.waker.take() {
                        waker.wake();
                    }
                });
            }
        }) as Box<dyn FnMut(web_sys::Event)>);
        let _ = target.add_event_listener_with_callback(event_type, a.as_ref().unchecked_ref());
        a.forget();

        Self { state }
    }

    pub fn poll(&self) -> DomEventPoller {
        DomEventPoller {
            state: Arc::clone(&self.state),
        }
    }
}

impl Future for DomEventPoller {
    type Output = web_sys::Event;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(mut state) = self.as_mut().state.try_lock_arc() {
            state.waker = Some(cx.waker().clone());
            if let Some(event) = state.event_queue.pop_front() {
                return Poll::Ready(event);
            }
        }
        Poll::Pending
    }
}
