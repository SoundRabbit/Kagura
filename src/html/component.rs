pub mod assembled_component;
mod cmd;
pub mod prepacked_component;
mod sub;

pub use assembled_component::{AssembledChildComponent, AssembledDemirootComponent};
pub use cmd::Cmd;
pub use prepacked_component::PrepackedComponent;
pub use sub::Sub;

pub type TaskResolver<Msg> = Box<dyn FnOnce(Msg)>;
pub type BatchResolver<Msg> = Box<dyn FnMut(Msg)>;

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
    fn update(&mut self, _: &Self::Props, _: Self::Msg) -> Cmd<Self> {
        Cmd::None
    }
    fn on_assemble(&mut self, _: &Self::Props) -> Cmd<Self> {
        Cmd::None
    }
    fn on_load(&mut self, _: &Self::Props) -> Cmd<Self> {
        Cmd::None
    }
    fn ref_node(&mut self, _: &Self::Props, _: String, _: web_sys::Node) -> Cmd<Self> {
        Cmd::None
    }
}

pub trait Render: Component {
    fn render(&self, _: &Self::Props, _: Vec<Html<Self>>) -> Html<Self> {
        Html::Fragment(vec![])
    }
}
