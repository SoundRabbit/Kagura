use super::VNode;
use kagura::node::Msg;
use std::cell::Cell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use wasm_bindgen::JsCast;

pub struct VElement {
    pub tag_name: Rc<String>,
    pub attributes: VAttributes,
    pub events: VEvents,
    pub children: VecDeque<VNode>,
    pub index_id: Option<String>,
    pub namespace: Option<String>,
}

pub type VAttributes = HashMap<String, VAttributeValues>;

#[derive(Debug)]
pub struct VEvents {
    pub events: HashMap<String, VEventHandlers>,
    pub refers: Vec<VReferHandler>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct VAttributeValues {
    pub values: VecDeque<VAttributeValue>,
    pub delimiter: String,
}

#[derive(Clone, PartialEq, Debug)]
pub enum VAttributeValue {
    Str(Rc<String>),
    Nut(u64),
    Int(i64),
    Num(f64),
    None,
}

pub struct VEventHandlers {
    pub bubbles: Vec<VEventHandler>,
    pub captures: Vec<VEventHandler>,
}

pub type VEventHandler = Box<dyn FnOnce(VEvent<web_sys::Event>) -> Msg>;

pub struct VReferHandler {
    pub target: usize,
    handler: Option<Box<dyn FnOnce(web_sys::Node) -> Msg>>,
}

pub struct VEvent<T> {
    data: T,
    stop_propagation: Rc<Cell<bool>>,
}

impl VElement {
    pub fn as_rendered(&self) -> Self {
        Self {
            tag_name: Rc::clone(&self.tag_name),
            attributes: self.attributes.clone(),
            events: self.events.as_rendered(),
            children: self
                .children
                .iter()
                .map(|v_node| v_node.as_rendered())
                .collect(),
            index_id: self.index_id.clone(),
            namespace: self.namespace.clone(),
        }
    }
}

impl std::fmt::Debug for VElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&self.tag_name)
            .field("children", &self.children)
            .finish()
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
            Self::None => String::from(""),
        }
    }
}

impl VEvents {
    pub fn new() -> Self {
        Self {
            events: HashMap::new(),
            refers: vec![],
        }
    }

    pub fn as_rendered(&self) -> Self {
        Self {
            events: HashMap::new(),
            refers: self
                .refers
                .iter()
                .map(|refer| refer.as_rendered())
                .collect(),
        }
    }
}

impl VEventHandlers {
    pub fn new() -> Self {
        Self {
            bubbles: vec![],
            captures: vec![],
        }
    }
}

impl std::fmt::Debug for VEventHandlers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VEventHandlers")
    }
}

impl<T> VEvent<T> {
    pub fn new(data: T, stop_propagation: Rc<Cell<bool>>) -> Self {
        Self {
            data,
            stop_propagation,
        }
    }

    pub fn stop_propagation(&self) {
        self.stop_propagation.set(true);
    }
}

impl<T: Clone> VEvent<T> {
    pub fn data(&self) -> T {
        self.data.clone()
    }
}

impl<T: JsCast> VEvent<T> {
    pub fn dyn_into<U: JsCast>(self) -> Result<VEvent<U>, Self> {
        match self.data.dyn_into::<U>() {
            Ok(data) => Ok(VEvent::new(data, self.stop_propagation)),
            Err(data) => Err(Self::new(data, self.stop_propagation)),
        }
    }
}

impl VReferHandler {
    pub fn new(target: usize, handler: Box<dyn FnOnce(web_sys::Node) -> Msg>) -> Self {
        Self {
            target,
            handler: Some(handler),
        }
    }

    pub fn take(&mut self) -> Option<Box<dyn FnOnce(web_sys::Node) -> Msg>> {
        self.handler.take()
    }

    pub fn as_rendered(&self) -> Self {
        Self {
            target: self.target,
            handler: None,
        }
    }
}

impl std::fmt::Debug for VReferHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VReferHandler")
    }
}

impl<T> std::ops::Deref for VEvent<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> std::ops::DerefMut for VEvent<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
