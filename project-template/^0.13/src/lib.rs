extern crate isaribi;
extern crate js_sys;
extern crate kagura;
extern crate wasm_bindgen;
extern crate web_sys;

use kagura::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    Kagura::mount(entry_point(), || {
        vec![Html::h1(
            Attributes::new(),
            Events::new(),
            vec![Html::text("Hello Kagura")],
        )]
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
