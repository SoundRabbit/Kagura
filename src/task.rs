use crate::bin;
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;

thread_local!(static TASK_RESOLVERS: RefCell<HashMap<u128, Box<FnMut(Box<Any>) -> (u128, Box<Any>)>>> = RefCell::new(HashMap::new()));

pub fn dispatch(id: u128, msg: Box<Any>) {
    let msg = TASK_RESOLVERS.with(|task_resolvers| {
        task_resolvers
            .borrow_mut()
            .get_mut(&id)
            .map(|resolver| resolver(msg))
    });
    if let Some((component_id, msg)) = msg {
        bin::update(component_id, msg);
    }
}

pub fn add(id: u128, resolver: impl FnMut(Box<Any>) -> (u128, Box<Any>) + 'static) {
    TASK_RESOLVERS.with(|task_resolvers| {
        task_resolvers.borrow_mut().insert(id, Box::new(resolver));
    });
}
