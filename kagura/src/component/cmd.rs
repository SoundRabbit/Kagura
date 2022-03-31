use super::Component;
use std::future::Future;
use std::pin::Pin;

pub enum Cmd<C: Component> {
    None,
    Submit(C::Event),
    Chain(C::Msg),
    Task(Pin<Box<dyn Future<Output = Self>>>),
    List(Vec<Self>),
}

impl<C: Component> Cmd<C> {
    pub fn none() -> Self {
        Self::None
    }
    pub fn submit(e: C::Event) -> Self {
        Self::Submit(e)
    }
    pub fn chain(msg: C::Msg) -> Self {
        Self::Chain(msg)
    }
    pub fn task(task: impl Future<Output = Self> + 'static) -> Self {
        Self::Task(Box::pin(task))
    }
    pub fn list(cmds: Vec<Self>) -> Self {
        Self::List(cmds)
    }
}
