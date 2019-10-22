# Add event listener

Event-listeners of an element are presented by `Events` structure. `Events` is initialized by `Events::new()`. You can add an event-listener to `Events` by method chain.

like this:

```rust
Events::new()
    .on_click(|_| Msg::SomeMsg); 
```
