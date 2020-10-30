use super::html::Html;
use super::{Events, Node};
use crate::{state, task};
use std::any::{self, Any};
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

    fn with_children(
        props: Self::Props,
        sub_map: Subscription<Self::Sub>,
        children: Vec<Html>,
    ) -> Html {
        Html::component::<Self, Self::Props, Self::Msg, Self::Sub>(props, sub_map, children)
    }

    fn with_child(props: Self::Props, sub_map: Subscription<Self::Sub>, child: Html) -> Html {
        Html::component::<Self, Self::Props, Self::Msg, Self::Sub>(props, sub_map, vec![child])
    }

    fn empty(props: Self::Props, sub_map: Subscription<Self::Sub>) -> Html {
        Html::component::<Self, Self::Props, Self::Msg, Self::Sub>(props, sub_map, vec![])
    }
}

pub trait Composed: Any + 'static {
    fn init(&mut self, props: Box<dyn Any>);
    fn update(&mut self, msg: Box<dyn Any>);
    fn render(&mut self, is_forced: bool) -> Vec<Node>;
    fn set_children(&mut self, children: Vec<Html>);
    fn set_this(&mut self, this: Weak<RefCell<Box<dyn Composed>>>);
    fn set_parent(&mut self, parent: Weak<RefCell<Box<dyn Composed>>>);
    fn is(&self, type_id: any::TypeId) -> bool;
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

pub struct Subscription<Sub: 'static> {
    payload: Option<Box<dyn FnOnce(Sub) -> Box<dyn Any>>>,
}

pub struct ComposedComponent<Props: 'static, Msg: 'static, Sub: 'static> {
    component: Box<dyn Component<Props = Props, Msg = Msg, Sub = Sub>>,
    this: Weak<RefCell<Box<dyn Composed>>>,
    parent: Weak<RefCell<Box<dyn Composed>>>,
    children: Vec<Html>,
    rendered_cache: Html,
    is_updated: bool,
    sub_map: Option<Box<dyn FnOnce(Sub) -> Box<dyn Any>>>,
    builder: Option<ComponentBuilder<Msg, Sub>>,
    self_id: any::TypeId,
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

impl<Sub> Subscription<Sub> {
    pub fn new<Msg: 'static>(mapper: impl FnOnce(Sub) -> Msg + 'static) -> Self {
        Self {
            payload: Some(Box::new(move |sub| Box::new(mapper(sub)) as Box<dyn Any>)),
        }
    }

    pub fn none() -> Self {
        Self { payload: None }
    }
}

impl<Props: 'static, Msg: 'static, Sub: 'static> ComposedComponent<Props, Msg, Sub> {
    pub fn new<C: Component<Props = Props, Msg = Msg, Sub = Sub> + 'static>(
        component: C,
        builder: ComponentBuilder<Msg, Sub>,
        sub_map: Subscription<Sub>,
    ) -> Rc<RefCell<Box<dyn Composed>>> {
        let this = Self {
            component: Box::new(component),
            this: Weak::new(),
            parent: Weak::new(),
            children: Vec::new(),
            rendered_cache: Html::none(),
            is_updated: true,
            sub_map: sub_map.payload,
            builder: Some(builder),
            self_id: any::TypeId::of::<C>(),
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
                task::add(move || {
                    worker(Box::new(move |msg| {
                        if let Some(this) = this.upgrade() {
                            this.borrow_mut().update(Box::new(msg));
                            state::render();
                        }
                    }))
                })
            }
            Cmd::Sub(sub) => {
                if let Some(parent) = self.parent.upgrade() {
                    if let Some(sub_map) = self.sub_map.take() {
                        let msg = sub_map(sub);
                        parent.borrow_mut().update(msg);
                    }
                }
            }
        }
    }

    fn render_lazy(&self, before: &Html) -> Vec<Node> {
        match before {
            Html::TextNode(text) => vec![Node::Text(text.clone())],
            Html::None => vec![],
            Html::ComponentNode(component) => component.borrow_mut().render(false),
            Html::Fragment(children) => children
                .iter()
                .map(|child| self.render_lazy(child))
                .flatten()
                .collect::<Vec<_>>(),
            Html::ComponentBuilder { .. } => vec![],
            Html::ElementNode {
                tag_name,
                attributes,
                events: _,
                children,
                parent: _,
            } => {
                let children = children
                    .into_iter()
                    .map(|child| self.render_lazy(child))
                    .flatten()
                    .collect::<Vec<Node>>();

                let dom_events = Events::new();

                vec![Node::element(
                    tag_name,
                    attributes.clone().into(),
                    dom_events,
                    children,
                    false,
                )]
            }
        }
    }

    fn render_force(&self, before: Html, after: &mut Html) -> Vec<Node> {
        match after {
            Html::None => vec![],
            Html::TextNode(text) => vec![Node::Text(text.clone())],
            Html::ComponentNode(component) => component.borrow_mut().render(true),
            Html::ComponentBuilder {
                builder,
                children,
                parent,
            } => {
                if let Some(component_builder) = builder.take() {
                    let component = if let Html::ComponentNode(component) = before {
                        component_builder(Some(component))
                    } else if let Html::ComponentBuilder { .. } = before {
                        panic!();
                    } else {
                        use wasm_bindgen::prelude::*;
                        web_sys::console::log_1(&JsValue::from(format!("{:?}", before)));
                        component_builder(None)
                    };
                    let parent = if let Some(parent) = parent {
                        Weak::clone(parent)
                    } else {
                        Weak::clone(&self.this)
                    };
                    component.borrow_mut().set_parent(parent);
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
                    vec![]
                }
            }
            Html::Fragment(children) => {
                let mut before = match before {
                    Html::Fragment(before) => before,
                    _ => vec![before],
                };
                while children.len() > before.len() {
                    before.push(Html::none());
                }
                let children = children
                    .into_iter()
                    .zip(before.into_iter())
                    .map(|(child, cache)| self.render_force(cache, child))
                    .flatten()
                    .collect::<Vec<_>>();
                children
            }
            Html::ElementNode {
                tag_name,
                children,
                attributes,
                events,
                parent,
            } => {
                let mut before = match before {
                    Html::ElementNode { children, .. } => children,
                    _ => vec![],
                };
                while children.len() > before.len() {
                    before.push(Html::none());
                }
                let children = children
                    .into_iter()
                    .zip(before.into_iter())
                    .map(|(child, cache)| self.render_force(cache, child))
                    .flatten()
                    .collect::<Vec<_>>();

                let parent = parent
                    .as_ref()
                    .map(|x| Weak::clone(x))
                    .unwrap_or(Weak::clone(&self.this));

                let mut dom_events = Events::new();
                for (name, handlers) in &mut events.handlers {
                    for handler in handlers.drain(..) {
                        let parent = Weak::clone(&parent);

                        dom_events.add(name, move |e| {
                            if let Some(parent) = parent.upgrade() {
                                let msg = handler(e);
                                parent.borrow_mut().update(msg);
                                state::render();
                            }
                        });
                    }
                }

                if let Some(rendered) = events.rendered.take() {
                    let parent = Weak::clone(&parent);

                    dom_events.rendered = Some(Box::new(move |e| {
                        if let Some(parent) = parent.upgrade() {
                            let msg = rendered(e);
                            parent.borrow_mut().update(msg);
                            state::render();
                        }
                    }))
                }

                vec![Node::element(
                    tag_name.as_str(),
                    attributes.clone().into(),
                    dom_events,
                    children,
                    true,
                )]
            }
        }
    }

    fn set_component_id(&self, html: &mut Vec<Html>) {
        for html_node in html {
            match html_node {
                Html::ElementNode {
                    parent, children, ..
                } => {
                    if parent.is_none() {
                        *parent = Some(Weak::clone(&self.this));
                    }
                    self.set_component_id(children);
                }
                Html::ComponentBuilder {
                    children, parent, ..
                } => {
                    if parent.is_none() {
                        *parent = Some(Weak::clone(&self.this));
                    }
                    self.set_component_id(children);
                }
                _ => {}
            }
        }
    }
}

impl<Props, Msg: 'static, Sub> Composed for ComposedComponent<Props, Msg, Sub> {
    fn init(&mut self, props: Box<dyn Any>) {
        if let Ok(props) = props.downcast::<Props>() {
            self.init(*props);
        }
    }

    fn update(&mut self, msg: Box<dyn Any>) {
        if let Ok(msg) = msg.downcast::<Msg>() {
            self.is_updated = true;
            let cmd = self.component.update(*msg);
            self.proc_cmd(cmd);
        }
    }

    fn render(&mut self, is_forced: bool) -> Vec<Node> {
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
                batch(Box::new(move |msg| {
                    if let Some(this) = this.upgrade() {
                        this.borrow_mut().update(Box::new(msg));
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

    fn is(&self, type_id: any::TypeId) -> bool {
        self.self_id == type_id
    }
}
