use kagura::node::Msg;
use std::cell::Cell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

pub struct VElement {
    tag_name: String,
    attributes: HashMap<String, VAttributeValues>,
}

pub struct VAttributeValues {
    values: VecDeque<VAttributeValue>,
    delimiter: String,
}

pub enum VAttributeValue {
    Str(Rc<String>),
    Nut(u64),
    Int(i64),
    Num(f64),
}

pub type VEventListener = Box<dyn FnOnce(VEvent) -> Msg>;

pub struct VEvent {
    data: web_sys::Event,
    stop_propagation: Rc<Cell<bool>>,
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
