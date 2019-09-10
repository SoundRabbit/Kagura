extern crate osashimi;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    osashimi::run(osashimi::Component::new(0, update, render), "app");
}

type State = u64;

struct Msg;

fn update(_: &mut State, _: &Msg) {}

fn render(_: &State) -> osashimi::Html<Msg> {
    use osashimi::Html;
    use osashimi::Attributes;
    use osashimi::Events;
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
    pub fn new() -> osashimi::Component<Msg, State> {
        osashimi::Component::new(0, update, render)
    }

    pub type State = u64;

    pub enum Msg {
        CountUp,
    }

    fn update(state: &mut State, _: &Msg) {
        *state += 1;
    }

    fn render(state: &State) -> osashimi::Html<Msg> {
        use osashimi::Events;
        use osashimi::Attributes;
        use osashimi::Html;
        Html::h1(
            Attributes::new(),
            Events::new()
                .with_on_click(|| {Msg::CountUp}),
            vec![Html::unsafe_text(state.to_string())],
        )
    }
}
