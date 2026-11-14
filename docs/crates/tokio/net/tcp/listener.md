**tokio > net > tcp > listener**

# Module: net::tcp::listener

## Contents

**Structs**

- [`TcpListener`](#tcplistener) - A TCP socket server, listening for connections.

---

## tokio::net::tcp::listener::TcpListener

*Struct*

A TCP socket server, listening for connections.

You can accept a new connection by using the [`accept`](`TcpListener::accept`)
method.

A `TcpListener` can be turned into a `Stream` with [`TcpListenerStream`].

The socket will be closed when the value is dropped.

[`TcpListenerStream`]: https://docs.rs/tokio-stream/0.1/tokio_stream/wrappers/struct.TcpListenerStream.html

# Errors

Note that accepting a connection can lead to various errors and not all
of them are necessarily fatal ‒ for example having too many open file
descriptors or the other side closing the connection while it waits in
an accept queue. These would terminate the stream if not handled in any
way.

# Examples

Using `accept`:
```no_run
use tokio::net::TcpListener;

use std::io;

async fn process_socket<T>(socket: T) {
    # drop(socket);
    // do work with socket here
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        process_socket(socket).await;
    }
}
```

**Methods:**

- `fn bind<A>(addr: A) -> io::Result<TcpListener>` - Creates a new `TcpListener`, which will be bound to the specified address.
- `fn accept(self: &Self) -> io::Result<(TcpStream, SocketAddr)>` - Accepts a new incoming connection from this listener.
- `fn poll_accept(self: &Self, cx: & mut Context) -> Poll<io::Result<(TcpStream, SocketAddr)>>` - Polls to accept a new incoming connection to this listener.
- `fn from_std(listener: net::TcpListener) -> io::Result<TcpListener>` - Creates new `TcpListener` from a `std::net::TcpListener`.
- `fn into_std(self: Self) -> io::Result<std::net::TcpListener>` - Turns a [`tokio::net::TcpListener`] into a [`std::net::TcpListener`].
- `fn local_addr(self: &Self) -> io::Result<SocketAddr>` - Returns the local address that this listener is bound to.
- `fn ttl(self: &Self) -> io::Result<u32>` - Gets the value of the `IP_TTL` option for this socket.
- `fn set_ttl(self: &Self, ttl: u32) -> io::Result<()>` - Sets the value for the `IP_TTL` option on this socket.

**Trait Implementations:**

- **AsRawFd**
  - `fn as_raw_fd(self: &Self) -> RawFd`
- **TryFrom**
  - `fn try_from(stream: net::TcpListener) -> Result<Self, <Self as >::Error>` - Consumes stream, returning the tokio I/O object.
- **AsFd**
  - `fn as_fd(self: &Self) -> BorrowedFd`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



