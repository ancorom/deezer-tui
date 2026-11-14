**toml > ser > document > Serializer**

# Module: ser::document::Serializer

## Contents

**Functions**

- [`pretty`](#pretty) - Apply a default "pretty" policy to the document

---

## toml::ser::document::Serializer::pretty

*Function*

Apply a default "pretty" policy to the document

For greater customization, instead serialize to a
[`toml_edit::DocumentMut`](https://docs.rs/toml_edit/latest/toml_edit/struct.DocumentMut.html).

```rust
fn pretty(buf: &'d  mut Buffer) -> Self
```



