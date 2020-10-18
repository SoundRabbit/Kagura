use super::html::Html;
use super::{Events, Node};
use crate::{state, task};
use std::any::Any;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub trait Component: 'static {
    type Props;
    type Msg;
    type Sub;
    fn init(&mut self, props: Self::Props, builder: &mut ComponentBuilder<Self::Msg, Self::Sub>);
    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg, Self::Sub>;
    fn render(&self, children: Vec<Html>) -> Html;
}

pub trait Constructor: Component + Sized {
    fn constructor(
        props: Self::Props,
        builder: &mut ComponentBuilder<Self::Msg, Self::Sub>,
    ) -> Self;

    fn view(props: Self::Props, sub_map: SubMap<Self::Sub>, children: Vec<Html>) -> Html {
        Html::component::<Self, Self::Props, Self::Msg, Self::Sub>(props, sub_map, children)
    }
}

pub trait Composed {
    fn update(&mut self, msg: Message);
    fn subscribe(&mut self, msg: Box<dyn Any>);
    fn render(&mut self, is_forced: bool) -> Option<Node>;
    fn set_children(&mut self, children: Vec<Html>);
    fn set_this(&mut self, this: Weak<RefCell<Box<dyn Composed>>>);
    fn set_parent(&mut self, parent: Weak<RefCell<Box<dyn Composed>>>);
}

pub struct Message {
    pub payload: Box<dyn Any>,
    pub component_id: crate::uid::IdType,
}

pub type TaskResolver<Msg> = Box<dyn FnOnce(Msg)>;
pub type BatchResolver<Msg> = Box<dyn FnMut(Msg)>;

pub enum Cmd<Msg, Sub> {
    None,
    Task(Box<dyn FnOnce(TaskResolver<Msg>)>),
    Sub(Sub),
}

pub struct ComponentBuilder<Msg: 'static, Sub> {
    batches: Vec<Box<dyn FnOnce(BatchResolver<Msg>)>>,
    cmd: Cmd<Msg, Sub>,
}

pub struct SubMap<Sub: 'static> {
    payload: Option<Box<dyn FnOnce(Sub) -> Box<dyn Any>>>,
}

pub struct ComposedComponent<Props: 'static, Msg: 'static, Sub: 'static> {
    component_id: crate::uid::IdType,
    component: Box<dyn Component<Props = Props, Msg = Msg, Sub = Sub>>,
    this: Weak<RefCell<Box<dyn Composed>>>,
    parent: Weak<RefCell<Box<dyn Composed>>>,
    children: Vec<Html>,
    rendered_cache: Html,
    is_updated: bool,
    sub_map: Option<Box<dyn FnOnce(Sub) -> Box<dyn Any>>>,
    builder: Option<ComponentBuilder<Msg, Sub>>,
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

impl<Msg, Sub> ComponentBuilder<Msg, Sub> {
    pub fn new() -> Self {
        Self {
            batches: Vec::new(),
            cmd: Cmd::none(),
        }
    }

    pub fn add_batch(&mut self, f: impl FnOnce(BatchResolver<Msg>) + 'static) {
        self.batches.push(Box::new(f));
    }

    pub fn set_cmd(&mut self, cmd: Cmd<Msg, Sub>) {
        self.cmd = cmd;
    }
}

impl<Sub> SubMap<Sub> {
    pub fn new<Msg: 'static>(mapper: impl FnOnce(Sub) -> Msg + 'static) -> Self {
        Self {
            payload: Some(Box::new(move |sub| Box::new(mapper(sub)) as Box<dyn Any>)),
        }
    }

    pub fn empty() -> Self {
        Self { payload: None }
    }
}

impl<Props: 'static, Msg: 'static, Sub: 'static> ComposedComponent<Props, Msg, Sub> {
    pub fn new(
        component_id: crate::uid::IdType,
        component: impl Component<Props = Props, Msg = Msg, Sub = Sub> + 'static,
        builder: ComponentBuilder<Msg, Sub>,
        sub_map: SubMap<Sub>,
    ) -> Rc<RefCell<Box<dyn Composed>>> {
        let this = Self {
            component_id: component_id,
            component: Box::new(component),
            this: Weak::new(),
            parent: Weak::new(),
            children: Vec::new(),
            rendered_cache: Html::none(),
            is_updated: true,
            sub_map: sub_map.payload,
            builder: Some(builder),
        };
        let this = Rc::new(RefCell::new(Box::new(this) as Box<dyn Composed>));
        let weak_this = Rc::downgrade(&this);
        this.borrow_mut().set_this(Weak::clone(&weak_this));
        this
    }

    pub fn init(&mut self, props: Props) {
        let mut builder = ComponentBuilder::new();
        self.component.init(props, &mut builder);
    }

    fn proc_cmd(&mut self, cmd: Cmd<Msg, Sub>) {
        match cmd {
            Cmd::None => {}
            Cmd::Task(worker) => {
                let this = Weak::clone(&self.this);
                let component_id = self.component_id;
                task::add(move || {
                    worker(Box::new(move |msg| {
                        if let Some(this) = this.upgrade() {
                            let msg = Message {
                                payload: Box::new(msg),
                                component_id: component_id,
                            };
                            this.borrow_mut().update(msg);
                            state::render();
                        }
                    }))
                })
            }
            Cmd::Sub(sub) => {
                if let Some(parent) = self.parent.upgrade() {
                    if let Some(sub_map) = self.sub_map.take() {
                        let msg = sub_map(sub);
                        parent.borrow_mut().subscribe(msg);
                    }
                }
            }
        }
    }

    fn render_lazy(&self, before: &Html) -> Option<Node> {
        match before {
            Html::TextNode(text) => Some(Node::Text(text.clone())),
            Html::None => None,
            Html::ComponentNode(component) => component.borrow_mut().render(false),
            Html::ComponentBuilder { .. } => None,
            Html::ElementNode {
                tag_name,
                attributes,
                events: _,
                children,
                component_id: _,
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

    fn render_force(&self, before: Html, after: &mut Html) -> Option<Node> {
        match after {
            Html::None => None,
            Html::TextNode(text) => Some(Node::Text(text.clone())),
            Html::ComponentNode(component) => component.borrow_mut().render(true),
            Html::ComponentBuilder { builder, children } => {
                if let Some(component_builder) = builder.take() {
                    let component = if let Html::ComponentNode(component) = before {
                        component_builder(Some(component))
                    } else {
                        component_builder(None)
                    };
                    component.borrow_mut().set_parent(Weak::clone(&self.this));
                    self.set_component_id(children);
                    let children = children.drain(..).collect::<Vec<_>>();
                    component.borrow_mut().set_children(children);
                    let node = component.borrow_mut().render(true);
                    *after = Html::ComponentNode(Rc::clone(&component));
                    node
                } else if let Html::ComponentNode(component) = before {
                    *after = Html::ComponentNode(Rc::clone(&component));
                    component.borrow_mut().render(true)
                } else {
                    None
                }
            }
            Html::ElementNode {
                tag_name,
                children,
                attributes,
                events,
                component_id,
            } => {
                let children = match before {
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
                            .filter_map(|(child, cache)| self.render_force(cache, child))
                            .collect::<Vec<Node>>()
                    }
                    _ => children
                        .into_iter()
                        .filter_map(|child| self.render_force(Html::none(), child))
                        .collect::<Vec<_>>(),
                };

                let component_id = if let Some(component_id) = component_id {
                    *component_id
                } else {
                    self.component_id
                };

                let mut dom_events = Events::new();
                for (name, handlers) in &mut events.handlers {
                    for handler in handlers.drain(..) {
                        let this = Weak::clone(&self.this);

                        dom_events.add(name, move |e| {
                            if let Some(this) = this.upgrade() {
                                let msg = handler(e);
                                let msg = Message {
                                    payload: msg,
                                    component_id: component_id,
                                };
                                this.borrow_mut().update(msg);
                                state::render();
                            }
                        });
                    }
                }

                if let Some(rendered) = events.rendered.take() {
                    let this = Weak::clone(&self.this);

                    dom_events.rendered = Some(Box::new(move |e| {
                        if let Some(this) = this.upgrade() {
                            let msg = rendered(e);
                            let msg = Message {
                                payload: msg,
                                component_id: component_id,
                            };
                            this.borrow_mut().update(msg);
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

    fn set_component_id(&self, html: &mut Vec<Html>) {
        for html_node in html {
            match html_node {
                Html::ElementNode { component_id, .. } => {
                    *component_id = Some(self.component_id);
                }
                Html::ComponentBuilder { children, .. } => {
                    self.set_component_id(children);
                }
                _ => {}
            }
        }
    }
}

impl<Props, Msg: 'static, Sub> Composed for ComposedComponent<Props, Msg, Sub> {
    fn update(&mut self, msg: Message) {
        if msg.component_id == self.component_id {
            if let Ok(msg) = msg.payload.downcast::<Msg>() {
                self.is_updated = true;
                let cmd = self.component.update(*msg);
                self.proc_cmd(cmd);
            }
        } else if let Some(parent) = self.parent.upgrade() {
            parent.borrow_mut().update(msg);
        }
    }

    fn subscribe(&mut self, msg: Box<dyn Any>) {
        self.update(Message {
            payload: msg,
            component_id: self.component_id,
        });
    }

    fn render(&mut self, is_forced: bool) -> Option<Node> {
        if self.is_updated || is_forced {
            let mut children = self.children.clone();
            std::mem::swap(&mut children, &mut self.children);
            let mut html = self.component.render(children);
            let mut rendered_cache = Html::none();
            std::mem::swap(&mut self.rendered_cache, &mut rendered_cache);
            let node = self.render_force(rendered_cache, &mut html);
            self.rendered_cache = html;
            self.is_updated = false;
            node
        } else {
            self.render_lazy(&self.rendered_cache)
        }
    }

    fn set_children(&mut self, children: Vec<Html>) {
        self.is_updated = true;
        self.children = children;
    }

    fn set_this(&mut self, this: Weak<RefCell<Box<dyn Composed>>>) {
        self.is_updated = true;
        self.this = this;
        if let Some(builder) = self.builder.take() {
            self.proc_cmd(builder.cmd);
            for batch in builder.batches {
                let this = Weak::clone(&self.this);
                let component_id = self.component_id;
                batch(Box::new(move |msg| {
                    if let Some(this) = this.upgrade() {
                        let msg = Message {
                            payload: Box::new(msg),
                            component_id: component_id,
                        };
                        this.borrow_mut().update(msg);
                        state::render();
                    }
                }))
            }
        }
    }

    fn set_parent(&mut self, parent: Weak<RefCell<Box<dyn Composed>>>) {
        self.is_updated = true;
        self.parent = parent;
    }
}
