use crate::Html;
use crate::VNode;
use kagura::component::Render;
use kagura::node::FutureMsg;
use std::collections::VecDeque;
use std::pin::Pin;

pub struct DomRenderer {}

impl DomRenderer {
    pub fn render<C: Render<Html>>(&mut self, state: &Pin<Box<C>>) -> VecDeque<FutureMsg> {
        VecDeque::new()
    }
}
