extern crate kagura;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    kagura::run(kagura::Component::new(State, update, render), "app");
}

struct State;

struct Msg;

fn update(_: &mut State, _: Msg) -> Option<()> {None}

fn render(_: &State) -> kagura::Html<Msg> {
    use kagura::Html;
    use kagura::Attributes;
    use kagura::Events;
    Html::h1(
        Attributes::new(),
        Events::new(),
        vec![
            Html::unsafe_text("hello kagura"),
        ],
    )
}
