pub mod component {

    trait Component {
        type Msg;
        type Propagation;
        fn update(&mut self, msg: Self::Msg) -> Self::Propagation;
        fn view(&self) -> super::dom::Node;
    }

}

pub mod dom {
    extern crate wasm_bindgen;

    use std::collections::HashMap;
    use std::collections::HashSet;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern {
        fn get_element_by_id(s: &str);
    }

    pub struct Render<'a> {
        rootId: &'a str,
        rootNode: Node<'a>,
    }

    impl<'a> Render<'a> {
        pub fn new(rootId: &'a str) -> Render<'a> {
            Render {
                rootId: rootId,
                rootNode: Node::Text(""),
            }
        }

        pub fn update(&mut self) {
            
        }

        pub fn render(node: &Node) {
            get_element_by_id("");
        }
    }

    pub enum Node<'a> {
        Element {
            tag_name: String,
            attributes: Attributes,
            children: Vec<Node<'a>>,
        },
        Text(&'a str),
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

        pub fn add_class(mut self, class: String) -> Self {
            self.class.insert(class);
            self
        }

        pub fn add_id(mut self, id: String) -> Self {
            self.id.insert(id);
            self
        }

        pub fn add_attribute(mut self, name: String, value: String) -> Self {
            self.attributes.insert(name, value);
            self
        }
    }

    pub struct Events<Msg> {
        on_click: Box<FnOnce()->Msg>,
    }

}
