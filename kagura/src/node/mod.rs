pub mod msg;

use crate::component::Update;
pub use msg::FutureMsg;
pub use msg::Msg;
use std::collections::VecDeque;

pub trait UpdateNode {
    fn update(&mut self, msg: Msg) -> VecDeque<FutureMsg>;
}

pub trait RenderNode<T> {
    fn render(&mut self) -> T;
}

use crate::component::Cmd;
use crate::Component;
use std::pin::Pin;

pub struct BasicComponentState<C: Update + 'static> {
    state: Pin<Box<C>>,
}

pub enum BasicNodeMsg<C: Component + 'static> {
    ComponentMsg(C::Msg),
    ComponentCmd(Cmd<C>),
}

impl<C: Update> BasicComponentState<C> {
    pub fn new(state: Pin<Box<C>>) -> Self {
        Self { state }
    }

    pub fn eval_cmd(&mut self, cmd: Cmd<C>) {
        match cmd {
            Cmd::None => {}
            Cmd::List(cmds) => {
                for cmd in cmds {
                    self.eval_cmd(cmd);
                }
            }
            Cmd::Msg(msg) => {
                self.on_update(msg);
            }
            Cmd::Task(task) => {
                let target_id = Msg::target_id(&self.state);
                let future_msg: FutureMsg = Box::pin(async move {
                    Msg::new(target_id, Box::new(BasicNodeMsg::ComponentCmd(task.await)))
                });
            }
            Cmd::Sub(sub) => {}
        }
    }

    pub fn on_load(&mut self, props: C::Props) {
        let cmd = self.state.as_mut().on_load(props);
        self.eval_cmd(cmd);
    }

    pub fn on_update(&mut self, msg: C::Msg) {
        let cmd = self.state.as_mut().update(msg);
        self.eval_cmd(cmd);
    }

    pub fn update(&mut self, msg: BasicNodeMsg<C>) {
        match msg {
            BasicNodeMsg::ComponentCmd(cmd) => {
                self.eval_cmd(cmd);
            }
            BasicNodeMsg::ComponentMsg(msg) => {
                self.on_update(msg);
            }
        }
    }
}

impl<C: Update> std::ops::Deref for BasicComponentState<C> {
    type Target = Pin<Box<C>>;
    fn deref(&self) -> &Pin<Box<C>> {
        &self.state
    }
}

impl<C: Update> std::ops::DerefMut for BasicComponentState<C> {
    fn deref_mut(&mut self) -> &mut Pin<Box<C>> {
        &mut self.state
    }
}
