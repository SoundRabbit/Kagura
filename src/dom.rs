pub mod html;
pub mod renderer;

use crate::native::Event;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, PartialEq)]
pub enum Node {
    Element(Element),
    Text(String),
}

#[derive(Clone)]
pub struct Element {
    pub tag_name: String,
    pub attributes: Attributes,
    pub events: Events,
    pub children: Vec<Node>,
    pub need_rerendering: bool,
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
    pub handlers: HashMap<String, Box<FnMut(Event)>>,
}

impl Node {
    pub fn element(
        tag_name: impl Into<String>,
        attributes: Attributes,
        events: Events,
        children: Vec<Node>,
        need_rerendering: bool,
    ) -> Self {
        Node::Element(Element {
            tag_name: tag_name.into(),
            attributes,
            events,
            children,
            need_rerendering,
        })
    }

    pub fn text(t: impl Into<String>) -> Self {
        Node::Text(t.into())
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.attributes == other.attributes && self.tag_name == other.tag_name
    }
}

impl Attributes {
    pub fn new() -> Self {
        let mut attr = Self {
            attributes: HashMap::new(),
            delimiters: HashMap::new(),
        };
        attr.delimit("style", ";");
        attr.delimit("class", " ");
        attr.delimit("id", " ");
        attr
    }

    /// add attribute with name-value pair
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

    /// add empty attribute
    pub fn set(&mut self, name: impl Into<String>) {
        let name: String = name.into();
        if self.attributes.get(&name).is_none() {
            self.attributes.insert(name, HashSet::new());
        }
    }

    /// set delimiter
    pub fn delimit(&mut self, name: impl Into<String>, dlm: impl Into<String>) {
        self.delimiters.insert(name.into(), dlm.into());
    }
}

impl PartialEq for Attributes {
    fn eq(&self, other: &Self) -> bool {
        self.attributes.iter().fold(true, |r, (k, v)| {
            if let Some(a) = other.attributes.get(k) {
                r && (a == v)
            } else {
                false
            }
        }) && other.attributes.iter().fold(true, |r, (k, v)| {
            if let Some(a) = self.attributes.get(k) {
                r && (a == v)
            } else {
                false
            }
        })
    }
}

impl Into<String> for &Value {
    fn into(self) -> String {
        match self {
            Value::Int(v) => v.to_string(),
            Value::Nut(v) => v.to_string(),
            Value::Str(v) => v.clone(),
        }
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

impl PartialEq for Events {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Clone for Events {
    fn clone(&self) -> Self {
        Self::new()
    }
}
