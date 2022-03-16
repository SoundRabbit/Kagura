use crate::Html;
use crate::HtmlNode;
use std::any::Any;

pub trait HtmlPrefab {
    fn as_any(&self) -> &dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
    fn into_node(self: Box<Self>) -> Box<dyn HtmlNode>;
}

use crate::html_node::BasicHtmlNode;
use kagura::component::{Render, Update};

pub struct BasicHtmlPrefab<C: Update + Render<Html> + 'static> {
    constructor: Box<dyn FnOnce(C::Props) -> C>,
    children: C::Children,
    props: C::Props,
    index_id: Option<String>,
}

impl<C: Update + Render<Html>> BasicHtmlPrefab<C> {
    pub fn new(
        constructor: impl FnOnce(C::Props) -> C + 'static,
        index_id: Option<String>,
        props: C::Props,
        children: C::Children,
    ) -> Self {
        Self {
            constructor: Box::new(constructor),
            index_id,
            props,
            children,
        }
    }

    pub fn index_id_is(&self, index_id: &Option<String>) -> bool {
        self.index_id == *index_id
    }

    pub fn into_data(self) -> (C::Props, Option<String>, C::Children) {
        (self.props, self.index_id, self.children)
    }
}

impl<C: Update + Render<Html>> HtmlPrefab for BasicHtmlPrefab<C> {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        Box::new(*self)
    }

    fn into_node(self: Box<Self>) -> Box<dyn HtmlNode> {
        let state = (self.constructor)(self.props);
        let index_id = self.index_id;
        Box::new(BasicHtmlNode::new(index_id, Box::pin(state)))
    }
}
