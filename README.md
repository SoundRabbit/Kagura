<div align="center">

![logo](kagura.png)

# 神楽 : Kagura

Frontend frame-work for wasm on Rust.

</div>

## Usage

### hello world

See example/hello-world

### create component

```Rust
kagura::Component::new( /*initial_state*/, /*update*/, /*render*/);
```

### set component to application

```Rust
kagura::run( /*component*/, /*id of entry point in html*/);
```

### render component in render

```Rust
fn render( /*state*/ ) -> HTML< /*Msg*/ > {
    Html::component( /*component*/ )
}
```

### subscribe component message

```Rust
Html::component( /*component*/.subscribe(| /*sub msg*/ | {
    /*bind sub msg to msg*/
}));
```
