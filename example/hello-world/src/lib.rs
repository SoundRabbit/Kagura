extern crate osashimi;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn f() {
    use osashimi::dom::*;
    let node = Node::Element {
        tag_name: "h1".to_string(),
        attributes: Attributes::new().with_attribute("style", "color: red"),
        children: vec![
            Node::Text("Hello Osashimi".to_string())
        ]
    };

    render(&node);
}