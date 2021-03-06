use super::html::Html;
use super::Events;
use super::Node;
use crate::state;
use crate::task;
use std::any::Any;
use std::clone::Clone;

pub mod component;
mod controller;
pub use controller::Controller;
pub use controller::RcController;
use controller::WeakController;

pub type TaskResolver<Msg> = Box<dyn FnOnce(Msg)>;
pub type BatchResolver<Msg> = Box<dyn FnMut(Msg)>;

trait Component<Props, Sub> {
    fn init(&mut self);
    fn update(&mut self, msg: Box<dyn Any>);
    fn render(&mut self) -> Option<Node>;
    fn set_this(&mut self, this: Box<dyn Controller>);
    fn set_children(&mut self, children: Vec<Html>);
    fn set_parent(&mut self, parent: Box<dyn Controller>);
    fn set_messenger(&mut self, messenger: Box<dyn FnOnce(Sub) -> Box<dyn Any>>);
    fn set_props(&mut self, props: Props);
    fn set_state(&mut self, state: Box<dyn Any>);
    fn take_state(&mut self) -> Option<Box<dyn Any>>;
    fn set_key(&mut self, key: u64);
    fn key(&self) -> u64;
}

pub enum Cmd<Msg, Sub> {
    None,
    Task(Box<dyn FnOnce(TaskResolver<Msg>)>),
    Sub(Sub),
}

pub struct Batch<Msg: 'static> {
    payload: Vec<Box<dyn FnOnce(BatchResolver<Msg>)>>,
}

struct ImplComponent<Msg: 'static, Props: 'static, State: 'static, Sub: 'static> {
    this: Box<dyn Controller>,
    parent: Box<dyn Controller>,
    messenger: Option<Box<dyn FnOnce(Sub) -> Box<dyn Any>>>,
    state: Option<Box<State>>,
    init: Box<dyn Fn(Option<State>, Props) -> (State, Cmd<Msg, Sub>, Batch<Msg>)>,
    update: Box<dyn Fn(&mut State, Msg) -> Cmd<Msg, Sub>>,
    render: Box<dyn Fn(&State, Vec<Html>) -> Html>,
    children: Vec<Html>,
    cache: Html,
    is_updated: bool,
    props: Option<Props>,
    key: u64,
}

impl<Msg, Sub> Cmd<Msg, Sub> {
    pub fn none() -> Self {
        Cmd::None
    }

    pub fn task(worker: impl FnOnce(TaskResolver<Msg>) + 'static) -> Self {
        Self::Task(Box::new(worker))
    }

    pub fn sub(sub: Sub) -> Self {
        Cmd::Sub(sub)
    }
}

impl<Msg> Batch<Msg> {
    pub fn new() -> Self {
        Self {
            payload: Vec::new(),
        }
    }

    pub fn add(&mut self, f: impl FnOnce(BatchResolver<Msg>) + 'static) {
        self.payload.push(Box::new(f));
    }
}

impl<Msg, Props, State, Sub> ImplComponent<Msg, Props, State, Sub> {
    fn new(
        init: impl Fn(Option<State>, Props) -> (State, Cmd<Msg, Sub>, Batch<Msg>) + 'static,
        update: impl Fn(&mut State, Msg) -> Cmd<Msg, Sub> + 'static,
        render: impl Fn(&State, Vec<Html>) -> Html + 'static,
    ) -> Self {
        Self {
            this: Box::new(WeakController::<Props, Sub>::new()),
            parent: Box::new(WeakController::<Props, Sub>::new()),
            messenger: None,
            state: None,
            init: Box::new(init),
            update: Box::new(update),
            render: Box::new(render),
            children: vec![],
            cache: Html::None,
            is_updated: false,
            props: None,
            key: 0,
        }
    }

    fn proc_cmd(&mut self, cmd: Cmd<Msg, Sub>) {
        match cmd {
            Cmd::None => {}
            Cmd::Task(worker) => {
                let this = self.this.clone();
                task::add(move || {
                    worker(Box::new(move |msg| {
                        this.update(Box::new(msg));
                        state::render()
                    }))
                });
            }
            Cmd::Sub(sub) => {
                if let Some(messenger) = self.messenger.take() {
                    let msg = messenger(sub);
                    self.parent.update(msg);
                }
            }
        }
    }

    fn render_lazy(&self, html: &Html) -> Option<Node> {
        match html {
            Html::ComponentNode(controller) => controller.render(),
            Html::TextNode(text) => Some(Node::Text(text.clone())),
            Html::None => None,
            Html::ElementNode {
                tag_name,
                attributes,
                events: _,
                children,
            } => {
                let children = children
                    .into_iter()
                    .filter_map(|child| self.render_lazy(child))
                    .collect::<Vec<Node>>();

                let dom_events = Events::new();

                Some(Node::element(
                    tag_name,
                    attributes.clone().into(),
                    dom_events,
                    children,
                    false,
                ))
            }
        }
    }

    fn render_force(&self, html: &mut Html, cache: Html) -> Option<Node> {
        match html {
            Html::None => None,
            Html::ComponentNode(controller) => {
                if let Html::ComponentNode(before) = cache {
                    if Any::type_id(controller) == Any::type_id(&before)
                        && controller.key() == before.key()
                    {
                        let state = before.take_state().unwrap();
                        controller.set_state(state);
                    }
                }
                controller.init();
                controller.set_parent(self.this.clone());
                controller.render()
            }
            Html::TextNode(text) => Some(Node::text(text.clone())),
            Html::ElementNode {
                tag_name,
                attributes,
                events,
                children,
            } => {
                let children = match cache {
                    Html::ElementNode {
                        children: mut before,
                        ..
                    } => {
                        while children.len() > before.len() {
                            before.push(Html::none());
                        }
                        children
                            .into_iter()
                            .zip(before.into_iter())
                            .filter_map(|(child, cache)| self.render_force(child, cache))
                            .collect::<Vec<Node>>()
                    }
                    _ => children
                        .into_iter()
                        .filter_map(|child| self.render_force(child, Html::none()))
                        .collect::<Vec<Node>>(),
                };

                let mut dom_events = Events::new();
                for (name, handlers) in &mut events.handlers {
                    for handler in handlers.drain(..) {
                        let this = self.this.clone();
                        dom_events.add(name, move |e| {
                            this.update(handler(e));
                            state::render();
                        });
                    }
                }
                if let Some(rendered) = events.rendered.take() {
                    let this = self.this.clone();
                    dom_events.rendered = Some(Box::new(move |e| {
                        this.update(rendered(e));
                        state::render();
                    }))
                }
                Some(Node::element(
                    tag_name.as_str(),
                    attributes.clone().into(),
                    dom_events,
                    children,
                    true,
                ))
            }
        }
    }
}

impl<Msg, Props, State, Sub> Component<Props, Sub> for ImplComponent<Msg, Props, State, Sub> {
    fn init(&mut self) {
        if let Some(props) = self.props.take() {
            let (state, cmd, batchs) = (self.init)(self.state.take().map(|x| *x), props);
            self.is_updated = true;
            self.state = Some(Box::new(state));
            self.proc_cmd(cmd);
            for batch in batchs.payload {
                let this = self.this.clone();
                batch(Box::new(move |msg| {
                    this.update(Box::new(msg));
                    state::render()
                }));
            }
        }
    }

    fn update(&mut self, msg: Box<dyn Any>) {
        match msg.downcast::<Msg>() {
            Ok(msg) => {
                if let Some(state) = &mut self.state {
                    self.is_updated = true;
                    let cmd = (self.update)(state, *msg);
                    self.proc_cmd(cmd);
                }
            }
            Err(msg) => {
                self.parent.update(msg);
            }
        }
    }

    fn render(&mut self) -> Option<Node> {
        if self.is_updated {
            if let Some(state) = &self.state {
                self.is_updated = false;
                let mut children = self.children.clone();
                std::mem::swap(&mut children, &mut self.children);
                let mut html = (self.render)(state, children);
                let mut cache = Html::none();
                std::mem::swap(&mut self.cache, &mut cache);
                let node = self.render_force(&mut html, cache);
                self.cache = html;
                node
            } else {
                None
            }
        } else {
            self.render_lazy(&self.cache)
        }
    }

    fn set_this(&mut self, this: Box<dyn Controller>) {
        self.this = this;
    }

    fn set_children(&mut self, children: Vec<Html>) {
        self.children = children;
    }

    fn set_parent(&mut self, parent: Box<dyn Controller>) {
        self.parent = parent;
    }

    fn set_messenger(&mut self, messenger: Box<dyn FnOnce(Sub) -> Box<dyn Any>>) {
        self.messenger = Some(Box::new(messenger));
    }

    fn set_props(&mut self, props: Props) {
        self.props = Some(props);
    }

    fn set_state(&mut self, state: Box<dyn Any>) {
        if let Ok(state) = state.downcast::<State>() {
            self.state = Some(state);
        }
    }

    fn take_state(&mut self) -> Option<Box<dyn Any>> {
        self.state.take().map(|x| x as Box<dyn Any>)
    }

    fn set_key(&mut self, key: u64) {
        self.key = key;
    }

    fn key(&self) -> u64 {
        self.key
    }
}
