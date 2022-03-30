use super::VNode;
use kagura::node::Msg;
use std::cell::Cell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

pub struct VElement {
    pub tag_name: Rc<String>,
    pub attributes: VAttributes,
    pub events: VEvents,
    pub children: VecDeque<VNode>,
    pub index_id: Option<String>,
}

pub type VAttributes = HashMap<String, VAttributeValues>;
pub type VEvents = HashMap<String, Vec<VEventHandler>>;

#[derive(Clone, PartialEq)]
pub struct VAttributeValues {
    pub values: VecDeque<VAttributeValue>,
    pub delimiter: String,
}

#[derive(Clone, PartialEq)]
pub enum VAttributeValue {
    Str(Rc<String>),
    Nut(u64),
    Int(i64),
    Num(f64),
}

pub type VEventHandler = Box<dyn FnOnce(VEvent) -> Msg>;

pub struct VEvent {
    data: web_sys::Event,
    stop_propagation: Rc<Cell<bool>>,
}

impl VElement {
    pub fn as_rendered(&self) -> Self {
        Self {
            tag_name: Rc::clone(&self.tag_name),
            attributes: self.attributes.clone(),
            events: HashMap::new(),
            children: self
                .children
                .iter()
                .map(|v_node| v_node.as_rendered())
                .collect(),
            index_id: self.index_id.clone(),
        }
    }
}

impl std::string::ToString for VAttributeValues {
    fn to_string(&self) -> String {
        self.values
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(&self.delimiter)
    }
}

impl std::string::ToString for VAttributeValue {
    fn to_string(&self) -> String {
        match self {
            Self::Str(v) => v.to_string(),
            Self::Int(v) => v.to_string(),
            Self::Num(v) => v.to_string(),
            Self::Nut(v) => v.to_string(),
        }
    }
}

impl VEvent {
    pub fn new(data: web_sys::Event, stop_propagation: Rc<Cell<bool>>) -> Self {
        Self {
            data,
            stop_propagation,
        }
    }

    pub fn data(&self) -> web_sys::Event {
        self.data.clone()
    }

    pub fn stop_propagation(&self) {
        self.stop_propagation.set(true);
    }
}

impl std::ops::Deref for VEvent {
    type Target = web_sys::Event;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl std::ops::DerefMut for VEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
