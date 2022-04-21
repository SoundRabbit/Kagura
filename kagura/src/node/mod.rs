use crate::FutureMsg;
use std::collections::VecDeque;

pub mod basic_component_state;
pub mod msg;

pub use basic_component_state::BasicComponentState;
pub use basic_component_state::BasicNodeMsg;
pub use basic_component_state::SubHandler;
pub use msg::Msg;

pub struct NodeCmd {
    msgs: VecDeque<Msg>,
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
        Self {
            scedules,
            msgs: VecDeque::new(),
        }
    }

    pub fn append(&mut self, other: &mut Self) {
        self.append_msgs(&mut other.msgs);
        self.append_scedules(&mut other.scedules);
    }

    pub fn into_scedules(self) -> VecDeque<FutureMsg> {
        self.scedules
    }

    pub fn append_scedules(&mut self, scedules: &mut VecDeque<FutureMsg>) {
        self.scedules.append(scedules);
    }

    pub fn into_msgs(self) -> VecDeque<Msg> {
        self.msgs
    }

    pub fn msgs(&self) -> &VecDeque<Msg> {
        &self.msgs
    }

    pub fn msgs_mut(&mut self) -> &mut VecDeque<Msg> {
        &mut self.msgs
    }

    pub fn push_msg(&mut self, msg: Msg) {
        self.msgs.push_back(msg);
    }

    pub fn append_msgs(&mut self, msgs: &mut VecDeque<Msg>) {
        self.msgs.append(msgs);
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
