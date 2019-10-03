use web_sys;

pub fn create_text_node(data: &str) -> web_sys::Text {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_text_node(data)
}
