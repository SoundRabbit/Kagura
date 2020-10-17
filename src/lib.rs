//! # Example
//!
//! ```
//! extern crate kagura;
//! extern crate wasm_bindgen;
//!
//! use kagura::prelude::*;
//! use wasm_bindgen::prelude::*;
//!
//! #[wasm_bindgen(start)]
//! pub fn main() {
//!     kagura::run(Component::new(State, update, render), "app");
//! }
//!
//! struct State;
//!
//! struct Msg;
//!
//! struct Sub;
//!
//! fn update(_: &mut State, _: Msg) -> Cmd<Msg, Sub> {Cmd::none()}
//!
//! fn render(_: &State) -> Html<Msg> {
//!     Html::h1(
//!         Attributes::new(),
//!         Events::new(),
//!         vec![
//!             Html::text("hello kagura"),
//!         ],
//!     )
//! }
//! ```

extern crate js_sys;
extern crate rand;
extern crate serde_derive;
extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;
extern crate web_sys;

pub mod component;
mod dom;
mod event;
mod native;
mod state;
mod task;

use dom::component::RcController as Component;

/// Starts application with component
pub fn run<Props: 'static, Sub: 'static>(composed: Component<Props, Sub>, id: &str) {
    state::init(composed, id);
}

pub mod prelude {
    pub use crate::component::*;
    pub use crate::dom::component::Batch;
    pub use crate::dom::component::Cmd;
    pub use crate::dom::component::RcController as Component;
    pub use crate::dom::html::Attributes;
    pub use crate::dom::html::Events;
    pub use crate::dom::html::Html;
}
