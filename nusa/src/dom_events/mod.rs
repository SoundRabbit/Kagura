use crate::dom_renderer::VEventListeners;
use kagura::node::{FutureMsg, Msg};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub mod dom_event;

pub use dom_event::DomEvent;

pub struct DomEvents {
    root: web_sys::EventTarget,
    event_pollers: HashMap<String, DomEvent>,
}

pub struct DomEventsPoller {
    event_pollers: VecDeque<FutureMsg>,
}

impl DomEvents {
    pub fn new(root: web_sys::EventTarget) -> Self {
        Self {
            root,
            event_pollers: HashMap::new(),
        }
    }

    pub fn listen(&mut self, event_listeners: VEventListeners) -> FutureMsg {
        let mut event_pollers = VecDeque::new();

        for (event_type, event_listener) in event_listeners {
            let event_poller = self.get_event_poller(&event_type);
            event_pollers.push_back(Box::pin(async move {
                let e = event_poller.await;
                event_listener(e).1.into()
            }) as FutureMsg);
        }

        Box::pin(DomEventsPoller { event_pollers })
    }

    pub fn get_event_poller(
        &mut self,
        event_type: &String,
    ) -> impl Future<Output = web_sys::Event> {
        if let Some(dom_event) = self.event_pollers.get(event_type) {
            dom_event.poll()
        } else {
            let dom_event = DomEvent::new(&self.root, event_type);
            let event_poller = dom_event.poll();
            self.event_pollers.insert(event_type.clone(), dom_event);
            event_poller
        }
    }
}

impl Future for DomEventsPoller {
    type Output = Vec<Msg>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut new_event_pollers = VecDeque::new();
        let mut output = vec![];

        while let Some(mut event_poller) = self.event_pollers.pop_front() {
            match event_poller.as_mut().poll(cx) {
                Poll::Pending => {
                    new_event_pollers.push_back(event_poller);
                }
                Poll::Ready(mut msg) => {
                    output.append(&mut msg);
                }
            }
        }

        self.event_pollers.append(&mut new_event_pollers);

        if !output.is_empty() {
            return Poll::Ready(output);
        }

        Poll::Pending
    }
}
