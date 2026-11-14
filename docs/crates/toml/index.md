# toml

A [serde]-compatible [TOML]-parsing library

TOML itself is a simple, ergonomic, and readable configuration format:

```toml
[package]
name = "toml"

[dependencies]
serde = "1.0"
```

The TOML format tends to be relatively common throughout the Rust community
for configuration, notably being used by [Cargo], Rust's package manager.

## TOML values

A TOML document is represented with the [`Table`] type which maps `String` to the [`Value`] enum:

 ```
# use toml::value::{Datetime, Array, Table};
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Datetime(Datetime),
    Array(Array),
    Table(Table),
}
```

## Parsing TOML

The easiest way to parse a TOML document is via the [`Table`] type:

 ```
use toml::Table;

let value = "foo = 'bar'".parse::<Table>().unwrap();

assert_eq!(value["foo"].as_str(), Some("bar"));
```

The [`Table`] type implements a number of convenience methods and
traits; the example above uses [`FromStr`] to parse a [`str`] into a
[`Table`].

## Deserialization and Serialization

This crate supports [`serde`] 1.0 with a number of
implementations of the `Deserialize`, `Serialize`, `Deserializer`, and
`Serializer` traits. Namely, you'll find:

* `Deserialize for Table`
* `Serialize for Table`
* `Deserialize for Value`
* `Serialize for Value`
* `Deserialize for Datetime`
* `Serialize for Datetime`
* `Deserializer for de::Deserializer`
* `Serializer for ser::Serializer`
* `Deserializer for Table`
* `Deserializer for Value`

This means that you can use Serde to deserialize/serialize the
[`Table`] type as well as [`Value`] and [`Datetime`] type in this crate. You can also
use the [`Deserializer`], [`Serializer`], or [`Table`] type itself to act as
a deserializer/serializer for arbitrary types.

An example of deserializing with TOML is:

 ```
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    ip: String,
    port: Option<u16>,
    keys: Keys,
}

#[derive(Deserialize)]
struct Keys {
    github: String,
    travis: Option<String>,
}

let config: Config = toml::from_str(r#"
    ip = '127.0.0.1'

    [keys]
    github = 'xxxxxxxxxxxxxxxxx'
    travis = 'yyyyyyyyyyyyyyyyy'
"#).unwrap();

assert_eq!(config.ip, "127.0.0.1");
assert_eq!(config.port, None);
assert_eq!(config.keys.github, "xxxxxxxxxxxxxxxxx");
assert_eq!(config.keys.travis.as_ref().unwrap(), "yyyyyyyyyyyyyyyyy");
```

You can serialize types in a similar fashion:

 ```
use serde::Serialize;

#[derive(Serialize)]
struct Config {
    ip: String,
    port: Option<u16>,
    keys: Keys,
}

#[derive(Serialize)]
struct Keys {
    github: String,
    travis: Option<String>,
}

let config = Config {
    ip: "127.0.0.1".to_string(),
    port: None,
    keys: Keys {
        github: "xxxxxxxxxxxxxxxxx".to_string(),
        travis: Some("yyyyyyyyyyyyyyyyy".to_string()),
    },
};

let toml = toml::to_string(&config).unwrap();
```

[TOML]: https://github.com/toml-lang/toml
[Cargo]: https://crates.io/
[`serde`]: https://serde.rs/
[serde]: https://serde.rs/

## Modules

### [`toml`](toml.md)

*1 macro, 4 modules*

### [`de`](de.md)

*2 functions*

### [`de::deserializer`](de/deserializer.md)

*1 struct*

### [`de::deserializer::Deserializer`](de/deserializer/Deserializer.md)

*1 function*

### [`de::deserializer::value`](de/deserializer/value.md)

*1 struct*

### [`de::deserializer::value::ValueDeserializer`](de/deserializer/value/ValueDeserializer.md)

*1 function*

### [`de::error`](de/error.md)

*1 struct*

### [`de::parser::dearray`](de/parser/dearray.md)

*1 struct*

### [`de::parser::detable`](de/parser/detable.md)

*1 type alias*

### [`de::parser::devalue`](de/parser/devalue.md)

*1 enum, 1 trait, 1 type alias, 2 structs*

### [`de::parser::devalue::DeInteger`](de/parser/devalue/DeInteger.md)

*2 functions*

### [`map`](map.md)

*1 enum, 8 structs*

### [`ser`](ser.md)

*2 functions*

### [`ser::document`](ser/document.md)

*1 struct*

### [`ser::document::Serializer`](ser/document/Serializer.md)

*1 function*

### [`ser::document::buffer`](ser/document/buffer.md)

*1 struct*

### [`ser::error`](ser/error.md)

*1 struct*

### [`ser::value`](ser/value.md)

*1 struct*

### [`ser::value::array`](ser/value/array.md)

*1 struct*

### [`ser::value::map`](ser/value/map.md)

*1 struct*

### [`table`](table.md)

*1 type alias*

### [`value`](value.md)

*1 enum, 1 trait, 1 type alias*

