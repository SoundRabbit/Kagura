use super::msg::{FutureMsg, Msg};
use crate::component::Cmd;
use crate::component::Update;
use crate::Component;
use std::collections::VecDeque;
use std::pin::Pin;

#[allow(type_alias_bounds)]
pub type SubHandler<This: Component> = Box<dyn FnMut(This::Event) -> Msg>;

pub struct BasicComponentState<C: Update + 'static> {
    state: Pin<Box<C>>,
    sub_handler: Option<SubHandler<C>>,
}

pub enum BasicNodeMsg<C: Component + 'static> {
    ComponentMsg(C::Msg),
    ComponentCmd(Cmd<C>),
}

impl<C: Update> BasicComponentState<C> {
    pub fn new(state: Pin<Box<C>>, sub_handler: Option<SubHandler<C>>) -> Self {
        Self { state, sub_handler }
    }

    pub fn eval_cmd(&mut self, cmd: Cmd<C>) -> VecDeque<FutureMsg> {
        match cmd {
            Cmd::None => VecDeque::new(),
            Cmd::List(cmds) => cmds
                .into_iter()
                .map(|cmd| self.eval_cmd(cmd))
                .flatten()
                .collect(),
            Cmd::Chain(msg) => self.on_update(msg),
            Cmd::Task(task) => {
                let target_id = self.target_id();
                let future_msg = async move {
                    let cmd = task.await;
                    let msg = Msg::new(target_id, Box::new(BasicNodeMsg::ComponentCmd(cmd)));
                    vec![msg]
                };
                vec![Box::pin(future_msg) as FutureMsg].into()
            }
            Cmd::Submit(sub) => {
                if let Some(sub_handler) = &mut self.sub_handler {
                    let msg = sub_handler(sub);
                    vec![Box::pin(std::future::ready(vec![msg])) as FutureMsg].into()
                } else {
                    VecDeque::new()
                }
            }
        }
    }

    pub fn on_assemble(&mut self) -> VecDeque<FutureMsg> {
        let cmd = self.state.as_mut().on_assemble();
        self.eval_cmd(cmd)
    }

    pub fn on_load(&mut self, props: C::Props) -> VecDeque<FutureMsg> {
        let cmd = self.state.as_mut().on_load(props);
        self.eval_cmd(cmd)
    }

    pub fn on_update(&mut self, msg: C::Msg) -> VecDeque<FutureMsg> {
        let cmd = self.state.as_mut().update(msg);
        self.eval_cmd(cmd)
    }

    pub fn update(&mut self, msg: BasicNodeMsg<C>) -> VecDeque<FutureMsg> {
        match msg {
            BasicNodeMsg::ComponentCmd(cmd) => self.eval_cmd(cmd),
            BasicNodeMsg::ComponentMsg(msg) => self.on_update(msg),
        }
    }

    pub fn set_sub_handler(&mut self, sub_handler: Option<SubHandler<C>>) {
        self.sub_handler = sub_handler;
    }

    pub fn target_id(&self) -> usize {
        Msg::target_id(&self.state as &C)
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
