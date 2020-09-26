use super::super::html::Html;
use super::super::Node;
use super::Batch;
use super::Cmd;
use super::Component;
use super::ImplComponent;
use std::any::Any;
use std::cell::RefCell;
use std::clone::Clone;
use std::rc::Rc;
use std::rc::Weak;

pub trait Controller: 'static {
    fn init(&self);
    fn update(&self, msg: Box<dyn Any>);
    fn render(&self) -> Option<Node>;
    fn clone(&self) -> Box<dyn Controller>;
    fn set_children(&self, children: Vec<Html>);
    fn set_parent(&self, parent: Box<dyn Controller>);
    fn set_state(&self, state: Box<dyn Any>);
    fn take_state(&self) -> Option<Box<dyn Any>>;
    fn set_key(&self, key: u64);
    fn key(&self) -> u64;
}

pub struct RcController<Props: 'static, Sub: 'static>(Rc<RefCell<Box<dyn Component<Props, Sub>>>>);

pub struct WeakController<Props: 'static, Sub: 'static>(
    Weak<RefCell<Box<dyn Component<Props, Sub>>>>,
);

impl<Props, Sub> RcController<Props, Sub> {
    pub fn new<Msg: 'static, State: 'static>(
        init: impl Fn(Option<State>, Props) -> (State, Cmd<Msg, Sub>, Vec<Batch<Msg>>) + 'static,
        update: impl Fn(&mut State, Msg) -> Cmd<Msg, Sub> + 'static,
        render: impl Fn(&State, Vec<Html>) -> Html + 'static,
    ) -> Self {
        let implement = Rc::new(RefCell::new(
            Box::new(ImplComponent::new(init, update, render)) as Box<dyn Component<Props, Sub>>,
        ));
        implement
            .borrow_mut()
            .set_this(Box::new(WeakController(Rc::downgrade(&implement))));
        Self(implement)
    }

    pub fn with(&self, props: Props) -> Self {
        let this = Self(Rc::clone(&self.0));
        this.0.borrow_mut().set_props(props);
        this
    }

    pub fn subscribe<M: 'static>(self, messenger: impl FnOnce(Sub) -> M + 'static) -> Self {
        self.0
            .borrow_mut()
            .set_messenger(Box::new(|sub| Box::new(messenger(sub))));
        self
    }
}

impl<Props, Sub> Controller for RcController<Props, Sub> {
    fn init(&self) {
        self.0.borrow_mut().init();
    }

    fn update(&self, msg: Box<dyn Any>) {
        self.0.borrow_mut().update(msg);
    }

    fn render(&self) -> Option<Node> {
        self.0.borrow_mut().render()
    }

    fn clone(&self) -> Box<dyn Controller> {
        let implement = Rc::clone(&self.0);
        Box::new(Self(implement))
    }

    fn set_children(&self, children: Vec<Html>) {
        self.0.borrow_mut().set_children(children);
    }

    fn set_parent(&self, parent: Box<dyn Controller>) {
        self.0.borrow_mut().set_parent(parent);
    }

    fn set_state(&self, state: Box<dyn Any>) {
        self.0.borrow_mut().set_state(state);
    }

    fn take_state(&self) -> Option<Box<dyn Any>> {
        self.0.borrow_mut().take_state()
    }

    fn set_key(&self, key: u64) {
        self.0.borrow_mut().set_key(key);
    }

    fn key(&self) -> u64 {
        self.0.borrow().key()
    }
}

impl<Props, Sub> WeakController<Props, Sub> {
    pub fn new() -> Self {
        Self(Weak::new())
    }
}

impl<Props, Sub> Controller for WeakController<Props, Sub> {
    fn init(&self) {
        if let Some(this) = self.0.upgrade() {
            this.borrow_mut().init();
        }
    }

    fn update(&self, msg: Box<dyn Any>) {
        if let Some(this) = self.0.upgrade() {
            this.borrow_mut().update(msg);
        }
    }

    fn render(&self) -> Option<Node> {
        if let Some(this) = self.0.upgrade() {
            this.borrow_mut().render()
        } else {
            None
        }
    }

    fn clone(&self) -> Box<dyn Controller> {
        let implement = Weak::clone(&self.0);
        Box::new(Self(implement))
    }

    fn set_children(&self, children: Vec<Html>) {
        if let Some(this) = self.0.upgrade() {
            this.borrow_mut().set_children(children);
        }
    }

    fn set_parent(&self, parent: Box<dyn Controller>) {
        if let Some(this) = self.0.upgrade() {
            this.borrow_mut().set_parent(parent);
        }
    }

    fn set_state(&self, state: Box<dyn Any>) {
        if let Some(this) = self.0.upgrade() {
            this.borrow_mut().set_state(state);
        }
    }

    fn take_state(&self) -> Option<Box<dyn Any>> {
        if let Some(this) = self.0.upgrade() {
            this.borrow_mut().take_state()
        } else {
            None
        }
    }

    fn set_key(&self, key: u64) {
        if let Some(this) = self.0.upgrade() {
            this.borrow_mut().set_key(key);
        }
    }

    fn key(&self) -> u64 {
        if let Some(this) = self.0.upgrade() {
            this.borrow().key()
        } else {
            0
        }
    }
}
