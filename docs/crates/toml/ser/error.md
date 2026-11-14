**toml > ser > error**

# Module: ser::error

## Contents

**Structs**

- [`Error`](#error) - Errors that can occur when serializing a type.

---

## toml::ser::error::Error

*Struct*

Errors that can occur when serializing a type.

**Traits:** Eq, Error

**Trait Implementations:**

- **Error**
  - `fn custom<T>(msg: T) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **From**
  - `fn from(_: core::fmt::Error) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



