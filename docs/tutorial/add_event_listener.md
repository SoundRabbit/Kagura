# Add event listener

Event-listeners of an element are presented by `Events` structure. `Events` is initialized by `Events::new()`. You can add an event-listener to `Events` by method chain.

like this:

```rust
Events::new()
    .on_click(|_| Msg::SomeMsg); 
```

## Example

```rs
extern crate kagura;
extern crate wasm_bindgen;

use kagura::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    kagura::run(Component::new(init(), update, render), "app");
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
```
