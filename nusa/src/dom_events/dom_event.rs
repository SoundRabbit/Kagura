use async_std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use wasm_bindgen::{prelude::*, JsCast};

pub struct DomEvent {
    event_queue: Arc<Mutex<VecDeque<web_sys::Event>>>,
}

pub struct DomEventPoller {
    event_queue: Arc<Mutex<VecDeque<web_sys::Event>>>,
}

impl DomEvent {
    pub fn new(target: &web_sys::EventTarget, event_type: &str) -> Self {
        let event_queue = Arc::new(Mutex::new(VecDeque::new()));

        let a = Closure::wrap(Box::new({
            let event_queue = Arc::clone(&event_queue);
            move |e| {
                let event_queue = Arc::clone(&event_queue);
                wasm_bindgen_futures::spawn_local(async move {
                    event_queue.lock().await.push_back(e);
                });
            }
        }) as Box<dyn FnMut(web_sys::Event)>);
        target.add_event_listener_with_callback(event_type, a.as_ref().unchecked_ref());
        a.forget();

        Self { event_queue }
    }

    pub fn poll(&self) -> impl Future<Output = web_sys::Event> {
        DomEventPoller {
            event_queue: Arc::clone(&self.event_queue),
        }
    }
}

impl Future for DomEventPoller {
    type Output = web_sys::Event;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(mut event_queue) = self.as_mut().event_queue.try_lock() {
            if let Some(event) = event_queue.pop_front() {
                return Poll::Ready(event);
            }
        }
        Poll::Pending
    }
}
