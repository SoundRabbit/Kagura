use crate::dom;
use crate::dom::component::Composed;
use crate::native;
use crate::task;
use std::cell::RefCell;
use std::rc::Rc;
thread_local!(static APP: RefCell<Option<App>> = RefCell::new(None));

struct App {
    root_component: Rc<RefCell<Box<dyn Composed>>>,
    dom_renderer: dom::Renderer,
}

pub fn init(root_component: Rc<RefCell<Box<dyn Composed>>>, id: &str) {
    let node = root_component.borrow_mut().render(true);
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
            let node = app.root_component.borrow_mut().render(false);
            app.dom_renderer.update(node);
        }
    });
    task::dispatch();
}
