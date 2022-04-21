use crate::v_node::{VElement, VText};
use crate::Html;
use crate::HtmlNode;
use crate::VNode;
use kagura::component::Render;
use kagura::node::{Msg, NodeCmd};
use std::collections::VecDeque;
use std::pin::Pin;
use std::rc::Rc;

enum RenderedNode {
    None,
    RNode(web_sys::Node),
    Text,
    Fragment(VecDeque<RenderedNode>),
    Element(VecDeque<RenderedNode>),
    Component(Box<dyn HtmlNode>),
}

pub struct HtmlRenderer<This: Render<Html>> {
    children: Option<This::Children>,
    rendered_node: RenderedNode,
}

impl<This: Render<Html>> HtmlRenderer<This> {
    pub fn new() -> Self {
        Self {
            children: None,
            rendered_node: RenderedNode::Fragment(VecDeque::new()),
        }
    }

    pub fn set_children(&mut self, children: This::Children) {
        self.children = Some(children);
    }

    pub fn update(&mut self, msg: Msg) -> NodeCmd {
        Self::update_rendered(&mut self.rendered_node, msg)
    }

    fn update_rendered(node: &mut RenderedNode, msg: Msg) -> NodeCmd {
        match node {
            RenderedNode::Component(node) => node.update(msg),
            RenderedNode::Element(children) => {
                let mut scedules = NodeCmd::new(VecDeque::new());
                for child in children {
                    let mut node_cmd = Self::update_rendered(child, msg.clone());
                    scedules.append(&mut node_cmd);
                }
                scedules
            }
            RenderedNode::Fragment(children) => {
                let mut scedules = NodeCmd::new(VecDeque::new());
                for child in children {
                    let mut node_cmd = Self::update_rendered(child, msg.clone());
                    scedules.append(&mut node_cmd);
                }
                scedules
            }
            RenderedNode::RNode(..) => NodeCmd::new(VecDeque::new()),
            RenderedNode::None => NodeCmd::new(VecDeque::new()),
            RenderedNode::Text => NodeCmd::new(VecDeque::new()),
        }
    }

    pub fn render(&mut self, state: &Pin<Box<This>>) -> (VecDeque<VNode>, NodeCmd) {
        let html = state
            .as_ref()
            .render(self.children.take().unwrap_or_default());

        let mut rendered_node = RenderedNode::None;
        std::mem::swap(&mut self.rendered_node, &mut rendered_node);

        let (rendered_node, v_nodes, node_cmd) = Self::render_html(rendered_node, html);
        self.rendered_node = rendered_node;
        (v_nodes, node_cmd)
    }

    fn render_html(
        rendered_node: RenderedNode,
        html: Html,
    ) -> (RenderedNode, VecDeque<VNode>, NodeCmd) {
        match html {
            Html::Fragment(htmls) => {
                let rendered_nodes = if let RenderedNode::Fragment(rendered_nodes) = rendered_node {
                    rendered_nodes
                } else {
                    VecDeque::new()
                };

                let rendered = Self::render_html_group(rendered_nodes, htmls.into());

                (RenderedNode::Fragment(rendered.0), rendered.1, rendered.2)
            }
            Html::HtmlElement(element) => {
                let rendered_nodes = if let RenderedNode::Element(rendered_nodes) = rendered_node {
                    rendered_nodes
                } else {
                    VecDeque::new()
                };

                let children = Self::render_html_group(rendered_nodes, element.children.into());

                (
                    RenderedNode::Element(children.0),
                    vec![VNode::VElement(VElement {
                        tag_name: Rc::new(element.tag_name),
                        attributes: element.attributes,
                        events: element.events,
                        children: children.1,
                        index_id: element.index_id,
                    })]
                    .into(),
                    children.2,
                )
            }
            Html::Component(prefab) => match rendered_node {
                RenderedNode::Component(mut component) if component.is(prefab.as_ref()) => {
                    let mut node_cmd = component.update_by_prefab(prefab);
                    let (v_nodes, mut child_node_cmd) = component.render();
                    node_cmd.append(&mut child_node_cmd);
                    (RenderedNode::Component(component), v_nodes, node_cmd)
                }
                _ => {
                    let mut component = prefab.into_node();
                    let mut node_cmd = component.on_assemble();
                    let (v_nodes, mut child_node_cmd) = component.render();
                    node_cmd.append(&mut child_node_cmd);
                    (RenderedNode::Component(component), v_nodes, node_cmd)
                }
            },
            Html::HtmlText(text) => (
                RenderedNode::Text,
                vec![VNode::VText(VText {
                    text: Rc::new(text.text),
                })]
                .into(),
                NodeCmd::new(VecDeque::new()),
            ),
            Html::RNode(r_node) => (
                RenderedNode::RNode(r_node.clone()),
                vec![VNode::RNode(r_node)].into(),
                NodeCmd::new(VecDeque::new()),
            ),
            Html::None => (
                RenderedNode::None,
                VecDeque::new(),
                NodeCmd::new(VecDeque::new()),
            ),
        }
    }

    fn render_html_group(
        prev_rendered_nodes: VecDeque<RenderedNode>,
        htmls: VecDeque<Html>,
    ) -> (VecDeque<RenderedNode>, VecDeque<VNode>, NodeCmd) {
        let mixeds = crate::util::mix(
            prev_rendered_nodes,
            htmls.into(),
            Self::compare_node_and_html,
            1.0,
            1.0,
            1.0,
        );

        mixeds.into_iter().fold(
            (
                VecDeque::new(),
                VecDeque::new(),
                NodeCmd::new(VecDeque::new()),
            ),
            |(mut now_rendered_nodes, mut v_nodes, mut node_cmd), mixed| {
                let rendered = match mixed {
                    crate::util::mix::Edit::Append(html) => {
                        Some(Self::render_html(RenderedNode::None, html))
                    }
                    crate::util::mix::Edit::Keep(prev_rendered_node, html) => {
                        Some(Self::render_html(prev_rendered_node, html))
                    }
                    crate::util::mix::Edit::Remove(..) => None,
                    crate::util::mix::Edit::Replace(prev_rendered_node, html) => {
                        Some(Self::render_html(prev_rendered_node, html))
                    }
                };
                if let Some(mut rendered) = rendered {
                    now_rendered_nodes.push_back(rendered.0);
                    v_nodes.append(&mut rendered.1);
                    node_cmd.append(&mut rendered.2);
                }
                (now_rendered_nodes, v_nodes, node_cmd)
            },
        )
    }

    fn compare_node_and_html(rendered_node: &RenderedNode, html: &Html) -> bool {
        match rendered_node {
            RenderedNode::Component(component) => match html {
                Html::Component(prefab) => component.is(prefab.as_ref()),
                _ => false,
            },
            RenderedNode::RNode(prev_r_node) => match html {
                Html::RNode(now_r_node) => prev_r_node.is_same_node(Some(&now_r_node)),
                _ => false,
            },
            RenderedNode::Element(..) => match html {
                Html::HtmlElement(..) => true,
                _ => false,
            },
            RenderedNode::Fragment(..) => match html {
                Html::Fragment(..) => true,
                _ => false,
            },
            RenderedNode::None => match html {
                Html::None => true,
                _ => false,
            },
            RenderedNode::Text => match html {
                Html::HtmlText(..) => true,
                _ => false,
            },
        }
    }
}
