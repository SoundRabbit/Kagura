use super::Html;
use crate::v_node::v_element::{VAttributes, VEvents};

pub mod attributes;
pub mod events;

pub use attributes::Attributes;
pub use events::Events;

pub struct HtmlElement {
    pub tag_name: String,
    pub namespace_name: Option<String>,
    pub children: Vec<Html>,
    pub attributes: VAttributes,
    pub events: VEvents,
    pub index_id: Option<String>,
}
