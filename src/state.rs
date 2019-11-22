use crate::dom;
use crate::dom::component::Component;
use crate::dom::component::DomComponent;
use crate::native;
use crate::task;
use std::cell::RefCell;
use std::rc::Rc;
thread_local!(static APP: RefCell<Option<App>> = RefCell::new(None));

struct App {
    root_component: Rc<RefCell<Box<dyn DomComponent>>>,
    dom_renderer: dom::Renderer,
}

pub fn init<M, S, B>(root_component: Component<M, S, B>, id: &str)
where
    M: 'static,
    S: 'static,
    B: 'static,
{
    let root_component = Rc::new(RefCell::new(
        Box::new(root_component) as Box<dyn DomComponent>
    ));
    root_component
        .borrow_mut()
        .set_me(Rc::downgrade(&root_component));
    let node = root_component.borrow_mut().render();
    let root = native::get_element_by_id(id);
    let dom_renderer = dom::Renderer::new(node, root.into());
    APP.with(|app| {
        *app.borrow_mut() = Some(App {
            root_component,
            dom_renderer,
        })
    });
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
