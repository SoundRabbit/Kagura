# Hello Kagura

lib.rs:

```rs
extern crate kagura;
extern crate wasm_bindgen;

use kagura::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    kagura::run(Component::new(State, update, render), "app");
}

struct State;

struct Msg;

struct Sub;

fn update(_: &mut State, _: Msg) -> Cmd<Msg, Sub> {Cmd::none()}

fn render(_: &State) -> Html<Msg> {
    Html::h1(
        Attributes::new(),
        Events::new(),
        vec![
            Html::unsafe_text("hello kagura"),
        ],
    )
}
```

start by:

```shell
$ npm start
```
