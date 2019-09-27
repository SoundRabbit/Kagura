use crate::dom;
use crate::event;
use crate::Html;
use std::any::Any;
use std::collections::hash_set::HashSet;

/// Wrapper of Component
pub trait Composable {
    fn update(&mut self, id: u128, msg: Box<Any>) -> Option<(Box<Any>, u128)>;
    fn render(&mut self, id: Option<u128>) -> dom::Node;
    fn get_id(&self) -> u128;
    fn set_parent_id(&mut self, id: u128);
    fn get_children_ids<'a>(&'a self) -> &'a HashSet<u128>;
}

/// Component constructed by State-update-render
pub struct Component<Msg, State, Sub>
where
    Msg: 'static,
    State: 'static,
    Sub: 'static,
{
    state: State,
    update: fn(&mut State, Msg) -> Option<Sub>,
    subscribe: Option<Box<FnMut(Sub) -> Box<Any>>>,
    render: fn(&State) -> Html<Msg>,
    children: Vec<Box<Composable>>,
    id: u128,
    parent_id: Option<u128>,
    events: HashSet<u128>,
    children_ids: HashSet<u128>,
}

impl<Msg, State, Sub> Component<Msg, State, Sub> {
    /// Creates new component ftom initial state, update, render
    ///
    /// # Example
    ///
    /// ```
    /// fn hello_world_component() -> Component<Msg, State, Sub> {
    ///     Component::new(initial_state, update, render)
    /// }
    ///
    /// struct Msg;
    /// struct State;
    /// struct Sub;
    ///
    /// fn update(_: &mut State, _: &Msg) -> Option<Sub> { None }
    ///
    /// fn render(_: &State) -> Html<Msg> {
    ///     Html::h1(
    ///     Attributes::new(),
    ///     Events::new(),
    ///     vec![
    ///         Html::unsafe_text("hello kagura"),
    ///     ],
    /// )
    /// }
    /// ```
    pub fn new(
        state: State,
        update: fn(&mut State, Msg) -> Option<Sub>,
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
            events: HashSet::new(),
            children_ids: HashSet::new(),
        }
    }

    fn append_composable(&mut self, mut composable: Box<Composable>) {
        composable.set_parent_id(self.id);
        self.children_ids.insert(composable.get_id());
        self.children.push(composable);
    }

    /// Regists binder from child Sub to parent Msg
    ///
    /// #Example
    ///
    /// ```
    /// create_a_child_component(props).subscribe(|sub| {
    ///     match sub {
    ///         ChildComponent::Sub::Input(value) => Box::new(Msg::Send(value))
    ///     }
    /// })
    /// ```
    pub fn subscribe<Msg_>(mut self, mut sub: impl FnMut(Sub) -> Msg_ + 'static) -> Self
    where
        Msg_: 'static,
    {
        self.subscribe = Some(Box::new(move |s| Box::new(sub(s))));
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

                for (name, mut handler) in events.handlers {
                    let event_id = rand::random::<u128>();
                    event::add(event_id, move |e| (component_id, Box::new(handler(e))));
                    dom_events.add(name, move |e| {
                        event::dispatch(event_id, e);
                    });
                    self.events.insert(event_id);
                }

                dom::Node::element(tag_name, attributes.attributes, dom_events, children, true)
            }
        }
    }
}

impl<Msg, State, Sub> Composable for Component<Msg, State, Sub> {
    fn update(&mut self, id: u128, msg: Box<Any>) -> Option<(Box<Any>, u128)> {
        if id == self.id {
            if let Ok(msg) = msg.downcast::<Msg>() {
                if let Some(sub) = (self.update)(&mut self.state, *msg) {
                    if let Some(parent_id) = self.parent_id {
                        if let Some(subscribe) = &mut self.subscribe {
                            return Some((subscribe(sub), parent_id));
                        }
                    }
                }
            }
        } else {
            for child in &mut self.children {
                if child.get_id() == id || child.get_children_ids().get(&id).is_some() {
                    if let Some(sub) = (*child).update(id, msg) {
                        return Some(sub);
                    }
                    break;
                }
            }
        }
        None
    }

    fn render(&mut self, id: Option<u128>) -> dom::Node {
        let html = (self.render)(&self.state);
        if let Some(id) = id {
            if id == self.id {
                self.children.clear();
                for event_id in &self.events {
                    event::remove(*event_id);
                }
                self.events.clear();
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

    fn get_children_ids<'a>(&'a self) -> &'a HashSet<u128> {
        &self.children_ids
    }
}
