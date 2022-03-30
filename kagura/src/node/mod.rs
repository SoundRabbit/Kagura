use std::collections::VecDeque;

pub mod basic_component_state;
pub mod msg;

pub use basic_component_state::BasicComponentState;
pub use basic_component_state::BasicNodeMsg;
pub use basic_component_state::SubHandler;
pub use msg::FutureMsg;
pub use msg::Msg;

pub trait UpdateNode {
    fn update(&mut self, msg: Msg) -> VecDeque<FutureMsg>;
}

pub trait RenderNode<T> {
    fn render(&mut self) -> T;
}
