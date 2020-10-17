use super::html::Html;
use super::{Attributes, Events, Node};
use crate::state;
use std::any::Any;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub trait Component {
    type Props;
    type Msg;
    type Sub;
    fn init(&mut self, props: Self::Props);
    fn update(&mut self, msg: Self::Msg);
    fn render(&self);
}

pub trait Constructor {
    type Props;
    fn new(props: Self::Props) -> Self;
}

pub trait Composed {
    fn update(&mut self, msg: Message);
    fn render(&mut self) -> Option<Node>;
    fn set_children(&mut self, children: Vec<Html>);
    fn set_this(&mut self, this: Weak<RefCell<Box<dyn Composed>>>);
}

pub struct Message {
    pub payload: Box<dyn Any>,
    pub component_id: u32,
}

pub struct ComposedComponent<Props: 'static, Msg: 'static, Sub: 'static> {
    component_id: u32,
    component: Box<dyn Component<Props = Props, Msg = Msg, Sub = Sub>>,
    this: Weak<RefCell<Box<dyn Composed>>>,
    parent: Weak<RefCell<Box<dyn Composed>>>,
    children: Vec<Html>,
    rendered_cache: Html,
    is_updated: bool,
}

impl<Props: 'static, Msg: 'static, Sub: 'static> ComposedComponent<Props, Msg, Sub> {
    pub fn new(
        component_id: u32,
        component: impl Component<Props = Props, Msg = Msg, Sub = Sub> + 'static,
    ) -> Rc<RefCell<Box<dyn Composed>>> {
        let this = Self {
            component_id: component_id,
            component: Box::new(component),
            this: Weak::new(),
            parent: Weak::new(),
            children: Vec::new(),
            rendered_cache: Html::none(),
            is_updated: true,
        };
        let this = Rc::new(RefCell::new(Box::new(this) as Box<dyn Composed>));
        let weak_this = Rc::downgrade(&this);
        this.borrow_mut().set_this(weak_this);
        this
    }

    pub fn render_force(&mut self, after: &mut Html) -> Option<Node> {
        match after {
            Html::None => None,
            Html::TextNode(text) => Some(Node::Text(text.clone())),
            Html::ComponentBuilder { builder, children } => {
                if let Some(component_builder) = builder.take() {
                    let component = component_builder(None);
                    self.set_component_id(children);
                    let children = children.drain(..).collect::<Vec<_>>();
                    component.borrow_mut().set_children(children);
                    let node = component.borrow_mut().render();
                    *after = Html::ComponentNode(Rc::clone(&component));
                    node
                } else {
                    None
                }
            }
            Html::ComponentNode(component) => component.borrow_mut().render(),
            Html::ElementNode {
                tag_name,
                children,
                attributes,
                events,
                component_id,
            } => {
                let children = children
                    .into_iter()
                    .filter_map(|child| self.render_force(child))
                    .collect::<Vec<_>>();

                let mut dom_events = Events::new();
                for (name, handlers) in &mut events.handlers {
                    for handler in handlers.drain(..) {
                        let this = Weak::clone(&self.this);
                        let component_id = self.component_id;

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
                self.component.update(*msg);
            }
        } else if let Some(parent) = self.parent.upgrade() {
            parent.borrow_mut().update(msg);
        }
    }

    fn render(&mut self) -> Option<Node> {
        unimplemented!();
    }

    fn set_children(&mut self, children: Vec<Html>) {
        self.children = children;
    }

    fn set_this(&mut self, this: Weak<RefCell<Box<dyn Composed>>>) {
        self.this = this;
    }
}
