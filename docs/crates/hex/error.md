**hex > error**

# Module: error

## Contents

**Enums**

- [`FromHexError`](#fromhexerror) - The error type for decoding a hex string into `Vec<u8>` or `[u8; N]`.

---

## hex::error::FromHexError

*Enum*

The error type for decoding a hex string into `Vec<u8>` or `[u8; N]`.

**Variants:**
- `InvalidHexCharacter{ c: char, index: usize }` - An invalid character was found. Valid ones are: `0...9`, `a...f`
- `OddLength` - A hex string's length needs to be even, as two digits correspond to
- `InvalidStringLength` - If the hex string is decoded into a fixed sized container, such as an

**Traits:** Copy, Error

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



