**tokio > net > tcp > socket**

# Module: net::tcp::socket

## Contents

**Structs**

- [`TcpSocket`](#tcpsocket) - A TCP socket that has not yet been converted to a `TcpStream` or

---

## tokio::net::tcp::socket::TcpSocket

*Struct*

A TCP socket that has not yet been converted to a `TcpStream` or
`TcpListener`.

`TcpSocket` wraps an operating system socket and enables the caller to
configure the socket before establishing a TCP connection or accepting
inbound connections. The caller is able to set socket option and explicitly
bind the socket with a socket address.

The underlying socket is closed when the `TcpSocket` value is dropped.

`TcpSocket` should only be used directly if the default configuration used
by `TcpStream::connect` and `TcpListener::bind` does not meet the required
use case.

Calling `TcpStream::connect("127.0.0.1:8080")` is equivalent to:

```no_run
use tokio::net::TcpSocket;

use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = "127.0.0.1:8080".parse().unwrap();

    let socket = TcpSocket::new_v4()?;
    let stream = socket.connect(addr).await?;
# drop(stream);

    Ok(())
}
```

Calling `TcpListener::bind("127.0.0.1:8080")` is equivalent to:

```no_run
use tokio::net::TcpSocket;

use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = "127.0.0.1:8080".parse().unwrap();

    let socket = TcpSocket::new_v4()?;
    // On platforms with Berkeley-derived sockets, this allows to quickly
    // rebind a socket, without needing to wait for the OS to clean up the
    // previous one.
    //
    // On Windows, this allows rebinding sockets which are actively in use,
    // which allows "socket hijacking", so we explicitly don't set it here.
    // https://docs.microsoft.com/en-us/windows/win32/winsock/using-so-reuseaddr-and-so-exclusiveaddruse
    socket.set_reuseaddr(true)?;
    socket.bind(addr)?;

    // Note: the actual backlog used by `TcpListener::bind` is platform-dependent,
    // as Tokio relies on Mio's default backlog value configuration. The `1024` here is only
    // illustrative and does not reflect the real value used.
    let listener = socket.listen(1024)?;
# drop(listener);

    Ok(())
}
```

Setting socket options not explicitly provided by `TcpSocket` may be done by
accessing the `RawFd`/`RawSocket` using [`AsRawFd`]/[`AsRawSocket`] and
setting the option with a crate like [`socket2`].

[`RawFd`]: https://doc.rust-lang.org/std/os/fd/type.RawFd.html
[`RawSocket`]: https://doc.rust-lang.org/std/os/windows/io/type.RawSocket.html
[`AsRawFd`]: https://doc.rust-lang.org/std/os/fd/trait.AsRawFd.html
[`AsRawSocket`]: https://doc.rust-lang.org/std/os/windows/io/trait.AsRawSocket.html
[`socket2`]: https://docs.rs/socket2/

**Methods:**

- `fn new_v4() -> io::Result<TcpSocket>` - Creates a new socket configured for IPv4.
- `fn new_v6() -> io::Result<TcpSocket>` - Creates a new socket configured for IPv6.
- `fn set_keepalive(self: &Self, keepalive: bool) -> io::Result<()>` - Sets value for the `SO_KEEPALIVE` option on this socket.
- `fn keepalive(self: &Self) -> io::Result<bool>` - Gets the value of the `SO_KEEPALIVE` option on this socket.
- `fn set_reuseaddr(self: &Self, reuseaddr: bool) -> io::Result<()>` - Allows the socket to bind to an in-use address.
- `fn reuseaddr(self: &Self) -> io::Result<bool>` - Retrieves the value set for `SO_REUSEADDR` on this socket.
- `fn set_reuseport(self: &Self, reuseport: bool) -> io::Result<()>` - Allows the socket to bind to an in-use port. Only available for unix systems
- `fn reuseport(self: &Self) -> io::Result<bool>` - Allows the socket to bind to an in-use port. Only available for unix systems
- `fn set_send_buffer_size(self: &Self, size: u32) -> io::Result<()>` - Sets the size of the TCP send buffer on this socket.
- `fn send_buffer_size(self: &Self) -> io::Result<u32>` - Returns the size of the TCP send buffer for this socket.
- `fn set_recv_buffer_size(self: &Self, size: u32) -> io::Result<()>` - Sets the size of the TCP receive buffer on this socket.
- `fn recv_buffer_size(self: &Self) -> io::Result<u32>` - Returns the size of the TCP receive buffer for this socket.
- `fn set_linger(self: &Self, dur: Option<Duration>) -> io::Result<()>` - Sets the linger duration of this socket by setting the `SO_LINGER` option.
- `fn set_zero_linger(self: &Self) -> io::Result<()>` - Sets a linger duration of zero on this socket by setting the `SO_LINGER` option.
- `fn linger(self: &Self) -> io::Result<Option<Duration>>` - Reads the linger duration for this socket by getting the `SO_LINGER`
- `fn set_nodelay(self: &Self, nodelay: bool) -> io::Result<()>` - Sets the value of the `TCP_NODELAY` option on this socket.
- `fn nodelay(self: &Self) -> io::Result<bool>` - Gets the value of the `TCP_NODELAY` option on this socket.
- `fn tclass_v6(self: &Self) -> io::Result<u32>` - Gets the value of the `IPV6_TCLASS` option for this socket.
- `fn set_tclass_v6(self: &Self, tclass: u32) -> io::Result<()>` - Sets the value for the `IPV6_TCLASS` option on this socket.
- `fn tos_v4(self: &Self) -> io::Result<u32>` - Gets the value of the `IP_TOS` option for this socket.
- `fn set_tos_v4(self: &Self, tos: u32) -> io::Result<()>` - Sets the value for the `IP_TOS` option on this socket.
- `fn device(self: &Self) -> io::Result<Option<Vec<u8>>>` - Gets the value for the `SO_BINDTODEVICE` option on this socket
- `fn bind_device(self: &Self, interface: Option<&[u8]>) -> io::Result<()>` - Sets the value for the `SO_BINDTODEVICE` option on this socket
- `fn local_addr(self: &Self) -> io::Result<SocketAddr>` - Gets the local address of this socket.
- `fn take_error(self: &Self) -> io::Result<Option<io::Error>>` - Returns the value of the `SO_ERROR` option.
- `fn bind(self: &Self, addr: SocketAddr) -> io::Result<()>` - Binds the socket to the given address.
- `fn connect(self: Self, addr: SocketAddr) -> io::Result<TcpStream>` - Establishes a TCP connection with a peer at the specified socket address.
- `fn listen(self: Self, backlog: u32) -> io::Result<TcpListener>` - Converts the socket into a `TcpListener`.
- `fn from_std_stream(std_stream: std::net::TcpStream) -> TcpSocket` - Converts a [`std::net::TcpStream`] into a `TcpSocket`. The provided

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **FromRawFd**
  - `fn from_raw_fd(fd: RawFd) -> TcpSocket` - Converts a `RawFd` to a `TcpSocket`.
- **AsRawFd**
  - `fn as_raw_fd(self: &Self) -> RawFd`
- **IntoRawFd**
  - `fn into_raw_fd(self: Self) -> RawFd`
- **AsFd**
  - `fn as_fd(self: &Self) -> BorrowedFd`



