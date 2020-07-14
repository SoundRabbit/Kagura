use super::html::Html;
use super::Events;
use super::Node;
use crate::basic_component::BasicComponent;
use crate::state;
use crate::task;
use std::any::Any;
use std::cell::RefCell;
use std::clone::Clone;
use std::rc::Rc;
use std::rc::Weak;

/// Wrapper of Component
pub trait DomComponent: BasicComponent<Option<Node>> {
    fn set_me(&mut self, me: Weak<RefCell<Box<dyn DomComponent>>>);
    fn set_parent(&mut self, parent: Weak<RefCell<Box<dyn DomComponent>>>);
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

/// Component constructed by State-update-render
pub struct Component<Msg, State, Sub>
where
    Msg: 'static,
    State: 'static,
    Sub: 'static,
{
    state: State,
    update: Box<dyn Fn(&mut State, Msg) -> Cmd<Msg, Sub>>,
    render: Box<dyn Fn(&State) -> Html<Msg>>,
    subscribe: Option<Box<dyn FnMut(Sub) -> Box<dyn Any>>>,
    batch_handlers: Option<Vec<Box<dyn FnOnce(Messenger<Msg>)>>>,
    initial_cmd: Option<Cmd<Msg, Sub>>,
    cash: Html<Msg>,
    me: Weak<RefCell<Box<dyn DomComponent>>>,
    parent: Weak<RefCell<Box<dyn DomComponent>>>,
    is_changed: bool,
}

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

impl<Msg, State, Sub> Component<Msg, State, Sub>
where
    Msg: 'static,
    State: 'static,
    Sub: 'static,
{
    pub fn new(
        init: impl FnOnce() -> (State, Cmd<Msg, Sub>),
        update: impl Fn(&mut State, Msg) -> Cmd<Msg, Sub> + 'static,
        render: impl Fn(&State) -> Html<Msg> + 'static,
    ) -> Self {
        let (state, cmd) = init();
        let component = Component {
            state: state,
            update: Box::new(update),
            render: Box::new(render),
            subscribe: None,
            batch_handlers: Some(vec![]),
            initial_cmd: Some(cmd),
            cash: Html::none(),
            me: Weak::new(),
            parent: Weak::new(),
            is_changed: true,
        };
        component
    }

    /// set subscription which bind from child sub to parent msg
    pub fn subscribe<Msg_>(mut self, mut sub: impl FnMut(Sub) -> Msg_ + 'static) -> Self
    where
        Msg_: 'static,
    {
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

    fn deal_cmd(&mut self, cmd: Cmd<Msg, Sub>) {
        match cmd {
            Cmd::None => {}
            Cmd::Sub(sub) => {
                if let (Some(subscribe), Some(parent)) =
                    (&mut self.subscribe, &self.parent.upgrade())
                {
                    parent.borrow_mut().update(subscribe(sub));
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

    fn render_lazy(&self, html: &Html<Msg>) -> Option<Node> {
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
    fn render_force(&self, html: &mut Html<Msg>) -> Option<Node> {
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
                for (name, handler) in &mut events.handlers {
                    if let Some(handler) = handler.take() {
                        let me = Weak::clone(&self.me);
                        dom_events.add(name, move |e| {
                            if let Some(me) = me.upgrade() {
                                me.borrow_mut().update(Box::new(handler(e)));
                                state::render();
                            }
                        });
                    }
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

impl<Msg, State, Sub> DomComponent for Component<Msg, State, Sub> {
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
        if let Some(cmd) = self.initial_cmd.take() {
            self.deal_cmd(cmd);
        }
    }

    fn set_parent(&mut self, parent: Weak<RefCell<Box<dyn DomComponent>>>) {
        self.parent = parent;
    }

    fn update(&mut self, msg: Box<dyn Any>) {
        if let Ok(msg) = msg.downcast::<Msg>() {
            let cmd = (self.update)(&mut self.state, *msg);
            self.is_changed = true;
            self.deal_cmd(cmd);
        }
    }
}

impl<Msg, State, Sub> BasicComponent<Option<Node>> for Component<Msg, State, Sub> {
    fn render(&mut self) -> Option<Node> {
        if self.is_changed {
            self.is_changed = false;
            self.cash = Html::none();
            let mut html = (self.render)(&self.state);
            let node = self.render_force(&mut html);
            self.cash = html;
            node
        } else {
            self.render_lazy(&self.cash)
        }
    }
}

impl<Msg, State, Sub> Into<Rc<RefCell<Box<dyn DomComponent>>>> for Component<Msg, State, Sub> {
    fn into(self) -> Rc<RefCell<Box<dyn DomComponent>>> {
        let component: Rc<RefCell<Box<dyn DomComponent>>> = Rc::new(RefCell::new(Box::new(self)));
        component.borrow_mut().set_me(Rc::downgrade(&component));
        component
    }
}
