use kagura::node::NodeCmd;
use kagura::node::RenderNode;

pub mod basic_dom_node;
pub use basic_dom_node::BasicDomNode;

pub trait DomNode: RenderNode<NodeCmd> {}
