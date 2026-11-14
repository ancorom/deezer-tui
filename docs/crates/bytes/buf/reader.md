**bytes > buf > reader**

# Module: buf::reader

## Contents

**Structs**

- [`Reader`](#reader) - A `Buf` adapter which implements `io::Read` for the inner value.

---

## bytes::buf::reader::Reader

*Struct*

A `Buf` adapter which implements `io::Read` for the inner value.

This struct is generally created by calling `reader()` on `Buf`. See
documentation of [`reader()`](Buf::reader) for more
details.

**Generic Parameters:**
- B

**Methods:**

- `fn get_ref(self: &Self) -> &B` - Gets a reference to the underlying `Buf`.
- `fn get_mut(self: & mut Self) -> & mut B` - Gets a mutable reference to the underlying `Buf`.
- `fn into_inner(self: Self) -> B` - Consumes this `Reader`, returning the underlying value.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **BufRead**
  - `fn fill_buf(self: & mut Self) -> io::Result<&[u8]>`
  - `fn consume(self: & mut Self, amt: usize)`
- **Read**
  - `fn read(self: & mut Self, dst: & mut [u8]) -> io::Result<usize>`



