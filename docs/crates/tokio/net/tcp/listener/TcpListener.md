**tokio > net > tcp > listener > TcpListener**

# Module: net::tcp::listener::TcpListener

## Contents

**Functions**

- [`accept`](#accept) - Accepts a new incoming connection from this listener.
- [`from_std`](#from_std) - Creates new `TcpListener` from a `std::net::TcpListener`.
- [`set_ttl`](#set_ttl) - Sets the value for the `IP_TTL` option on this socket.

---

## tokio::net::tcp::listener::TcpListener::accept

*Function*

Accepts a new incoming connection from this listener.

This function will yield once a new TCP connection is established. When
established, the corresponding [`TcpStream`] and the remote peer's
address will be returned.

# Cancel safety

This method is cancel safe. If the method is used as a branch in
[`tokio::select!`](crate::select) and another branch
completes first, then it is guaranteed that no new connections were
accepted by this method.

[`TcpStream`]: struct@crate::net::TcpStream

# Examples

```no_run
use tokio::net::TcpListener;

use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    match listener.accept().await {
        Ok((_socket, addr)) => println!("new client: {:?}", addr),
        Err(e) => println!("couldn't get client: {:?}", e),
    }

    Ok(())
}
```

```rust
fn accept(self: &Self) -> io::Result<(TcpStream, SocketAddr)>
```



## tokio::net::tcp::listener::TcpListener::from_std

*Function*

Creates new `TcpListener` from a `std::net::TcpListener`.

This function is intended to be used to wrap a TCP listener from the
standard library in the Tokio equivalent.

This API is typically paired with the `socket2` crate and the `Socket`
type to build up and customize a listener before it's shipped off to the
backing event loop. This allows configuration of options like
`SO_REUSEPORT`, binding to multiple addresses, etc.

# Notes

The caller is responsible for ensuring that the listener is in
non-blocking mode. Otherwise all I/O operations on the listener
will block the thread, which will cause unexpected behavior.
Non-blocking mode can be set using [`set_nonblocking`].

Passing a listener in blocking mode is always erroneous,
and the behavior in that case may change in the future.
For example, it could panic.

[`set_nonblocking`]: std::net::TcpListener::set_nonblocking

# Examples

```rust,no_run
use std::error::Error;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let std_listener = std::net::TcpListener::bind("127.0.0.1:0")?;
    std_listener.set_nonblocking(true)?;
    let listener = TcpListener::from_std(std_listener)?;
    Ok(())
}
```

# Panics

This function panics if it is not called from within a runtime with
IO enabled.

The runtime is usually set implicitly when this function is called
from a future driven by a tokio runtime, otherwise runtime can be set
explicitly with [`Runtime::enter`](crate::runtime::Runtime::enter) function.

```rust
fn from_std(listener: net::TcpListener) -> io::Result<TcpListener>
```



## tokio::net::tcp::listener::TcpListener::set_ttl

*Function*

Sets the value for the `IP_TTL` option on this socket.

This value sets the time-to-live field that is used in every packet sent
from this socket.

# Examples

```no_run
use tokio::net::TcpListener;

use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;

    listener.set_ttl(100).expect("could not set TTL");

    Ok(())
}
```

```rust
fn set_ttl(self: &Self, ttl: u32) -> io::Result<()>
```



