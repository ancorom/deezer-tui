**symphonia_core > io > scoped_stream**

# Module: io::scoped_stream

## Contents

**Structs**

- [`ScopedStream`](#scopedstream) - A `ScopedStream` restricts the number of bytes that may be read to an upper limit.

---

## symphonia_core::io::scoped_stream::ScopedStream

*Struct*

A `ScopedStream` restricts the number of bytes that may be read to an upper limit.

**Generic Parameters:**
- B

**Methods:**

- `fn new(inner: B, len: u64) -> Self` - Instantiates a new `ScopedStream` with an upper limit on the number of bytes that can be
- `fn inner(self: &Self) -> &B` - Returns an immutable reference to the inner stream.
- `fn inner_mut(self: & mut Self) -> & mut B` - Returns a mutable reference to the inner stream.
- `fn ignore(self: & mut Self) -> io::Result<()>` - Ignores the remainder of the `ScopedStream`.
- `fn into_inner(self: Self) -> B` - Convert the `ScopedStream` to the inner stream.

**Trait Implementations:**

- **FiniteStream**
  - `fn byte_len(self: &Self) -> u64` - Returns the length of the the `ScopedStream`.
  - `fn bytes_read(self: &Self) -> u64` - Returns the number of bytes read.
  - `fn bytes_available(self: &Self) -> u64` - Returns the number of bytes available to read.
- **ReadBytes**
  - `fn read_byte(self: & mut Self) -> io::Result<u8>`
  - `fn read_double_bytes(self: & mut Self) -> io::Result<[u8; 2]>`
  - `fn read_triple_bytes(self: & mut Self) -> io::Result<[u8; 3]>`
  - `fn read_quad_bytes(self: & mut Self) -> io::Result<[u8; 4]>`
  - `fn read_buf(self: & mut Self, buf: & mut [u8]) -> io::Result<usize>`
  - `fn read_buf_exact(self: & mut Self, buf: & mut [u8]) -> io::Result<()>`
  - `fn read_boxed_slice(self: & mut Self, len: usize) -> io::Result<Box<[u8]>>`
  - `fn read_boxed_slice_exact(self: & mut Self, len: usize) -> io::Result<Box<[u8]>>`
  - `fn scan_bytes_aligned<'a>(self: & mut Self, pattern: &[u8], align: usize, buf: &'a  mut [u8]) -> io::Result<&'a  mut [u8]>`
  - `fn ignore_bytes(self: & mut Self, count: u64) -> io::Result<()>`
  - `fn pos(self: &Self) -> u64`
- **SeekBuffered**
  - `fn ensure_seekback_buffer(self: & mut Self, len: usize)`
  - `fn unread_buffer_len(self: &Self) -> usize`
  - `fn read_buffer_len(self: &Self) -> usize`
  - `fn seek_buffered(self: & mut Self, pos: u64) -> u64`
  - `fn seek_buffered_rel(self: & mut Self, delta: isize) -> u64`



