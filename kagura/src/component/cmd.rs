use super::Component;
use std::future::Future;
use std::pin::Pin;

pub enum Cmd<C: Component> {
    None,
    Submit(C::Event),
    Chain(C::Msg),
    Task(Pin<Box<dyn Future<Output = Self>>>),
    Batch(Box<dyn BatchProcess<C>>),
    List(Vec<Self>),
}

pub trait BatchProcess<C: Component> {
    fn poll(&mut self) -> Pin<Box<dyn Future<Output = Cmd<C>>>>;
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
    pub fn batch(batch: impl BatchProcess<C> + 'static) -> Self {
        Self::Batch(Box::new(batch))
    }
    pub fn list(cmds: Vec<Self>) -> Self {
        Self::List(cmds)
    }
}
