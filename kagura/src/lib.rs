extern crate rand;

mod dom;
pub mod html;
pub mod component;
mod main;

#[allow(unused_imports)]
use rand::prelude::*;

pub use html::Html;
pub use html::Attributes;
pub use html::Events;
pub use component::Component;

pub fn run<M, S, B>(component: Component<M, S, B>, id: &str)
where
    M: 'static,
    S: 'static,
    B: 'static,
{
    main::run(component, id);
}