**tokio > io > util > mem**

# Module: io::util::mem

## Contents

**Structs**

- [`DuplexStream`](#duplexstream) - A bidirectional pipe to read and write bytes in memory.
- [`SimplexStream`](#simplexstream) - A unidirectional pipe to read and write bytes in memory.

**Functions**

- [`duplex`](#duplex) - Create a new pair of `DuplexStream`s that act like a pair of connected sockets.
- [`simplex`](#simplex) - Creates unidirectional buffer that acts like in memory pipe.

---

## tokio::io::util::mem::DuplexStream

*Struct*

A bidirectional pipe to read and write bytes in memory.

A pair of `DuplexStream`s are created together, and they act as a "channel"
that can be used as in-memory IO types. Writing to one of the pairs will
allow that data to be read from the other, and vice versa.

# Closing a `DuplexStream`

If one end of the `DuplexStream` channel is dropped, any pending reads on
the other side will continue to read data until the buffer is drained, then
they will signal EOF by returning 0 bytes. Any writes to the other side,
including pending ones (that are waiting for free space in the buffer) will
return `Err(BrokenPipe)` immediately.

# Example

```
# async fn ex() -> std::io::Result<()> {
# use tokio::io::{AsyncReadExt, AsyncWriteExt};
let (mut client, mut server) = tokio::io::duplex(64);

client.write_all(b"ping").await?;

let mut buf = [0u8; 4];
server.read_exact(&mut buf).await?;
assert_eq!(&buf, b"ping");

server.write_all(b"pong").await?;

client.read_exact(&mut buf).await?;
assert_eq!(&buf, b"pong");
# Ok(())
# }
```

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **AsyncRead**
  - `fn poll_read(self: Pin<& mut Self>, cx: & mut task::Context, buf: & mut ReadBuf) -> Poll<std::io::Result<()>>`
- **Drop**
  - `fn drop(self: & mut Self)`
- **AsyncWrite**
  - `fn poll_write(self: Pin<& mut Self>, cx: & mut task::Context, buf: &[u8]) -> Poll<std::io::Result<usize>>`
  - `fn poll_write_vectored(self: Pin<& mut Self>, cx: & mut task::Context, bufs: &[std::io::IoSlice]) -> Poll<Result<usize, std::io::Error>>`
  - `fn is_write_vectored(self: &Self) -> bool`
  - `fn poll_flush(self: Pin<& mut Self>, cx: & mut task::Context) -> Poll<std::io::Result<()>>`
  - `fn poll_shutdown(self: Pin<& mut Self>, _: & mut task::Context) -> Poll<std::io::Result<()>>`



## tokio::io::util::mem::SimplexStream

*Struct*

A unidirectional pipe to read and write bytes in memory.

It can be constructed by [`simplex`] function which will create a pair of
reader and writer or by calling [`SimplexStream::new_unsplit`] that will
create a handle for both reading and writing.

# Example

```
# async fn ex() -> std::io::Result<()> {
# use tokio::io::{AsyncReadExt, AsyncWriteExt};
let (mut receiver, mut sender) = tokio::io::simplex(64);

sender.write_all(b"ping").await?;

let mut buf = [0u8; 4];
receiver.read_exact(&mut buf).await?;
assert_eq!(&buf, b"ping");
# Ok(())
# }
```

**Methods:**

- `fn new_unsplit(max_buf_size: usize) -> SimplexStream` - Creates unidirectional buffer that acts like in memory pipe. To create split

**Trait Implementations:**

- **AsyncWrite**
  - `fn poll_write(self: Pin<& mut Self>, cx: & mut task::Context, buf: &[u8]) -> Poll<std::io::Result<usize>>`
  - `fn poll_write_vectored(self: Pin<& mut Self>, cx: & mut task::Context, bufs: &[std::io::IoSlice]) -> Poll<Result<usize, std::io::Error>>`
  - `fn is_write_vectored(self: &Self) -> bool`
  - `fn poll_flush(self: Pin<& mut Self>, _: & mut task::Context) -> Poll<std::io::Result<()>>`
  - `fn poll_shutdown(self: Pin<& mut Self>, _: & mut task::Context) -> Poll<std::io::Result<()>>`
- **AsyncRead**
  - `fn poll_read(self: Pin<& mut Self>, cx: & mut task::Context, buf: & mut ReadBuf) -> Poll<std::io::Result<()>>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## tokio::io::util::mem::duplex

*Function*

Create a new pair of `DuplexStream`s that act like a pair of connected sockets.

The `max_buf_size` argument is the maximum amount of bytes that can be
written to a side before the write returns `Poll::Pending`.

# Panics

This function panics if `max_buf_size` is 0.

```rust
fn duplex(max_buf_size: usize) -> (DuplexStream, DuplexStream)
```



## tokio::io::util::mem::simplex

*Function*

Creates unidirectional buffer that acts like in memory pipe.

The `max_buf_size` argument is the maximum amount of bytes that can be
written to a buffer before it returns `Poll::Pending`.

# Unify reader and writer

The reader and writer half can be unified into a single structure
of `SimplexStream` that supports both reading and writing or
the `SimplexStream` can be already created as unified structure
using [`SimplexStream::new_unsplit()`].

```
# async fn ex() -> std::io::Result<()> {
# use tokio::io::{AsyncReadExt, AsyncWriteExt};
let (reader, writer) = tokio::io::simplex(64);
let mut simplex_stream = reader.unsplit(writer);
simplex_stream.write_all(b"hello").await?;

let mut buf = [0u8; 5];
simplex_stream.read_exact(&mut buf).await?;
assert_eq!(&buf, b"hello");
# Ok(())
# }
```

# Panics

This function panics if `max_buf_size` is 0.

```rust
fn simplex(max_buf_size: usize) -> (crate::io::ReadHalf<SimplexStream>, crate::io::WriteHalf<SimplexStream>)
```



