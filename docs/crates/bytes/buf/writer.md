**bytes > buf > writer**

# Module: buf::writer

## Contents

**Structs**

- [`Writer`](#writer) - A `BufMut` adapter which implements `io::Write` for the inner value.

---

## bytes::buf::writer::Writer

*Struct*

A `BufMut` adapter which implements `io::Write` for the inner value.

This struct is generally created by calling `writer()` on `BufMut`. See
documentation of [`writer()`](BufMut::writer) for more
details.

**Generic Parameters:**
- B

**Methods:**

- `fn get_ref(self: &Self) -> &B` - Gets a reference to the underlying `BufMut`.
- `fn get_mut(self: & mut Self) -> & mut B` - Gets a mutable reference to the underlying `BufMut`.
- `fn into_inner(self: Self) -> B` - Consumes this `Writer`, returning the underlying value.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Write**
  - `fn write(self: & mut Self, src: &[u8]) -> io::Result<usize>`
  - `fn flush(self: & mut Self) -> io::Result<()>`



