# nusa

The features for `kagura` to generate web page by virtual-DOM.

## Documents

<https://kagura.gitbook.io/kagura-nusa-en/>

## Usage

### mount to real-DOM

You can begin application with `kagura::Runtime::run`. `nusa::dom_node::BasicDomNode` is features to mount virtual-DOM to real-DOM.

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
