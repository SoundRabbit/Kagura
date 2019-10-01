# Add attributes

Attributes of an element are presented by `Attributes` structure. `Attributes` is initialize by `Attributes.new()`. You can add an attribute to `Attributes by method chain.

like this:

```rust
Attributes::new()
    .class("fizz")
    .class("bazz")
    .string("data-original-attribute", "some-value");
```

## Set delimiter of an attribute

You can set a delimiter of an attribute by `delimit_with` method. This method set a delimiter to last added attribute.

like this

```rust
Attributes::new()
    .string("data-original-attribute-1", "some-value-1")
    .string("data-original-attribute-1", "some-value-2")
    .delimit_with(" ")
    .string("data-original-attribute-2", "some-value-1")
    .string("data-original-attribute-2", "some-value-2")
    .delimit_with(";");
```
