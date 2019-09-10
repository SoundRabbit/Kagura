extern crate rand;
extern crate wasm_bindgen;

mod dom;

use rand::prelude::*;
use std::any::Any;
use wasm_bindgen::prelude::*;

static mut APP: Option<(Box<Composable>, dom::native::Renderer)> = None;

trait Composable {
    fn update(&mut self, id: u128, msg: &Any);
    fn render(&mut self, id: Option<u128>) -> dom::Node;
    fn get_id(&self) -> u128;
}

pub struct Component<Msg, State>
where
    Msg: 'static,
    State: 'static,
{
    state: State,
    update: fn(&mut State, &Msg),
    render: fn(&State) -> Html<Msg>,
    children: Vec<Box<Composable>>,
    id: u128,
}

pub enum Html<Msg> {
    Composable(Box<Composable>),
    TextNode(String),
    ElementNode {
        tag_name: String,
        children: Vec<Html<Msg>>,
        attributes: Vec<Attribute>,
        events: Vec<Event<Msg>>,
    },
}

pub enum Attribute {
    Attribute(String, String),
}

pub enum Event<Msg> {
    OnClick(Box<FnMut() -> Msg>),
}

impl<Msg, State> Component<Msg, State> {
    pub fn new(
        state: State,
        update: fn(&mut State, &Msg),
        render: fn(&State) -> Html<Msg>,
    ) -> Component<Msg, State> {
        let id = rand::random::<u128>();
        Component {
            state,
            update,
            render,
            children: vec![],
            id: id,
        }
    }

    fn append_composable(&mut self, composable: Box<Composable>) {
        self.children.push(composable);
    }

    fn adapt_html_lazy(&mut self, html: Html<Msg>, child_index: &mut usize, id: u128) -> dom::Node {
        match html {
            Html::Composable(mut composable) => {
                if let Some(child) = self.children.get_mut(*child_index) {
                    *child_index += 1;
                    (*child).render(Some(id))
                } else {
                    let node = composable.render(Some(id));
                    self.append_composable(composable);
                    node
                }
            }
            Html::TextNode(text) => dom::Node::Text(text),
            Html::ElementNode {
                tag_name,
                attributes,
                events,
                children,
            } => {
                let children = children
                    .into_iter()
                    .map(|child| self.adapt_html_lazy(child, child_index, id))
                    .collect::<Vec<dom::Node>>();
                dom::Node::Element {
                    tag_name,
                    attributes: dom::Attributes::new(),
                    events: dom::Events::new(),
                    children,
                    rerender: false,
                }
            }
        }
    }

    fn adapt_html_force(&mut self, html: Html<Msg>) -> dom::Node {
        match html {
            Html::Composable(mut composable) => {
                let node = composable.render(None);
                self.append_composable(composable);
                node
            }
            Html::TextNode(text) => dom::Node::Text(text),
            Html::ElementNode {
                tag_name,
                attributes,
                events,
                children,
            } => {
                let children = children
                    .into_iter()
                    .map(|child| self.adapt_html_force(child))
                    .collect::<Vec<dom::Node>>();
                let attributes =
                    attributes
                        .into_iter()
                        .fold(
                            dom::Attributes::new(),
                            |attributes, attribute| match attribute {
                                Attribute::Attribute(attr, val) => {
                                    attributes.with_attribute(attr, val)
                                }
                            },
                        );
                let component_id = self.id;
                let events =
                    events
                        .into_iter()
                        .fold(dom::Events::new(), |events, event| match event {
                            Event::OnClick(mut handler) => events.with_on_click(move || {
                                update(component_id, &handler());
                            }),
                        });
                dom::Node::Element {
                    tag_name,
                    attributes,
                    events,
                    children,
                    rerender: true,
                }
            }
        }
    }
}

impl<Msg, State> Composable for Component<Msg, State> {
    fn update(&mut self, id: u128, msg: &Any) {
        if id == self.id {
            if let Some(msg) = msg.downcast_ref::<Msg>() {
                (self.update)(&mut self.state, msg);
            }
        } else {
            for child in &mut self.children {
                (*child).update(id, msg);
            }
        }
    }

    fn render(&mut self, id: Option<u128>) -> dom::Node {
        let html = (self.render)(&self.state);
        if let Some(id) = id {
            if id == self.id {
                self.children.clear();
                self.adapt_html_force(html)
            } else {
                self.adapt_html_lazy(html, &mut 0, id)
            }
        } else {
            self.adapt_html_force(html)
        }
    }

    fn get_id(&self) -> u128 {
        self.id
    }
}

pub fn run<M, S>(mut component: Component<M, S>, id: &str)
where
    M: 'static,
    S: 'static,
{
    let node = component.render(None);
    let root = dom::native::get_element_by_id(id);
    let renderer = dom::native::Renderer::new(node, root.into());
    let composable: Box<Composable> = Box::new(component);
    unsafe {
        APP = Some((composable, renderer));
    }
}

fn update(id: u128, msg: &Any) {
    unsafe {
        if let Some((app, renderer)) = &mut APP {
            app.update(id, msg);
            let node = app.render(Some(id));
            renderer.update(node);
        }
    }
}

impl<Msg> Html<Msg> {
    pub fn component<M, S>(component: Component<M, S>) -> Self
    where
        M: 'static,
        S: 'static,
    {
        Html::Composable(Box::new(component))
    }
    pub fn unsafe_text(text: impl Into<String>) -> Self {
        Html::TextNode(text.into())
    }
    pub fn node(
        tag_name: impl Into<String>,
        attributes: Vec<Attribute>,
        events: Vec<Event<Msg>>,
        children: Vec<Html<Msg>>,
    ) -> Self {
        Html::ElementNode {
            tag_name: tag_name.into(),
            children,
            attributes,
            events,
        }
    }
    pub fn a(
        attributes: Vec<Attribute>,
        events: Vec<Event<Msg>>,
        children: Vec<Html<Msg>>,
    ) -> Self {
        Html::node("a", attributes, events, children)
    }
    pub fn button(
        attributes: Vec<Attribute>,
        events: Vec<Event<Msg>>,
        children: Vec<Html<Msg>>,
    ) -> Self {
        Html::node("button", attributes, events, children)
    }
    pub fn div(
        attributes: Vec<Attribute>,
        events: Vec<Event<Msg>>,
        children: Vec<Html<Msg>>,
    ) -> Self {
        Html::node("div", attributes, events, children)
    }
    pub fn h1(
        attributes: Vec<Attribute>,
        events: Vec<Event<Msg>>,
        children: Vec<Html<Msg>>,
    ) -> Self {
        Html::node("h1", attributes, events, children)
    }
    pub fn h2(
        attributes: Vec<Attribute>,
        events: Vec<Event<Msg>>,
        children: Vec<Html<Msg>>,
    ) -> Self {
        Html::node("h2", attributes, events, children)
    }
    pub fn h3(
        attributes: Vec<Attribute>,
        events: Vec<Event<Msg>>,
        children: Vec<Html<Msg>>,
    ) -> Self {
        Html::node("h3", attributes, events, children)
    }
    pub fn h4(
        attributes: Vec<Attribute>,
        events: Vec<Event<Msg>>,
        children: Vec<Html<Msg>>,
    ) -> Self {
        Html::node("h4", attributes, events, children)
    }
    pub fn h5(
        attributes: Vec<Attribute>,
        events: Vec<Event<Msg>>,
        children: Vec<Html<Msg>>,
    ) -> Self {
        Html::node("h5", attributes, events, children)
    }
    pub fn h6(
        attributes: Vec<Attribute>,
        events: Vec<Event<Msg>>,
        children: Vec<Html<Msg>>,
    ) -> Self {
        Html::node("h6", attributes, events, children)
    }
    pub fn span(
        attributes: Vec<Attribute>,
        events: Vec<Event<Msg>>,
        children: Vec<Html<Msg>>,
    ) -> Self {
        Html::node("span", attributes, events, children)
    }
}
