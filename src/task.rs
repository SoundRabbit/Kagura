use std::cell::RefCell;
use wasm_bindgen::JsValue;

thread_local!(static TASKS: RefCell<Vec<Box<dyn FnOnce()>>> = RefCell::new(vec![]));

pub fn dispatch() {
    web_sys::console::log_1(&JsValue::from("task::dispatch"));
    let tasks: Vec<_> = TASKS.with(|tasks| tasks.borrow_mut().drain(..).collect());
    for task in tasks {
        task();
    }
}

pub fn add(task: impl FnOnce() + 'static) {
    web_sys::console::log_1(&JsValue::from("task::add"));
    TASKS.with(|tasks| {
        tasks.borrow_mut().push(Box::new(task));
    });
}
