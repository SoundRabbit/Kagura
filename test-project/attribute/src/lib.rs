extern crate kagura;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    kagura::run(kagura::Component::new(State, update, render), "app");
}

struct State;

struct Msg;

fn update(_: &mut State, _: &Msg) -> Option<()> {None}

fn render(_: &State) -> kagura::Html<Msg> {
    use kagura::Html;
    use kagura::Attributes;
    use kagura::Events;
    Html::div(
        Attributes::new()
            .style("color", "#FFF")
            .style("background-color", "#D3381C")
            .style("width", "100vw")
            .style("height", "100vh")
            .style("position", "absolute")
            .style("top", "0px")
            .style("left", "0px")
            .id("app"),
        Events::new(),
        vec![
            Html::h1(
                Attributes::new(),
                Events::new(),
                vec![
                    Html::unsafe_text("hello kagura")
                ]
            )
        ],
    )
}
