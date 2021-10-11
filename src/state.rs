use crate::Kagura;
use std::cell::RefCell;

thread_local!(static STATE: RefCell<Option<Kagura>> = RefCell::new(None));

pub fn mount(kagura: Kagura) {
    STATE.with(|state| {
        *state.borrow_mut() = Some(kagura);
    });
}

pub fn render() {
    STATE.with(|state| {
        if let Some(state) = state.borrow_mut().as_mut() {
            state.render();
        }
    });
    crate::env::dispatch_task();
}
