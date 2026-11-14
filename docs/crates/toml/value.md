**toml > value**

# Module: value

## Contents

**Enums**

- [`Value`](#value) - Representation of a TOML value.

**Traits**

- [`Index`](#index) - Types that can be used to index a `toml::Value`

**Type Aliases**

- [`Array`](#array) - Type representing a TOML array, payload of the `Value::Array` variant

---

## toml::value::Array

*Type Alias*: `alloc::vec::Vec<Value>`

Type representing a TOML array, payload of the `Value::Array` variant



## toml::value::Index

*Trait*

Types that can be used to index a `toml::Value`

Currently this is implemented for `usize` to index arrays and `str` to index
tables.

This trait is sealed and not intended for implementation outside of the
`toml` crate.



## toml::value::Value

*Enum*

Representation of a TOML value.

**Variants:**
- `String(alloc::string::String)` - Represents a TOML string
- `Integer(i64)` - Represents a TOML integer
- `Float(f64)` - Represents a TOML float
- `Boolean(bool)` - Represents a TOML boolean
- `Datetime(Datetime)` - Represents a TOML datetime
- `Array(Array)` - Represents a TOML array
- `Table(Table)` - Represents a TOML table

**Methods:**

- `fn try_from<T>(value: T) -> Result<Self, crate::ser::Error>` - Convert a `T` into `toml::Value` which is an enum that can represent
- `fn try_into<'de, T>(self: Self) -> Result<T, crate::de::Error>` - Interpret a `toml::Value` as an instance of type `T`.
- `fn get<I>(self: &Self, index: I) -> Option<&Self>` - Index into a TOML array or map. A string index can be used to access a
- `fn get_mut<I>(self: & mut Self, index: I) -> Option<& mut Self>` - Mutably index into a TOML array or map. A string index can be used to
- `fn as_integer(self: &Self) -> Option<i64>` - Extracts the integer value if it is an integer.
- `fn is_integer(self: &Self) -> bool` - Tests whether this value is an integer.
- `fn as_float(self: &Self) -> Option<f64>` - Extracts the float value if it is a float.
- `fn is_float(self: &Self) -> bool` - Tests whether this value is a float.
- `fn as_bool(self: &Self) -> Option<bool>` - Extracts the boolean value if it is a boolean.
- `fn is_bool(self: &Self) -> bool` - Tests whether this value is a boolean.
- `fn as_str(self: &Self) -> Option<&str>` - Extracts the string of this value if it is a string.
- `fn is_str(self: &Self) -> bool` - Tests if this value is a string.
- `fn as_datetime(self: &Self) -> Option<&Datetime>` - Extracts the datetime value if it is a datetime.
- `fn is_datetime(self: &Self) -> bool` - Tests whether this value is a datetime.
- `fn as_array(self: &Self) -> Option<&Vec<Self>>` - Extracts the array value if it is an array.
- `fn as_array_mut(self: & mut Self) -> Option<& mut Vec<Self>>` - Extracts the array value if it is an array.
- `fn is_array(self: &Self) -> bool` - Tests whether this value is an array.
- `fn as_table(self: &Self) -> Option<&Table>` - Extracts the table value if it is a table.
- `fn as_table_mut(self: & mut Self) -> Option<& mut Table>` - Extracts the table value if it is a table.
- `fn is_table(self: &Self) -> bool` - Tests whether this value is a table.
- `fn same_type(self: &Self, other: &Self) -> bool` - Tests whether this and another value have the same type.
- `fn type_str(self: &Self) -> &'static str` - Returns a human-readable representation of the type of this value.

**Trait Implementations:**

- **From**
  - `fn from(val: f64) -> Value`
- **Deserializer**
  - `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, crate::de::Error>`
  - `fn deserialize_enum<V>(self: Self, _name: &'static str, _variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, crate::de::Error>`
  - `fn deserialize_option<V>(self: Self, visitor: V) -> Result<<V as >::Value, crate::de::Error>`
  - `fn deserialize_newtype_struct<V>(self: Self, _name: &'static str, visitor: V) -> Result<<V as >::Value, crate::de::Error>`
  - `fn deserialize_struct<V>(self: Self, name: &'static str, _fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value, crate::de::Error>`
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
  - `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
  - `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`
- **From**
  - `fn from(val: u8) -> Value`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **From**
  - `fn from(val: i32) -> Value`
- **Serialize**
  - `fn serialize<S>(self: &Self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`
- **From**
  - `fn from(val: String) -> Value`
- **IntoDeserializer**
  - `fn into_deserializer(self: Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Deserialize**
  - `fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`
- **From**
  - `fn from(val: BTreeMap<S, V>) -> Self`
- **From**
  - `fn from(val: Datetime) -> Value`
- **From**
  - `fn from(val: &'a str) -> Self`
- **From**
  - `fn from(val: f32) -> Value`
- **Index**
  - `fn index(self: &Self, index: I) -> &Self`
- **From**
  - `fn from(val: u32) -> Value`
- **From**
  - `fn from(val: i8) -> Value`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **From**
  - `fn from(val: i64) -> Value`
- **From**
  - `fn from(val: HashMap<S, V>) -> Self`
- **From**
  - `fn from(val: Table) -> Value`
- **FromStr**
  - `fn from_str(s: &str) -> Result<Self, <Self as >::Err>`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **From**
  - `fn from(val: Vec<V>) -> Self`
- **From**
  - `fn from(val: bool) -> Value`
- **IndexMut**
  - `fn index_mut(self: & mut Self, index: I) -> & mut Self`



