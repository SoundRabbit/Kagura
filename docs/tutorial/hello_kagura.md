# Hello Kagura

You can import all necessities by `use kagura::prelude::*;`, and start your application by `kagura::run`. `kagura::run` need you a component and a id of entrypoint.

Creating compoent, use `Component::new()`. `Component::new()` need you **initial state**, **update** and **render**. To create thease parameters, it is needed to define type of **state**, **message** and **subscription-message**. In this sample, thease types are defined as `State`, `Msg`, `Sub`.

**update** and **render** will be called by Kagura at the right time.

**src/lib.rs:**

```rs
extern crate kagura;
extern crate wasm_bindgen;

use kagura::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    kagura::run(Component::new(State{}, update, render), "app");
}

struct State{}

enum Msg{}

enum Sub{}

fn update(_: &mut State, _: Msg) -> Cmd<Msg, Sub> {Cmd::none()}

//  <h1>hello kagura</h1>
fn render(_: &State) -> Html<Msg> {
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
