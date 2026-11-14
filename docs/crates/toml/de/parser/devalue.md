**toml > de > parser > devalue**

# Module: de::parser::devalue

## Contents

**Structs**

- [`DeFloat`](#defloat) - Represents a TOML integer
- [`DeInteger`](#deinteger) - Represents a TOML integer

**Enums**

- [`DeValue`](#devalue) - Representation of a TOML value.

**Traits**

- [`Index`](#index) - Types that can be used to index a `toml::Value`

**Type Aliases**

- [`DeString`](#destring) - Type representing a TOML string, payload of the `DeValue::String` variant

---

## toml::de::parser::devalue::DeFloat

*Struct*

Represents a TOML integer

**Generic Parameters:**
- 'i

**Methods:**

- `fn as_str(self: &Self) -> &str` - [`FromStr`][std::str::FromStr]-compatible representation of a float
- `fn make_owned(self: & mut Self)` - Ensure no data is borrowed

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, formatter: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## toml::de::parser::devalue::DeInteger

*Struct*

Represents a TOML integer

**Generic Parameters:**
- 'i

**Methods:**

- `fn as_str(self: &Self) -> &str` - [`from_str_radix`][i64::from_str_radix]-compatible representation of an integer
- `fn radix(self: &Self) -> u32` - Numeric base of [`DeInteger::as_str`]
- `fn make_owned(self: & mut Self)` - Ensure no data is borrowed

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Display**
  - `fn fmt(self: &Self, formatter: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## toml::de::parser::devalue::DeString

*Type Alias*: `alloc::borrow::Cow<'i, str>`

Type representing a TOML string, payload of the `DeValue::String` variant



## toml::de::parser::devalue::DeValue

*Enum*

Representation of a TOML value.

**Generic Parameters:**
- 'i

**Variants:**
- `String(DeString<'i>)` - Represents a TOML string
- `Integer(DeInteger<'i>)` - Represents a TOML integer
- `Float(DeFloat<'i>)` - Represents a TOML float
- `Boolean(bool)` - Represents a TOML boolean
- `Datetime(toml_datetime::Datetime)` - Represents a TOML datetime
- `Array(crate::de::DeArray<'i>)` - Represents a TOML array
- `Table(crate::de::DeTable<'i>)` - Represents a TOML table

**Methods:**

- `fn parse(input: &'i str) -> Result<Spanned<Self>, crate::de::Error>` - Parse a TOML value
- `fn parse_recoverable(input: &'i str) -> (Spanned<Self>, Vec<crate::de::Error>)` - Parse a TOML value, with best effort recovery on error
- `fn make_owned(self: & mut Self)` - Ensure no data is borrowed
- `fn get<I>(self: &Self, index: I) -> Option<&Spanned<Self>>` - Index into a TOML array or map. A string index can be used to access a
- `fn as_integer(self: &Self) -> Option<&DeInteger<'i>>` - Extracts the integer value if it is an integer.
- `fn is_integer(self: &Self) -> bool` - Tests whether this value is an integer.
- `fn as_float(self: &Self) -> Option<&DeFloat<'i>>` - Extracts the float value if it is a float.
- `fn is_float(self: &Self) -> bool` - Tests whether this value is a float.
- `fn as_bool(self: &Self) -> Option<bool>` - Extracts the boolean value if it is a boolean.
- `fn is_bool(self: &Self) -> bool` - Tests whether this value is a boolean.
- `fn as_str(self: &Self) -> Option<&str>` - Extracts the string of this value if it is a string.
- `fn is_str(self: &Self) -> bool` - Tests if this value is a string.
- `fn as_datetime(self: &Self) -> Option<&Datetime>` - Extracts the datetime value if it is a datetime.
- `fn is_datetime(self: &Self) -> bool` - Tests whether this value is a datetime.
- `fn as_array(self: &Self) -> Option<&DeArray<'i>>` - Extracts the array value if it is an array.
- `fn is_array(self: &Self) -> bool` - Tests whether this value is an array.
- `fn as_table(self: &Self) -> Option<&DeTable<'i>>` - Extracts the table value if it is a table.
- `fn is_table(self: &Self) -> bool` - Tests whether this value is a table.
- `fn same_type(self: &Self, other: &DeValue) -> bool` - Tests whether this and another value have the same type.
- `fn type_str(self: &Self) -> &'static str` - Returns a human-readable representation of the type of this value.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Index**
  - `fn index(self: &Self, index: I) -> &Spanned<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## toml::de::parser::devalue::Index

*Trait*

Types that can be used to index a `toml::Value`

Currently this is implemented for `usize` to index arrays and `str` to index
tables.

This trait is sealed and not intended for implementation outside of the
`toml` crate.



