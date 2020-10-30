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
    let mut node = root_component.borrow_mut().render(true);
    let root = native::get_element_by_id(id);
    if node.len() != 1 {
        native::error(format!("Kagura needs only a node in root of DOM tree. But, Kagura finds {} nodes in root of DOM tree", node.len()));
    }
    let dom_renderer = dom::Renderer::new(node.remove(0), root.into());
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
            let mut node = app.root_component.borrow_mut().render(true);
            if node.len() != 1 {
                native::error(format!("Kagura needs only a node in root of DOM tree. But, Kagura finds {} nodes in root of DOM tree", node.len()));
            }
            app.dom_renderer.update(node.remove(0));
        }
    });
    task::dispatch();
}
