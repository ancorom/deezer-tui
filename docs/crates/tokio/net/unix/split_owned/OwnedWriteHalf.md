**tokio > net > unix > split_owned > OwnedWriteHalf**

# Module: net::unix::split_owned::OwnedWriteHalf

## Contents

**Functions**

- [`try_write`](#try_write) - Tries to write a buffer to the stream, returning how many bytes were
- [`writable`](#writable) - Waits for the socket to become writable.

---

## tokio::net::unix::split_owned::OwnedWriteHalf::try_write

*Function*

Tries to write a buffer to the stream, returning how many bytes were
written.

The function will attempt to write the entire contents of `buf`, but
only part of the buffer may be written.

This function is usually paired with `writable()`.

# Return

If data is successfully written, `Ok(n)` is returned, where `n` is the
number of bytes written. If the stream is not ready to write data,
`Err(io::ErrorKind::WouldBlock)` is returned.

```rust
fn try_write(self: &Self, buf: &[u8]) -> io::Result<usize>
```



## tokio::net::unix::split_owned::OwnedWriteHalf::writable

*Function*

Waits for the socket to become writable.

This function is equivalent to `ready(Interest::WRITABLE)` and is usually
paired with `try_write()`.

# Cancel safety

This method is cancel safe. Once a readiness event occurs, the method
will continue to return immediately until the readiness event is
consumed by an attempt to write that fails with `WouldBlock` or
`Poll::Pending`.

```rust
fn writable(self: &Self) -> io::Result<()>
```



