use crate::dom;
use crate::dom::component::Controller as Composed;
use crate::dom::component::RcController as Component;
use crate::native;
use crate::task;
use std::cell::RefCell;
use std::rc::Rc;
thread_local!(static APP: RefCell<Option<App>> = RefCell::new(None));

struct App {
    root_component: Rc<RefCell<Box<dyn Composed>>>,
    dom_renderer: dom::Renderer,
}

pub fn init<Props: 'static, Sub: 'static>(root_component: Component<Props, Sub>, id: &str) {
    let root_component = Rc::new(RefCell::new(Box::new(root_component) as Box<dyn Composed>));
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
