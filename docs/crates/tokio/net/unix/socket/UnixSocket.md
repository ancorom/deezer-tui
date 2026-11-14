**tokio > net > unix > socket > UnixSocket**

# Module: net::unix::socket::UnixSocket

## Contents

**Functions**

- [`new_datagram`](#new_datagram) - Creates a new Unix datagram socket.
- [`new_stream`](#new_stream) - Creates a new Unix stream socket.

---

## tokio::net::unix::socket::UnixSocket::new_datagram

*Function*

Creates a new Unix datagram socket.

Calls `socket(2)` with `AF_UNIX` and `SOCK_DGRAM`.

# Returns

On success, the newly created [`UnixSocket`] is returned. If an error is
encountered, it is returned instead.

```rust
fn new_datagram() -> io::Result<UnixSocket>
```



## tokio::net::unix::socket::UnixSocket::new_stream

*Function*

Creates a new Unix stream socket.

Calls `socket(2)` with `AF_UNIX` and `SOCK_STREAM`.

# Returns

On success, the newly created [`UnixSocket`] is returned. If an error is
encountered, it is returned instead.

```rust
fn new_stream() -> io::Result<UnixSocket>
```



