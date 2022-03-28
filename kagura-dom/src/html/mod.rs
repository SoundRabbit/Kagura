use crate::HtmlPrefab;

mod html_element;
mod html_text;

pub use html_element::HtmlElement;
pub use html_text::HtmlText;

pub enum Html {
    Component(Box<dyn HtmlPrefab>),
    HtmlElement(HtmlElement),
    HtmlText(HtmlText),
    Fragment(Vec<Html>),
    None,
}
