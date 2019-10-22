use crate::dom;
use crate::state;
use crate::Html;
use std::any::Any;
use std::collections::hash_set::HashSet;

/// Wrapper of Component
pub trait Composable {
    fn update(&mut self, id: u128, msg: Box<dyn Any>) -> Option<(Box<dyn Any>, u128)>;
    fn render_dom(&mut self, id: Option<u128>) -> dom::Node;
    fn get_id(&self) -> u128;
    fn set_parent_id(&mut self, id: u128);
    fn get_children_ids<'a>(&'a self) -> &'a HashSet<u128>;
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
    update: fn(&mut State, Msg) -> Cmd<Msg, Sub>,
    subscribe: Option<Box<dyn FnMut(Sub) -> Box<dyn Any>>>,
    dom_render: fn(&State) -> Html<Msg>,
    children: Vec<Box<dyn Composable>>,
    id: u128,
    parent_id: Option<u128>,
    children_ids: HashSet<u128>,
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
    /// Creates new component ftom initial state, update, render
    ///
    /// # Example
    ///
    /// ```
    /// use kagura::Attributes;
    /// use kagura::Events;
    /// use kagura::Html;
    /// use kagura::Cmd;
    /// use kagura::Component;
    ///
    /// fn hello_world_component() -> Component<Msg, State, Sub> {
    ///     Component::new(init(), update, render)
    /// }
    ///
    /// struct Msg();
    /// struct State();
    /// struct Sub();
    ///
    /// fn init() -> State {
    ///     State()
    /// }
    ///
    /// fn update(_: &mut State, _: Msg) -> Cmd<Msg, Sub> { Cmd::none() }
    ///
    /// fn render(_: &State) -> Html<Msg> {
    ///     Html::h1(
    ///         Attributes::new(),
    ///         Events::new(),
    ///         vec![
    ///             Html::text("hello kagura"),
    ///         ],
    ///     )
    /// }
    /// ```
    pub fn new(
        state: State,
        update: fn(&mut State, Msg) -> Cmd<Msg, Sub>,
        dom_render: fn(&State) -> Html<Msg>,
    ) -> Component<Msg, State, Sub> {
        let id = rand::random::<u128>();
        Component {
            state,
            update,
            dom_render,
            children: vec![],
            id: id,
            subscribe: None,
            parent_id: None,
            children_ids: HashSet::new(),
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
    fn adapt_html_lazy(&mut self, html: Html<Msg>, child_index: &mut usize, id: u128) -> dom::Node {
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
    fn adapt_html_force(&mut self, html: Html<Msg>) -> dom::Node {
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

    /// dispatch message to update
    fn dispatch(&mut self, msg: Msg) -> Option<(Box<dyn Any>, u128)> {
        let cmd = (self.update)(&mut self.state, msg);
        match cmd {
            Cmd::None => None,
            Cmd::Sub(sub) => {
                if let (Some(parent_id), Some(subscribe)) = (self.parent_id, &mut self.subscribe) {
                    Some((subscribe(sub), parent_id))
                } else {
                    None
                }
            }
            Cmd::Task(task) => {
                let component_id = self.id;
                let resolver = Box::new(move |msg: Msg| {
                    state::update(component_id, Box::new(msg));
                });
                task(resolver);
                None
            }
        }
    }
}

impl<Msg, State, Sub> Composable for Component<Msg, State, Sub> {
    fn update(&mut self, id: u128, msg: Box<dyn Any>) -> Option<(Box<dyn Any>, u128)> {
        if id == self.id {
            match msg.downcast::<Msg>() {
                Ok(msg) => self.dispatch(*msg),
                Err(_) => None,
            }
        } else {
            for child in &mut self.children {
                if child.get_id() == id || child.get_children_ids().get(&id).is_some() {
                    return (*child).update(id, msg);
                }
            }
            None
        }
    }

    fn render_dom(&mut self, parent_id: Option<u128>) -> dom::Node {
        let html = (self.dom_render)(&self.state);
        if let Some(parent_id) = parent_id {
            if parent_id == self.id {
                self.children.clear();
                self.adapt_html_force(html)
            } else {
                self.adapt_html_lazy(html, &mut 0, parent_id)
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

    fn get_children_ids<'a>(&'a self) -> &'a HashSet<u128> {
        &self.children_ids
    }
}
