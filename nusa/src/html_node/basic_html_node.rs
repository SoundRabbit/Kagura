use super::HtmlNode;
use crate::html_prefab::BasicHtmlPrefab;
use crate::HtmlRenderer;
use crate::{Html, HtmlPrefab, VNode};
use kagura::component::{Render, Update};
use kagura::node::{
    BasicComponentState, BasicNodeMsg, FutureMsg, Msg, RenderNode, SubHandler, UpdateNode,
};
use std::collections::VecDeque;
use std::pin::Pin;

pub struct BasicHtmlNode<This: Render<Html> + Update + 'static> {
    state: BasicComponentState<This>,
    html_renderer: HtmlRenderer<This>,
    index_id: Option<String>,
}

impl<This: Render<Html> + Update> BasicHtmlNode<This> {
    pub fn new(
        index_id: Option<String>,
        sub_handler: Option<SubHandler<This>>,
        state: Pin<Box<This>>,
    ) -> Self {
        Self {
            state: BasicComponentState::new(state, sub_handler),
            html_renderer: HtmlRenderer::new(),
            index_id,
        }
    }
}

impl<This: Render<Html> + Update> UpdateNode for BasicHtmlNode<This> {
    fn update(&mut self, mut msg: Msg) -> VecDeque<FutureMsg> {
        if msg.target() == self.state.target_id() {
            if let Some(msg) = msg
                .take()
                .and_then(|msg| msg.downcast::<BasicNodeMsg<This>>().ok())
            {
                self.state.update(*msg)
            } else {
                VecDeque::new()
            }
        } else {
            self.html_renderer.update(msg)
        }
    }
}

impl<This: Render<Html> + Update> RenderNode<(VecDeque<VNode>, VecDeque<FutureMsg>)>
    for BasicHtmlNode<This>
{
    fn render(&mut self) -> (VecDeque<VNode>, VecDeque<FutureMsg>) {
        self.html_renderer.render(&self.state)
    }
}

impl<This: Render<Html> + Update> HtmlNode for BasicHtmlNode<This> {
    fn is(&self, prefab: &dyn HtmlPrefab) -> bool {
        compare_node_and_prefab::<This>(
            &self.index_id,
            prefab.component_type_id(),
            prefab.index_id(),
        )
    }

    fn on_assemble(&mut self) -> VecDeque<FutureMsg> {
        self.state.on_assemble()
    }

    fn update_by_prefab(&mut self, prefab: Box<dyn HtmlPrefab>) -> VecDeque<FutureMsg> {
        if self.is(prefab.as_ref()) {
            if let Ok(prefab) = prefab.into_any().downcast::<BasicHtmlPrefab<This>>() {
                let (props, index_id, sub_handler, children) = prefab.into_data();
                self.index_id = index_id;
                self.html_renderer.set_children(children);
                self.state.set_sub_handler(sub_handler);
                return self.state.on_load(props);
            }
        }
        VecDeque::new()
    }
}

pub fn compare_node_and_prefab<This: Update + Render<Html> + 'static>(
    this_index_id: &Option<String>,
    component_type_id: std::any::TypeId,
    index_id: &Option<String>,
) -> bool {
    if std::any::TypeId::of::<This>() == component_type_id {
        return *this_index_id == *index_id;
    } else {
        false
    }
}
