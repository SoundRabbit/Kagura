use crate::HtmlNode;
use std::any::Any;

pub mod basic_html_prefab;

pub use basic_html_prefab::BasicHtmlPrefab;

pub trait HtmlPrefab {
    fn component_type_id(&self) -> std::any::TypeId;
    fn index_id(&self) -> &Option<String>;
    fn as_any(&self) -> &dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
    fn into_node(self: Box<Self>) -> Box<dyn HtmlNode>;
}
