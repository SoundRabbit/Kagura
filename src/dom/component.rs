use super::html::Html;
use crate::component;
use crate::dom;
use crate::state;
use crate::task;
use std::any::Any;
use std::collections::hash_set::HashSet;
use std::rc::Rc;
use std::rc::Weak;

/// Wrapper of Component
pub trait Composable: component::Composable<dom::Node> {
    fn dispatch_msg(&mut self, msg: Box<dyn Any>);
    fn set_parent(&mut self, parent: Weak<dyn Composable>, me: Weak<dyn Composable>);
}

enum Message<Msg> {
    None,
    Changed(Msg),
}

type Resolver<Msg> = Box<dyn FnOnce(Msg)>;

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
    subscribe: Option<Box<dyn Fn(Sub) -> Box<dyn Any>>>,
    children: Vec<Rc<dyn Composable>>,
    me: Weak<dyn Composable>,
    parent: Weak<dyn Composable>,
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
        init: impl FnOnce() -> (State, Msg),
        update: impl Fn(&mut State, Msg) -> Cmd<Msg, Sub> + 'static,
        render: impl Fn(&State) -> Html<Msg> + 'static,
    ) -> Rc<dyn Composable> {
        let (state, msg) = init();
        let component = Component {
            state: state,
            update: Box::new(update),
            render: Box::new(render),
            subscribe: None,
            children: vec![],
            me: Weak::new(),
            parent: Weak::new(),
        };
        let mut component = Rc::new(component);
        component.me = component.downgrade();
        component
    }

    /// set subscription witch bind from child sub to parent msg
    pub fn subscribe<Msg_>(mut self, mut sub: impl FnMut(Sub) -> Msg_ + 'static) -> Self
    where
        Msg_: 'static,
    {
        self.subscribe = Some(Box::new(move |s| Box::new(sub(s))));
        self
    }

    /// append component to children components buffer
    fn append_composable(&mut self, mut composable: Box<dyn Composable>) {
        composable.set_parent_id(self.id);
        self.children_ids.insert(composable.get_id());
        let child_children_ids = composable.get_children_ids();
        for child_id in child_children_ids {
            self.children_ids.insert(*child_id);
        }
        self.children.push(composable);
    }

    /// render on non-update
    fn render_lazy(&mut self, html: Html<Msg>, child_index: &mut usize, id: u128) -> dom::Node {
        match html {
            Html::Composable(mut composable) => {
                if let Some(child) = self.children.get_mut(*child_index) {
                    *child_index += 1;
                    (*child).render_dom(Some(id))
                } else {
                    let node = composable.render_dom(Some(id));
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
                dom::Node::element(
                    tag_name,
                    dom::Attributes::new(),
                    dom::Events::new(),
                    children,
                    false,
                )
            }
        }
    }

    /// render on updated
    fn render_force(&mut self, html: Html<Msg>) -> dom::Node {
        match html {
            Html::Composable(mut composable) => {
                let node = composable.render_dom(None);
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
                let mut dom_events = dom::Events::new();
                for (name, handler) in events.handlers {
                    let component_id = self.id.clone();
                    dom_events.add(name, move |e| {
                        let msg = handler(e);
                        state::update(component_id, Box::new(msg));
                    });
                }
                dom::Node::element(tag_name, attributes.attributes, dom_events, children, true)
            }
        }
    }

    fn update(&mut self, msg: Msg) {
        let cmd = (self.update)(&mut self.state, msg);
        match cmd {
            Cmd::None => (),
            Cmd::Sub(sub) => {
                if let (Some(subscribe), Some(parent)) =
                    (&mut self.subscribe, self.parent.upgrade())
                {
                    parent.dispatch_msg(subscribe(sub));
                }
            }
            Cmd::Task(task) => {
                let me = Weak::clone(&self.me);
                let resolver = Box::new(move |msg: Msg| {
                    if let Some(me) = me.upgrade() {
                        me.dispatch_msg(Box::new(msg));
                    }
                });
                task::add(|| task(resolver));
            }
        }
    }
}

impl<Msg, State, Sub> Composable for Component<Msg, State, Sub> {
    fn dispatch_msg(&mut self, msg: Box<dyn Any>) {
        if let Ok(msg) = msg.downcast::<Msg>() {
            self.update(*msg);
        }
    }
}

impl<Msg, State, Sub> component::Composable<dom::Node> for Component<Msg, State, Sub> {
    fn render(&mut self) -> dom::Node {
        let html = (self.render)(&self.state);
        if let Message::Changed(msg) = self.message {
            self.render_force(html)
        } else {
            self.render_lazy(html, 0)
        }
    }
}
