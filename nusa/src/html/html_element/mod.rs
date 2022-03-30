use super::Html;
use crate::v_node::v_element::{VAttributeValues, VEventHandler};
use std::collections::HashMap;

pub mod attributes;
pub mod events;

pub use attributes::Attributes;
pub use events::Events;

pub struct HtmlElement {
    pub tag_name: String,
    pub children: Vec<Html>,
    pub attributes: HashMap<String, VAttributeValues>,
    pub events: HashMap<String, Vec<VEventHandler>>,
    pub index_id: Option<String>,
}
