use crate::bin;
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;

thread_local!(static EVENT_LISTENRS: RefCell<HashMap<u128, Box<FnMut(web_sys::Event) -> (u128, Box<Any>)>>> = RefCell::new(HashMap::new()));

pub fn dispatch(id: u128, e: web_sys::Event) {
    let msg = EVENT_LISTENRS.with(|event_listeners| {
        if let Some(litener) = event_listeners.borrow_mut().get_mut(&id) {
            Some(litener(e))
        } else {
            None
        }
    });
    if let Some((component_id, msg)) = msg {
        bin::update(component_id, msg);
    }
}

pub fn add(id: u128, listener: impl FnMut(web_sys::Event) -> (u128, Box<Any>) + 'static) {
    EVENT_LISTENRS.with(|event_listeners| {
        event_listeners.borrow_mut().insert(id, Box::new(listener));
    });
}

pub fn remove(id: u128) {
    EVENT_LISTENRS.with(|event_listeners| {
        event_listeners.borrow_mut().remove(&id);
    });
}
