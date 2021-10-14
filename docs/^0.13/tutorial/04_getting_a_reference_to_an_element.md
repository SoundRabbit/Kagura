# Getting a reference to an element.

When you want to get a reference to an Element to draw to HtmlCanvasElement, Kagura can give the reference to your component.

This feature has danger which cause problems in rendering process. But, probably, it is sometimes needed.

This is an example:

<div align="center">cargo.toml</div>

```toml
[package]
name = "kagura-project"
version = "0.1.0"
edition="2018"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
# with "isaribi", you can write style sheet like css-module or styled-component.
isaribi="^0.2"
js-sys="^0.3"
kagura="^0.13"
wasm-bindgen="^0.2"

[dependencies.web-sys]
version="^0.3"
features=[
# If you want to use HtmlCanvasElement, Blob, IdbDatabase and so on, 
# add features hear.
    "CanvasRenderingContext2d",
    "HtmlCanvasElement"
]
```

---

<div align="center">src/lib.rs</div>

```rust
extern crate isaribi;
extern crate js_sys;
extern crate kagura;
extern crate wasm_bindgen;
extern crate web_sys;

use kagura::prelude::*;
use wasm_bindgen::prelude::*;

mod example_component;

use example_component::ExampleComponent;

#[wasm_bindgen(start)]
pub fn main() {
    Kagura::mount(entry_point(), || {
        vec![ExampleComponent::empty(
            example_component::Props {},
            component::Sub::none(),
        )]
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

---

<div align="center">src/example_component.rs</div>

```rust
use kagura::html::component::Cmd;
use kagura::prelude::*;
use wasm_bindgen::JsCast;

pub struct Props {}

pub enum Msg {}

pub enum On {}

pub struct ExampleComponent {}

impl Component for ExampleComponent {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for ExampleComponent {
    fn constructor(_props: &Self::Props) -> Self {
        Self {}
    }
}

impl Update for ExampleComponent {
    fn ref_node(&mut self, _props: &Props, name: String, node: web_sys::Node) -> Cmd<Self> {
        if name == "canvas" {
            if let Some(canvas_element) = node.dyn_ref::<web_sys::HtmlCanvasElement>() {
                let context = canvas_element
                    .get_context("2d")
                    .ok()
                    .unwrap_or(None)
                    .and_then(|context| {
                        context.dyn_into::<web_sys::CanvasRenderingContext2d>().ok()
                    });
                if let Some(context) = context {
                    context.set_line_width(10.0);
                    context.stroke_rect(75.0, 140.0, 150.0, 110.0);
                    context.fill_rect(130.0, 190.0, 40.0, 60.0);
                    context.move_to(50.0, 140.0);
                    context.line_to(150.0, 60.0);
                    context.line_to(250.0, 140.0);
                    context.close_path();
                    context.stroke();
                }
            }
        }
        Cmd::none()
    }

    fn update(&mut self, _props: &Props, msg: Msg) -> Cmd<Self> {
        Cmd::none()
    }
}

impl Render for ExampleComponent {
    fn render(&self, _props: &Props, _children: Vec<Html<Self>>) -> Html<Self> {
        Html::canvas(
            Attributes::new().nut("width", 300).nut("height", 300),
            Events::new(),
            vec![],
        )
        .ref_name("canvas")
    }
}
```

---

<div align="center">expected results</div>

![expected results](./img/04_getting_a_reference_to_an_element/expected_results.png)

This example refered to [MDN](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D).

## To set a name for reference

```rust
Html::canvas(
    Attributes::new().nut("width", 300).nut("height", 300),
    Events::new(),
    vec![],
)
.ref_name("canvas")
```

By `ref_name` function, you can set a name for reference to an element. The name will be used to identify elements in `ref_node` function.

## To get a reference to an element

In `ref_node` function, you can use elements which you set a name to refer. 

Everytime Kagura rerenders DOM, the function will be called. Therefore, unlike `update`, Kagura does not rerender DOM after calling `ref_node`. This is a guard not to loop rendering process infinity.

## How to rerender after ref_node

Having a lot of dangers, but, Kagura supports rerendering after `ref_node`. To do this, you can use `Cmd::chain`.

`Cmd::chain` needs a message to `update`. When Kagura receives `Cmd::chain`, Kagura calls `update` by the message given by `Cmd::chain`. And that Kagura rerender DOM after calling `update`.

This is the way to rerender DOM after `ref_node`.

This is an example:

```rust
fn ref_node(&mut self, _props: &Props, name: String, node: web_sys::Node) -> Cmd<Self> {
    // A guard not to loop infinity
    if self.an_element.is_none() {
        // checks a name of a node
        if name == "element name" {
            if let Ok(element) = node.dyn_into::<web_sys::Element>() {
                self.an_element = Some(element);
                return Cmd::chain(Msg::AMessageWhenElementIsSetted);
            }
        }
    }

    Cmd::none()
}
```
