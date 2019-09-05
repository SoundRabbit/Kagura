pub mod dom {
    extern crate wasm_bindgen;
    extern crate web_sys;

    use std::collections::HashMap;
    use std::collections::HashSet;
    use wasm_bindgen::prelude::*;

    pub fn render(before: &Node, after: Node, root: &mut web_sys::Element) -> Result<Node, JsValue>{
        match &after {
            Node::Element {tag_name, attributes, children} =>
                for node in children {
                }
            Node::Text(text) => ()
        }
        Ok(after)
    }

    pub enum Node {
        Element {
            tag_name: String,
            attributes: Attributes,
            children: Vec<Node>,
        },
        Text(String),
    }

    pub struct Attributes {
        class: HashSet<String>,
        id: HashSet<String>,
        attributes: HashMap<String, String>,
    }

    impl Attributes {
        pub fn new() -> Attributes {
            Attributes {
                class: HashSet::new(),
                id: HashSet::new(),
                attributes: HashMap::new(),
            }
        }

        pub fn with_class(mut self, class_name: impl Into<String>) -> Self {
            self.class.insert(class_name.into());
            self
        }

        pub fn with_id(mut self, id_name: impl Into<String>) -> Self {
            self.id.insert(id_name.into());
            self
        }

        pub fn with_attribute(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
            self.attributes.insert(name.into(), value.into());
            self
        }
    }

    pub struct Events {
        on_click: Box<FnOnce()->EventResult>,
    }

    pub struct EventResult {
        prevent_default: bool,
        stop_propagation: bool,
    }

}
