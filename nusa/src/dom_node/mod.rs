use kagura::node::{FutureMsg, RenderNode};
use std::collections::VecDeque;

pub mod basic_dom_node;
pub use basic_dom_node::BasicDomNode;

pub trait DomNode: RenderNode<VecDeque<FutureMsg>> {}
