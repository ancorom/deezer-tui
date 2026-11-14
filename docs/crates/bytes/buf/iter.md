**bytes > buf > iter**

# Module: buf::iter

## Contents

**Structs**

- [`IntoIter`](#intoiter) - Iterator over the bytes contained by the buffer.

---

## bytes::buf::iter::IntoIter

*Struct*

Iterator over the bytes contained by the buffer.

# Examples

Basic usage:

```
use bytes::Bytes;

let buf = Bytes::from(&b"abc"[..]);
let mut iter = buf.into_iter();

assert_eq!(iter.next(), Some(b'a'));
assert_eq!(iter.next(), Some(b'b'));
assert_eq!(iter.next(), Some(b'c'));
assert_eq!(iter.next(), None);
```

**Generic Parameters:**
- T

**Methods:**

- `fn new(inner: T) -> IntoIter<T>` - Creates an iterator over the bytes contained by the buffer.
- `fn into_inner(self: Self) -> T` - Consumes this `IntoIter`, returning the underlying value.
- `fn get_ref(self: &Self) -> &T` - Gets a reference to the underlying `Buf`.
- `fn get_mut(self: & mut Self) -> & mut T` - Gets a mutable reference to the underlying `Buf`.

**Traits:** ExactSizeIterator

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<u8>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



