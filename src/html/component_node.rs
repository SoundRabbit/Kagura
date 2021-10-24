use std::cell::RefCell;
use std::rc::Rc;

use super::*;
use component::assembled_component::AssembledComponentInstance;
use component::{AssembledChildComponent, Render, Update};

impl<ThisComp: Update + Render, DemirootComp: Component>
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

impl<ThisComp: Update + Render, DemirootComp: Component> PackedComponentNode
    for PackedComponentNodeInstance<ThisComp, DemirootComp>
{
    type DemirootComp = DemirootComp;
    fn assemble(
        &mut self,
        before: Option<Rc<RefCell<dyn AssembledChildComponent<DemirootComp = Self::DemirootComp>>>>,
    ) -> (
        Rc<RefCell<dyn AssembledChildComponent<DemirootComp = Self::DemirootComp>>>,
        Vec<Html<Self::DemirootComp>>,
    ) {
        let before = before.and_then(|before| before.borrow_mut().as_any());
        let before = before.and_then(|before| {
            let before = Rc::clone(&before);

            let mut before = before.borrow_mut();
            if let Some(before_instance) =
                before.downcast_mut::<AssembledComponentInstance<ThisComp, DemirootComp>>()
            {
                let data = self.data.take().unwrap();
                before_instance.set_props(data.props);
                before_instance.set_sub_mapper(data.sub_mapper);
                before_instance.on_load();
                Some((before_instance.this().upgrade().unwrap(), data.children))
            } else {
                None
            }
        });

        if let Some((data, children)) = before {
            (data, children)
        } else {
            let data = self.data.take().unwrap();
            let props = data.props;
            let sub_mapper = data.sub_mapper;
            let children = data.children;
            let data = (data.constructor)(&props);
            let data =
                AssembledComponentInstance::new_ref(Rc::new(RefCell::new(data)), props, sub_mapper);

            data.borrow_mut().on_assemble();

            (data, children)
        }
    }
}

impl<ThisComp: Update + Render, DemirootComp: Component>
    PrepackedComponentNodeInstance<ThisComp, DemirootComp>
{
    pub fn new(
        data: Rc<RefCell<ThisComp>>,
        props: ThisComp::Props,
        sub_mapper: component::Sub<ThisComp::Sub, DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self {
            data: Some(PrepackedComponentNodeInstanceData {
                data,
                props,
                sub_mapper,
                children,
            }),
        }
    }
}

impl<ThisComp: Update + Render, DemirootComp: Component> PrepackedComponentNode
    for PrepackedComponentNodeInstance<ThisComp, DemirootComp>
{
    type DemirootComp = DemirootComp;

    fn assemble(
        &mut self,
        before: Option<Rc<RefCell<dyn AssembledChildComponent<DemirootComp = Self::DemirootComp>>>>,
    ) -> (
        Rc<RefCell<dyn AssembledChildComponent<DemirootComp = Self::DemirootComp>>>,
        Vec<Html<Self::DemirootComp>>,
    ) {
        let before = before.and_then(|before| before.borrow_mut().as_any());
        let before = before.and_then(|before| {
            let before = Rc::clone(&before);

            let mut before = before.borrow_mut();
            if let Some(before_instance) =
                before.downcast_mut::<AssembledComponentInstance<ThisComp, DemirootComp>>()
            {
                let data = self.data.take().unwrap();
                before_instance.set_data(data.data);
                before_instance.set_props(data.props);
                before_instance.set_sub_mapper(data.sub_mapper);
                before_instance.on_load();
                Some((before_instance.this().upgrade().unwrap(), data.children))
            } else {
                None
            }
        });

        if let Some((data, children)) = before {
            (data, children)
        } else {
            let data = self.data.take().unwrap();
            let props = data.props;
            let sub_mapper = data.sub_mapper;
            let children = data.children;
            let data = data.data;
            let data = AssembledComponentInstance::new_ref(data, props, sub_mapper);

            data.borrow_mut().on_assemble();

            (data, children)
        }
    }
}
