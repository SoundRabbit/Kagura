# Add attributes

Attributes of an element are presented by `Attributes` structure. `Attributes` is initialize by `Attributes.new()`. You can add an attribute to `Attributes` by method chain.

like this:

```rust
Attributes::new()
    .class("fizz")
    .class("bazz")
    .string("data-original-attribute", "some-value");
```

## Set delimiter of an attribute

You can set a delimiter of an attribute by `delimit_with` method. This method set a delimiter to last added attribute.

like this:

```rust
Attributes::new()
    .string("data-original-attribute-1", "some-value-1")
    .string("data-original-attribute-1", "some-value-2")
    .delimit_with(" ")
    .string("data-original-attribute-2", "some-value-1")
    .string("data-original-attribute-2", "some-value-2")
    .delimit_with(";");
```

## Example

```rs
extern crate kagura;
extern crate wasm_bindgen;

use kagura::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    kagura::run(Component::new(init, update, render), "app");
}

struct State{}

enum Msg{}

enum Sub{}

fn init() -> State {
    State{}
}

fn update(_: &mut State, _: Msg) -> Cmd<Msg, Sub> {Cmd::none()}

//  <h1 style="color:#FFFFFF;background-color:#D3381C;">hello kagura</h1>
fn render(_: &State) -> Html<Msg> {
    Html::h1(
        Attributes::new()
            .style("color", "#FFFFFF")
            .style("background-color", "#D3381C;"),
        Events::new(),
        vec![
            Html::text("hello kagura"),
        ],
    )
}
```
