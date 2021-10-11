use std::cell::RefCell;
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

    pub fn map<T>(&self, f: impl FnOnce(&ThisComp) -> T) -> T {
        f(&self.data.borrow())
    }

    pub fn map_mut<T>(&mut self, f: impl FnOnce(&mut ThisComp) -> T) -> T {
        f(&mut self.data.borrow_mut())
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
