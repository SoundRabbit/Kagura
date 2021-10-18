use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::{Rc, Weak};
mod render;
mod wrapped_assembled_component;
use super::*;
use crate::kagura::Node;
use std::any::Any;
use wrapped_assembled_component::WrappedAssembledComponentInstance;

pub enum Msg<ThisComp: Component> {
    Wrapped(Box<dyn Any>),
    Data(ThisComp::Msg),
}

pub trait AssembledDemirootComponent {
    type ThisComp: Component;

    fn post(&mut self, msg: Msg<Self::ThisComp>);
    fn update(&mut self, msg: <Self::ThisComp as Component>::Msg);
    fn ref_node(&mut self, name: String, node: web_sys::Node);
}

pub trait AssembledChildComponent {
    type DemirootComp: Component;

    fn as_any(&mut self) -> Option<Rc<RefCell<dyn std::any::Any + 'static>>>;

    fn on_assemble(&mut self);
    fn on_load(&mut self);

    fn load_lazy_cmd(&mut self) -> Vec<Msg<Self::DemirootComp>>;

    fn render(&mut self, children: Vec<Html<Self::DemirootComp>>) -> VecDeque<Node>;

    fn set_demiroot(
        &mut self,
        demiroot: Option<
            Weak<RefCell<dyn AssembledDemirootComponent<ThisComp = Self::DemirootComp>>>,
        >,
    );
}

pub struct AssembledComponentInstance<ThisComp: Update + Render, DemirootComp: Component> {
    demiroot: Option<Weak<RefCell<dyn AssembledDemirootComponent<ThisComp = DemirootComp>>>>,
    this: Weak<RefCell<Self>>,
    data: Rc<RefCell<ThisComp>>,
    props: ThisComp::Props,
    sub_mapper: sub::Mapper<ThisComp::Sub, DemirootComp::Msg>,
    is_updated: bool,
    lazy_cmd: VecDeque<AssembledCmd<ThisComp, DemirootComp>>,
    children_tree: ComponentTree<ThisComp, DemirootComp>,
    children: Vec<ComponentTree<ThisComp, DemirootComp>>,
}

enum ComponentTree<ThisComp: Component, DemirootComp: Component> {
    None,
    TextNode,
    Fragment(VecDeque<Self>),
    Element(VecDeque<Self>),
    ThisComp(Rc<RefCell<dyn AssembledChildComponent<DemirootComp = ThisComp>>>),
    DemirootComp(Rc<RefCell<dyn AssembledChildComponent<DemirootComp = DemirootComp>>>),
}

enum AssembledCmd<ThisComp: Component, DemirootComp: Component> {
    None,
    Sub(ThisComp::Sub),
    Task(Box<dyn FnOnce(TaskResolver<ThisComp::Msg>)>),
    Batch(Box<dyn FnOnce(BatchResolver<ThisComp::Msg>)>),
    List(Vec<Cmd<ThisComp>>),
    Msg(Msg<DemirootComp>),
}

impl<ThisComp: Update + Render, DemirootComp: Component>
    AssembledComponentInstance<ThisComp, DemirootComp>
{
    pub fn new_ref(
        data: Rc<RefCell<ThisComp>>,
        props: ThisComp::Props,
        sub_mapper: Sub<ThisComp::Sub, DemirootComp::Msg>,
    ) -> Rc<RefCell<Self>> {
        let this = Self {
            demiroot: None,
            data: data,
            props,
            sub_mapper: sub::Mapper::from(sub_mapper),
            this: Weak::new(),
            is_updated: true,
            lazy_cmd: VecDeque::new(),
            children_tree: ComponentTree::None,
            children: vec![],
        };

        let this = Rc::new(RefCell::new(this));
        this.borrow_mut().this = Rc::downgrade(&this);
        this
    }

    pub fn set_props(&mut self, props: ThisComp::Props) {
        self.props = props;
    }

    pub fn set_sub_mapper(&mut self, sub_mapper: Sub<ThisComp::Sub, DemirootComp::Msg>) {
        self.sub_mapper = sub::Mapper::from(sub_mapper);
    }

    fn force_update(&mut self, msg: ThisComp::Msg) {
        let cmd = self.data.borrow_mut().update(&self.props, msg);
        self.is_updated = true;
        self.load_cmd(cmd, false);
    }

    fn send_sub(&mut self, sub: ThisComp::Sub) {
        if let Some(msg) = self.sub_mapper.try_map(sub) {
            self.send_msg(Msg::Data(msg));
        }
    }

    fn send_msg(&self, msg: Msg<DemirootComp>) {
        if let Some(demiroot) = self.demiroot() {
            demiroot.borrow_mut().post(msg);
        }
    }

    fn lazy_update(&mut self, msg: ThisComp::Msg) {
        let cmd = self.data.borrow_mut().update(&self.props, msg);
        self.is_updated = true;
        self.lazy_cmd.push_back(AssembledCmd::from(cmd));
    }

    fn lazy_post(&mut self, msg: Msg<ThisComp>) {
        match msg {
            Msg::Data(msg) => {
                self.lazy_update(msg);
            }
            Msg::Wrapped(msg) => {
                if let Ok(msg) = msg.downcast::<Msg<DemirootComp>>() {
                    self.lazy_cmd.push_back(AssembledCmd::Msg(*msg))
                }
            }
        }
    }

    fn load_cmd(&mut self, cmd: Cmd<ThisComp>, is_lazy_sub: bool) -> Vec<ThisComp::Sub> {
        match cmd {
            Cmd::None => vec![],
            Cmd::Sub(sub) => {
                if is_lazy_sub {
                    vec![sub]
                } else {
                    self.send_sub(sub);
                    vec![]
                }
            }
            Cmd::Task(task) => {
                let this = Weak::clone(&self.this);
                let resolver = Box::new(move |msg| {
                    if let Some(this) = this.upgrade() {
                        this.borrow_mut().force_update(msg);
                        crate::state::render();
                    }
                });
                crate::env::add_task(move || {
                    task(resolver);
                });
                vec![]
            }
            Cmd::Batch(batch) => {
                let this = Weak::clone(&self.this);
                let resolver = Box::new(move |msg| {
                    if let Some(this) = this.upgrade() {
                        this.borrow_mut().force_update(msg);
                        crate::state::render();
                    }
                });
                crate::env::add_task(move || {
                    batch(resolver);
                });
                vec![]
            }
            Cmd::List(cmds) => {
                let mut subs = vec![];

                for cmd in cmds {
                    subs.append(&mut self.load_cmd(cmd, is_lazy_sub));
                }

                subs
            }
        }
    }

    fn demiroot(
        &self,
    ) -> Option<Rc<RefCell<dyn AssembledDemirootComponent<ThisComp = DemirootComp>>>> {
        self.demiroot
            .as_ref()
            .and_then(|demiroot| demiroot.upgrade())
    }

    fn demiroot_clone(
        &self,
    ) -> Option<Weak<RefCell<dyn AssembledDemirootComponent<ThisComp = DemirootComp>>>> {
        self.demiroot
            .as_ref()
            .map(|demiroot| Weak::clone(&demiroot))
    }

    fn this_as_demiroot(
        &self,
    ) -> Option<Weak<RefCell<dyn AssembledDemirootComponent<ThisComp = ThisComp>>>> {
        let this = Weak::clone(&self.this);
        Some(this)
    }

    pub fn this(&self) -> Weak<RefCell<Self>> {
        Weak::clone(&self.this)
    }
}

impl<ThisComp: Update + Render, DemirootComp: Component> AssembledDemirootComponent
    for AssembledComponentInstance<ThisComp, DemirootComp>
{
    type ThisComp = ThisComp;

    fn post(&mut self, msg: Msg<ThisComp>) {
        match msg {
            Msg::Data(msg) => {
                self.force_update(msg);
            }
            Msg::Wrapped(msg) => {
                if let Ok(msg) = msg.downcast::<Msg<DemirootComp>>() {
                    self.send_msg(*msg);
                }
            }
        }
    }

    fn update(&mut self, msg: ThisComp::Msg) {
        self.force_update(msg);
    }

    fn ref_node(&mut self, name: String, node: web_sys::Node) {
        let cmd = self.data.borrow_mut().ref_node(&self.props, name, node);
        self.load_cmd(cmd, false);
    }
}

impl<ThisComp: Update + Render, DemirootComp: Component> AssembledChildComponent
    for AssembledComponentInstance<ThisComp, DemirootComp>
{
    type DemirootComp = DemirootComp;

    fn as_any(&mut self) -> Option<Rc<RefCell<dyn std::any::Any>>> {
        if let Some(this) = self.this.upgrade() {
            Some(this as Rc<RefCell<dyn std::any::Any>>)
        } else {
            None
        }
    }

    fn on_assemble(&mut self) {
        let cmd = self.data.borrow_mut().on_assemble(&self.props);
        self.is_updated = true;
        self.lazy_cmd.push_back(AssembledCmd::from(cmd));
    }

    fn on_load(&mut self) {
        let cmd = self.data.borrow_mut().on_load(&self.props);
        self.is_updated = true;
        self.lazy_cmd.push_back(AssembledCmd::from(cmd));
    }

    fn load_lazy_cmd(&mut self) -> Vec<Msg<DemirootComp>> {
        let mut res = vec![];
        while let Some(cmd) = self.lazy_cmd.pop_front() {
            if let AssembledCmd::Msg(msg) = cmd {
                res.push(msg);
            } else {
                let subs = self.load_cmd(cmd.into(), true);
                for sub in subs {
                    if let Some(msg) = self.sub_mapper.try_map(sub) {
                        res.push(Msg::Data(msg));
                    }
                }
            }
        }
        res
    }

    fn render(&mut self, children: Vec<Html<Self::DemirootComp>>) -> VecDeque<Node> {
        self.render(children)
    }

    fn set_demiroot(
        &mut self,
        demiroot: Option<
            Weak<RefCell<dyn AssembledDemirootComponent<ThisComp = Self::DemirootComp>>>,
        >,
    ) {
        self.demiroot = demiroot;
    }
}

impl<ThisComp: Component, DemirootComp: Component> ComponentTree<ThisComp, DemirootComp> {
    fn into_deq(self) -> VecDeque<Self> {
        match self {
            Self::Fragment(x) => x,
            Self::None => VecDeque::new(),
            _ => vec![self].into_iter().collect(),
        }
    }
}

impl<ThisComp: Component, DemirootComp: Component> From<Cmd<ThisComp>>
    for AssembledCmd<ThisComp, DemirootComp>
{
    fn from(cmd: Cmd<ThisComp>) -> Self {
        match cmd {
            Cmd::None => Self::None,
            Cmd::Sub(sub) => Self::Sub(sub),
            Cmd::Task(task) => Self::Task(task),
            Cmd::Batch(batch) => Self::Batch(batch),
            Cmd::List(cmds) => Self::List(cmds),
        }
    }
}

impl<ThisComp: Component, DemirootComp: Component> Into<Cmd<ThisComp>>
    for AssembledCmd<ThisComp, DemirootComp>
{
    fn into(self) -> Cmd<ThisComp> {
        match self {
            Self::None => Cmd::None,
            Self::Sub(sub) => Cmd::Sub(sub),
            Self::Task(task) => Cmd::Task(task),
            Self::Batch(batch) => Cmd::Batch(batch),
            Self::List(cmds) => Cmd::List(cmds),
            Self::Msg(..) => Cmd::None,
        }
    }
}
