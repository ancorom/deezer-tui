**toml > ser > value**

# Module: ser::value

## Contents

**Structs**

- [`ValueSerializer`](#valueserializer) - Serialization for TOML [values][crate::Value].

---

## toml::ser::value::ValueSerializer

*Struct*

Serialization for TOML [values][crate::Value].

This structure implements serialization support for TOML to serialize an
arbitrary type to TOML. Note that the TOML format does not support all
datatypes in Rust, such as enums, tuples, and tuple structs. These types
will generate an error when serialized.

Currently a serializer always writes its output to an in-memory `String`,
which is passed in when creating the serializer itself.

# Examples

```
use serde::Serialize;

#[derive(Serialize)]
struct Config {
    database: Database,
}

#[derive(Serialize)]
struct Database {
    ip: String,
    port: Vec<u16>,
    connection_max: u32,
    enabled: bool,
}

let config = Config {
    database: Database {
        ip: "192.168.1.1".to_string(),
        port: vec![8001, 8002, 8003],
        connection_max: 5000,
        enabled: false,
    },
};

let mut value = String::new();
serde::Serialize::serialize(
    &config,
    toml::ser::ValueSerializer::new(&mut value)
).unwrap();
println!("{}", value)
```

**Generic Parameters:**
- 'd

**Methods:**

- `fn new(dst: &'d  mut String) -> Self` - Creates a new serializer which will emit TOML into the buffer provided.

**Trait Implementations:**

- **Serializer**
  - `fn serialize_bool(self: Self, v: bool) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_i8(self: Self, v: i8) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_i16(self: Self, v: i16) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_i32(self: Self, v: i32) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_i64(self: Self, v: i64) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_i128(self: Self, v: i128) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_u8(self: Self, v: u8) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_u16(self: Self, v: u16) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_u32(self: Self, v: u32) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_u64(self: Self, v: u64) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_u128(self: Self, v: u128) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_f32(self: Self, v: f32) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_f64(self: Self, v: f64) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_char(self: Self, v: char) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_str(self: Self, v: &str) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_bytes(self: Self, value: &[u8]) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_none(self: Self) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_some<T>(self: Self, value: &T) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_unit(self: Self) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_unit_struct(self: Self, name: &'static str) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_unit_variant(self: Self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_newtype_struct<T>(self: Self, _name: &'static str, value: &T) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_newtype_variant<T>(self: Self, _name: &'static str, _variant_index: u32, variant: &'static str, value: &T) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_seq(self: Self, len: Option<usize>) -> Result<<Self as >::SerializeSeq, <Self as >::Error>`
  - `fn serialize_tuple(self: Self, len: usize) -> Result<<Self as >::SerializeTuple, <Self as >::Error>`
  - `fn serialize_tuple_struct(self: Self, _name: &'static str, len: usize) -> Result<<Self as >::SerializeTupleStruct, <Self as >::Error>`
  - `fn serialize_tuple_variant(self: Self, _name: &'static str, _variant_index: u32, variant: &'static str, len: usize) -> Result<<Self as >::SerializeTupleVariant, <Self as >::Error>`
  - `fn serialize_map(self: Self, _len: Option<usize>) -> Result<<Self as >::SerializeMap, <Self as >::Error>`
  - `fn serialize_struct(self: Self, name: &'static str, _len: usize) -> Result<<Self as >::SerializeStruct, <Self as >::Error>`
  - `fn serialize_struct_variant(self: Self, _name: &'static str, _variant_index: u32, variant: &'static str, len: usize) -> Result<<Self as >::SerializeStructVariant, <Self as >::Error>`



