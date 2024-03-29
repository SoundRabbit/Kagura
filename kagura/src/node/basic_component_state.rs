use super::msg::Msg;
use super::NodeCmd;
use crate::component::cmd::BatchProcess;
use crate::component::{Cmd, Update};
use crate::future_msg::Batch;
use crate::{Component, FutureMsg};
use std::cell::Cell;
use std::collections::VecDeque;
use std::pin::Pin;
use std::sync::Arc;

#[allow(type_alias_bounds)]
pub type SubHandler<This: Component> = Box<dyn FnMut(This::Event) -> Msg>;

pub struct BasicComponentState<C: Update + 'static> {
    state: Pin<Box<C>>,
    sub_handler: Option<SubHandler<C>>,
    batch_is_enable: Arc<Cell<bool>>,
}

pub enum BasicNodeMsg<C: Component + 'static> {
    ComponentMsg(C::Msg),
    ComponentCmd(Cmd<C>),
}

pub struct BasicNodeBatch<C: Component + 'static> {
    is_enable: Arc<Cell<bool>>,
    target_id: usize,
    batch: Box<dyn BatchProcess<C>>,
}

impl<C: Update> BasicComponentState<C> {
    pub fn new(state: Pin<Box<C>>, sub_handler: Option<SubHandler<C>>) -> Self {
        Self {
            state,
            sub_handler,
            batch_is_enable: Arc::new(Cell::new(true)),
        }
    }

    pub fn eval_cmd(&mut self, cmd: Cmd<C>) -> NodeCmd {
        match cmd {
            Cmd::None => NodeCmd::new(VecDeque::new()),
            Cmd::List(cmds) => {
                cmds.into_iter()
                    .fold(NodeCmd::new(VecDeque::new()), |mut node_cmd, child_cmd| {
                        let mut child_node_cmd = self.eval_cmd(child_cmd);
                        node_cmd.append(&mut child_node_cmd);
                        node_cmd
                    })
            }
            Cmd::Chain(msg) => self.on_update(msg),
            Cmd::Task(task) => {
                let target_id = self.target_id();
                let future_msg = async move {
                    let cmd = task.await;
                    let msg = Msg::new(target_id, Box::new(BasicNodeMsg::ComponentCmd(cmd)));
                    vec![msg]
                };
                NodeCmd::new(vec![FutureMsg::Task(Box::pin(future_msg))].into())
            }
            Cmd::Batch(batch) => NodeCmd::new(
                vec![FutureMsg::Batch(Box::new(BasicNodeBatch::new(
                    self.target_id(),
                    Arc::clone(&self.batch_is_enable),
                    batch,
                )))]
                .into(),
            ),
            Cmd::Submit(sub) => {
                if let Some(sub_handler) = &mut self.sub_handler {
                    let msg = sub_handler(sub);
                    let mut node_cmd = NodeCmd::new(VecDeque::new());
                    node_cmd.push_msg(msg);
                    node_cmd
                } else {
                    NodeCmd::new(VecDeque::new())
                }
            }
        }
    }

    pub fn on_assemble(&mut self) -> NodeCmd {
        let cmd = self.state.as_mut().on_assemble();
        let tasks = self.eval_cmd(cmd);
        tasks
    }

    pub fn on_load(&mut self, props: C::Props) -> NodeCmd {
        let cmd = self.state.as_mut().on_load(props);
        let tasks = self.eval_cmd(cmd);
        tasks
    }

    pub fn on_update(&mut self, msg: C::Msg) -> NodeCmd {
        let cmd = self.state.as_mut().update(msg);
        let tasks = self.eval_cmd(cmd);
        tasks
    }

    pub fn update(&mut self, msg: BasicNodeMsg<C>) -> NodeCmd {
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

impl<C: Update> std::ops::Drop for BasicComponentState<C> {
    fn drop(&mut self) {
        self.batch_is_enable.set(false);
    }
}

impl<C: Component> BasicNodeBatch<C> {
    pub fn new(
        target_id: usize,
        is_enable: Arc<Cell<bool>>,
        batch: Box<dyn BatchProcess<C>>,
    ) -> Self {
        Self {
            target_id,
            is_enable,
            batch,
        }
    }
}

impl<C: Component> Batch for BasicNodeBatch<C> {
    fn poll(&mut self) -> Option<crate::future_msg::Task> {
        if self.is_enable.get() {
            let task = self.batch.poll();
            let target_id = self.target_id;
            let task = Box::pin(async move {
                let cmd = task.await;
                let msg = Msg::new(target_id, Box::new(BasicNodeMsg::ComponentCmd(cmd)));
                vec![msg]
            }) as crate::future_msg::Task;
            Some(task)
        } else {
            None
        }
    }
}
