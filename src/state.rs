use crate::basic_component::BasicComponent;
use crate::dom;
use crate::dom::component::Component;
use crate::dom::component::DomComponent;
use crate::native;
use crate::task;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;

thread_local!(static APP: RefCell<Option<App>> = RefCell::new(None));

struct App {
    root_component: Rc<RefCell<Box<dyn DomComponent>>>,
    dom_renderer: dom::Renderer,
}

pub fn init<M, S, B>(mut root_component: Component<M, S, B>, id: &str)
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
    web_sys::console::log_1(&JsValue::from("00"));
    APP.with(|app| {
        *app.borrow_mut() = Some(App {
            root_component,
            dom_renderer,
        })
    });
    web_sys::console::log_1(&JsValue::from("01"));
}

pub fn render() {
    web_sys::console::log_1(&JsValue::from("02"));
    APP.with(|app| {
        if let Some(app) = &mut (*app.borrow_mut()) {
            let node = app.root_component.borrow_mut().render();
            web_sys::console::log_1(&JsValue::from("02-2"));
            app.dom_renderer.update(node);
        }
    });
    web_sys::console::log_1(&JsValue::from("03"));
    task::dispatch();
}
