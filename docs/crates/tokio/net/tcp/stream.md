**tokio > net > tcp > stream**

# Module: net::tcp::stream

## Contents

**Structs**

- [`TcpStream`](#tcpstream) - A TCP stream between a local and a remote socket.

---

## tokio::net::tcp::stream::TcpStream

*Struct*

A TCP stream between a local and a remote socket.

A TCP stream can either be created by connecting to an endpoint, via the
[`connect`] method, or by [accepting] a connection from a [listener]. A
TCP stream can also be created via the [`TcpSocket`] type.

Reading and writing to a `TcpStream` is usually done using the
convenience methods found on the [`AsyncReadExt`] and [`AsyncWriteExt`]
traits.

[`connect`]: method@TcpStream::connect
[accepting]: method@crate::net::TcpListener::accept
[listener]: struct@crate::net::TcpListener
[`TcpSocket`]: struct@crate::net::TcpSocket
[`AsyncReadExt`]: trait@crate::io::AsyncReadExt
[`AsyncWriteExt`]: trait@crate::io::AsyncWriteExt

# Examples

```no_run
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    // Write some data.
    stream.write_all(b"hello world!").await?;

    Ok(())
}
```

The [`write_all`] method is defined on the [`AsyncWriteExt`] trait.

[`write_all`]: fn@crate::io::AsyncWriteExt::write_all
[`AsyncWriteExt`]: trait@crate::io::AsyncWriteExt

To shut down the stream in the write direction, you can call the
[`shutdown()`] method. This will cause the other peer to receive a read of
length 0, indicating that no more data will be sent. This only closes
the stream in one direction.

[`shutdown()`]: fn@crate::io::AsyncWriteExt::shutdown

**Methods:**

- `fn connect<A>(addr: A) -> io::Result<TcpStream>` - Opens a TCP connection to a remote host.
- `fn from_std(stream: std::net::TcpStream) -> io::Result<TcpStream>` - Creates new `TcpStream` from a `std::net::TcpStream`.
- `fn into_std(self: Self) -> io::Result<std::net::TcpStream>` - Turns a [`tokio::net::TcpStream`] into a [`std::net::TcpStream`].
- `fn local_addr(self: &Self) -> io::Result<SocketAddr>` - Returns the local address that this stream is bound to.
- `fn take_error(self: &Self) -> io::Result<Option<io::Error>>` - Returns the value of the `SO_ERROR` option.
- `fn peer_addr(self: &Self) -> io::Result<SocketAddr>` - Returns the remote address that this stream is connected to.
- `fn poll_peek(self: &Self, cx: & mut Context, buf: & mut ReadBuf) -> Poll<io::Result<usize>>` - Attempts to receive data on the socket, without removing that data from
- `fn ready(self: &Self, interest: Interest) -> io::Result<Ready>` - Waits for any of the requested ready states.
- `fn readable(self: &Self) -> io::Result<()>` - Waits for the socket to become readable.
- `fn poll_read_ready(self: &Self, cx: & mut Context) -> Poll<io::Result<()>>` - Polls for read readiness.
- `fn try_read(self: &Self, buf: & mut [u8]) -> io::Result<usize>` - Tries to read data from the stream into the provided buffer, returning how
- `fn try_read_vectored(self: &Self, bufs: & mut [io::IoSliceMut]) -> io::Result<usize>` - Tries to read data from the stream into the provided buffers, returning
- `fn try_read_buf<B>(self: &Self, buf: & mut B) -> io::Result<usize>` - Tries to read data from the stream into the provided buffer, advancing the
- `fn writable(self: &Self) -> io::Result<()>` - Waits for the socket to become writable.
- `fn poll_write_ready(self: &Self, cx: & mut Context) -> Poll<io::Result<()>>` - Polls for write readiness.
- `fn try_write(self: &Self, buf: &[u8]) -> io::Result<usize>` - Try to write a buffer to the stream, returning how many bytes were
- `fn try_write_vectored(self: &Self, bufs: &[io::IoSlice]) -> io::Result<usize>` - Tries to write several buffers to the stream, returning how many bytes
- `fn try_io<R, impl FnOnce() -> io::Result<R>>(self: &Self, interest: Interest, f: impl Trait) -> io::Result<R>` - Tries to read or write from the socket using a user-provided IO operation.
- `fn async_io<R, impl FnMut() -> io::Result<R>>(self: &Self, interest: Interest, f: impl Trait) -> io::Result<R>` - Reads or writes from the socket using a user-provided IO operation.
- `fn peek(self: &Self, buf: & mut [u8]) -> io::Result<usize>` - Receives data on the socket from the remote address to which it is
- `fn nodelay(self: &Self) -> io::Result<bool>` - Gets the value of the `TCP_NODELAY` option on this socket.
- `fn set_nodelay(self: &Self, nodelay: bool) -> io::Result<()>` - Sets the value of the `TCP_NODELAY` option on this socket.
- `fn quickack(self: &Self) -> io::Result<bool>` - Gets the value of the `TCP_QUICKACK` option on this socket.
- `fn set_quickack(self: &Self, quickack: bool) -> io::Result<()>` - Enable or disable `TCP_QUICKACK`.
- `fn linger(self: &Self) -> io::Result<Option<Duration>>` - Reads the linger duration for this socket by getting the `SO_LINGER`
- `fn set_linger(self: &Self, dur: Option<Duration>) -> io::Result<()>` - Sets the linger duration of this socket by setting the `SO_LINGER` option.
- `fn set_zero_linger(self: &Self) -> io::Result<()>` - Sets a linger duration of zero on this socket by setting the `SO_LINGER` option.
- `fn ttl(self: &Self) -> io::Result<u32>` - Gets the value of the `IP_TTL` option for this socket.
- `fn set_ttl(self: &Self, ttl: u32) -> io::Result<()>` - Sets the value for the `IP_TTL` option on this socket.
- `fn split<'a>(self: &'a  mut Self) -> (ReadHalf<'a>, WriteHalf<'a>)` - Splits a `TcpStream` into a read half and a write half, which can be used
- `fn into_split(self: Self) -> (OwnedReadHalf, OwnedWriteHalf)` - Splits a `TcpStream` into a read half and a write half, which can be used

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **AsFd**
  - `fn as_fd(self: &Self) -> BorrowedFd`
- **AsyncWrite**
  - `fn poll_write(self: Pin<& mut Self>, cx: & mut Context, buf: &[u8]) -> Poll<io::Result<usize>>`
  - `fn poll_write_vectored(self: Pin<& mut Self>, cx: & mut Context, bufs: &[io::IoSlice]) -> Poll<io::Result<usize>>`
  - `fn is_write_vectored(self: &Self) -> bool`
  - `fn poll_flush(self: Pin<& mut Self>, _: & mut Context) -> Poll<io::Result<()>>`
  - `fn poll_shutdown(self: Pin<& mut Self>, _: & mut Context) -> Poll<io::Result<()>>`
- **AsRef**
  - `fn as_ref(self: &Self) -> &Self`
- **TryFrom**
  - `fn try_from(stream: std::net::TcpStream) -> Result<Self, <Self as >::Error>` - Consumes stream, returning the tokio I/O object.
- **AsRawFd**
  - `fn as_raw_fd(self: &Self) -> RawFd`
- **AsyncRead**
  - `fn poll_read(self: Pin<& mut Self>, cx: & mut Context, buf: & mut ReadBuf) -> Poll<io::Result<()>>`



