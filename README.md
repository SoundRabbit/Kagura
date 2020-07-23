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

use kagura::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    kagura::run(Component::new(init, update, render).with(Props), "app");
}

type HelloKagura = Component<Msg, Props, State, Sub>;

struct Props;

struct State;

struct Msg;

struct Sub;

fn init(_this: &mut HelloKagura, _state: Option<State>, _props: Props) -> (State, Cmd<Msg, Sub>) {
    (State, Cmd::none())
}

fn update(_: &mut State, _: Msg) -> Cmd<Msg, Sub> {Cmd::none()}

fn render(_: &State) -> Html {
    Html::h1(
        Attributes::new(),
        Events::new(),
        vec![
            Html::text("hello kagura"),
        ],
    )
}
```

## Usage

### Create component

```rust
kagura::Component::new(init, update, render)
```

`init`, `update` and `render` is function :


```rust
init : fn() -> State
```

```rust
update : fn(&mut State, Msg) -> Cmd<Msg, Sub>
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
fn render() -> Html {
    Html::component(
        child_component::new().with(props).subscribe(|sub| match sub {
            child_component::Sub::Foo => Msg::Bar
        })
    )
}

mod child_component {
    fn new() -> Component<Msg, Props, State, Sub> {
        Component::new(init, update, render)
    }

    .
    .
    .
}
```

### Cmd

#### `Cmd::none()`

`Cmd::none()` means nothing to do. If you return Cmd::none(), kagura will render.

#### `Cmd::sub(sub: Sub)`

If you send sub-message to parent component, use this.

#### `Cmd::task(task: impl FnOnce(Resolver<Msg>) + 'static)`

You can use this feature like callback function in JavaScript. like this:

```rust
fn update(state: &mut State, msg: Msg) -> kagura::Cmd<Msg, Sub> {
    use kagura::Cmd;
    match msg {
        Msg::ChangeMessage(message) => {
            state.message = message;
            Cmd::none()
        }
        Msg::ChangeMessageTask(message) => Cmd::task(|resolver| {
            let resolver = Closure::once(|| resolver(Msg::ChangeMessage(message)));
            web_sys::window()
                .unwrap()
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    resolver.as_ref().unchecked_ref(),
                    1000,
                );
            resolver.forget();
        }),
    }
}
```

### Batch

You can set a batch process to component like this:

```rust
fn init(this: &mut MyComponent, _state: Option<State>, props: Props) -> (State, Cmd<Msg, Sub>) {
    this.batch(batch::time::tick(1000, || Msg::SomeMsg));
    .
    .
    .
}
```
