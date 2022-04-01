# Kagura

`kagura` is a component-oriented GUI framework. This framework has extensibility to be used in different contexts.

When you create a web page, `kagura` needs `nusa`. `nusa` gives `kagura` features to manuplate DOM by virtual-DOM.

## Tutorial

<https://github.com/SoundRabbit/Kagura/blob/master/docs/%5E0.14/tutorial/01_preparation.md>

## Creating a web page

`kagura` needs `nusa` to generate a web page. like this:

```rust
extern crate js_sys;
extern crate kagura;
extern crate nusa;
extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;
extern crate web_sys;

use nusa::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    wasm_bindgen_futures::spawn_local(async {
        kagura::Runtime::run(nusa::dom_node::BasicDomNode::new(entry_point(), |_| {
            vec![Html::h1(
                Attributes::new(),
                Events::new(),
                vec![Html::text("Hello World")],
            )]
        }))
        .await;
    });
}

fn entry_point() -> web_sys::Node {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("app")
        .unwrap()
        .into()
}
```
