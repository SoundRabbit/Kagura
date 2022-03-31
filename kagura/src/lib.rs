extern crate async_std;

pub mod component;
pub mod node;
pub mod runtime;

pub use component::Component;
pub use runtime::Runtime;

pub mod prelude {
    pub use crate::component::{Cmd, Constructor, Render, Update};
    pub use crate::Component;
}
