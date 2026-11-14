**bytes > buf > chain**

# Module: buf::chain

## Contents

**Structs**

- [`Chain`](#chain) - A `Chain` sequences two buffers.

---

## bytes::buf::chain::Chain

*Struct*

A `Chain` sequences two buffers.

`Chain` is an adapter that links two underlying buffers and provides a
continuous view across both buffers. It is able to sequence either immutable
buffers ([`Buf`] values) or mutable buffers ([`BufMut`] values).

This struct is generally created by calling [`Buf::chain`]. Please see that
function's documentation for more detail.

# Examples

```
use bytes::{Bytes, Buf};

let mut buf = (&b"hello "[..])
    .chain(&b"world"[..]);

let full: Bytes = buf.copy_to_bytes(11);
assert_eq!(full[..], b"hello world"[..]);
```

[`Buf::chain`]: Buf::chain

**Generic Parameters:**
- T
- U

**Methods:**

- `fn first_ref(self: &Self) -> &T` - Gets a reference to the first underlying `Buf`.
- `fn first_mut(self: & mut Self) -> & mut T` - Gets a mutable reference to the first underlying `Buf`.
- `fn last_ref(self: &Self) -> &U` - Gets a reference to the last underlying `Buf`.
- `fn last_mut(self: & mut Self) -> & mut U` - Gets a mutable reference to the last underlying `Buf`.
- `fn into_inner(self: Self) -> (T, U)` - Consumes this `Chain`, returning the underlying values.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **BufMut**
  - `fn remaining_mut(self: &Self) -> usize`
  - `fn chunk_mut(self: & mut Self) -> & mut UninitSlice`
  - `fn advance_mut(self: & mut Self, cnt: usize)`
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`
- **Buf**
  - `fn remaining(self: &Self) -> usize`
  - `fn chunk(self: &Self) -> &[u8]`
  - `fn advance(self: & mut Self, cnt: usize)`
  - `fn chunks_vectored<'a>(self: &'a Self, dst: & mut [IoSlice<'a>]) -> usize`
  - `fn copy_to_bytes(self: & mut Self, len: usize) -> crate::Bytes`



