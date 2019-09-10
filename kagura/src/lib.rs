extern crate rand;

mod dom;

#[allow(unused_imports)]
use rand::prelude::*;
use std::any::Any;

pub use dom::Attributes;

static mut APP: Option<(Box<Composable>, dom::native::Renderer)> = None;

pub trait Composable {
    fn update(&mut self, id: u128, msg: &Any) -> bool;
    fn render(&mut self, id: Option<u128>) -> dom::Node;
    fn get_id(&self) -> u128;
    fn set_parent_id(&mut self, id: u128);
}

pub struct Component<Msg, State, Sub>
where
    Msg: 'static,
    State: 'static,
    Sub: 'static
{
    state: State,
    update: fn(&mut State, &Msg) -> Option<Sub>,
    subscribe: Option<Box<FnMut(Sub) -> Box<Any>>>,
    render: fn(&State) -> Html<Msg>,
    children: Vec<Box<Composable>>,
    id: u128,
    parent_id: Option<u128>,
}

pub enum Html<Msg> {
    Composable(Box<Composable>),
    TextNode(String),
    ElementNode {
        tag_name: String,
        children: Vec<Html<Msg>>,
        attributes: Attributes,
        events: Events<Msg>,
    },
}

pub struct Events<Msg> {
    on_click: Option<Box<FnMut() -> Msg>>,
}

impl<Msg, State, Sub> Component<Msg, State, Sub> {
    pub fn new(
        state: State,
        update: fn(&mut State, &Msg) -> Option<Sub>,
        render: fn(&State) -> Html<Msg>,
    ) -> Component<Msg, State, Sub> {
        let id = rand::random::<u128>();
        Component {
            state,
            update,
            render,
            children: vec![],
            id: id,
            subscribe: None,
            parent_id: None,
        }
    }

    fn append_composable(&mut self, mut composable: Box<Composable>) {
        composable.set_parent_id(self.id);
        self.children.push(composable);
    }

    pub fn subscribe(mut self, sub: impl FnMut(Sub) -> Box<Any> + 'static) -> Self {
        self.subscribe = Some(Box::new(sub));
        self
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
                attributes: _,
                events: _,
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
                let component_id = self.id;
                let mut dom_events = dom::Events::new();
                if let Some(mut handler) = events.on_click {
                    dom_events = dom_events.with_on_click(move || {update(component_id, &handler());});
                }
                dom::Node::Element {
                    tag_name,
                    attributes,
                    events: dom_events,
                    children,
                    rerender: true,
                }
            }
        }
    }
}

impl<Msg, State, Sub> Composable for Component<Msg, State, Sub> {
    fn update(&mut self, id: u128, msg: &Any) -> bool {
        if id == self.id {
            if let Some(msg) = msg.downcast_ref::<Msg>() {
                if let Some(sub) = (self.update)(&mut self.state, msg) {
                    if let Some(parent_id) = self.parent_id {
                        if let Some(subscribe) = &mut self.subscribe {
                            let msg = subscribe(sub);
                            update(parent_id, &(*msg));
                            return false;
                        }
                    }
                }
            }
        } else {
            for child in &mut self.children {
                (*child).update(id, msg);
            }
        }
        true
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

    fn set_parent_id(&mut self, id: u128) {
        self.parent_id = Some(id);
    }
}

pub fn run<M, S, B>(mut component: Component<M, S, B>, id: &str)
where
    M: 'static,
    S: 'static,
    B: 'static,
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
            if app.update(id, msg) {
                let node = app.render(Some(id));
                renderer.update(node);
            }
        }
    }
}

impl<Msg> Html<Msg> {
    pub fn component<M, S, B>(component: Component<M, S, B>) -> Self
    where
        M: 'static,
        S: 'static,
        B: 'static,
    {
        Html::Composable(Box::new(component))
    }
    pub fn unsafe_text(text: impl Into<String>) -> Self {
        Html::TextNode(text.into())
    }
    pub fn node(
        tag_name: impl Into<String>,
        attributes: Attributes,
        events: Events<Msg>,
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
        attributes: Attributes,
        events: Events<Msg>,
        children: Vec<Html<Msg>>,
    ) -> Self {
        Html::node("a", attributes, events, children)
    }
    pub fn button(
        attributes: Attributes,
        events: Events<Msg>,
        children: Vec<Html<Msg>>,
    ) -> Self {
        Html::node("button", attributes, events, children)
    }
    pub fn div(
        attributes: Attributes,
        events: Events<Msg>,
        children: Vec<Html<Msg>>,
    ) -> Self {
        Html::node("div", attributes, events, children)
    }
    pub fn h1(
        attributes: Attributes,
        events: Events<Msg>,
        children: Vec<Html<Msg>>,
    ) -> Self {
        Html::node("h1", attributes, events, children)
    }
    pub fn h2(
        attributes: Attributes,
        events: Events<Msg>,
        children: Vec<Html<Msg>>,
    ) -> Self {
        Html::node("h2", attributes, events, children)
    }
    pub fn h3(
        attributes: Attributes,
        events: Events<Msg>,
        children: Vec<Html<Msg>>,
    ) -> Self {
        Html::node("h3", attributes, events, children)
    }
    pub fn h4(
        attributes: Attributes,
        events: Events<Msg>,
        children: Vec<Html<Msg>>,
    ) -> Self {
        Html::node("h4", attributes, events, children)
    }
    pub fn h5(
        attributes: Attributes,
        events: Events<Msg>,
        children: Vec<Html<Msg>>,
    ) -> Self {
        Html::node("h5", attributes, events, children)
    }
    pub fn h6(
        attributes: Attributes,
        events: Events<Msg>,
        children: Vec<Html<Msg>>,
    ) -> Self {
        Html::node("h6", attributes, events, children)
    }
    pub fn span(
        attributes: Attributes,
        events: Events<Msg>,
        children: Vec<Html<Msg>>,
    ) -> Self {
        Html::node("span", attributes, events, children)
    }
}

impl<Msg> Events<Msg> {
    pub fn new() -> Self{
        Self {
            on_click: None,
        }
    }

    pub fn with_on_click(mut self, handler: impl FnMut() -> Msg + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}