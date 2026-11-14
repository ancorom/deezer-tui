**toml > de**

# Module: de

## Contents

**Functions**

- [`from_slice`](#from_slice) - Deserializes bytes into a type.
- [`from_str`](#from_str) - Deserializes a string into a type.

---

## toml::de::from_slice

*Function*

Deserializes bytes into a type.

This function will attempt to interpret `s` as a TOML document and
deserialize `T` from the document.

To deserializes TOML values, instead of documents, see [`ValueDeserializer`].

```rust
fn from_slice<'de, T>(s: &'de [u8]) -> Result<T, Error>
```



## toml::de::from_str

*Function*

Deserializes a string into a type.

This function will attempt to interpret `s` as a TOML document and
deserialize `T` from the document.

To deserializes TOML values, instead of documents, see [`ValueDeserializer`].

# Examples

```
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    title: String,
    owner: Owner,
}

#[derive(Deserialize)]
struct Owner {
    name: String,
}

let config: Config = toml::from_str(r#"
    title = 'TOML Example'

    [owner]
    name = 'Lisa'
"#).unwrap();

assert_eq!(config.title, "TOML Example");
assert_eq!(config.owner.name, "Lisa");
```

```rust
fn from_str<'de, T>(s: &'de str) -> Result<T, Error>
```



