extern crate osashimi;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    osashimi::run(osashimi::Component::new(0, update, render), "app");
}

type State = u64;

enum Msg {}

fn update(state: &mut State, msg: &Msg) {}

fn render(state: &State) -> osashimi::Html {
    use osashimi::Html;
    Html::h1(vec![Html::text("hello osashimi")])
}
