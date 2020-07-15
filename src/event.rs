use std::cell::{Cell, RefCell};
use std::collections::HashMap;

pub type HandlerId = u64;

thread_local!(static HANDLERS: RefCell<HashMap<HandlerId, Vec<Box<dyn FnOnce(web_sys::Event)>>>> = RefCell::new(HashMap::new()));

thread_local!(static HANDLER_COUNT: Cell<HandlerId> = Cell::new(0));

pub fn new_handler_id() -> HandlerId {
    HANDLER_COUNT.with(|handler_count| {
        let handler_id = handler_count.get();
        handler_count.set(handler_id + 1);
        handler_id
    })
}

pub fn dispatch(handler_id: HandlerId, e: web_sys::Event) {
    let mut handlers =
        HANDLERS.with(|handlers| handlers.borrow_mut().remove(&handler_id).unwrap_or(vec![]));
    for handler in handlers.drain(..) {
        handler(e.clone());
    }
}

pub fn add(handler_id: HandlerId, handler: impl FnOnce(web_sys::Event) + 'static) -> HandlerId {
    HANDLERS.with(|handlers| {
        if let Some(handlers) = handlers.borrow_mut().get_mut(&handler_id) {
            handlers.push(Box::new(handler));
        } else {
            handlers
                .borrow_mut()
                .insert(handler_id, vec![Box::new(handler)]);
        }
    });
    handler_id
}

pub fn remove(handler_id: &HandlerId) {
    HANDLERS.with(|handlers| {
        handlers.borrow_mut().remove(handler_id);
    });
}
