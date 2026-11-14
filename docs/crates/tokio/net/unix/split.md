**tokio > net > unix > split**

# Module: net::unix::split

## Contents

**Structs**

- [`ReadHalf`](#readhalf) - Borrowed read half of a [`UnixStream`], created by [`split`].
- [`WriteHalf`](#writehalf) - Borrowed write half of a [`UnixStream`], created by [`split`].

---

## tokio::net::unix::split::ReadHalf

*Struct*

Borrowed read half of a [`UnixStream`], created by [`split`].

Reading from a `ReadHalf` is usually done using the convenience methods found on the
[`AsyncReadExt`] trait.

[`UnixStream`]: UnixStream
[`split`]: UnixStream::split()
[`AsyncReadExt`]: trait@crate::io::AsyncReadExt

**Generic Parameters:**
- 'a

**Tuple Struct**: `()`

**Methods:**

- `fn ready(self: &Self, interest: Interest) -> io::Result<Ready>` - Wait for any of the requested ready states.
- `fn readable(self: &Self) -> io::Result<()>` - Waits for the socket to become readable.
- `fn try_read(self: &Self, buf: & mut [u8]) -> io::Result<usize>` - Tries to read data from the stream into the provided buffer, returning how
- `fn try_read_buf<B>(self: &Self, buf: & mut B) -> io::Result<usize>` - Tries to read data from the stream into the provided buffer, advancing the
- `fn try_read_vectored(self: &Self, bufs: & mut [io::IoSliceMut]) -> io::Result<usize>` - Tries to read data from the stream into the provided buffers, returning
- `fn peer_addr(self: &Self) -> io::Result<SocketAddr>` - Returns the socket address of the remote half of this connection.
- `fn local_addr(self: &Self) -> io::Result<SocketAddr>` - Returns the socket address of the local half of this connection.

**Trait Implementations:**

- **AsyncRead**
  - `fn poll_read(self: Pin<& mut Self>, cx: & mut Context, buf: & mut ReadBuf) -> Poll<io::Result<()>>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **AsRef**
  - `fn as_ref(self: &Self) -> &UnixStream`



## tokio::net::unix::split::WriteHalf

*Struct*

Borrowed write half of a [`UnixStream`], created by [`split`].

Note that in the [`AsyncWrite`] implementation of this type, [`poll_shutdown`] will
shut down the [`UnixStream`] stream in the write direction.

Writing to an `WriteHalf` is usually done using the convenience methods found
on the [`AsyncWriteExt`] trait.

[`UnixStream`]: UnixStream
[`split`]: UnixStream::split()
[`AsyncWrite`]: trait@crate::io::AsyncWrite
[`poll_shutdown`]: fn@crate::io::AsyncWrite::poll_shutdown
[`AsyncWriteExt`]: trait@crate::io::AsyncWriteExt

**Generic Parameters:**
- 'a

**Tuple Struct**: `()`

**Methods:**

- `fn ready(self: &Self, interest: Interest) -> io::Result<Ready>` - Waits for any of the requested ready states.
- `fn writable(self: &Self) -> io::Result<()>` - Waits for the socket to become writable.
- `fn try_write(self: &Self, buf: &[u8]) -> io::Result<usize>` - Tries to write a buffer to the stream, returning how many bytes were
- `fn try_write_vectored(self: &Self, buf: &[io::IoSlice]) -> io::Result<usize>` - Tries to write several buffers to the stream, returning how many bytes
- `fn peer_addr(self: &Self) -> io::Result<SocketAddr>` - Returns the socket address of the remote half of this connection.
- `fn local_addr(self: &Self) -> io::Result<SocketAddr>` - Returns the socket address of the local half of this connection.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **AsRef**
  - `fn as_ref(self: &Self) -> &UnixStream`
- **AsyncWrite**
  - `fn poll_write(self: Pin<& mut Self>, cx: & mut Context, buf: &[u8]) -> Poll<io::Result<usize>>`
  - `fn poll_write_vectored(self: Pin<& mut Self>, cx: & mut Context, bufs: &[io::IoSlice]) -> Poll<io::Result<usize>>`
  - `fn is_write_vectored(self: &Self) -> bool`
  - `fn poll_flush(self: Pin<& mut Self>, _: & mut Context) -> Poll<io::Result<()>>`
  - `fn poll_shutdown(self: Pin<& mut Self>, _: & mut Context) -> Poll<io::Result<()>>`



