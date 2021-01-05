# Hello Kagura

**src/lib.rs:**

```rs
extern crate kagura;
extern crate wasm_bindgen;

mod app;

use kagura::prelude::*;
use wasm_bindgen::prelude::*;
use app::App;

#[wasm_bindgen(start)]
pub fn main() {
    kagura::run::<App, _, _, _>("app", app::Props {}, vec![])
}

mod app {
    use kagura::prelude::*;

    pub struct Props {}

    pub enum Msg {}

    pub enum Sub {}

    pub struct App {}

    impl Constructor for App {
        fn constructor(_: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) -> Self {
            Self {}
        }
    }

    
    impl Component for App {
        type Props = Props;
        type Msg = Msg;
        type Sub = Sub;

        fn init(&mut self, _: Self::Props, _: &mut ComponentBuilder<Self::Msg, Self::Sub>) {}

        fn update(&mut self, _: Self::Msg) -> Cmd<Self::Msg, Self::Sub> {
            Cmd::none()
        }

        fn render(&self, children: Vec<Html>) -> Html {
            Html::h1(Attributes::new(), Events::new(), vec![Html::text("Hello World")])
        }
    }
}
```

**start by:**

```shell
$ npm start
```
