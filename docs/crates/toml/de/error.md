**toml > de > error**

# Module: de::error

## Contents

**Structs**

- [`Error`](#error) - Errors that can occur when deserializing a type.

---

## toml::de::error::Error

*Struct*

Errors that can occur when deserializing a type.

**Methods:**

- `fn message(self: &Self) -> &str` - What went wrong
- `fn span(self: &Self) -> Option<core::ops::Range<usize>>` - The start/end index into the original document where the error occurred
- `fn set_input(self: & mut Self, input: Option<&str>)` - Provide the encoded TOML the error applies to

**Traits:** Eq, Error

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Error**
  - `fn custom<T>(msg: T) -> Self`



