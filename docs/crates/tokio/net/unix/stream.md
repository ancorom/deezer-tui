**tokio > net > unix > stream**

# Module: net::unix::stream

## Contents

**Structs**

- [`UnixStream`](#unixstream) - A structure representing a connected Unix socket.

---

## tokio::net::unix::stream::UnixStream

*Struct*

A structure representing a connected Unix socket.

This socket can be connected directly with [`UnixStream::connect`] or accepted
from a listener with [`UnixListener::accept`]. Additionally, a pair of
anonymous Unix sockets can be created with `UnixStream::pair`.

To shut down the stream in the write direction, you can call the
[`shutdown()`] method. This will cause the other peer to receive a read of
length 0, indicating that no more data will be sent. This only closes
the stream in one direction.

[`shutdown()`]: fn@crate::io::AsyncWriteExt::shutdown
[`UnixListener::accept`]: crate::net::UnixListener::accept

**Methods:**

- `fn connect<P>(path: P) -> io::Result<UnixStream>` - Connects to the socket named by `path`.
- `fn connect_addr(socket_addr: &SocketAddr) -> io::Result<UnixStream>` - Connects to the socket named by `socket_addr`.
- `fn ready(self: &Self, interest: Interest) -> io::Result<Ready>` - Waits for any of the requested ready states.
- `fn readable(self: &Self) -> io::Result<()>` - Waits for the socket to become readable.
- `fn poll_read_ready(self: &Self, cx: & mut Context) -> Poll<io::Result<()>>` - Polls for read readiness.
- `fn try_read(self: &Self, buf: & mut [u8]) -> io::Result<usize>` - Try to read data from the stream into the provided buffer, returning how
- `fn try_read_vectored(self: &Self, bufs: & mut [io::IoSliceMut]) -> io::Result<usize>` - Tries to read data from the stream into the provided buffers, returning
- `fn try_read_buf<B>(self: &Self, buf: & mut B) -> io::Result<usize>` - Tries to read data from the stream into the provided buffer, advancing the
- `fn writable(self: &Self) -> io::Result<()>` - Waits for the socket to become writable.
- `fn poll_write_ready(self: &Self, cx: & mut Context) -> Poll<io::Result<()>>` - Polls for write readiness.
- `fn try_write(self: &Self, buf: &[u8]) -> io::Result<usize>` - Tries to write a buffer to the stream, returning how many bytes were
- `fn try_write_vectored(self: &Self, buf: &[io::IoSlice]) -> io::Result<usize>` - Tries to write several buffers to the stream, returning how many bytes
- `fn try_io<R, impl FnOnce() -> io::Result<R>>(self: &Self, interest: Interest, f: impl Trait) -> io::Result<R>` - Tries to read or write from the socket using a user-provided IO operation.
- `fn async_io<R, impl FnMut() -> io::Result<R>>(self: &Self, interest: Interest, f: impl Trait) -> io::Result<R>` - Reads or writes from the socket using a user-provided IO operation.
- `fn from_std(stream: net::UnixStream) -> io::Result<UnixStream>` - Creates new [`UnixStream`] from a [`std::os::unix::net::UnixStream`].
- `fn into_std(self: Self) -> io::Result<std::os::unix::net::UnixStream>` - Turns a [`tokio::net::UnixStream`] into a [`std::os::unix::net::UnixStream`].
- `fn pair() -> io::Result<(UnixStream, UnixStream)>` - Creates an unnamed pair of connected sockets.
- `fn local_addr(self: &Self) -> io::Result<SocketAddr>` - Returns the socket address of the local half of this connection.
- `fn peer_addr(self: &Self) -> io::Result<SocketAddr>` - Returns the socket address of the remote half of this connection.
- `fn peer_cred(self: &Self) -> io::Result<UCred>` - Returns effective credentials of the process which called `connect` or `pair`.
- `fn take_error(self: &Self) -> io::Result<Option<io::Error>>` - Returns the value of the `SO_ERROR` option.
- `fn split<'a>(self: &'a  mut Self) -> (ReadHalf<'a>, WriteHalf<'a>)` - Splits a `UnixStream` into a read half and a write half, which can be used
- `fn into_split(self: Self) -> (OwnedReadHalf, OwnedWriteHalf)` - Splits a `UnixStream` into a read half and a write half, which can be used

**Trait Implementations:**

- **AsRef**
  - `fn as_ref(self: &Self) -> &Self`
- **AsyncRead**
  - `fn poll_read(self: Pin<& mut Self>, cx: & mut Context, buf: & mut ReadBuf) -> Poll<io::Result<()>>`
- **AsyncWrite**
  - `fn poll_write(self: Pin<& mut Self>, cx: & mut Context, buf: &[u8]) -> Poll<io::Result<usize>>`
  - `fn poll_write_vectored(self: Pin<& mut Self>, cx: & mut Context, bufs: &[io::IoSlice]) -> Poll<io::Result<usize>>`
  - `fn is_write_vectored(self: &Self) -> bool`
  - `fn poll_flush(self: Pin<& mut Self>, _: & mut Context) -> Poll<io::Result<()>>`
  - `fn poll_shutdown(self: Pin<& mut Self>, _: & mut Context) -> Poll<io::Result<()>>`
- **TryFrom**
  - `fn try_from(stream: net::UnixStream) -> io::Result<Self>` - Consumes stream, returning the tokio I/O object.
- **AsRawFd**
  - `fn as_raw_fd(self: &Self) -> RawFd`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **AsFd**
  - `fn as_fd(self: &Self) -> BorrowedFd`



