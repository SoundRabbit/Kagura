use crate::dom_renderer::VEventListener;
use kagura::node::Msg;
use std::cell::Cell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

pub mod dom_event;

pub use dom_event::DomEvent;

type EventPollers = Arc<Mutex<VecDeque<(Pin<Box<dom_event::DomEventPoller>>, VEventListener)>>>;

pub struct DomEvents {
    root: web_sys::EventTarget,
    created_event_pollers: HashMap<String, DomEvent>,
    using_event_pollers: EventPollers,
    batch_is_enebale: Arc<Cell<bool>>,
}

pub struct DomEventsBatch {
    is_enebale: Arc<Cell<bool>>,
    event_pollers: EventPollers,
}

pub struct DomEventsPoller {
    event_pollers: EventPollers,
}

impl DomEvents {
    pub fn new(root: web_sys::EventTarget) -> Self {
        Self {
            root,
            created_event_pollers: HashMap::new(),
            using_event_pollers: Arc::new(Mutex::new(VecDeque::new())),
            batch_is_enebale: Arc::new(Cell::new(true)),
        }
    }

    pub fn batch(&mut self) -> impl kagura::future_msg::Batch {
        DomEventsBatch::new(
            Arc::clone(&self.batch_is_enebale),
            Arc::clone(&self.using_event_pollers),
        )
    }

    pub fn listen(&mut self, event_listeners: HashMap<String, VEventListener>) {
        if let Ok(mut using_event_pollers) = self.using_event_pollers.try_lock() {
            using_event_pollers.clear();
            for (event_type, event_listener) in event_listeners {
                let event_poller =
                    if let Some(dom_event) = self.created_event_pollers.get(&event_type) {
                        dom_event.poll()
                    } else {
                        let dom_event = DomEvent::new(&self.root, &event_type);
                        let event_poller = dom_event.poll();
                        self.created_event_pollers.insert(event_type, dom_event);
                        event_poller
                    };
                using_event_pollers.push_back((Box::pin(event_poller), event_listener));
            }
        }
    }
}

impl std::ops::Drop for DomEvents {
    fn drop(&mut self) {
        self.batch_is_enebale.set(false);
    }
}

impl DomEventsBatch {
    pub fn new(is_enebale: Arc<Cell<bool>>, event_pollers: EventPollers) -> Self {
        Self {
            is_enebale,
            event_pollers,
        }
    }
}

impl kagura::future_msg::Batch for DomEventsBatch {
    fn poll(&mut self) -> Option<kagura::future_msg::Task> {
        if self.is_enebale.get() {
            Some(Box::pin(DomEventsPoller::new(Arc::clone(
                &self.event_pollers,
            ))))
        } else {
            None
        }
    }
}

impl DomEventsPoller {
    pub fn new(event_pollers: EventPollers) -> Self {
        Self { event_pollers }
    }
}

impl Future for DomEventsPoller {
    type Output = Vec<Msg>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Ok(mut event_pollers) = self.event_pollers.try_lock() {
            let mut new_event_pollers = VecDeque::new();
            let mut output = vec![];
            let mut is_ready = false;

            for event_poller in event_pollers.iter_mut() {
                if let Poll::Ready(e) = event_poller.0.as_mut().poll(cx) {
                    is_ready = true;
                    let mut msg: Vec<_> = (event_poller.1)(e).1.into();
                    output.append(&mut msg);
                }
            }

            event_pollers.append(&mut new_event_pollers);

            if is_ready {
                return Poll::Ready(output);
            }
        }

        Poll::Pending
    }
}
