use std::cell::{Cell, RefCell};
use std::collections::HashMap;

pub type HandlerId = u64;

thread_local!(static HANDLERS: RefCell<HashMap<HandlerId, Box<dyn FnOnce(web_sys::Event)>>> = RefCell::new(HashMap::new()));

thread_local!(static HANDLER_COUNT: Cell<HandlerId> = Cell::new(0));

pub fn new_handler_id() -> HandlerId {
    HANDLER_COUNT.with(|handler_count| {
        let handler_id = handler_count.get();
        handler_count.set(handler_id + 1);
        handler_id
    })
}

pub fn dispatch(handler_id: HandlerId, e: web_sys::Event) {
    let handler = HANDLERS.with(|handlers| handlers.borrow_mut().remove(&handler_id));
    if let Some(handler) = handler {
        handler(e);
    }
}

pub fn add(handler_id: HandlerId, handler: impl FnOnce(web_sys::Event) + 'static) -> HandlerId {
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
