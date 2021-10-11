use std::any::Any;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use super::*;
use component::AssembledChildComponent;

impl<ThisComp: Component, DemirootComp: Component>
    PackedComponentNodeInstance<ThisComp, DemirootComp>
{
    pub fn new(
        constructor: fn(&ThisComp::Props) -> ThisComp,
        props: ThisComp::Props,
        sub_mapper: component::Sub<ThisComp::Sub, DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self {
            data: Some(PackedComponentNodeInstanceData {
                constructor,
                props,
                sub_mapper,
                children,
            }),
        }
    }
}

impl<ThisComp: Component, DemirootComp: Component> PackedComponentNode
    for PackedComponentNodeInstance<ThisComp, DemirootComp>
{
    type DemirootComp = DemirootComp;

    fn wrap(&mut self) -> Box<dyn WrappedPackedComponentNode> {
        let data = self.data.take();
        Box::new(WrappedPackedComponentNodeInstance {
            data: Box::new(Self { data }),
        })
    }

    fn assemble(
        &mut self,
        before: Option<Rc<RefCell<dyn AssembledChildComponent<DemirootComp = Self::DemirootComp>>>>,
    ) -> AssembledComponentNode<Self::DemirootComp> {
        unimplemented!();
    }
}

impl<SuperDemirootComp: Component> WrappedPackedComponentNodeInstance<SuperDemirootComp> {
    pub fn assemble(
        &mut self,
        before: Option<Rc<RefCell<dyn AssembledChildComponent<DemirootComp = SuperDemirootComp>>>>,
    ) -> AssembledComponentNode<SuperDemirootComp> {
        self.data.assemble(before)
    }
}

impl<SuperDemirootComp: Component> WrappedPackedComponentNode
    for WrappedPackedComponentNodeInstance<SuperDemirootComp>
{
}

impl<DemirootComp: Component> AssembledComponentNode<DemirootComp> {
    pub fn new(
        data: Rc<RefCell<dyn AssembledChildComponent<DemirootComp = DemirootComp>>>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self { data, children }
    }

    pub fn wrap(self) -> Box<dyn WrappedAssembledComponentNode> {
        Box::new(WrappedAssembledComponentNodeInstance { data: Some(self) })
    }
}

impl<SuperDemirootComp: Component> WrappedAssembledComponentNodeInstance<SuperDemirootComp> {
    pub fn take(&mut self) -> AssembledComponentNode<SuperDemirootComp> {
        self.data.take().unwrap()
    }
}

impl<SuperDemirootComp: Component> WrappedAssembledComponentNode
    for WrappedAssembledComponentNodeInstance<SuperDemirootComp>
{
}
