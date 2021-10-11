use super::*;

pub enum Cmd<C: Component> {
    None,
    Sub(C::Sub),
    Task(Box<dyn FnOnce(TaskResolver<C::Msg>)>),
    Batch(Box<dyn FnOnce(BatchResolver<C::Msg>)>),
}

impl<C: Component> Cmd<C> {
    pub fn none() -> Self {
        Self::None
    }
    pub fn task(task: impl FnOnce(TaskResolver<C::Msg>) + 'static) -> Self {
        Self::Task(Box::new(task))
    }
    pub fn batch(batch: impl FnOnce(BatchResolver<C::Msg>) + 'static) -> Self {
        Self::Batch(Box::new(batch))
    }
    pub fn chain(msg: C::Msg) -> Self {
        Self::task(|resolve| resolve(msg))
    }
}
