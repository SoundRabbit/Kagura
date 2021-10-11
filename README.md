<div align="center">

![logo](kagura.png)

# Kagura

A front-end framework that runs on WebAssembly written in Rust.

</div>

## Hello World

```rust
extern crate kagura;
extern crate wasm_bindgen;


use kagura::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    let node = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("app")
        .unwrap();
    Kagura::mount(node.into(), || {
        vec![Html::h1(
            Attributes::new(),
            Events::new(),
            vec![Html::text("Hello Kagura")],
        )]
    });
}
```
