use super::Html;

pub mod attributes;
pub mod events;

pub use attributes::Attributes;
pub use events::Events;

pub struct HtmlElement {
    pub tag_name: String,
    pub children: Vec<Html>,
    pub attributes: Attributes,
    pub events: Events,
}
