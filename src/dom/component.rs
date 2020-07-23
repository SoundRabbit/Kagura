use super::html::Html;
use super::Events;
use super::Node;
use crate::basic_component::BasicComponent;
use crate::state;
use crate::task;
use std::any::Any;
use std::cell::RefCell;
use std::clone::Clone;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::rc::Weak;

/// Wrapper of Component
pub trait DomComponent: BasicComponent<Option<Node>> {
    fn set_me(&mut self, me: Weak<RefCell<Box<dyn DomComponent>>>);
    fn set_parent(&mut self, parent: Weak<RefCell<Box<dyn DomComponent>>>);
    fn set_children(&mut self, children: Vec<Html>);
    fn update(&mut self, msg: Box<dyn Any>);
}

pub type Resolver<Msg> = Box<dyn FnOnce(Msg)>;
pub type Messenger<Msg> = Box<dyn FnMut(Msg)>;

/// Cmd
pub enum Cmd<Msg, Sub> {
    None,
    Sub(Sub),
    Task(Box<dyn FnOnce(Resolver<Msg>)>),
}

pub struct ImplComponent<Msg: 'static, Props: 'static, State: 'static, Sub: 'static> {
    state: Option<State>,
    children: Vec<Html>,
    init: Box<dyn Fn(Option<State>, Props) -> (State, Cmd<Msg, Sub>)>,
    update: Box<dyn Fn(&mut State, Msg) -> Cmd<Msg, Sub>>,
    render: Box<dyn Fn(&State) -> Html>,
    subscribe: Option<Box<dyn FnMut(Sub) -> Box<dyn Any>>>,
    batch_handlers: Option<Vec<Box<dyn FnOnce(Messenger<Msg>)>>>,
    cache: Html,
    me: Weak<RefCell<Box<dyn DomComponent>>>,
    parent: Weak<RefCell<Box<dyn DomComponent>>>,
    is_changed: bool,
    cmd: Option<Cmd<Msg, Sub>>,
}

/// Component constructed by State-update-render
pub struct Component<Msg: 'static, Props: 'static, State: 'static, Sub: 'static>(
    Rc<RefCell<ImplComponent<Msg, Props, State, Sub>>>,
);

impl<Msg, Sub> Cmd<Msg, Sub> {
    pub fn none() -> Self {
        Cmd::None
    }

    pub fn sub(sub: Sub) -> Self {
        Cmd::Sub(sub)
    }

    pub fn task(task: impl FnOnce(Resolver<Msg>) + 'static) -> Self {
        Cmd::Task(Box::new(task))
    }
}

impl<Msg, Props, State, Sub> Component<Msg, Props, State, Sub> {
    pub fn new(
        init: impl Fn(Option<State>, Props) -> (State, Cmd<Msg, Sub>) + 'static,
        update: impl Fn(&mut State, Msg) -> Cmd<Msg, Sub> + 'static,
        render: impl Fn(&State) -> Html + 'static,
    ) -> Self {
        let component = Self(Rc::new(RefCell::new(ImplComponent {
            state: None,
            children: vec![],
            init: Box::new(init),
            update: Box::new(update),
            render: Box::new(render),
            subscribe: None,
            batch_handlers: Some(vec![]),
            cache: Html::none(),
            me: Weak::new(),
            parent: Weak::new(),
            is_changed: true,
            cmd: None,
        })));
        component
    }

    /// set subscription which bind from child sub to parent msg
    pub fn subscribe<Msg_: 'static>(mut self, mut sub: impl FnMut(Sub) -> Msg_ + 'static) -> Self {
        self.subscribe = Some(Box::new(move |s| Box::new(sub(s))));
        self
    }

    /// append batch handler
    pub fn batch(mut self, handler: impl FnOnce(Messenger<Msg>) + 'static) -> Self {
        if let Some(handlers) = &mut self.batch_handlers {
            handlers.push(Box::new(handler));
        }
        self
    }

    pub fn with(&self, props: Props) -> Self {
        let state = self.0.borrow_mut().state.take();
        let (state, cmd) = (self.init)(state, props);
        self.0.borrow_mut().state = Some(state);
        self.0.borrow_mut().cmd = Some(cmd);
        self.0.borrow_mut().is_changed = true;
        Self(Rc::clone(&self.0))
    }

    fn deal_cmd(&mut self, cmd: Cmd<Msg, Sub>) {
        match cmd {
            Cmd::None => {}
            Cmd::Sub(sub) => {
                if let Some(parent) = self.parent.upgrade() {
                    if let Some(subscribe) = &mut self.subscribe {
                        parent.borrow_mut().update(subscribe(sub));
                    }
                }
            }
            Cmd::Task(task) => {
                let me = Weak::clone(&self.me);
                let resolver = Box::new(move |msg: Msg| {
                    if let Some(me) = me.upgrade() {
                        me.borrow_mut().update(Box::new(msg));
                        state::render();
                    }
                });
                task::add(|| task(resolver));
            }
        };
    }

    fn render_lazy(&self, html: &Html) -> Option<Node> {
        match html {
            Html::ComponentNode(composable) => composable.borrow_mut().render(),
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

    /// render on updated
    fn render_force(&self, html: &mut Html) -> Option<Node> {
        match html {
            Html::ComponentNode(composable) => {
                composable.borrow_mut().set_parent(Weak::clone(&self.me));
                composable.borrow_mut().render()
            }
            Html::TextNode(text) => Some(Node::Text(text.clone())),
            Html::None => None,
            Html::ElementNode {
                tag_name,
                attributes,
                events,
                children,
            } => {
                let children = children
                    .into_iter()
                    .filter_map(|child| self.render_force(child))
                    .collect::<Vec<Node>>();
                let mut dom_events = Events::new();
                for (name, handlers) in &mut events.handlers {
                    for handler in handlers.drain(..) {
                        let me = Weak::clone(&self.me);
                        dom_events.add(name, move |e| {
                            if let Some(me) = me.upgrade() {
                                me.borrow_mut().update(handler(e));
                                state::render();
                            }
                        });
                    }
                }
                if let Some(rendered) = events.rendered.take() {
                    let me = Weak::clone(&self.me);
                    dom_events.rendered = Some(Box::new(move |e| {
                        if let Some(me) = me.upgrade() {
                            me.borrow_mut().update(rendered(e));
                            state::render();
                        }
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

impl<Msg, Props, State, Sub> DomComponent for Component<Msg, Props, State, Sub> {
    fn set_me(&mut self, me: Weak<RefCell<Box<dyn DomComponent>>>) {
        if let Some(handlers) = self.batch_handlers.take() {
            for handler in handlers {
                let me = Weak::clone(&me);
                let messenger: Messenger<Msg> = Box::new(move |msg| {
                    if let Some(me) = me.upgrade() {
                        me.borrow_mut().update(Box::new(msg));
                        state::render();
                    }
                });
                handler(messenger);
            }
        }
        self.me = me;
        if let Some(cmd) = self.cmd.take() {
            self.deal_cmd(cmd);
        }
    }

    fn set_parent(&mut self, parent: Weak<RefCell<Box<dyn DomComponent>>>) {
        self.parent = parent;
    }

    fn set_children(&mut self, children: Vec<Html>) {
        self.children = children;
    }

    fn update(&mut self, msg: Box<dyn Any>) {
        match msg.downcast::<Msg>() {
            Ok(msg) => {
                let cmd = if let Some(state) = &mut self.0.borrow_mut().state {
                    let cmd = (self.0.borrow_mut().update)(state, *msg);
                    self.0.borrow_mut().is_changed = true;
                    Some(cmd)
                } else {
                    None
                };
                if let Some(cmd) = cmd {
                    self.deal_cmd(cmd);
                }
            }
            Err(msg) => {
                if let Some(parent) = self.parent.upgrade() {
                    parent.borrow_mut().update(msg);
                }
            }
        }
    }
}

impl<Msg, Props, State, Sub> BasicComponent<Option<Node>> for Component<Msg, Props, State, Sub> {
    fn render(&mut self) -> Option<Node> {
        if self.is_changed {
            self.is_changed = false;
            if let Some(state) = &self.state {
                let mut html = (self.render)(state);
                let node = self.render_force(&mut html);
                self.cache = html;
                node
            } else {
                None
            }
        } else {
            self.render_lazy(&self.cache)
        }
    }
}

impl<Msg, Props, State, Sub> Deref for Component<Msg, Props, State, Sub> {
    type Target = ImplComponent<Msg, Props, State, Sub>;
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ptr().as_ref().unwrap() }
    }
}

impl<Msg, Props, State, Sub> DerefMut for Component<Msg, Props, State, Sub> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_ptr().as_mut().unwrap() }
    }
}
