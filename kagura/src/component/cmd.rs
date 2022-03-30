use super::Component;
use std::future::Future;
use std::pin::Pin;

pub enum Cmd<C: Component> {
    None,
    Submit(C::Event),
    Chain(C::Msg),
    List(Vec<Cmd<C>>),
}
