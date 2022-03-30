mod cmd;

use std::pin::Pin;

pub use cmd::Cmd;

pub trait Component: Sized {
    type Props;
    type Event;
    type Msg;
}

pub trait Constructor: Component {
    fn constructor(_props: Self::Props) -> Self;
}

pub trait Update: Component {
    fn on_assemble(self: Pin<&mut Self>) -> Cmd<Self> {
        Cmd::None
    }
    fn on_load(self: Pin<&mut Self>, _props: Self::Props) -> Cmd<Self> {
        Cmd::None
    }
    fn update(self: Pin<&mut Self>, _msg: Self::Msg) -> Cmd<Self> {
        Cmd::None
    }
}

pub trait Render<T>: Component {
    type Children: Default;
    fn render(&self, children: Self::Children) -> T;
}
