**toml**

# Module: toml

## Contents

**Modules**

- [`de`](#de) - Deserializing TOML into Rust structures.
- [`map`](#map) - A map of `String` to [Value][crate::Value].
- [`ser`](#ser) - Serializing Rust structures into TOML.
- [`value`](#value) - Definition of a TOML [value][Value]

**Macros**

- [`toml`](#toml) - Construct a [`Table`] from TOML syntax.

---

## Module: de

Deserializing TOML into Rust structures.

This module contains all the Serde support for deserializing TOML documents
into Rust structures. Note that some top-level functions here are also
provided at the top of the crate.



## Module: map

A map of `String` to [Value][crate::Value].

By default the map is backed by a [`BTreeMap`]. Enable the `preserve_order`
feature of toml-rs to use [`IndexMap`] instead.

[`BTreeMap`]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
[`IndexMap`]: https://docs.rs/indexmap



## Module: ser

Serializing Rust structures into TOML.

This module contains all the Serde support for serializing Rust structures
into TOML documents (as strings). Note that some top-level functions here
are also provided at the top of the crate.



## toml::toml

*Declarative Macro*

Construct a [`Table`] from TOML syntax.

```rust
let cargo_toml = toml::toml! {
    [package]
    name = "toml"

    [dependencies]
    serde = "1.0"

    [dev-dependencies]
    serde_derive = "1.0"
    serde_json = "1.0"
};

println!("{:#?}", cargo_toml);
```

```rust
macro_rules! toml {
    ($($toml:tt)+) => { ... };
}
```



## Module: value

Definition of a TOML [value][Value]



