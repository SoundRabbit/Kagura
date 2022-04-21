extern crate async_std;

pub mod component;
pub mod future_msg;
pub mod node;
pub mod runtime;
pub mod util;

pub use component::Component;
pub use future_msg::FutureMsg;
pub use runtime::Runtime;

pub mod prelude {
    pub use crate::component::{BatchProcess, Cmd, Constructor, Render, Update};
    pub use crate::Component;
}
