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

// pub mod component;
mod dom;
mod event;
mod native;
mod state;
mod task;
mod uid;

use dom::component::{Component, ComponentBuilder, ComposedComponent, Constructor, SubMap};
use dom::html::Html;

/// Starts application with component
pub fn run<C: 'static, P: 'static, M: 'static, S: 'static>(id: &str, props: P, children: Vec<Html>)
where
    C: Component<Props = P, Msg = M, Sub = S> + Constructor<Props = P>,
{
    let mut builder = ComponentBuilder::new();
    let component = C::constructor(props, &mut builder);
    let component = ComposedComponent::new(uid::get(), component, builder, SubMap::empty());
    component.borrow_mut().set_children(children);
    state::init(component, id);
}

pub mod prelude {
    pub use crate::dom::component::Cmd;
    pub use crate::dom::component::Component;
    pub use crate::dom::component::ComponentBuilder;
    pub use crate::dom::component::Constructor;
    pub use crate::dom::component::SubMap;
    pub use crate::dom::html::Attributes;
    pub use crate::dom::html::Events;
    pub use crate::dom::html::Html;
}
