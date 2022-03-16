use crate::HtmlPrefab;
use crate::VNode;
use kagura::node::RenderNode;
use std::any::Any;
use std::collections::VecDeque;

pub trait HtmlNode: RenderNode<VecDeque<VNode>> {
    fn is(&self, prefab: &dyn HtmlPrefab) -> bool;
    fn update_by_prefab(&mut self, prefab: Box<dyn HtmlPrefab>);
}

use crate::html_prefab::BasicHtmlPrefab;
use crate::Html;
use crate::HtmlRenderer;
use kagura::component::{Render, Update};
use kagura::node::BasicComponentState;
use std::pin::Pin;

pub struct BasicHtmlNode<C: Render<Html> + Update + 'static> {
    state: BasicComponentState<C>,
    html_renderer: HtmlRenderer,
    index_id: Option<String>,
}

impl<C: Render<Html> + Update> BasicHtmlNode<C> {
    pub fn new(index_id: Option<String>, state: Pin<Box<C>>) -> Self {
        Self {
            state: BasicComponentState::new(state),
            html_renderer: HtmlRenderer::new(),
            index_id,
        }
    }
}

impl<C: Render<Html> + Update> RenderNode<VecDeque<VNode>> for BasicHtmlNode<C> {
    fn render(&mut self) -> VecDeque<VNode> {
        self.html_renderer.render(&self.state)
    }
}

impl<C: Render<Html> + Update> HtmlNode for BasicHtmlNode<C> {
    fn is(&self, prefab: &dyn HtmlPrefab) -> bool {
        if let Some(prefab) = prefab.as_any().downcast_ref::<BasicHtmlPrefab<C>>() {
            return prefab.index_id_is(&self.index_id);
        } else {
            false
        }
    }

    fn update_by_prefab(&mut self, prefab: Box<dyn HtmlPrefab>) {
        if let Ok(prefab) = prefab.into_any().downcast::<BasicHtmlPrefab<C>>() {
            let (props, index_id, children) = prefab.into_data();
        }
    }
}
