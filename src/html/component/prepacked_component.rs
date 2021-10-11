use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use super::*;

pub struct PrepackedComponent<ThisComp: Update + Render> {
    data: Rc<RefCell<ThisComp>>,
}

impl<ThisComp: Update + Render> PrepackedComponent<ThisComp> {
    pub fn new(data: ThisComp) -> Self {
        Self {
            data: Rc::new(RefCell::new(data)),
        }
    }

    pub fn borrow(&self) -> Ref<ThisComp> {
        self.borrow()
    }

    pub fn borrow_mut(&mut self) -> RefMut<ThisComp> {
        self.borrow_mut()
    }

    pub fn with_children<DemirootComp: Component>(
        &self,
        props: ThisComp::Props,
        sub: Sub<ThisComp::Sub, DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Html<DemirootComp> {
        let assembled = AssembledComponentInstance::new_ref(Rc::clone(&self.data), props, sub);
        Html::ComponentNode(ComponentNode::AssembledComponentNode(
            AssembledComponentNode::new(assembled, children),
        ))
    }
}
