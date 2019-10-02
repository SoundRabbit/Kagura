use crate::native;

pub mod node;

pub struct AudioContext {
    connection: Connection,
    context: Option<native::AudioContext>,
}

pub enum Connection {}

pub trait Node {
    fn native(context: &native::AudioContext) -> native::AudioNode;
}
