extern crate osashimi;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    osashimi::run(osashimi::Component::new(0, update, render), "app");
}

type State = u64;

enum Msg {
    CountUp,
}

fn update(state: &mut State, msg: &Msg) {
    *state += 1;
}

fn render(state: &State) -> osashimi::Html<Msg> {
    use osashimi::Event;
    use osashimi::Html;
    Html::h1(
        vec![],
        vec![Event::OnClick(Box::new(|| Msg::CountUp))],
        vec![Html::text(state.to_string())],
    )
}
