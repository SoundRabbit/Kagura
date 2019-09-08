extern crate osashimi;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    osashimi::Component::new(0, update, render);
}

type State = u64;

enum Msg {}

fn update(state: &mut State, msg: &Msg) {}

fn render(state: &State) -> osashimi::Html {
    use osashimi::Html;
    Html::text("hello osashimi")
}
