use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub mod assembled_component;
mod cmd;
pub mod prepacked_component;
mod sub;

pub use assembled_component::{AssembledChildComponent, AssembledDemirootComponent};
pub use cmd::Cmd;
pub use prepacked_component::PrepackedComponent;
pub use sub::Sub;

use super::*;
use assembled_component::AssembledComponentInstance;

pub trait Component: Sized + 'static {
    type Props;
    type Sub;
    type Msg;
}

pub trait Constructor: Update + Render {
    fn constructor(props: &Self::Props) -> Self;

    fn with_children<DemirootComp: Component>(
        props: Self::Props,
        sub: Sub<Self::Sub, DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Html<DemirootComp> {
        Html::ComponentNode(ComponentNode::PackedComponentNode(Box::new(
            PackedComponentNodeInstance::new(Self::constructor, props, sub, children),
        )))
    }
}

pub trait Update: Component {
    fn update(&mut self, props: &Self::Props, msg: Self::Msg) -> Cmd<Self::Sub> {
        Cmd::None
    }
    fn on_assemble(&mut self, props: &Self::Props) -> Cmd<Self::Sub> {
        Cmd::None
    }
    fn on_load(&mut self, props: &Self::Props) -> Cmd<Self::Sub> {
        Cmd::None
    }
}

pub trait Render: Component {
    fn render(&self, props: &Self::Props, children: Vec<Html<Self>>) -> Html<Self> {
        Html::Fragment(vec![])
    }
}
