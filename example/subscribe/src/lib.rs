extern crate kagura;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    kagura::run(kagura::Component::new(0, update, render), "app");
}

type State = u64;

struct Msg;

struct Sub;

fn update(state: &mut State, _: &Msg) -> Option<Sub> {
    *state += 1;
    Some(Sub)
}

fn render(state: &State) -> kagura::Html<Msg> {
    use kagura::Html;
    use kagura::Attributes;
    use kagura::Events;
    Html::div(
        Attributes::new(),
        Events::new(),
        vec![
            Html::unsafe_text(state.to_string()),
            Html::component(child::new().subscribe(|_| {
                Box::new(Msg)
            })),
        ],
    )
}

mod child {
    pub fn new() -> kagura::Component<Msg, State, Sub> {
        kagura::Component::new(State, update, render)
    }

    pub struct State;

    pub struct Msg;

    pub struct Sub;

    fn update(_: &mut State, _: &Msg) -> Option<Sub> {
        Some(Sub)
    }

    fn render(_: &State) -> kagura::Html<Msg> {
        use kagura::Events;
        use kagura::Attributes;
        use kagura::Html;
        Html::h1(
            Attributes::new(),
            Events::new()
                .with_on_click(|| {Msg}),
            vec![Html::unsafe_text("click here")],
        )
    }
}
