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
}
