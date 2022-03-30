use super::DomNode;
use crate::DomEvents;
use crate::{DomRenderer, Html, HtmlRenderer};
use kagura::node::{FutureMsg, Msg, RenderNode, UpdateNode};
use std::collections::VecDeque;
use std::pin::Pin;

pub mod basic_dom_component;

pub use basic_dom_component::BasicDomComponent;

pub struct BasicDomNode {
    dummy_state: Pin<Box<BasicDomComponent>>,
    dom_renderer: DomRenderer,
    dom_events: DomEvents,
    html_renderer: HtmlRenderer<BasicDomComponent>,
    render: Box<dyn FnMut(&BasicDomComponent) -> Vec<Html>>,
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
        }
    }
}

impl UpdateNode for BasicDomNode {
    fn update(&mut self, msg: Msg) -> VecDeque<FutureMsg> {
        self.html_renderer.update(msg)
    }
}

impl RenderNode<VecDeque<FutureMsg>> for BasicDomNode {
    fn render(&mut self) -> VecDeque<FutureMsg> {
        self.html_renderer
            .set_children((self.render)(&self.dummy_state.as_ref()));
        let v_nodes = self.html_renderer.render(&self.dummy_state);
        let event_listeners = self.dom_renderer.render(v_nodes);

        let mut tasks = VecDeque::new();
        tasks.push_back(self.dom_events.listen(event_listeners));

        tasks
    }
}

impl DomNode for BasicDomNode {}
