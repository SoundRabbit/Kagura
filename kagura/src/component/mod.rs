mod cmd;

use async_trait::async_trait;
use std::pin::Pin;

pub use cmd::Cmd;

pub trait Component: Sized {
    type Props;
    type Sub;
    type Msg;
}

pub trait Constructor: Component {
    fn constructor(_props: Self::Props) -> Self;
}

pub trait Update: Component {
    fn on_assemble(self: Pin<&mut Self>) -> Cmd<Self>;
    fn on_load(self: Pin<&mut Self>, props: Self::Props) -> Cmd<Self>;
    fn update(self: Pin<&mut Self>, msg: Self::Msg) -> Cmd<Self>;
}

pub trait Render<T>: Component {
    type Children: Default;
    type Context;
    fn render(&self, context: Self::Context, children: Self::Children) -> T;
}
