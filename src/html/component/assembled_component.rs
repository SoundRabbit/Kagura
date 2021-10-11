use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::{Rc, Weak};
mod render;
use super::*;
use crate::kagura::Node;

pub trait AssembledDemirootComponent {
    type ThisComp: Component;

    fn post(&mut self, msg: <Self::ThisComp as Component>::Msg);
    fn update(&mut self, msg: <Self::ThisComp as Component>::Msg);
}

pub trait AssembledChildComponent {
    type DemirootComp: Component;

    fn as_any(&mut self) -> &mut dyn std::any::Any;

    fn set_demiroot(
        &mut self,
        demiroot: Option<
            Weak<RefCell<dyn AssembledDemirootComponent<ThisComp = Self::DemirootComp>>>,
        >,
    );

    fn on_assemble(&mut self);
    fn on_load(&mut self);

    fn load_lazy_cmd(&mut self) -> Option<<Self::DemirootComp as Component>::Msg>;

    fn render(&mut self, children: Vec<Html<Self::DemirootComp>>) -> VecDeque<Node>;
}

pub struct AssembledComponentInstance<ThisComp: Update + Render, DemirootComp: Component> {
    demiroot: Option<Weak<RefCell<dyn AssembledDemirootComponent<ThisComp = DemirootComp>>>>,
    this: Weak<RefCell<Self>>,
    data: Rc<RefCell<ThisComp>>,
    props: ThisComp::Props,
    sub_mapper: sub::Mapper<ThisComp::Sub, DemirootComp::Msg>,
    is_updated: bool,
    lazy_cmd: VecDeque<AssembledCmd<ThisComp, DemirootComp::Msg>>,
    children_tree: ComponentTree<ThisComp, DemirootComp>,
    children: Vec<ChildComponent<ThisComp, DemirootComp>>,
}

enum ComponentTree<ThisComp: Component, DemirootComp: Component> {
    None,
    Fragment(VecDeque<Self>),
    ThisComp(Rc<RefCell<dyn AssembledChildComponent<DemirootComp = ThisComp>>>),
    DemirootComp(Rc<RefCell<dyn AssembledChildComponent<DemirootComp = DemirootComp>>>),
}

enum ChildComponent<ThisComp: Component, DemirootComp: Component> {
    ThisComp(Rc<RefCell<dyn AssembledChildComponent<DemirootComp = ThisComp>>>),
    DemirootComp(Rc<RefCell<dyn AssembledChildComponent<DemirootComp = DemirootComp>>>),
}

enum AssembledCmd<ThisComp: Component, DemirootMsg> {
    None,
    Sub(ThisComp::Sub),
    Task(Box<dyn FnOnce(TaskResolver<ThisComp::Msg>)>),
    Batch(Box<dyn FnOnce(BatchResolver<ThisComp::Msg>)>),
    Msg(DemirootMsg),
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
        if let Some(sub) = self.load_cmd(cmd) {
            if let Some(demiroot) = self.demiroot() {
                if let Some(msg) = self.sub_mapper.try_map(sub) {
                    demiroot.borrow_mut().post(msg);
                }
            }
        }
    }

    fn lazy_update(&mut self, msg: ThisComp::Msg) {
        let cmd = self.data.borrow_mut().update(&self.props, msg);
        self.is_updated = true;
        self.lazy_cmd.push_back(AssembledCmd::from(cmd));
    }

    fn load_cmd(&mut self, cmd: Cmd<ThisComp>) -> Option<ThisComp::Sub> {
        match cmd {
            Cmd::None => None,
            Cmd::Sub(sub) => Some(sub),
            Cmd::Task(task) => {
                let this = Weak::clone(&self.this);
                let resolver = Box::new(move |msg| {
                    if let Some(this) = this.upgrade() {
                        this.borrow_mut().update(msg);
                        crate::state::render();
                    }
                });
                crate::env::add_task(move || {
                    task(resolver);
                });
                None
            }
            Cmd::Batch(batch) => {
                let this = Weak::clone(&self.this);
                let resolver = Box::new(move |msg| {
                    if let Some(this) = this.upgrade() {
                        this.borrow_mut().update(msg);
                        crate::state::render();
                    }
                });
                crate::env::add_task(move || {
                    batch(resolver);
                });
                None
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
}

impl<ThisComp: Update + Render, DemirootComp: Component> AssembledDemirootComponent
    for AssembledComponentInstance<ThisComp, DemirootComp>
{
    type ThisComp = ThisComp;

    fn post(&mut self, msg: ThisComp::Msg) {
        self.force_update(msg);
    }

    fn update(&mut self, msg: ThisComp::Msg) {
        self.force_update(msg);
    }
}

impl<ThisComp: Update + Render, DemirootComp: Component> AssembledChildComponent
    for AssembledComponentInstance<ThisComp, DemirootComp>
{
    type DemirootComp = DemirootComp;

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn set_demiroot(
        &mut self,
        demiroot: Option<
            Weak<RefCell<dyn AssembledDemirootComponent<ThisComp = Self::DemirootComp>>>,
        >,
    ) {
        self.demiroot = demiroot;
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

    fn load_lazy_cmd(&mut self) -> Option<DemirootComp::Msg> {
        while let Some(cmd) = self.lazy_cmd.pop_front() {
            if let AssembledCmd::Msg(msg) = cmd {
                return Some(msg);
            } else if let Some(sub) = self.load_cmd(cmd.into()) {
                if let Some(msg) = self.sub_mapper.try_map(sub) {
                    return Some(msg);
                }
            }
        }
        None
    }

    fn render(&mut self, children: Vec<Html<Self::DemirootComp>>) -> VecDeque<Node> {
        self.render(children)
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

impl<ThisComp: Component, DemirootMsg> From<Cmd<ThisComp>> for AssembledCmd<ThisComp, DemirootMsg> {
    fn from(cmd: Cmd<ThisComp>) -> Self {
        match cmd {
            Cmd::None => Self::None,
            Cmd::Sub(sub) => Self::Sub(sub),
            Cmd::Task(task) => Self::Task(task),
            Cmd::Batch(batch) => Self::Batch(batch),
        }
    }
}

impl<ThisComp: Component, DemirootMsg> Into<Cmd<ThisComp>> for AssembledCmd<ThisComp, DemirootMsg> {
    fn into(self) -> Cmd<ThisComp> {
        match self {
            Self::None => Cmd::None,
            Self::Sub(sub) => Cmd::Sub(sub),
            Self::Task(task) => Cmd::Task(task),
            Self::Batch(batch) => Cmd::Batch(batch),
            Self::Msg(..) => Cmd::None,
        }
    }
}
