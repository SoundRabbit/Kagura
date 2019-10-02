//! # Kagura
//! Kagura is a web-frontend framework for wasm on Rust.
//!
//! ## example for "hello-world"
//!
//! ```
//! extern crate kagura;
//! extern crate wasm_bindgen;
//!
//! use wasm_bindgen::prelude::*;
//!
//! #[wasm_bindgen(start)]
//! pub fn main() {
//!     kagura::run(kagura::Component::new(State, update, render), "app");
//! }
//!
//! struct State;
//!
//! struct Msg;
//!
//! fn update(_: &mut State, _: &Msg) -> Option<()> {None}
//!
//! fn render(_: &State) -> kagura::Html<Msg> {
//!     use kagura::Html;
//!     use kagura::Attributes;
//!     use kagura::Events;
//!     Html::h1(
//!         Attributes::new(),
//!         Events::new(),
//!         vec![
//!             Html::unsafe_text("hello kagura"),
//!         ],
//!     )
//! }
//! ```

#[macro_use]
extern crate serde_derive;
extern crate rand;
extern crate wasm_bindgen;

mod audio;
mod bin;
mod component;
mod dom;
mod event;
pub mod native;
mod task;

#[allow(unused_imports)]
use rand::prelude::*;

pub use component::Cmd;
pub use component::Component;
pub use dom::html::Attributes;
pub use dom::html::Events;
pub use dom::html::Html;

/// Starts application with component
///
/// # Examples
///
/// ```
/// kagura::run(component, "id of entry point");
/// ```
pub fn run<Msg, State, Sub>(component: Component<Msg, State, Sub>, id: &str)
where
    Msg: 'static,
    State: 'static,
    Sub: 'static,
{
    bin::run(component, id);
}

pub mod prelude {
    pub use crate::Attributes;
    pub use crate::Cmd;
    pub use crate::Component;
    pub use crate::Events;
    pub use crate::Html;
}
