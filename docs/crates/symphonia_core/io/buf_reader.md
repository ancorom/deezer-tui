**symphonia_core > io > buf_reader**

# Module: io::buf_reader

## Contents

**Structs**

- [`BufReader`](#bufreader) - A `BufReader` reads bytes from a byte buffer.

---

## symphonia_core::io::buf_reader::BufReader

*Struct*

A `BufReader` reads bytes from a byte buffer.

**Generic Parameters:**
- 'a

**Methods:**

- `fn new(buf: &'a [u8]) -> Self` - Instantiate a new `BufReader` with a given byte buffer.
- `fn scan_bytes_ref(self: & mut Self, pattern: &[u8], scan_len: usize) -> io::Result<&'a [u8]>` - Scans up-to `scan_len` bytes from the stream until a byte pattern is matched. A reference to
- `fn scan_bytes_aligned_ref(self: & mut Self, pattern: &[u8], align: usize, scan_len: usize) -> io::Result<&'a [u8]>` - Scans up-to `scan_len` bytes from the stream until a byte pattern is matched on the
- `fn read_buf_bytes_ref(self: & mut Self, len: usize) -> io::Result<&'a [u8]>` - Returns a reference to the next `len` bytes in the buffer and advances the stream.
- `fn read_buf_bytes_available_ref(self: & mut Self) -> &'a [u8]` - Returns a reference to the remaining bytes in the buffer and advances the stream to the end.

**Trait Implementations:**

- **FiniteStream**
  - `fn byte_len(self: &Self) -> u64`
  - `fn bytes_read(self: &Self) -> u64`
  - `fn bytes_available(self: &Self) -> u64`
- **ReadBytes**
  - `fn read_byte(self: & mut Self) -> io::Result<u8>`
  - `fn read_double_bytes(self: & mut Self) -> io::Result<[u8; 2]>`
  - `fn read_triple_bytes(self: & mut Self) -> io::Result<[u8; 3]>`
  - `fn read_quad_bytes(self: & mut Self) -> io::Result<[u8; 4]>`
  - `fn read_buf(self: & mut Self, buf: & mut [u8]) -> io::Result<usize>`
  - `fn read_buf_exact(self: & mut Self, buf: & mut [u8]) -> io::Result<()>`
  - `fn read_boxed_slice(self: & mut Self, len: usize) -> io::Result<Box<[u8]>>`
  - `fn read_boxed_slice_exact(self: & mut Self, len: usize) -> io::Result<Box<[u8]>>`
  - `fn scan_bytes_aligned<'b>(self: & mut Self, pattern: &[u8], align: usize, buf: &'b  mut [u8]) -> io::Result<&'b  mut [u8]>`
  - `fn ignore_bytes(self: & mut Self, count: u64) -> io::Result<()>`
  - `fn pos(self: &Self) -> u64`



