**tokio > io > join**

# Module: io::join

## Contents

**Structs**

- [`Join`](#join) - Joins two values implementing `AsyncRead` and `AsyncWrite` into a

**Functions**

- [`join`](#join) - Join two values implementing `AsyncRead` and `AsyncWrite` into a

---

## tokio::io::join::Join

*Struct*

Joins two values implementing `AsyncRead` and `AsyncWrite` into a
single handle.

**Generic Parameters:**
- R
- W

**Methods:**

- `fn into_inner(self: Self) -> (R, W)` - Splits this `Join` back into its `AsyncRead` and `AsyncWrite`
- `fn reader(self: &Self) -> &R` - Returns a reference to the inner reader.
- `fn writer(self: &Self) -> &W` - Returns a reference to the inner writer.
- `fn reader_mut(self: & mut Self) -> & mut R` - Returns a mutable reference to the inner reader.
- `fn writer_mut(self: & mut Self) -> & mut W` - Returns a mutable reference to the inner writer.
- `fn reader_pin_mut(self: Pin<& mut Self>) -> Pin<& mut R>` - Returns a pinned mutable reference to the inner reader.
- `fn writer_pin_mut(self: Pin<& mut Self>) -> Pin<& mut W>` - Returns a pinned mutable reference to the inner writer.

**Trait Implementations:**

- **AsyncRead**
  - `fn poll_read(self: Pin<& mut Self>, cx: & mut Context, buf: & mut ReadBuf) -> Poll<Result<(), io::Error>>`
- **AsyncWrite**
  - `fn poll_write(self: Pin<& mut Self>, cx: & mut Context, buf: &[u8]) -> Poll<Result<usize, io::Error>>`
  - `fn poll_flush(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<(), io::Error>>`
  - `fn poll_shutdown(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<(), io::Error>>`
  - `fn poll_write_vectored(self: Pin<& mut Self>, cx: & mut Context, bufs: &[io::IoSlice]) -> Poll<Result<usize, io::Error>>`
  - `fn is_write_vectored(self: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **AsyncBufRead**
  - `fn poll_fill_buf(self: Pin<& mut Self>, cx: & mut Context) -> Poll<io::Result<&[u8]>>`
  - `fn consume(self: Pin<& mut Self>, amt: usize)`



## tokio::io::join::join

*Function*

Join two values implementing `AsyncRead` and `AsyncWrite` into a
single handle.

```rust
fn join<R, W>(reader: R, writer: W) -> Join<R, W>
```



