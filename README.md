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

## Usage

### Create a project

See `/project-template/^0.13` in this repogitory. This is a template of project of Kagura. You can custamize to use.

### Mount to web_sys::Node

Kagura only changes children of mounted web_sys::Node.

```rust
Kagura::mount(web_sys::Node, impl FnMut() -> Vec<Html<Terminator>>)
```

`Terminator` is a component, which is a marker of root of dom.
