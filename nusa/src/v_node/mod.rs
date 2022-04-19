pub mod v_element;
pub mod v_text;

pub use v_element::VElement;
pub use v_text::VText;

pub enum VNode {
    VElement(VElement),
    VText(VText),
    RNode(web_sys::Node),
}

impl VNode {
    pub fn as_rendered(&self) -> Self {
        match self {
            Self::VText(v_node) => Self::VText(v_node.clone()),
            Self::VElement(v_node) => Self::VElement(v_node.as_rendered()),
            Self::RNode(r_node) => Self::RNode(r_node.clone()),
        }
    }
}

impl std::fmt::Debug for VNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RNode(..) => write!(f, "[RNode]"),
            Self::VText(text) => text.fmt(f),
            Self::VElement(element) => element.fmt(f),
        }
    }
}
