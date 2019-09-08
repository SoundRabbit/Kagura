extern crate rand;

use rand::prelude::*;

pub mod dom;

static mut APP: Option<Box<Composable>> = None;

trait Composable {
    fn update(&mut self, id: u128);
    fn render(&mut self) -> dom::Node;
    fn get_id(&self) -> u128;
}

pub struct Component<Msg, State> {
    state: State,
    update: fn(&mut State, &Msg),
    render: fn(&State) -> Html,
    msg: Option<Msg>,
    children: Vec<Box<Composable>>,
    id: u128,
}

pub enum Html {
    Composable(Box<Composable>),
    TextNode(String),
    ElementNode {
        tag_name: String,
        children: Vec<Html>,
    },
}

impl<Msg, State> Component<Msg, State> {
    pub fn new(
        state: State,
        update: fn(&mut State, &Msg),
        render: fn(&State) -> Html,
    ) -> Component<Msg, State> {
        Component {
            state,
            update,
            render,
            msg: None,
            children: vec![],
            id: rand::random::<u128>(),
        }
    }

    fn append_composable(&mut self, composable: Box<Composable>) {
        self.children.push(composable);
    }

    fn adapt_html(&mut self, html: Html) -> dom::Node {
        match html {
            Html::Composable(mut composable) => {
                let node = composable.render();
                self.append_composable(composable);
                node
            }
            Html::TextNode(text) => dom::Node::Text(text),
            Html::ElementNode { tag_name, children } => dom::Node::Element {
                tag_name: tag_name,
                attributes: dom::Attributes::new(),
                events: dom::Events::new(),
                children: children
                    .into_iter()
                    .map(|child| self.adapt_html(child))
                    .collect::<Vec<dom::Node>>(),
            },
        }
    }
}

impl<Msg, State> Composable for Component<Msg, State> {
    fn update(&mut self, id: u128) {
        if id == self.id {
            if let Some(msg) = &self.msg {
                (self.update)(&mut self.state, msg);
            }
        } else {
            for child in &mut self.children {
                (*child).update(id);
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

impl Html {
    pub fn component<M, S>(component: Component<M, S>) -> Html
    where
        M: 'static,
        S: 'static,
    {
        Html::Composable(Box::new(component))
    }
    pub fn text(text: impl Into<String>) -> Html {
        Html::TextNode(text.into())
    }
    pub fn node(tag_name: impl Into<String>, children: Vec<Html>) -> Html {
        Html::ElementNode {
            tag_name: tag_name.into(),
            children,
        }
    }
}

pub fn run<M, S>(mut component: Component<M, S>)
where
        M: 'static,
        S: 'static,
{
    component.render();
    let composable: Box<Composable> = Box::new(component);
    // unsafe {
    //     APP = Some(composable);
    // }
}

fn update(id: u128) {
    // unsafe {
    //     if let Some(app) = &mut APP {
    //         app.update(id);
    //         app.render();
    //     }
    // }
}
