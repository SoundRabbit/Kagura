use crate::env::IdType;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::rc::Rc;

pub enum Node {
    Element(ElementNode),
    Text(TextNode),
}

pub struct ElementNode {
    pub tag_name: String,
    pub attributes: Attributes,
    pub events: Events,
    pub children: VecDeque<Node>,
    pub ref_marker: Vec<Box<dyn FnOnce(web_sys::Node)>>,
}

pub struct TextNode {
    pub text: String,
    pub events: Events,
}

#[derive(Clone, Eq, PartialEq)]
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
    pub handler_table: HashMap<String, Vec<Event>>,
}

pub enum Event {
    Handler(Box<dyn FnOnce(web_sys::Event)>),
    HandlerId(IdType),
}

impl Node {
    pub fn element(
        tag_name: impl Into<String>,
        attributes: Attributes,
        events: Events,
        children: VecDeque<Node>,
        ref_marker: Vec<Box<dyn FnOnce(web_sys::Node)>>,
    ) -> Self {
        Node::Element(ElementNode {
            tag_name: tag_name.into(),
            attributes,
            events,
            children,
            ref_marker,
        })
    }

    pub fn text(t: impl Into<String>, events: Events) -> Self {
        Node::Text(TextNode {
            text: t.into(),
            events,
        })
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

impl Value {
    pub fn as_rc_string(&self) -> Rc<String> {
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
            handler_table: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: impl Into<String>, handler: impl FnOnce(web_sys::Event) + 'static) {
        let name = name.into();
        if let Some(handlers) = self.handler_table.get_mut(&name) {
            handlers.push(Event::Handler(Box::new(handler)));
        } else {
            self.handler_table
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

    pub fn take_with_id(&mut self, handler_id: IdType) -> Option<Box<dyn FnOnce(web_sys::Event)>> {
        let mut handler = Event::HandlerId(handler_id);
        std::mem::swap(self, &mut handler);
        match handler {
            Self::Handler(handler) => Some(handler),
            _ => None,
        }
    }
}
