use std::cell::RefCell;

thread_local!(static TASKS: RefCell<Vec<Box<dyn FnOnce()>>> = RefCell::new(vec![]));

pub fn dispatch() {
    let tasks: Vec<_> = TASKS.with(|tasks| tasks.borrow_mut().drain(..).collect());
    for task in tasks {
        task();
    }
}

pub fn add(task: impl FnOnce() + 'static) {
    TASKS.with(|tasks| {
        tasks.borrow_mut().push(Box::new(task));
    });
}
