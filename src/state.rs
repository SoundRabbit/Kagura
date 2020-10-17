use crate::dom;
use crate::dom::component::{Component, Composed, ComposedComponent};
use crate::native;
use crate::task;
use crate::uid;
use std::cell::RefCell;
use std::rc::Rc;
thread_local!(static APP: RefCell<Option<App>> = RefCell::new(None));

struct App {
    root_component: Rc<RefCell<Box<dyn Composed>>>,
    dom_renderer: dom::Renderer,
}

pub fn init(root_component: Rc<RefCell<Box<dyn Composed>>>, id: &str) {
    let node = root_component.borrow_mut().render();
    let root = native::get_element_by_id(id);
    let dom_renderer = dom::Renderer::new(node, root.into());
    APP.with(|app| {
        *app.borrow_mut() = Some(App {
            root_component,
            dom_renderer,
        })
    });
    task::dispatch();
}

pub fn render() {
    APP.with(|app| {
        if let Some(app) = &mut (*app.borrow_mut()) {
            let node = app.root_component.borrow_mut().render();
            app.dom_renderer.update(node);
        }
    });
    task::dispatch();
}
