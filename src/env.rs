use std::cell::{Cell, RefCell};
use std::collections::HashMap;

pub type IdType = u64;

thread_local!(static ID_COUNT: Cell<u64> = Cell::new(0));
thread_local!(static EVENT_HANDLERS: RefCell<HashMap<IdType, Box<dyn FnOnce(web_sys::Event)>>> = RefCell::new(HashMap::new()));
thread_local!(static TASK_QUEUE: RefCell<Vec<Box<dyn FnOnce()>>> = RefCell::new(Vec::new()));
thread_local!(static REF_NODE_QUEUE: RefCell<Vec<Box<dyn FnOnce()>>> = RefCell::new(Vec::new()));

pub fn gen_id() -> IdType {
    ID_COUNT.with(|id_count| {
        let id = id_count.get();
        id_count.set(id + 1);
        id
    })
}

pub fn dispatch_event(h_id: IdType, e: web_sys::Event) {
    let handler = EVENT_HANDLERS.with(|handlers| handlers.borrow_mut().remove(&h_id));
    if let Some(handler) = handler {
        handler(e);
    }
}

pub fn add_event_handler(h_id: IdType, handler: impl FnOnce(web_sys::Event) + 'static) -> IdType {
    EVENT_HANDLERS.with(|handlers| {
        handlers.borrow_mut().insert(h_id, Box::new(handler));
    });
    h_id
}

pub fn remove_event_handler(h_id: &IdType) {
    EVENT_HANDLERS.with(|handlers| {
        handlers.borrow_mut().remove(h_id);
    });
}

pub fn dispatch_task() {
    let tasks: Vec<_> = TASK_QUEUE.with(|tasks| tasks.borrow_mut().drain(..).collect());
    for task in tasks {
        task();
    }
}

pub fn add_task(task: impl FnOnce() + 'static) {
    TASK_QUEUE.with(|tasks| {
        tasks.borrow_mut().push(Box::new(task));
    });
}

pub fn dispatch_ref_node() {
    let ref_nodes: Vec<_> =
        REF_NODE_QUEUE.with(|ref_nodes| ref_nodes.borrow_mut().drain(..).collect());
    for ref_node in ref_nodes {
        ref_node();
    }
}

pub fn add_ref_node(ref_node: impl FnOnce() + 'static) {
    REF_NODE_QUEUE.with(|ref_nodes| {
        ref_nodes.borrow_mut().push(Box::new(ref_node));
    });
}
