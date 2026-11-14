**bytes > buf > limit**

# Module: buf::limit

## Contents

**Structs**

- [`Limit`](#limit) - A `BufMut` adapter which limits the amount of bytes that can be written

---

## bytes::buf::limit::Limit

*Struct*

A `BufMut` adapter which limits the amount of bytes that can be written
to an underlying buffer.

**Generic Parameters:**
- T

**Methods:**

- `fn into_inner(self: Self) -> T` - Consumes this `Limit`, returning the underlying value.
- `fn get_ref(self: &Self) -> &T` - Gets a reference to the underlying `BufMut`.
- `fn get_mut(self: & mut Self) -> & mut T` - Gets a mutable reference to the underlying `BufMut`.
- `fn limit(self: &Self) -> usize` - Returns the maximum number of bytes that can be written
- `fn set_limit(self: & mut Self, lim: usize)` - Sets the maximum number of bytes that can be written.

**Trait Implementations:**

- **BufMut**
  - `fn remaining_mut(self: &Self) -> usize`
  - `fn chunk_mut(self: & mut Self) -> & mut UninitSlice`
  - `fn advance_mut(self: & mut Self, cnt: usize)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



