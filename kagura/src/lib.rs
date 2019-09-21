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

mod bin;
mod component;
mod dom;
mod html;
pub mod native;
mod renderer;
mod event;

#[allow(unused_imports)]
use rand::prelude::*;

pub use component::Component;
pub use html::Attributes;
pub use html::Events;
pub use html::Html;

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
