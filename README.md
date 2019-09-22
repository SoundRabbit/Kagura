<div align="center">

![logo](kagura.png)

# Kagura

Frontend frame-work for wasm on Rust.

</div>

## Tutorial

### In Japanese

[[Kagura] Kagura + Rust でWebページを作成](https://qiita.com/ne_no_usa/items/0d8e33bad3aa7ec6d8fb)

## Hello World

```rust
extern crate kagura;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    kagura::run(kagura::Component::new(State, update, render), "app");
}

struct State;

struct Msg;

fn update(_: &mut State, _: &Msg) -> Option<()> {None}

fn render(_: &State) -> kagura::Html<Msg> {
    use kagura::Html;
    use kagura::Attributes;
    use kagura::Events;
    Html::h1(
        Attributes::new(),
        Events::new(),
        vec![
            Html::unsafe_text("hello kagura"),
        ],
    )
}
```

## Usage

### Create component

```rust
kagura::Component::new(initial_state, update, render)
```

`update` and `render` is function :

```rust
update : fn(&mut State, &Msg) -> Option<Sub>
```

```rust
render : fn(&State) -> Html<Msg>
```

### Set a component to application

```rust
kagura::run(component, id_of_entry_point_in_html)
```

### Render Element

```rust
kagura::Html::html_tag(attributes, events, children)
```

`attributes` : instance of `kagura::Attributes`

`events` : instance of `kagura::Events`

`children` : Vec&lt;Html&gt;

#### Example

```Html
<ul class="list example" id="my-list" data-fizz="bazz">
    <li>foo</li>
    <li>bar</li>
    <li>baz</li>
</ul>
```

is made by

```rust
use kagura::Html;
use kagura::Attributes;
use kagura::Events;

Html::ul(
    Attributes::new()
        .class("list")
        .class("example")
        .id("my-list")
        .string("data-fizz", "bazz"),
    Events::new(),
    vec![
        Html::li(Attributes::new(), Events::new(), vec![Html::unsafe_text("foo")]),
        Html::li(Attributes::new(), Events::new(), vec![Html::unsafe_text("bar")]),
        Html::li(Attributes::new(), Events::new(), vec![Html::unsafe_text("baz")])
    ]
)
```

### Render Component

```rust
kagura::Html::component(component)
```

`component` : instance of `kagura::Component`

### Transmit message to a parent component

`update` can send message to parent compoent as Some(message).

### Receive child message and bind to own message

`component.subscribe(impl: Sub -> Box<Any>)` can receive message from child component and bind to own message.

#### Example

```rust
fn render() -> Html<Msg> {
    Html::component(
        child_component::new().subscribe(|sub| match sub {
            child_component::Sub::Foo => Msg::Bar
        })
    )
}

mod child_component {
    fn new() -> Component<Msg, State, Sub> {
        Component::new(initial_state, update, render)
    }

    .
    .
    .
}
```
