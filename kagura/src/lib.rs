extern crate rand;

mod bin;
mod component;
mod dom;
mod html;

#[allow(unused_imports)]
use rand::prelude::*;

pub use component::Component;
pub use html::Attributes;
pub use html::Events;
pub use html::Html;

pub fn run<M, S, B>(component: Component<M, S, B>, id: &str)
where
    M: 'static,
    S: 'static,
    B: 'static,
{
    bin::run(component, id);
}
