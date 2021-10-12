use crate::Kagura;
use std::cell::RefCell;

thread_local!(static STATE: RefCell<Option<State>> = RefCell::new(None));

struct State {
    kagura: Kagura,
    render: fn(&mut Kagura),
}

pub fn mount(kagura: Kagura, render: fn(&mut Kagura)) {
    STATE.with(|state| {
        *state.borrow_mut() = Some(State { kagura, render });
    });
    crate::state::render();
}

pub fn render() {
    STATE.with(|state| {
        if let Some(state) = state.borrow_mut().as_mut() {
            (state.render)(&mut state.kagura);
        }
    });
    crate::env::dispatch_ref_node();
    crate::env::dispatch_task();
}
