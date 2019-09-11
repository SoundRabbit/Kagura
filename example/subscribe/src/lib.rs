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
            Html::component(child::new(state%2==1).subscribe(|_| {
                Box::new(Msg)
            })),
        ],
    )
}

mod child {
    use kagura::Component;
    use kagura::Events;
    use kagura::Attributes;
    use kagura::Html;

    pub fn new(is_odd_number: bool) -> Component<Msg, State, Sub> {
        Component::new(is_odd_number, update, render)
    }

    type State = bool;

    pub struct Msg;

    pub struct Sub;

    fn update(_: &mut State, _: &Msg) -> Option<Sub> {
        Some(Sub)
    }

    fn render(state: &State) -> Html<Msg> {
        if *state {
            render_odd()
        }else{
            render_even()
        }
    }

    fn render_odd() -> Html<Msg> {
        Html::h1(
            Attributes::new()
                .with_style("color: red"),
            Events::new()
                .with_on_click(|_| {Msg}),
            vec![Html::unsafe_text("click here")],
        )
    }

    fn render_even() -> Html<Msg> {
        Html::h1(
            Attributes::new()
                .with_style("color: black"),
            Events::new()
                .with_on_click(|_| {Msg}),
            vec![Html::unsafe_text("click here")],
        )
    }
}
