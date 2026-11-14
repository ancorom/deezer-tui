**symphonia_metadata > utils > itunes**

# Module: utils::itunes

## Contents

**Functions**

- [`parse_itunes_tag`](#parse_itunes_tag) - Try to parse iTunes metadata from a raw tag and add it to the builder.

---

## symphonia_metadata::utils::itunes::parse_itunes_tag

*Function*

Try to parse iTunes metadata from a raw tag and add it to the builder.

```rust
fn parse_itunes_tag(key: String, value: symphonia_core::meta::RawValue, builder: & mut symphonia_core::meta::MetadataBuilder) -> symphonia_core::errors::Result<()>
```



