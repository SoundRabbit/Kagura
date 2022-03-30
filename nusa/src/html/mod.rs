use crate::HtmlPrefab;

pub mod html_element;
pub mod html_text;

pub use html_element::HtmlElement;
pub use html_text::HtmlText;

pub enum Html {
    Component(Box<dyn HtmlPrefab>),
    HtmlElement(HtmlElement),
    HtmlText(HtmlText),
    Fragment(Vec<Html>),
    None,
}

impl Html {
    pub fn text(text: impl Into<String>) -> Self {
        Html::HtmlText(HtmlText { text: text.into() })
    }
}
