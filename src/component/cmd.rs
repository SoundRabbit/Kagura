use super::Component;

pub struct Cmd<C: Component> {
    data: implement::Cmd<C>,
}

mod implement {
    use super::Component;

    pub type TaskResolver<M> = Box<dyn FnOnce(M)>;

    pub enum Cmd<C: Component> {
        None,
        Task(Box<dyn FnOnce(TaskResolver<C::Msg>)>),
    }
}
