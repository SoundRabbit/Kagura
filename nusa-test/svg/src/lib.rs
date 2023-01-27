extern crate js_sys;
extern crate kagura;
extern crate nusa;
extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;
extern crate web_sys;

use nusa::html::html_element::Attributes;
use nusa::html::html_element::Events;
use nusa::Html;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    wasm_bindgen_futures::spawn_local(async {
        kagura::Runtime::run(nusa::dom_node::BasicDomNode::new(entry_point(), |_| {
            vec![Html::element(
                "svg",
                Attributes::new()
                    .string("width", "400")
                    .string("height", "200")
                    .string("viewBox", "0 0 400 200")
                    .string("xmlns", "http://www.w3.org/2000/svg"),
                Events::new(),
                vec![Html::element(
                    "rect",
                    Attributes::new()
                        .num("x", 10.0)
                        .num("y", 10.0)
                        .num("width", 380.0)
                        .num("height", 180.0)
                        .string("fill", "#e74c3c"),
                    Events::new(),
                    vec![],
                )],
            )]
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
