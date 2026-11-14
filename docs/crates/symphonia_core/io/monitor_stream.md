**symphonia_core > io > monitor_stream**

# Module: io::monitor_stream

## Contents

**Structs**

- [`MonitorStream`](#monitorstream) - A `MonitorStream` is a passive stream that observes all operations performed on the inner

**Traits**

- [`Monitor`](#monitor) - A `Monitor` provides a common interface to examine the operations observed be

---

## symphonia_core::io::monitor_stream::Monitor

*Trait*

A `Monitor` provides a common interface to examine the operations observed be
a [`MonitorStream`].

**Methods:**

- `process_byte`
- `process_double_bytes`
- `process_triple_bytes`
- `process_quad_bytes`
- `process_buf_bytes`



## symphonia_core::io::monitor_stream::MonitorStream

*Struct*

A `MonitorStream` is a passive stream that observes all operations performed on the inner
stream and forwards an immutable reference of the result to a [`Monitor`].

**Generic Parameters:**
- B
- M

**Methods:**

- `fn new(inner: B, monitor: M) -> MonitorStream<B, M>`
- `fn inner(self: &Self) -> &B`
- `fn inner_mut(self: & mut Self) -> & mut B`
- `fn into_inner(self: Self) -> B`
- `fn monitor(self: &Self) -> &M`
- `fn monitor_mut(self: & mut Self) -> & mut M`

**Trait Implementations:**

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



