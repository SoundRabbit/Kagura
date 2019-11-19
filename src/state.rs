use crate::dom;
use crate::basic_component::BasicComponent;
use crate::dom::component::DomComponent;
use crate::dom::component::Component;
use crate::native;
use crate::task;
use std::cell::RefCell;
use wasm_bindgen::JsValue;

thread_local!(static APP: RefCell<Option<App>> = RefCell::new(None));

struct App {
    root_component: Box<dyn DomComponent>,
    dom_renderer: dom::Renderer,
}

pub fn init<M, S, B>(mut root_component: Component<M, S, B>, id: &str)
where
    M: 'static,
    S: 'static,
    B: 'static,
{
    let node = root_component.render();
    let root = native::get_element_by_id(id);
    let dom_renderer = dom::Renderer::new(node, root.into());
    let root_component: Box<dyn DomComponent> = Box::new(root_component);
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
            let node = app.root_component.render();
            app.dom_renderer.update(node);
        }
    });
    web_sys::console::log_1(&JsValue::from("03"));
    task::dispatch();
}
