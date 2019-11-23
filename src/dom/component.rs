use super::html::Html;
use super::Attributes;
use super::Events;
use super::Node;
use crate::basic_component::BasicComponent;
use crate::state;
use crate::task;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

/// Wrapper of Component
pub trait DomComponent: BasicComponent<Node> {
    fn set_me(&mut self, me: Weak<RefCell<Box<dyn DomComponent>>>);
    fn set_parent(&mut self, parent: Weak<RefCell<Box<dyn DomComponent>>>);
    fn update(&mut self, msg: Box<dyn Any>);
}

type Resolver<Msg> = Box<dyn FnOnce(Msg)>;
pub type Messenger<Msg> = Box<dyn FnMut(Msg)>;

/// Cmd
pub enum Cmd<Msg, Sub> {
    None,
    Sub(Sub),
    Task(Box<dyn FnOnce(Resolver<Msg>)>),
}

enum BatchHandlers<Msg> {
    Handlers(Vec<Box<dyn FnOnce(Messenger<Msg>)>>),
    Binded()
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
    batch_handlers: BatchHandlers<Msg>,
    children: Vec<Rc<RefCell<Box<dyn DomComponent>>>>,
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
        init: impl FnOnce() -> State,
        update: impl Fn(&mut State, Msg) -> Cmd<Msg, Sub> + 'static,
        render: impl Fn(&State) -> Html<Msg> + 'static,
    ) -> Self {
        Component {
            state: init(),
            update: Box::new(update),
            render: Box::new(render),
            subscribe: None,
            batch_handlers: BatchHandlers::Handlers(vec![]),
            children: vec![],
            me: Weak::new(),
            parent: Weak::new(),
            is_changed: true,
        }
    }

    /// set subscription witch bind from child sub to parent msg
    pub fn subscribe<Msg_>(mut self, mut sub: impl FnMut(Sub) -> Msg_ + 'static) -> Self
    where
        Msg_: 'static,
    {
        self.subscribe = Some(Box::new(move |s| Box::new(sub(s))));
        self
    }

    /// append batch handler
    pub fn batch(mut self, handler: impl FnMut(Messenger<Msg>) + 'static) -> Self {
        if let BatchHandlers::Handlers(handlers) = &mut self.batch_handlers {
            handlers.push(Box::new(handler));
        }
        self
    }

    /// append component to children components buffer
    fn append_component(&mut self, component: Rc<RefCell<Box<dyn DomComponent>>>) {
        component.borrow_mut().set_parent(Weak::clone(&self.me));
        self.children.push(component);
    }

    /// render on non-update
    fn render_lazy(&mut self, html: Html<Msg>, child_index: &mut usize) -> Node {
        match html {
            Html::ComponentNode(composable) => {
                if let Some(child) = self.children.get_mut(*child_index) {
                    *child_index += 1;
                    (*child).borrow_mut().render()
                } else {
                    let node = composable.borrow_mut().render();
                    self.append_component(composable);
                    node
                }
            }
            Html::TextNode(text) => Node::Text(text),
            Html::ElementNode {
                tag_name,
                attributes: _,
                events: _,
                children,
            } => {
                let children = children
                    .into_iter()
                    .map(|child| self.render_lazy(child, child_index))
                    .collect::<Vec<Node>>();
                Node::element(tag_name, Attributes::new(), Events::new(), children, false)
            }
        }
    }

    /// render on updated
    fn render_force(&mut self, html: Html<Msg>) -> Node {
        match html {
            Html::ComponentNode(composable) => {
                let node = composable.borrow_mut().render();
                self.append_component(composable);
                node
            }
            Html::TextNode(text) => Node::Text(text),
            Html::ElementNode {
                tag_name,
                attributes,
                events,
                children,
            } => {
                let children = children
                    .into_iter()
                    .map(|child| self.render_force(child))
                    .collect::<Vec<Node>>();
                let mut dom_events = Events::new();
                for (name, handler) in events.handlers {
                    let me = Weak::clone(&self.me);
                    dom_events.add(name, move |e| {
                        if let Some(me) = me.upgrade() {
                            me.borrow_mut().update(Box::new(handler(e)));
                            state::render();
                        }
                    });
                }
                Node::element(tag_name, attributes.into(), dom_events, children, true)
            }
        }
    }
}

impl<Msg, State, Sub> DomComponent for Component<Msg, State, Sub> {
    fn set_me(&mut self, me: Weak<RefCell<Box<dyn DomComponent>>>) {
        let mut batch_handlers = BatchHandlers::Binded();
        std::mem::swap(&mut self.batch_handlers, &mut batch_handlers);
        if let BatchHandlers::Handlers(handlers) = batch_handlers {
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
    }

    fn set_parent(&mut self, parent: Weak<RefCell<Box<dyn DomComponent>>>) {
        self.parent = parent;
    }

    fn update(&mut self, msg: Box<dyn Any>) {
        if let Ok(msg) = msg.downcast::<Msg>() {
            let cmd = (self.update)(&mut self.state, *msg);
            self.is_changed = true;
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
    }
}

impl<Msg, State, Sub> BasicComponent<Node> for Component<Msg, State, Sub> {
    fn render(&mut self) -> Node {
        let html = (self.render)(&self.state);
        if self.is_changed {
            self.is_changed = false;
            self.render_force(html)
        } else {
            self.render_lazy(html, &mut 0)
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
