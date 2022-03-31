use crate::v_node::{VElement, VText};
use crate::Html;
use crate::HtmlNode;
use crate::VNode;
use kagura::component::Render;
use kagura::node::{FutureMsg, Msg};
use std::collections::VecDeque;
use std::pin::Pin;
use std::rc::Rc;

enum RenderedNode {
    None,
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

    pub fn update(&mut self, msg: Msg) -> VecDeque<FutureMsg> {
        Self::update_rendered(&mut self.rendered_node, msg)
    }

    fn update_rendered(node: &mut RenderedNode, msg: Msg) -> VecDeque<FutureMsg> {
        match node {
            RenderedNode::Component(node) => node.update(msg),
            RenderedNode::Element(children) => {
                let mut tasks = VecDeque::new();
                for child in children {
                    let mut tasks_buf = Self::update_rendered(child, msg.clone());
                    tasks.append(&mut tasks_buf);
                }
                tasks
            }
            RenderedNode::Fragment(children) => {
                let mut tasks = VecDeque::new();
                for child in children {
                    let mut tasks_buf = Self::update_rendered(child, msg.clone());
                    tasks.append(&mut tasks_buf);
                }
                tasks
            }
            RenderedNode::None => VecDeque::new(),
            RenderedNode::Text => VecDeque::new(),
        }
    }

    pub fn render(&mut self, state: &Pin<Box<This>>) -> VecDeque<VNode> {
        let html = state
            .as_ref()
            .render(self.children.take().unwrap_or_default());

        let mut rendered_node = RenderedNode::None;
        std::mem::swap(&mut self.rendered_node, &mut rendered_node);

        let (rendered_node, v_nodes) = Self::render_html(rendered_node, html);
        self.rendered_node = rendered_node;
        v_nodes
    }

    fn render_html(rendered_node: RenderedNode, html: Html) -> (RenderedNode, VecDeque<VNode>) {
        match html {
            Html::Fragment(htmls) => {
                let rendered_nodes = if let RenderedNode::Fragment(rendered_nodes) = rendered_node {
                    rendered_nodes
                } else {
                    VecDeque::new()
                };

                let rendered = Self::render_html_group(rendered_nodes, htmls.into());

                (RenderedNode::Fragment(rendered.0), rendered.1)
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
                )
            }
            Html::Component(prefab) => match rendered_node {
                RenderedNode::Component(mut component) if component.is(prefab.as_ref()) => {
                    component.update_by_prefab(prefab);
                    let v_nodes = component.render();
                    (RenderedNode::Component(component), v_nodes)
                }
                _ => {
                    let mut component = prefab.into_node();
                    let v_nodes = component.render();
                    (RenderedNode::Component(component), v_nodes)
                }
            },
            Html::HtmlText(text) => (
                RenderedNode::Text,
                vec![VNode::VText(VText {
                    text: Rc::new(text.text),
                })]
                .into(),
            ),
            Html::None => (RenderedNode::None, VecDeque::new()),
        }
    }

    fn render_html_group(
        rendered_nodes: VecDeque<RenderedNode>,
        htmls: VecDeque<Html>,
    ) -> (VecDeque<RenderedNode>, VecDeque<VNode>) {
        let mixeds = crate::util::mix(
            rendered_nodes,
            htmls.into(),
            Self::compare_node_and_html,
            1.0,
            1.0,
            1.0,
        );

        mixeds.into_iter().fold(
            (VecDeque::new(), VecDeque::new()),
            |(mut rendered_nodes, mut v_nodes), mixed| {
                let rendered = match mixed {
                    crate::util::mix::Edit::Append(html) => {
                        Some(Self::render_html(RenderedNode::None, html))
                    }
                    crate::util::mix::Edit::Keep(rendered_node, html) => {
                        Some(Self::render_html(rendered_node, html))
                    }
                    crate::util::mix::Edit::Remove(..) => None,
                    crate::util::mix::Edit::Replace(rendered_node, html) => {
                        Some(Self::render_html(rendered_node, html))
                    }
                };
                if let Some(mut rendered) = rendered {
                    rendered_nodes.push_back(rendered.0);
                    v_nodes.append(&mut rendered.1);
                }
                (rendered_nodes, v_nodes)
            },
        )
    }

    fn compare_node_and_html(rendered_node: &RenderedNode, html: &Html) -> bool {
        match rendered_node {
            RenderedNode::Component(component) => match html {
                Html::Component(prefab) => component.is(prefab.as_ref()),
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