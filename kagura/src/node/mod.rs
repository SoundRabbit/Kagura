use crate::FutureMsg;
use std::collections::VecDeque;

pub mod basic_component_state;
pub mod msg;

pub use basic_component_state::BasicComponentState;
pub use basic_component_state::BasicNodeMsg;
pub use basic_component_state::SubHandler;
pub use msg::Msg;

pub struct NodeCmd {
    scedules: VecDeque<FutureMsg>,
}

pub trait UpdateNode {
    fn update(&mut self, msg: Msg) -> NodeCmd;
}

pub trait RenderNode<T> {
    fn render(&mut self) -> T;
}

impl NodeCmd {
    pub fn new(scedules: VecDeque<FutureMsg>) -> Self {
        Self { scedules }
    }

    pub fn scedules(self) -> VecDeque<FutureMsg> {
        self.scedules
    }
}

impl std::ops::Deref for NodeCmd {
    type Target = VecDeque<FutureMsg>;
    fn deref(&self) -> &Self::Target {
        &self.scedules
    }
}

impl std::ops::DerefMut for NodeCmd {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.scedules
    }
}
