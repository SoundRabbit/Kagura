pub mod dom {
    extern crate wasm_bindgen;
    extern crate web_sys;

    use std::collections::HashMap;
    use std::collections::HashSet;
    use wasm_bindgen::prelude::*;

    pub enum Rendered {
        Element(web_sys::Element),
        Text(String)
    }

    pub fn render(node: &Node) {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");
        let rendered = render_all(node, node, &document).expect("some error is occured in rendering");
        match &rendered {
            Rendered::Element(element) => {
                body.append_child(element);
            }
            _ => ()
        }
    }

    pub fn render_all(
        before: &Node,
        after: &Node,
        document: &web_sys::Document,
    ) -> Result<Rendered, JsValue> {
        match after {
            Node::Element {
                tag_name,
                attributes,
                children,
            } => {
                let el = document.create_element(tag_name)?;
                let class: Vec<&str> = attributes
                    .class
                    .iter()
                    .map(|class_name| -> &str { &class_name })
                    .collect();
                let id: Vec<&str> = attributes
                    .id
                    .iter()
                    .map(|class_name| -> &str { &class_name })
                    .collect();

                el.set_class_name(&class.join(" "));
                el.set_id(&id.join(" "));
                for attr in &attributes.attributes {
                    let (attr, value) = attr;
                    el.set_attribute(attr, value);
                    el.set_inner_html("");
                }

                for child in children {
                    match render_all(child, child, document)? {
                        Rendered::Element(element) => { el.append_child(&element); },
                        Rendered::Text(text) => { el.set_inner_html(&text); }
                    }
                }

                Ok(Rendered::Element(el))
            }
            Node::Text(text) => Ok(Rendered::Text(text.to_string())),
        }
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
        on_click: Box<FnOnce() -> EventResult>,
    }

    pub struct EventResult {
        prevent_default: bool,
        stop_propagation: bool,
    }

}
