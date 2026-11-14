**tokio > net > tcp > split**

# Module: net::tcp::split

## Contents

**Structs**

- [`ReadHalf`](#readhalf) - Borrowed read half of a [`TcpStream`], created by [`split`].
- [`WriteHalf`](#writehalf) - Borrowed write half of a [`TcpStream`], created by [`split`].

---

## tokio::net::tcp::split::ReadHalf

*Struct*

Borrowed read half of a [`TcpStream`], created by [`split`].

Reading from a `ReadHalf` is usually done using the convenience methods found on the
[`AsyncReadExt`] trait.

[`TcpStream`]: TcpStream
[`split`]: TcpStream::split()
[`AsyncReadExt`]: trait@crate::io::AsyncReadExt

**Generic Parameters:**
- 'a

**Tuple Struct**: `()`

**Methods:**

- `fn poll_peek(self: & mut Self, cx: & mut Context, buf: & mut ReadBuf) -> Poll<io::Result<usize>>` - Attempts to receive data on the socket, without removing that data from
- `fn peek(self: & mut Self, buf: & mut [u8]) -> io::Result<usize>` - Receives data on the socket from the remote address to which it is
- `fn ready(self: &Self, interest: Interest) -> io::Result<Ready>` - Waits for any of the requested ready states.
- `fn readable(self: &Self) -> io::Result<()>` - Waits for the socket to become readable.
- `fn try_read(self: &Self, buf: & mut [u8]) -> io::Result<usize>` - Tries to read data from the stream into the provided buffer, returning how
- `fn try_read_vectored(self: &Self, bufs: & mut [io::IoSliceMut]) -> io::Result<usize>` - Tries to read data from the stream into the provided buffers, returning
- `fn try_read_buf<B>(self: &Self, buf: & mut B) -> io::Result<usize>` - Tries to read data from the stream into the provided buffer, advancing the
- `fn peer_addr(self: &Self) -> io::Result<SocketAddr>` - Returns the remote address that this stream is connected to.
- `fn local_addr(self: &Self) -> io::Result<SocketAddr>` - Returns the local address that this stream is bound to.

**Trait Implementations:**

- **AsRef**
  - `fn as_ref(self: &Self) -> &TcpStream`
- **AsyncRead**
  - `fn poll_read(self: Pin<& mut Self>, cx: & mut Context, buf: & mut ReadBuf) -> Poll<io::Result<()>>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## tokio::net::tcp::split::WriteHalf

*Struct*

Borrowed write half of a [`TcpStream`], created by [`split`].

Note that in the [`AsyncWrite`] implementation of this type, [`poll_shutdown`] will
shut down the TCP stream in the write direction.

Writing to an `WriteHalf` is usually done using the convenience methods found
on the [`AsyncWriteExt`] trait.

[`TcpStream`]: TcpStream
[`split`]: TcpStream::split()
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
- `fn try_write_vectored(self: &Self, bufs: &[io::IoSlice]) -> io::Result<usize>` - Tries to write several buffers to the stream, returning how many bytes
- `fn peer_addr(self: &Self) -> io::Result<SocketAddr>` - Returns the remote address that this stream is connected to.
- `fn local_addr(self: &Self) -> io::Result<SocketAddr>` - Returns the local address that this stream is bound to.

**Trait Implementations:**

- **AsRef**
  - `fn as_ref(self: &Self) -> &TcpStream`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **AsyncWrite**
  - `fn poll_write(self: Pin<& mut Self>, cx: & mut Context, buf: &[u8]) -> Poll<io::Result<usize>>`
  - `fn poll_write_vectored(self: Pin<& mut Self>, cx: & mut Context, bufs: &[io::IoSlice]) -> Poll<io::Result<usize>>`
  - `fn is_write_vectored(self: &Self) -> bool`
  - `fn poll_flush(self: Pin<& mut Self>, _: & mut Context) -> Poll<io::Result<()>>`
  - `fn poll_shutdown(self: Pin<& mut Self>, _: & mut Context) -> Poll<io::Result<()>>`



