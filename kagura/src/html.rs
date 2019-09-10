pub mod attributes;
pub mod events;

pub use attributes::Attributes;
pub use events::Events;

use crate::component::Composable;
use crate::component::Component;

pub enum Html<Msg> {
    Composable(Box<Composable>),
    TextNode(String),
    ElementNode {
        tag_name: String,
        children: Vec<Html<Msg>>,
        attributes: Attributes,
        events: Events<Msg>,
    },
}

impl<Msg> Html<Msg> {
    pub fn component<M, S, B>(component: Component<M, S, B>) -> Self
    where
        M: 'static,
        S: 'static,
        B: 'static,
    {
        Html::Composable(Box::new(component))
    }
    pub fn unsafe_text(text: impl Into<String>) -> Self {
        Html::TextNode(text.into())
    }
    pub fn node(
        tag_name: impl Into<String>,
        attributes: Attributes,
        events: Events<Msg>,
        children: Vec<Html<Msg>>,
    ) -> Self {
        Html::ElementNode {
            tag_name: tag_name.into(),
            children,
            attributes,
            events,
        }
    }
    pub fn a(
        attributes: Attributes,
        events: Events<Msg>,
        children: Vec<Html<Msg>>,
    ) -> Self {
        Html::node("a", attributes, events, children)
    }
    pub fn button(
        attributes: Attributes,
        events: Events<Msg>,
        children: Vec<Html<Msg>>,
    ) -> Self {
        Html::node("button", attributes, events, children)
    }
    pub fn div(
        attributes: Attributes,
        events: Events<Msg>,
        children: Vec<Html<Msg>>,
    ) -> Self {
        Html::node("div", attributes, events, children)
    }
    pub fn h1(
        attributes: Attributes,
        events: Events<Msg>,
        children: Vec<Html<Msg>>,
    ) -> Self {
        Html::node("h1", attributes, events, children)
    }
    pub fn h2(
        attributes: Attributes,
        events: Events<Msg>,
        children: Vec<Html<Msg>>,
    ) -> Self {
        Html::node("h2", attributes, events, children)
    }
    pub fn h3(
        attributes: Attributes,
        events: Events<Msg>,
        children: Vec<Html<Msg>>,
    ) -> Self {
        Html::node("h3", attributes, events, children)
    }
    pub fn h4(
        attributes: Attributes,
        events: Events<Msg>,
        children: Vec<Html<Msg>>,
    ) -> Self {
        Html::node("h4", attributes, events, children)
    }
    pub fn h5(
        attributes: Attributes,
        events: Events<Msg>,
        children: Vec<Html<Msg>>,
    ) -> Self {
        Html::node("h5", attributes, events, children)
    }
    pub fn h6(
        attributes: Attributes,
        events: Events<Msg>,
        children: Vec<Html<Msg>>,
    ) -> Self {
        Html::node("h6", attributes, events, children)
    }
    pub fn span(
        attributes: Attributes,
        events: Events<Msg>,
        children: Vec<Html<Msg>>,
    ) -> Self {
        Html::node("span", attributes, events, children)
    }
}