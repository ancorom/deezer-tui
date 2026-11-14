**tokio > net > unix > listener**

# Module: net::unix::listener

## Contents

**Structs**

- [`UnixListener`](#unixlistener) - A Unix socket which can accept connections from other Unix sockets.

---

## tokio::net::unix::listener::UnixListener

*Struct*

A Unix socket which can accept connections from other Unix sockets.

You can accept a new connection by using the [`accept`](`UnixListener::accept`) method.

A `UnixListener` can be turned into a `Stream` with [`UnixListenerStream`].

[`UnixListenerStream`]: https://docs.rs/tokio-stream/0.1/tokio_stream/wrappers/struct.UnixListenerStream.html

# Errors

Note that accepting a connection can lead to various errors and not all
of them are necessarily fatal ‒ for example having too many open file
descriptors or the other side closing the connection while it waits in
an accept queue. These would terminate the stream if not handled in any
way.

# Examples

```no_run
use tokio::net::UnixListener;

#[tokio::main]
async fn main() {
    let listener = UnixListener::bind("/path/to/the/socket").unwrap();
    loop {
        match listener.accept().await {
            Ok((stream, _addr)) => {
                println!("new client!");
            }
            Err(e) => { /* connection failed */ }
        }
    }
}
```

**Methods:**

- `fn bind<P>(path: P) -> io::Result<UnixListener>` - Creates a new `UnixListener` bound to the specified path.
- `fn bind_addr(socket_addr: &SocketAddr) -> io::Result<UnixListener>` - Creates a new `UnixListener` bound to the specified address.
- `fn from_std(listener: net::UnixListener) -> io::Result<UnixListener>` - Creates new [`UnixListener`] from a [`std::os::unix::net::UnixListener`].
- `fn into_std(self: Self) -> io::Result<std::os::unix::net::UnixListener>` - Turns a [`tokio::net::UnixListener`] into a [`std::os::unix::net::UnixListener`].
- `fn local_addr(self: &Self) -> io::Result<SocketAddr>` - Returns the local socket address of this listener.
- `fn take_error(self: &Self) -> io::Result<Option<io::Error>>` - Returns the value of the `SO_ERROR` option.
- `fn accept(self: &Self) -> io::Result<(UnixStream, SocketAddr)>` - Accepts a new incoming connection to this listener.
- `fn poll_accept(self: &Self, cx: & mut Context) -> Poll<io::Result<(UnixStream, SocketAddr)>>` - Polls to accept a new incoming connection to this listener.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **AsRawFd**
  - `fn as_raw_fd(self: &Self) -> RawFd`
- **TryFrom**
  - `fn try_from(stream: std::os::unix::net::UnixListener) -> io::Result<Self>` - Consumes stream, returning the tokio I/O object.
- **AsFd**
  - `fn as_fd(self: &Self) -> BorrowedFd`



