use wasm_bindgen::prelude::*;
use web_sys;

pub fn create_text_node(data: &str) -> web_sys::Text {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_text_node(data)
}

pub fn create_element(local_name: &str) -> web_sys::Element {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element(local_name)
        .unwrap()
}

pub fn get_element_by_id(element_id: &str) -> web_sys::Element {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(element_id)
        .unwrap()
}

pub fn error<T>(v: T)
where
    JsValue: From<T>,
{
    web_sys::console::log_1(&JsValue::from(v));
}
