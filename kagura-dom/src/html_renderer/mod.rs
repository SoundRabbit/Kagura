use crate::Html;
use crate::VNode;
use kagura::component::Render;
use std::collections::VecDeque;
use std::pin::Pin;

pub struct HtmlRenderer {}

impl HtmlRenderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render<C: Render<Html>>(&mut self, state: &Pin<Box<C>>) -> VecDeque<VNode> {
        VecDeque::new()
    }
}
