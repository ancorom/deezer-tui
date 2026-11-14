**tokio > io > util > split**

# Module: io::util::split

## Contents

**Structs**

- [`Split`](#split) - Splitter for the [`split`](crate::io::AsyncBufReadExt::split) method.

---

## tokio::io::util::split::Split

*Struct*

Splitter for the [`split`](crate::io::AsyncBufReadExt::split) method.

A `Split` can be turned into a `Stream` with [`SplitStream`].

[`SplitStream`]: https://docs.rs/tokio-stream/0.1/tokio_stream/wrappers/struct.SplitStream.html

**Generic Parameters:**
- R

**Methods:**

- `fn next_segment(self: & mut Self) -> io::Result<Option<Vec<u8>>>` - Returns the next segment in the stream.
- `fn poll_next_segment(self: Pin<& mut Self>, cx: & mut Context) -> Poll<io::Result<Option<Vec<u8>>>>` - Polls for the next segment in the stream.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



