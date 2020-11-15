<div align="center">

![logo](kagura.png)

# Kagura

A front-end framework that runs on WebAssembly written in Rust.

</div>

## Big changes

- Supporting a batch process
- Experimental supporting of websocket

## Tutorial

### In English

[tutorial](https://soundrabbit.github.io/Kagura/)

### In Japanese

[[Kagura] Kagura + Rust でWebページを作成](https://qiita.com/ne_no_usa/items/0d8e33bad3aa7ec6d8fb)

## Hello World

```rust
extern crate kagura;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

use hello_component::HelloComponent;

#[wasm_bindgen(start)]
pub fn main() {
    kagura::run::<HelloComponent, _, _, _>("app", hello_component::Props {}, vec![]);
}

mod hello_component {
    use kagura::prelude::*;

    pub struct Props {}
    pub enum Msg {}
    pub enum Sub {}

    pub struct HelloComponent {}

    impl Constructor for HelloComponent {
        fn constructor(_: Self::Props, _: &mut ComponentBuilder<Msg, Sub>) -> Self {
            Self {}
        }
    }

    impl Component for HelloComponent {
        type Props = Props;
        type Msg = Msg;
        type Sub = Sub;

        fn init(&mut self, _: Self::Props, _: &mut ComponentBuilder<Msg, Sub>) {}

        fn update(&mut self, _: Self::Msg) -> Cmd<Msg, Sub> {
            Cmd::none()
        }

        fn render(&self, _: Vec<Html>) -> Html {
            Html::h1(
                Attributes::new(),
                Events::new(),
                vec![Html::text("Hello Kagura")],
            )
        }
    }
}
```
