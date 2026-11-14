**cipher > errors**

# Module: errors

## Contents

**Structs**

- [`OverflowError`](#overflowerror) - The error type returned when a cipher position can not be represented
- [`StreamCipherError`](#streamciphererror) - This error is returned by the [`StreamCipher`][crate::stream::StreamCipher]

---

## cipher::errors::OverflowError

*Struct*

The error type returned when a cipher position can not be represented
by the requested type.

**Unit Struct**

**Traits:** Copy

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> Result<(), fmt::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## cipher::errors::StreamCipherError

*Struct*

This error is returned by the [`StreamCipher`][crate::stream::StreamCipher]
trait methods.

Usually it's used in cases when stream cipher has reached the end
of a keystream, but also can be used if lengths of provided input
and output buffers are not equal.

**Unit Struct**

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **From**
  - `fn from(_: OverflowError) -> StreamCipherError`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> Result<(), fmt::Error>`



