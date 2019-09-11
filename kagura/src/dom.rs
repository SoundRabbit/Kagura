extern crate wasm_bindgen;

use crate::native::Event;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone)]
pub enum Node {
    Element {
        tag_name: String,
        attributes: Attributes,
        events: Events,
        children: Vec<Node>,
        rerender: bool,
    },
    Text(String),
}

#[derive(Clone)]
pub struct Attributes {
    pub attributes: HashMap<String, HashSet<Value>>,
    pub delimiters: HashMap<String, String>,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Value {
    Str(String),
    Nut(u64),
    Int(i64),
}

pub struct Events {
    handlers: HashMap<String, Box<FnMut(Event)>>,
}

impl Node {
    pub fn element(
        tag_name: impl Into<String>,
        attributes: Attributes,
        events: Events,
        children: Vec<Node>,
        rerender: bool,
    ) -> Self {
        Node::Element {
            tag_name: tag_name.into(),
            attributes,
            events,
            children,
            rerender,
        }
    }

    pub fn text(t: impl Into<String>) -> Self {
        Node::Text(t.into())
    }
}

impl Attributes {
    pub fn new() -> Self {
        (Self {
            attributes: HashMap::new(),
            delimiters: HashMap::new(),
        })
        .delimiter("style", ";")
        .delimiter("class", " ")
        .delimiter("id", " ")
    }

    pub fn attribute(mut self, name: impl Into<String>, value: Value) -> Self {
        self.add(name, value);
        self
    }

    pub fn flag(mut self, name: impl Into<String>) -> Self{
        self.set(name);
        self
    }

    pub fn delimiter(mut self, name: impl Into<String>, dlm: impl Into<String>) -> Self {
        self.delimit(name, dlm);
        self
    }


    pub fn add(&mut self, name: impl Into<String>, value: Value) {
        let name: String = name.into();
        if let Some(attr) = self.attributes.get_mut(&name) {
            attr.insert(value);
        } else {
            let mut attr: HashSet<Value> = HashSet::new();
            attr.insert(value);
            self.attributes.insert(name, attr);
        }
    }

    pub fn set(&mut self, name: impl Into<String>) {
        let name: String = name.into();
        if let None = self.attributes.get(&name) {
            self.attributes.insert(name, HashSet::new());
        }
    }

    pub fn delimit(&mut self, name: impl Into<String>, dlm: impl Into<String>) {
        self.delimiters.insert(name.into(), dlm.into());
    }
}

impl Events {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: impl Into<String>, handler: impl FnMut(Event) + 'static) {
        self.handlers.insert(name.into(), Box::new(handler));
    }
}

impl Clone for Events {
    fn clone(&self) -> Self {
        Self::new()
    }
}
