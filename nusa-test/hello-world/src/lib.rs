extern crate js_sys;
extern crate kagura;
extern crate nusa;
extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;
extern crate web_sys;

use nusa::Html;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    wasm_bindgen_futures::spawn_local(async {
        kagura::Runtime::run(nusa::dom_node::BasicDomNode::new(entry_point(), || {
            vec![Html::text("Hello world")]
        }))
        .await;
    });
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
