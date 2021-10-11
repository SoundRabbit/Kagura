pub mod component;
pub mod html;
pub mod renderer;

use crate::event;
use std::collections::HashMap;
use std::rc::Rc;

pub use renderer::Renderer;

pub enum Node {
    Element(Element),
    Text(String),
}

pub struct Element {
    pub tag_name: String,
    pub attributes: Attributes,
    pub events: Events,
    pub children: Vec<Node>,
    pub need_rerendering: bool,
}

#[derive(Clone)]
pub struct Attributes {
    pub attributes: HashMap<String, Vec<Value>>,
    pub delimiters: HashMap<String, String>,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Value {
    Str(Rc<String>),
    Nut(u64),
    Int(i64),
}

pub struct Events {
    pub handlers: HashMap<String, Vec<Event>>,
    pub rendered: Option<Box<dyn FnOnce(web_sys::Element)>>,
}

pub enum Event {
    Handler(Box<dyn FnOnce(web_sys::Event)>),
    HandlerId(event::HandlerId),
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
            attr.push(value);
        } else {
            self.attributes.insert(name, vec![value]);
        }
    }

    /// add empty attribute
    pub fn set(&mut self, name: impl Into<String>) {
        let name: String = name.into();
        if self.attributes.get(&name).is_none() {
            self.attributes.insert(name, vec![]);
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

impl Value {
    fn as_rc_string(&self) -> Rc<String> {
        match &self {
            Value::Int(v) => Rc::new(v.to_string()),
            Value::Nut(v) => Rc::new(v.to_string()),
            Value::Str(v) => Rc::clone(v),
        }
    }
}

impl Events {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
            rendered: None,
        }
    }

    pub fn add(&mut self, name: impl Into<String>, handler: impl FnOnce(web_sys::Event) + 'static) {
        let name = name.into();
        if let Some(handlers) = self.handlers.get_mut(&name) {
            handlers.push(Event::Handler(Box::new(handler)));
        } else {
            self.handlers
                .insert(name, vec![Event::Handler(Box::new(handler))]);
        }
    }
}

impl Event {
    pub fn is_handler(&self) -> bool {
        match self {
            Self::Handler(..) => true,
            _ => false,
        }
    }

    pub fn take_with_id(
        &mut self,
        handler_id: event::HandlerId,
    ) -> Option<Box<dyn FnOnce(web_sys::Event)>> {
        let mut handler = Event::HandlerId(handler_id);
        std::mem::swap(self, &mut handler);
        match handler {
            Self::Handler(handler) => Some(handler),
            _ => None,
        }
    }
}
