use super::DomNode;
use crate::DomEvents;
use crate::{DomRenderer, Html, HtmlRenderer};
use kagura::node::{Msg, NodeCmd, RenderNode, UpdateNode};
use kagura::FutureMsg;
use std::future;
use std::pin::Pin;

pub mod basic_dom_component;

pub use basic_dom_component::BasicDomComponent;

pub struct BasicDomNode {
    dummy_state: Pin<Box<BasicDomComponent>>,
    dom_renderer: DomRenderer,
    dom_events: DomEvents,
    html_renderer: HtmlRenderer<BasicDomComponent>,
    render: Box<dyn FnMut(&BasicDomComponent) -> Vec<Html>>,
    is_first_render: bool,
}

impl BasicDomNode {
    pub fn new(
        entry: web_sys::Node,
        render: impl FnMut(&BasicDomComponent) -> Vec<Html> + 'static,
    ) -> Self {
        let dummy_state = Box::pin(BasicDomComponent::new());
        let dom_renderer = DomRenderer::new(entry.clone());
        let dom_events = DomEvents::new(entry.into());
        let render = Box::new(render);

        Self {
            dummy_state,
            dom_renderer,
            dom_events,
            html_renderer: HtmlRenderer::new(),
            render,
            is_first_render: true,
        }
    }
}

impl UpdateNode for BasicDomNode {
    fn update(&mut self, msg: Msg) -> NodeCmd {
        self.html_renderer.update(msg)
    }
}

impl RenderNode<NodeCmd> for BasicDomNode {
    fn render(&mut self) -> NodeCmd {
        self.html_renderer
            .set_children((self.render)(&self.dummy_state.as_ref()));
        let (v_nodes, mut node_cmd) = self.html_renderer.render(&self.dummy_state);
        let event_listeners = self.dom_renderer.render(v_nodes);

        for rendered_handler in event_listeners.rendered_handlers {
            let msg = rendered_handler();
            node_cmd.push_back(FutureMsg::Task(Box::pin(future::ready(vec![msg]))));
        }

        self.dom_events.listen(event_listeners.event_listeners);

        if self.is_first_render {
            node_cmd.push_back(FutureMsg::Batch(Box::new(self.dom_events.batch())));
            node_cmd.set_as_busy();
        }

        node_cmd
    }
}

impl DomNode for BasicDomNode {}
