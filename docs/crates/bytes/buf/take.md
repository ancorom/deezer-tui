**bytes > buf > take**

# Module: buf::take

## Contents

**Structs**

- [`Take`](#take) - A `Buf` adapter which limits the bytes read from an underlying buffer.

---

## bytes::buf::take::Take

*Struct*

A `Buf` adapter which limits the bytes read from an underlying buffer.

This struct is generally created by calling `take()` on `Buf`. See
documentation of [`take()`](Buf::take) for more details.

**Generic Parameters:**
- T

**Methods:**

- `fn into_inner(self: Self) -> T` - Consumes this `Take`, returning the underlying value.
- `fn get_ref(self: &Self) -> &T` - Gets a reference to the underlying `Buf`.
- `fn get_mut(self: & mut Self) -> & mut T` - Gets a mutable reference to the underlying `Buf`.
- `fn limit(self: &Self) -> usize` - Returns the maximum number of bytes that can be read.
- `fn set_limit(self: & mut Self, lim: usize)` - Sets the maximum number of bytes that can be read.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Buf**
  - `fn remaining(self: &Self) -> usize`
  - `fn chunk(self: &Self) -> &[u8]`
  - `fn advance(self: & mut Self, cnt: usize)`
  - `fn copy_to_bytes(self: & mut Self, len: usize) -> crate::Bytes`
  - `fn chunks_vectored<'a>(self: &'a Self, dst: & mut [IoSlice<'a>]) -> usize`



