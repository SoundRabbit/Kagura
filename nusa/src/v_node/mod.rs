pub mod v_element;
pub mod v_text;

pub use v_element::VElement;
pub use v_text::VText;

#[derive(Debug)]
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
