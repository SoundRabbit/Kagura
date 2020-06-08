use std::cell::RefCell;
use std::collections::HashMap;

pub type HandlerId = u128;

thread_local!(static HANDLERS: RefCell<HashMap<u128, Box<dyn FnOnce(web_sys::Event)>>> = RefCell::new(HashMap::new()));

pub fn new_handler_id() -> HandlerId {
    rand::random::<u128>()
}

pub fn dispatch(handler_id: HandlerId, e: web_sys::Event) {
    let handler = HANDLERS.with(|handlers| handlers.borrow_mut().remove(&handler_id));
    if let Some(handler) = handler {
        handler(e);
    }
}

pub fn add(handler_id: HandlerId, handler: impl FnOnce(web_sys::Event) + 'static) -> u128 {
    HANDLERS.with(|handlers| {
        handlers.borrow_mut().insert(handler_id, Box::new(handler));
    });
    handler_id
}

pub fn remove(handler_id: &HandlerId) {
    HANDLERS.with(|handlers| {
        handlers.borrow_mut().remove(handler_id);
    });
}
