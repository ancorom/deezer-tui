**toml > ser > document**

# Module: ser::document

## Contents

**Structs**

- [`Serializer`](#serializer) - Serialization for TOML documents.

---

## toml::ser::document::Serializer

*Struct*

Serialization for TOML documents.

This structure implements serialization support for TOML to serialize an
arbitrary type to TOML. Note that the TOML format does not support all
datatypes in Rust, such as enums, tuples, and tuple structs. These types
will generate an error when serialized.

Currently a serializer always writes its output to an in-memory `String`,
which is passed in when creating the serializer itself.

To serialize TOML values, instead of documents, see
[`ValueSerializer`][super::value::ValueSerializer].

**Generic Parameters:**
- 'd

**Methods:**

- `fn new(buf: &'d  mut Buffer) -> Self` - Creates a new serializer which will emit TOML into the buffer provided.
- `fn pretty(buf: &'d  mut Buffer) -> Self` - Apply a default "pretty" policy to the document

**Trait Implementations:**

- **Serializer**
  - `fn serialize_bool(self: Self, _v: bool) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_i8(self: Self, _v: i8) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_i16(self: Self, _v: i16) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_i32(self: Self, _v: i32) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_i64(self: Self, _v: i64) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_u8(self: Self, _v: u8) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_u16(self: Self, _v: u16) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_u32(self: Self, _v: u32) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_u64(self: Self, _v: u64) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_f32(self: Self, _v: f32) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_f64(self: Self, _v: f64) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_char(self: Self, _v: char) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_str(self: Self, _v: &str) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_bytes(self: Self, _v: &[u8]) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_none(self: Self) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_some<T>(self: Self, v: &T) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_unit(self: Self) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_unit_struct(self: Self, name: &'static str) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_unit_variant(self: Self, name: &'static str, _variant_index: u32, _variant: &'static str) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_newtype_struct<T>(self: Self, _name: &'static str, v: &T) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_newtype_variant<T>(self: Self, _name: &'static str, _variant_index: u32, variant: &'static str, value: &T) -> Result<<Self as >::Ok, <Self as >::Error>`
  - `fn serialize_seq(self: Self, _len: Option<usize>) -> Result<<Self as >::SerializeSeq, <Self as >::Error>`
  - `fn serialize_tuple(self: Self, len: usize) -> Result<<Self as >::SerializeTuple, <Self as >::Error>`
  - `fn serialize_tuple_struct(self: Self, _name: &'static str, len: usize) -> Result<<Self as >::SerializeTupleStruct, <Self as >::Error>`
  - `fn serialize_tuple_variant(self: Self, _name: &'static str, _variant_index: u32, variant: &'static str, len: usize) -> Result<<Self as >::SerializeTupleVariant, <Self as >::Error>`
  - `fn serialize_map(self: Self, _len: Option<usize>) -> Result<<Self as >::SerializeMap, <Self as >::Error>`
  - `fn serialize_struct(self: Self, _name: &'static str, len: usize) -> Result<<Self as >::SerializeStruct, <Self as >::Error>`
  - `fn serialize_struct_variant(self: Self, _name: &'static str, _variant_index: u32, variant: &'static str, _len: usize) -> Result<<Self as >::SerializeStructVariant, <Self as >::Error>`



