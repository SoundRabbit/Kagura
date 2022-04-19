extern crate js_sys;
extern crate kagura;
extern crate nusa;
extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;
extern crate web_sys;

use nusa::html_component::Sub;
use nusa::HtmlComponent;
use wasm_bindgen::prelude::*;

mod child;
mod middle;
mod parent;

use parent::Parent;

#[wasm_bindgen(start)]
pub fn main() {
    wasm_bindgen_futures::spawn_local(kagura::Runtime::run(nusa::dom_node::BasicDomNode::new(
        entry_point(),
        |this| vec![Parent::empty(this, None, parent::Props {}, Sub::none())],
    )));
}

fn entry_point() -> web_sys::Node {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("app")
        .unwrap()
        .into()
}
