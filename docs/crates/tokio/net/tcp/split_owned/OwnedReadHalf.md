**tokio > net > tcp > split_owned > OwnedReadHalf**

# Module: net::tcp::split_owned::OwnedReadHalf

## Contents

**Functions**

- [`readable`](#readable) - Waits for the socket to become readable.
- [`ready`](#ready) - Waits for any of the requested ready states.
- [`try_read`](#try_read) - Tries to read data from the stream into the provided buffer, returning how

---

## tokio::net::tcp::split_owned::OwnedReadHalf::readable

*Function*

Waits for the socket to become readable.

This function is equivalent to `ready(Interest::READABLE)` and is usually
paired with `try_read()`.

This function is also equivalent to [`TcpStream::ready`].

# Cancel safety

This method is cancel safe. Once a readiness event occurs, the method
will continue to return immediately until the readiness event is
consumed by an attempt to read that fails with `WouldBlock` or
`Poll::Pending`.

```rust
fn readable(self: &Self) -> io::Result<()>
```



## tokio::net::tcp::split_owned::OwnedReadHalf::ready

*Function*

Waits for any of the requested ready states.

This function is usually paired with [`try_read()`]. It can be used instead
of [`readable()`] to check the returned ready set for [`Ready::READABLE`]
and [`Ready::READ_CLOSED`] events.

The function may complete without the socket being ready. This is a
false-positive and attempting an operation will return with
`io::ErrorKind::WouldBlock`. The function can also return with an empty
[`Ready`] set, so you should always check the returned value and possibly
wait again if the requested states are not set.

This function is equivalent to [`TcpStream::ready`].

[`try_read()`]: Self::try_read
[`readable()`]: Self::readable

# Cancel safety

This method is cancel safe. Once a readiness event occurs, the method
will continue to return immediately until the readiness event is
consumed by an attempt to read or write that fails with `WouldBlock` or
`Poll::Pending`.

```rust
fn ready(self: &Self, interest: Interest) -> io::Result<Ready>
```



## tokio::net::tcp::split_owned::OwnedReadHalf::try_read

*Function*

Tries to read data from the stream into the provided buffer, returning how
many bytes were read.

Receives any pending data from the socket but does not wait for new data
to arrive. On success, returns the number of bytes read. Because
`try_read()` is non-blocking, the buffer does not have to be stored by
the async task and can exist entirely on the stack.

Usually, [`readable()`] or [`ready()`] is used with this function.

[`readable()`]: Self::readable()
[`ready()`]: Self::ready()

# Return

If data is successfully read, `Ok(n)` is returned, where `n` is the
number of bytes read. If `n` is `0`, then it can indicate one of two scenarios:

1. The stream's read half is closed and will no longer yield data.
2. The specified buffer was 0 bytes in length.

If the stream is not ready to read data,
`Err(io::ErrorKind::WouldBlock)` is returned.

```rust
fn try_read(self: &Self, buf: & mut [u8]) -> io::Result<usize>
```



