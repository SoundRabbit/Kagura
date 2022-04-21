use crate::{HtmlPrefab, VNode};
use kagura::node::{NodeCmd, RenderNode, UpdateNode};
use std::collections::VecDeque;

pub mod basic_html_node;

pub use basic_html_node::BasicHtmlNode;

pub trait HtmlNode: RenderNode<(VecDeque<VNode>, NodeCmd)> + UpdateNode {
    fn is(&self, prefab: &dyn HtmlPrefab) -> bool;
    fn on_assemble(&mut self) -> NodeCmd;
    fn update_by_prefab(&mut self, prefab: Box<dyn HtmlPrefab>) -> NodeCmd;
}
