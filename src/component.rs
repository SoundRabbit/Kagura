mod assembled_component;
mod cmd;
mod msg;
mod sub;

pub use cmd::Cmd;
pub use msg::Msg;
pub use sub::Sub;

pub trait Component {
    type Props;
    type Sub;
    type Msg;
}

pub trait Update: Component + Sized {
    fn on_assemble(&mut self, _props: &Self::Props) -> Cmd<Self>;
    fn on_load(&mut self, _props: &Self::Props) -> Cmd<Self>;
    fn update(&mut self, _props: &Self::Props, msg: Self::Msg) -> Cmd<Self>;
}

pub trait Render<Node>: Component {
    type Children;
    fn render(&self, _props: &Self::Props, children: Self::Children) -> Node;
}

pub trait Constructor: Component {
    fn constructor(_props: &Self::Props) -> Self;
}
