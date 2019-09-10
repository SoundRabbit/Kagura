extern crate kagura;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    kagura::run(kagura::Component::new(0, update, render), "app");
}

type State = u64;

struct Msg;

fn update(_: &mut State, _: &Msg) {}

fn render(_: &State) -> kagura::Html<Msg> {
    use kagura::Html;
    use kagura::Attributes;
    use kagura::Events;
    Html::div(
        Attributes::new()
            .with_style("color: red;"),
        Events::new(),
        vec![
            Html::component(child::new()),
            Html::component(child::new()),
            Html::component(child::new()),
            Html::component(child::new()),
        ],
    )
}

mod child {
    pub fn new() -> kagura::Component<Msg, State> {
        kagura::Component::new(0, update, render)
    }

    pub type State = u64;

    pub enum Msg {
        CountUp,
    }

    fn update(state: &mut State, msg: &Msg) {
        match msg {
            Msg::CountUp => {*state += 1;}
        }
    }

    fn render(state: &State) -> kagura::Html<Msg> {
        use kagura::Events;
        use kagura::Attributes;
        use kagura::Html;
        Html::h1(
            Attributes::new(),
            Events::new()
                .with_on_click(|| {Msg::CountUp}),
            vec![Html::unsafe_text(state.to_string())],
        )
    }
}
