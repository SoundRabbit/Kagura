```rust
extern crate kagura;
extern crate wasm_bindgen;

use kagura::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    kagura::run(Component::new(init, update, render), "app");
}

struct State {
    message: &'static str,
}

enum Msg {
    ChangeMessage(&'static str),
}

enum Sub {}

fn init() -> State {
    State {
        message: "hello kagura",
    }
}

fn update(state: &mut State, msg: Msg) -> Cmd<Msg, Sub> {
    match msg {
        Msg::ChangeMessage(new_message) => {
            state.message = new_message;
            Cmd::none()
        }
    }
}

//  <h1 style="color:#FFFFFF;background-color:#D3381C;" onClick="send_a_message">hello kagura</h1>
fn render(state: &State) -> Html<Msg> {
    Html::h1(
        Attributes::new()
            .style("color", "#FFFFFF")
            .style("background-color", "#D3381C;"),
        Events::new().on_click(|_| Msg::ChangeMessage("Kagura cathed an event")),
        vec![Html::text(state.message)],
    )
}

mod child {
    use kagura::prelude::*;

    fn new(value: String) -> Component<Msg, State, Sub> {

    }

    struct State {
        value: String,
    }

    enum Msg {
        ChangeValue(String),
        OkToChange,
    }

    enum Sub {
        ValueIsChanged(String),
    }

    fn init() -> State {
        State {
            value: ""
        }
    }

    fn update(state: &mut State, msg: Msg) -> Cmd<Msg, Sub> {
    }

    fn render(state: &State) -> Html<Msg> {
        Html::div(
            Attributes::new(),
            Events::new(),
            vec![
                Html::input(
                    Attributes::new().value(&State.value),
                    Events::new().input(|v| Msg::ChangeValue(v)),
                    vec![]
                ),
                Html::button(
                    Attributes::new(),
                    Events::new().on_click(|_| Msg::OkToChange),
                    vec![]
                    Html::text("ok")
                )
            ]
        )
    }
}
```