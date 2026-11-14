**toml > de > deserializer > value**

# Module: de::deserializer::value

## Contents

**Structs**

- [`ValueDeserializer`](#valuedeserializer) - Deserialization implementation for TOML [values][crate::Value].

---

## toml::de::deserializer::value::ValueDeserializer

*Struct*

Deserialization implementation for TOML [values][crate::Value].

# Example

```
# #[cfg(feature = "parse")] {
# #[cfg(feature = "display")] {
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

let value = r#"{ title = 'TOML Example', owner = { name = 'Lisa' } }"#;
let deserializer = toml::de::ValueDeserializer::parse(value).unwrap();
let config = Config::deserialize(deserializer).unwrap();
assert_eq!(config.title, "TOML Example");
assert_eq!(config.owner.name, "Lisa");
# }
# }
```

**Generic Parameters:**
- 'i

**Methods:**

- `fn parse(raw: &'i str) -> Result<Self, Error>` - Parse a TOML value
- `fn new(raw: &'i str) -> Result<Self, Error>` - Deprecated, replaced with [`ValueDeserializer::parse`]

**Trait Implementations:**

- **Deserializer**
  - `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`
  - `fn deserialize_u128<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`
  - `fn deserialize_i128<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`
  - `fn deserialize_option<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`
  - `fn deserialize_newtype_struct<V>(self: Self, _name: &'static str, visitor: V) -> Result<<V as >::Value, Error>`
  - `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>`
  - `fn deserialize_enum<V>(self: Self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>`
  - `fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
- **IntoDeserializer**
  - `fn into_deserializer(self: Self) -> <Self as >::Deserializer`
- **From**
  - `fn from(root: Spanned<DeValue<'i>>) -> Self`



