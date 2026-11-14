**tokio > net > unix > listener > UnixListener**

# Module: net::unix::listener::UnixListener

## Contents

**Functions**

- [`accept`](#accept) - Accepts a new incoming connection to this listener.
- [`bind`](#bind) - Creates a new `UnixListener` bound to the specified path.
- [`from_std`](#from_std) - Creates new [`UnixListener`] from a [`std::os::unix::net::UnixListener`].

---

## tokio::net::unix::listener::UnixListener::accept

*Function*

Accepts a new incoming connection to this listener.

# Cancel safety

This method is cancel safe. If the method is used as a branch in
[`tokio::select!`](crate::select) and another branch
completes first, then it is guaranteed that no new connections were
accepted by this method.

```rust
fn accept(self: &Self) -> io::Result<(UnixStream, SocketAddr)>
```



## tokio::net::unix::listener::UnixListener::bind

*Function*

Creates a new `UnixListener` bound to the specified path.

# Panics

This function panics if it is not called from within a runtime with
IO enabled.

The runtime is usually set implicitly when this function is called
from a future driven by a tokio runtime, otherwise runtime can be set
explicitly with [`Runtime::enter`](crate::runtime::Runtime::enter) function.

```rust
fn bind<P>(path: P) -> io::Result<UnixListener>
```



## tokio::net::unix::listener::UnixListener::from_std

*Function*

Creates new [`UnixListener`] from a [`std::os::unix::net::UnixListener`].

This function is intended to be used to wrap a `UnixListener` from the
standard library in the Tokio equivalent.

# Notes

The caller is responsible for ensuring that the listener is in
non-blocking mode. Otherwise all I/O operations on the listener
will block the thread, which will cause unexpected behavior.
Non-blocking mode can be set using [`set_nonblocking`].

Passing a listener in blocking mode is always erroneous,
and the behavior in that case may change in the future.
For example, it could panic.

[`set_nonblocking`]: std::os::unix::net::UnixListener::set_nonblocking

# Examples

```no_run
use tokio::net::UnixListener;
use std::os::unix::net::UnixListener as StdUnixListener;
# use std::error::Error;

# async fn dox() -> Result<(), Box<dyn Error>> {
let std_listener = StdUnixListener::bind("/path/to/the/socket")?;
std_listener.set_nonblocking(true)?;
let listener = UnixListener::from_std(std_listener)?;
# Ok(())
# }
```

# Panics

This function panics if it is not called from within a runtime with
IO enabled.

The runtime is usually set implicitly when this function is called
from a future driven by a tokio runtime, otherwise runtime can be set
explicitly with [`Runtime::enter`](crate::runtime::Runtime::enter) function.

```rust
fn from_std(listener: net::UnixListener) -> io::Result<UnixListener>
```



