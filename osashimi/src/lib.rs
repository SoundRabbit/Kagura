extern crate rand;
extern crate wasm_bindgen;

mod dom;

use rand::prelude::*;
use wasm_bindgen::prelude::*;
use std::any;
use std::any::Any;

static mut APP: Option<(Box<Composable>, dom::native::Element)> = None;

trait Composable {
    fn update(&mut self, id: u128, msg: &Any);
    fn render(&mut self) -> dom::Node;
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
        Component {
            state,
            update,
            render,
            children: vec![],
            id: rand::random::<u128>(),
        }
    }

    fn append_composable(&mut self, composable: Box<Composable>) {
        self.children.push(composable);
    }

    fn adapt_html(&mut self, html: Html<Msg>) -> dom::Node {
        match html {
            Html::Composable(mut composable) => {
                let node = composable.render();
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
                    .map(|child| self.adapt_html(child))
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
                            Event::OnClick(mut handler) => {
                                events.with_on_click(Closure::wrap(Box::new(move || {
                                    update(component_id, &handler());
                                })))
                            }
                        });
                dom::Node::Element {
                    tag_name,
                    attributes,
                    events,
                    children,
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

    fn render(&mut self) -> dom::Node {
        self.children.clear();
        let html = (self.render)(&self.state);
        self.adapt_html(html)
    }

    fn get_id(&self) -> u128 {
        self.id
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
    pub fn text(text: impl Into<String>) -> Self {
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
    pub fn h1(
        attributes: Vec<Attribute>,
        events: Vec<Event<Msg>>,
        children: Vec<Html<Msg>>,
    ) -> Self {
        Html::node("h1", attributes, events, children)
    }
}

pub fn run<M, S>(mut component: Component<M, S>, id: &str)
where
    M: 'static,
    S: 'static,
{
    let node = component.render();
    let root = dom::native::get_element_by_id(id);
    dom::native::render(
        node,
        &root,
    );
    let composable: Box<Composable> = Box::new(component);
    unsafe {
        APP = Some((composable, root));
    }
}

fn update (id: u128, msg: &Any) {
    dom::native::console_log("update");
    unsafe {
        if let Some((app, root)) = &mut APP {
            app.update(id, msg);
            let node = app.render();
            dom::native::render(
                node,
                root
            );
        }
    }
}
