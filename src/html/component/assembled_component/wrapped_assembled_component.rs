use super::*;

pub struct WrappedAssembledComponentInstance<DemirootComp: Component, VDemirootComp: Component> {
    data: Rc<RefCell<dyn AssembledChildComponent<DemirootComp = DemirootComp>>>,
    _phantom_v_demiroot: std::marker::PhantomData<VDemirootComp>,
}

impl<DemirootComp: Component, VDemirootComp: Component>
    WrappedAssembledComponentInstance<DemirootComp, VDemirootComp>
{
    pub fn wrap(
        data: Rc<RefCell<dyn AssembledChildComponent<DemirootComp = DemirootComp>>>,
    ) -> Rc<RefCell<dyn AssembledChildComponent<DemirootComp = VDemirootComp>>> {
        Rc::new(RefCell::new(Self {
            data,
            _phantom_v_demiroot: std::marker::PhantomData,
        }))
    }
}

impl<DemirootComp: Component, VDemirootComp: Component> AssembledChildComponent
    for WrappedAssembledComponentInstance<DemirootComp, VDemirootComp>
{
    type DemirootComp = VDemirootComp;

    fn as_any(&mut self) -> Option<Rc<RefCell<dyn std::any::Any>>> {
        self.data.borrow_mut().as_any()
    }

    fn on_assemble(&mut self) {
        self.data.borrow_mut().on_assemble();
    }

    fn on_load(&mut self) {
        self.data.borrow_mut().on_load();
    }

    fn load_lazy_cmd(&mut self) -> Vec<Msg<VDemirootComp>> {
        self.data
            .borrow_mut()
            .load_lazy_cmd()
            .into_iter()
            .map(|msg| Msg::Wrapped(Box::new(msg)))
            .collect()
    }

    fn render(&mut self, _: Vec<Html<VDemirootComp>>) -> VecDeque<Node> {
        VecDeque::new()
    }

    fn set_demiroot(
        &mut self,
        _: Option<Weak<RefCell<dyn AssembledDemirootComponent<ThisComp = Self::DemirootComp>>>>,
    ) {
    }
}
