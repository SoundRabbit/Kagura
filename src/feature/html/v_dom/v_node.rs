use super::VElement;
use super::VText;

pub enum VNode {
    Text(VText),
    Element(VElement),
}

pub struct AnnotVNode {
    data: VNode,
    r_node: web_sys::Node,
}

impl std::ops::Deref for AnnotVNode {
    type Target = VNode;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl std::ops::DerefMut for AnnotVNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
