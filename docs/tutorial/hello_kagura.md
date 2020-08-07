# Hello Kagura

You can import all necessities by `use kagura::prelude::*;`, and start your application by `kagura::run`. `kagura::run` need you a component and a id of entrypoint.

Creating compoent, you can use `Component::new()`. `Component::new()` need you **init**, **update** and **render**. To create thease parameters, it is needed to define type of **property**, **state**, **message** and **subscription-message**. In this sample, thease types are defined as `Props`, `State`, `Msg`, `Sub`.

**init**, **update** and **render** will be called by Kagura at the right time.

**src/lib.rs:**

```rs
extern crate kagura;
extern crate wasm_bindgen;

use kagura::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    kagura::run(Component::new(init, update, render).with(Props {}), "app");
}

struct Props {}

struct State{}

enum Msg{}

enum Sub{}

fn init(_previous_state: Option<State>, _props: Props) -> (State, Cmd<Msg, Sub>) {
    (State {}, Cmd::none())
}

fn update(_state: &mut State, _msg: Msg) -> Cmd<Msg, Sub> {Cmd::none()}

//  <h1>hello kagura</h1>
fn render(_state: &State) -> Html {
    Html::h1(
        Attributes::new(),
        Events::new(),
        vec![
            Html::text("hello kagura"),
        ],
    )
}
```

**start by:**

```shell
$ npm start
```
