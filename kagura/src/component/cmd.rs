use super::Component;
use std::future::Future;
use std::pin::Pin;

pub enum Cmd<C: Component> {
    None,
    Sub(C::Sub),
    Msg(C::Msg),
    Task(Pin<Box<dyn Future<Output = Cmd<C>>>>),
    List(Vec<Cmd<C>>),
}
