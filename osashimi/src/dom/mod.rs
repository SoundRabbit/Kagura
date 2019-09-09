extern crate wasm_bindgen;
extern crate web_sys;

use rand::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

pub mod native;

pub enum Node {
    Element {
        tag_name: String,
        attributes: Attributes,
        events: Events,
        children: Vec<Node>,
    },
    Text(String),
}

pub struct Attributes {
    class: HashSet<String>,
    id: HashSet<String>,
    attributes: HashMap<String, String>,
}

pub struct Events {
    on_click: Option<Closure<FnMut()>>,
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

impl Events {
    pub fn new() -> Events {
        Events { on_click: None }
    }
    pub fn with_on_click(mut self, handler: Closure<FnMut()>) -> Self {
        native::console_log("set on_click");
        self.on_click = Some(handler);
        self
    }
}
