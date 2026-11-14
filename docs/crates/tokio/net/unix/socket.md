**tokio > net > unix > socket**

# Module: net::unix::socket

## Contents

**Structs**

- [`UnixSocket`](#unixsocket) - A Unix socket that has not yet been converted to a [`UnixStream`], [`UnixDatagram`], or

---

## tokio::net::unix::socket::UnixSocket

*Struct*

A Unix socket that has not yet been converted to a [`UnixStream`], [`UnixDatagram`], or
[`UnixListener`].

`UnixSocket` wraps an operating system socket and enables the caller to
configure the socket before establishing a connection or accepting
inbound connections. The caller is able to set socket option and explicitly
bind the socket with a socket address.

The underlying socket is closed when the `UnixSocket` value is dropped.

`UnixSocket` should only be used directly if the default configuration used
by [`UnixStream::connect`], [`UnixDatagram::bind`], and [`UnixListener::bind`]
does not meet the required use case.

Calling `UnixStream::connect(path)` effectively performs the same function as:

```no_run
use tokio::net::UnixSocket;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("bind_path");
    let socket = UnixSocket::new_stream()?;

    let stream = socket.connect(path).await?;

    Ok(())
}
```

Calling `UnixDatagram::bind(path)` effectively performs the same function as:

```no_run
use tokio::net::UnixSocket;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("bind_path");
    let socket = UnixSocket::new_datagram()?;
    socket.bind(path)?;

    let datagram = socket.datagram()?;

    Ok(())
}
```

Calling `UnixListener::bind(path)` effectively performs the same function as:

```no_run
use tokio::net::UnixSocket;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("bind_path");
    let socket = UnixSocket::new_stream()?;
    socket.bind(path)?;

    let listener = socket.listen(1024)?;

    Ok(())
}
```

Setting socket options not explicitly provided by `UnixSocket` may be done by
accessing the [`RawFd`]/[`RawSocket`] using [`AsRawFd`]/[`AsRawSocket`] and
setting the option with a crate like [`socket2`].

[`RawFd`]: std::os::fd::RawFd
[`RawSocket`]: https://doc.rust-lang.org/std/os/windows/io/type.RawSocket.html
[`AsRawFd`]: std::os::fd::AsRawFd
[`AsRawSocket`]: https://doc.rust-lang.org/std/os/windows/io/trait.AsRawSocket.html
[`socket2`]: https://docs.rs/socket2/

**Methods:**

- `fn new_datagram() -> io::Result<UnixSocket>` - Creates a new Unix datagram socket.
- `fn new_stream() -> io::Result<UnixSocket>` - Creates a new Unix stream socket.
- `fn bind<impl AsRef<Path>>(self: &Self, path: impl Trait) -> io::Result<()>` - Binds the socket to the given address.
- `fn listen(self: Self, backlog: u32) -> io::Result<UnixListener>` - Converts the socket into a `UnixListener`.
- `fn connect<impl AsRef<Path>>(self: Self, path: impl Trait) -> io::Result<UnixStream>` - Establishes a Unix connection with a peer at the specified socket address.
- `fn datagram(self: Self) -> io::Result<UnixDatagram>` - Converts the socket into a [`UnixDatagram`].

**Trait Implementations:**

- **IntoRawFd**
  - `fn into_raw_fd(self: Self) -> RawFd`
- **AsFd**
  - `fn as_fd(self: &Self) -> BorrowedFd`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FromRawFd**
  - `fn from_raw_fd(fd: RawFd) -> UnixSocket`
- **AsRawFd**
  - `fn as_raw_fd(self: &Self) -> RawFd`



