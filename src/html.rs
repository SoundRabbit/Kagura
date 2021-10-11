use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub mod attributes;
pub mod component;
pub mod component_node;
pub mod events;

pub use component::Component;

use component::AssembledChildComponent;

use crate::document::node;

pub enum Html<DemirootComp: Component> {
    ComponentNode(ComponentNode<DemirootComp>),
    TextNode {
        text: String,
        events: Events<DemirootComp::Msg>,
    },
    ElementNode {
        tag_name: String,
        children: Vec<Self>,
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
    },
    Fragment(Vec<Self>),
}

/// Attributes for Html<Msg>
#[derive(Clone)]
pub struct Attributes {
    attributes: node::Attributes,
}

/// Events for Html<Msg>
pub struct Events<Msg> {
    handler_table: HashMap<String, Vec<EventHandler<Msg>>>,
}

pub enum EventHandler<Msg> {
    Unrwapped(Box<dyn FnOnce(web_sys::Event) -> Msg>),
    Wrapped(Box<dyn FnOnce(web_sys::Event)>),
}

pub enum ComponentNode<DemirootComp: Component> {
    PackedComponentNode(Box<dyn PackedComponentNode<DemirootComp = DemirootComp>>),
    WrappedPackedComponentNode(Box<dyn WrappedPackedComponentNode>),
    AssembledComponentNode(AssembledComponentNode<DemirootComp>),
    WrappedAssembledComponentNode(Box<dyn WrappedAssembledComponentNode>),
}

pub trait PackedComponentNode {
    type DemirootComp: Component;

    fn wrap(&mut self) -> Box<dyn WrappedPackedComponentNode>;

    fn assemble(
        &mut self,
        before: Option<Rc<RefCell<dyn AssembledChildComponent<DemirootComp = Self::DemirootComp>>>>,
    ) -> AssembledComponentNode<Self::DemirootComp>;
}

pub struct PackedComponentNodeInstance<ThisComp: Component, DemirootComp: Component> {
    data: Option<PackedComponentNodeInstanceData<ThisComp, DemirootComp>>,
}

struct PackedComponentNodeInstanceData<ThisComp: Component, DemirootComp: Component> {
    constructor: fn(&ThisComp::Props) -> ThisComp,
    props: ThisComp::Props,
    sub_mapper: component::Sub<ThisComp::Sub, DemirootComp::Msg>,
    children: Vec<Html<DemirootComp>>,
}

pub trait WrappedPackedComponentNode {}

pub struct WrappedPackedComponentNodeInstance<SuperDemirootComp: Component> {
    data: Box<dyn PackedComponentNode<DemirootComp = SuperDemirootComp>>,
}

pub struct AssembledComponentNode<DemirootComp: Component> {
    data: Rc<RefCell<dyn AssembledChildComponent<DemirootComp = DemirootComp>>>,
    children: Vec<Html<DemirootComp>>,
}

pub trait WrappedAssembledComponentNode {}

pub struct WrappedAssembledComponentNodeInstance<SuperDemirootComp: Component> {
    data: Option<AssembledComponentNode<SuperDemirootComp>>,
}

impl<DemirootComp: Component> std::fmt::Debug for Html<DemirootComp> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ComponentNode(..) => {
                write!(f, "[ComponentNode]")
            }
            Self::TextNode { text, .. } => write!(f, "[text]\n{}", text),
            Self::ElementNode {
                tag_name, children, ..
            } => write!(f, "[element: {}]\n{:?}", tag_name, children),
            Self::Fragment(children) => write!(f, "[Fragment]\n{:?}", children),
        }
    }
}

impl<DemirootComp: Component> Html<DemirootComp> {
    pub fn none() -> Self {
        Self::Fragment(vec![])
    }
}
