**tokio > net > tcp > stream > TcpStream**

# Module: net::tcp::stream::TcpStream

## Contents

**Functions**

- [`connect`](#connect) - Opens a TCP connection to a remote host.
- [`from_std`](#from_std) - Creates new `TcpStream` from a `std::net::TcpStream`.
- [`into_split`](#into_split) - Splits a `TcpStream` into a read half and a write half, which can be used
- [`peek`](#peek) - Receives data on the socket from the remote address to which it is
- [`poll_peek`](#poll_peek) - Attempts to receive data on the socket, without removing that data from
- [`poll_read_ready`](#poll_read_ready) - Polls for read readiness.
- [`readable`](#readable) - Waits for the socket to become readable.
- [`ready`](#ready) - Waits for any of the requested ready states.
- [`set_linger`](#set_linger) - Sets the linger duration of this socket by setting the `SO_LINGER` option.
- [`set_nodelay`](#set_nodelay) - Sets the value of the `TCP_NODELAY` option on this socket.
- [`set_quickack`](#set_quickack) - Enable or disable `TCP_QUICKACK`.
- [`set_ttl`](#set_ttl) - Sets the value for the `IP_TTL` option on this socket.
- [`set_zero_linger`](#set_zero_linger) - Sets a linger duration of zero on this socket by setting the `SO_LINGER` option.
- [`split`](#split) - Splits a `TcpStream` into a read half and a write half, which can be used
- [`try_read`](#try_read) - Tries to read data from the stream into the provided buffer, returning how
- [`try_write`](#try_write) - Try to write a buffer to the stream, returning how many bytes were
- [`writable`](#writable) - Waits for the socket to become writable.

---

## tokio::net::tcp::stream::TcpStream::connect

*Function*

Opens a TCP connection to a remote host.

`addr` is an address of the remote host. Anything which implements the
[`ToSocketAddrs`] trait can be supplied as the address.  If `addr`
yields multiple addresses, connect will be attempted with each of the
addresses until a connection is successful. If none of the addresses
result in a successful connection, the error returned from the last
connection attempt (the last address) is returned.

For example, on a host without IPv6 connectivity, the
`AddrNotAvailable` error of a later IPv6 attempt can hide an earlier
IPv4 failure such as connection refused.
To configure the socket before connecting, you can use the [`TcpSocket`]
type.

[`ToSocketAddrs`]: trait@crate::net::ToSocketAddrs
[`TcpSocket`]: struct@crate::net::TcpSocket

# Examples

```no_run
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    // Write some data.
    stream.write_all(b"hello world!").await?;

    Ok(())
}
```

The [`write_all`] method is defined on the [`AsyncWriteExt`] trait.

[`write_all`]: fn@crate::io::AsyncWriteExt::write_all
[`AsyncWriteExt`]: trait@crate::io::AsyncWriteExt

```rust
fn connect<A>(addr: A) -> io::Result<TcpStream>
```



## tokio::net::tcp::stream::TcpStream::from_std

*Function*

Creates new `TcpStream` from a `std::net::TcpStream`.

This function is intended to be used to wrap a TCP stream from the
standard library in the Tokio equivalent.

# Notes

The caller is responsible for ensuring that the stream is in
non-blocking mode. Otherwise all I/O operations on the stream
will block the thread, which will cause unexpected behavior.
Non-blocking mode can be set using [`set_nonblocking`].

Passing a listener in blocking mode is always erroneous,
and the behavior in that case may change in the future.
For example, it could panic.

[`set_nonblocking`]: std::net::TcpStream::set_nonblocking

# Examples

```rust,no_run
use std::error::Error;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let std_stream = std::net::TcpStream::connect("127.0.0.1:34254")?;
    std_stream.set_nonblocking(true)?;
    let stream = TcpStream::from_std(std_stream)?;
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
fn from_std(stream: std::net::TcpStream) -> io::Result<TcpStream>
```



## tokio::net::tcp::stream::TcpStream::into_split

*Function*

Splits a `TcpStream` into a read half and a write half, which can be used
to read and write the stream concurrently.

Unlike [`split`], the owned halves can be moved to separate tasks, however
this comes at the cost of a heap allocation.

**Note:** Dropping the write half will shut down the write half of the TCP
stream. This is equivalent to calling [`shutdown()`] on the `TcpStream`.

[`split`]: TcpStream::split()
[`shutdown()`]: fn@crate::io::AsyncWriteExt::shutdown

```rust
fn into_split(self: Self) -> (OwnedReadHalf, OwnedWriteHalf)
```



## tokio::net::tcp::stream::TcpStream::peek

*Function*

Receives data on the socket from the remote address to which it is
connected, without removing that data from the queue. On success,
returns the number of bytes peeked.

Successive calls return the same data. This is accomplished by passing
`MSG_PEEK` as a flag to the underlying `recv` system call.

# Cancel safety

This method is cancel safe. If the method is used as a branch in
[`tokio::select!`](crate::select) and another branch
completes first, then it is guaranteed that no peek was performed, and
that `buf` has not been modified.

# Examples

```no_run
use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    let mut b1 = [0; 10];
    let mut b2 = [0; 10];

    // Peek at the data
    let n = stream.peek(&mut b1).await?;

    // Read the data
    assert_eq!(n, stream.read(&mut b2[..n]).await?);
    assert_eq!(&b1[..n], &b2[..n]);

    Ok(())
}
```

The [`read`] method is defined on the [`AsyncReadExt`] trait.

[`read`]: fn@crate::io::AsyncReadExt::read
[`AsyncReadExt`]: trait@crate::io::AsyncReadExt

```rust
fn peek(self: &Self, buf: & mut [u8]) -> io::Result<usize>
```



## tokio::net::tcp::stream::TcpStream::poll_peek

*Function*

Attempts to receive data on the socket, without removing that data from
the queue, registering the current task for wakeup if data is not yet
available.

Note that on multiple calls to `poll_peek`, `poll_read` or
`poll_read_ready`, only the `Waker` from the `Context` passed to the
most recent call is scheduled to receive a wakeup. (However,
`poll_write` retains a second, independent waker.)

# Return value

The function returns:

* `Poll::Pending` if data is not yet available.
* `Poll::Ready(Ok(n))` if data is available. `n` is the number of bytes peeked.
* `Poll::Ready(Err(e))` if an error is encountered.

# Errors

This function may encounter any standard I/O error except `WouldBlock`.

# Examples

```no_run
use tokio::io::{self, ReadBuf};
use tokio::net::TcpStream;

use std::future::poll_fn;

#[tokio::main]
async fn main() -> io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:8000").await?;
    let mut buf = [0; 10];
    let mut buf = ReadBuf::new(&mut buf);

    poll_fn(|cx| {
        stream.poll_peek(cx, &mut buf)
    }).await?;

    Ok(())
}
```

```rust
fn poll_peek(self: &Self, cx: & mut Context, buf: & mut ReadBuf) -> Poll<io::Result<usize>>
```



## tokio::net::tcp::stream::TcpStream::poll_read_ready

*Function*

Polls for read readiness.

If the tcp stream is not currently ready for reading, this method will
store a clone of the `Waker` from the provided `Context`. When the tcp
stream becomes ready for reading, `Waker::wake` will be called on the
waker.

Note that on multiple calls to `poll_read_ready`, `poll_read` or
`poll_peek`, only the `Waker` from the `Context` passed to the most
recent call is scheduled to receive a wakeup. (However,
`poll_write_ready` retains a second, independent waker.)

This function is intended for cases where creating and pinning a future
via [`readable`] is not feasible. Where possible, using [`readable`] is
preferred, as this supports polling from multiple tasks at once.

# Return value

The function returns:

* `Poll::Pending` if the tcp stream is not ready for reading.
* `Poll::Ready(Ok(()))` if the tcp stream is ready for reading.
* `Poll::Ready(Err(e))` if an error is encountered.

# Errors

This function may encounter any standard I/O error except `WouldBlock`.

[`readable`]: method@Self::readable

```rust
fn poll_read_ready(self: &Self, cx: & mut Context) -> Poll<io::Result<()>>
```



## tokio::net::tcp::stream::TcpStream::readable

*Function*

Waits for the socket to become readable.

This function is equivalent to `ready(Interest::READABLE)` and is usually
paired with `try_read()`.

# Cancel safety

This method is cancel safe. Once a readiness event occurs, the method
will continue to return immediately until the readiness event is
consumed by an attempt to read that fails with `WouldBlock` or
`Poll::Pending`.

# Examples

```no_run
use tokio::net::TcpStream;
use std::error::Error;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    let stream = TcpStream::connect("127.0.0.1:8080").await?;

    let mut msg = vec![0; 1024];

    loop {
        // Wait for the socket to be readable
        stream.readable().await?;

        // Try to read data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match stream.try_read(&mut msg) {
            Ok(n) => {
                msg.truncate(n);
                break;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    println!("GOT = {:?}", msg);
    Ok(())
}
```

```rust
fn readable(self: &Self) -> io::Result<()>
```



## tokio::net::tcp::stream::TcpStream::ready

*Function*

Waits for any of the requested ready states.

This function is usually paired with `try_read()` or `try_write()`. It
can be used to concurrently read / write to the same socket on a single
task without splitting the socket.

The function may complete without the socket being ready. This is a
false-positive and attempting an operation will return with
`io::ErrorKind::WouldBlock`. The function can also return with an empty
[`Ready`] set, so you should always check the returned value and possibly
wait again if the requested states are not set.

# Cancel safety

This method is cancel safe. Once a readiness event occurs, the method
will continue to return immediately until the readiness event is
consumed by an attempt to read or write that fails with `WouldBlock` or
`Poll::Pending`.

# Examples

Concurrently read and write to the stream on the same task without
splitting.

```no_run
use tokio::io::Interest;
use tokio::net::TcpStream;
use std::error::Error;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let stream = TcpStream::connect("127.0.0.1:8080").await?;

    loop {
        let ready = stream.ready(Interest::READABLE | Interest::WRITABLE).await?;

        if ready.is_readable() {
            let mut data = vec![0; 1024];
            // Try to read data, this may still fail with `WouldBlock`
            // if the readiness event is a false positive.
            match stream.try_read(&mut data) {
                Ok(n) => {
                    println!("read {} bytes", n);
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    return Err(e.into());
                }
            }

        }

        if ready.is_writable() {
            // Try to write data, this may still fail with `WouldBlock`
            // if the readiness event is a false positive.
            match stream.try_write(b"hello world") {
                Ok(n) => {
                    println!("write {} bytes", n);
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }
    }
}
```

```rust
fn ready(self: &Self, interest: Interest) -> io::Result<Ready>
```



## tokio::net::tcp::stream::TcpStream::set_linger

*Function*

Sets the linger duration of this socket by setting the `SO_LINGER` option.

This option controls the action taken when a stream has unsent messages and the stream is
closed. If `SO_LINGER` is set, the system shall block the process until it can transmit the
data or until the time expires.

If `SO_LINGER` is not specified, and the stream is closed, the system handles the call in a
way that allows the process to continue as quickly as possible.

This option is deprecated because setting `SO_LINGER` on a socket used with Tokio is
always incorrect as it leads to blocking the thread when the socket is closed. For more
details, please see:

> Volumes of communications have been devoted to the intricacies of `SO_LINGER` versus
> non-blocking (`O_NONBLOCK`) sockets. From what I can tell, the final word is: don't
> do it. Rely on the `shutdown()`-followed-by-`read()`-eof technique instead.
>
> From [The ultimate `SO_LINGER` page, or: why is my tcp not reliable](https://blog.netherlabs.nl/articles/2009/01/18/the-ultimate-so_linger-page-or-why-is-my-tcp-not-reliable)

Although this method is deprecated, it will not be removed from Tokio.

Note that the special case of setting `SO_LINGER` to zero does not lead to blocking.
Tokio provides [`set_zero_linger`](Self::set_zero_linger) for this purpose.

# Examples

```no_run
# #![allow(deprecated)]
use tokio::net::TcpStream;

# async fn dox() -> Result<(), Box<dyn std::error::Error>> {
let stream = TcpStream::connect("127.0.0.1:8080").await?;

stream.set_linger(None)?;
# Ok(())
# }
```

```rust
fn set_linger(self: &Self, dur: Option<Duration>) -> io::Result<()>
```



## tokio::net::tcp::stream::TcpStream::set_nodelay

*Function*

Sets the value of the `TCP_NODELAY` option on this socket.

If set, this option disables the Nagle algorithm. This means that
segments are always sent as soon as possible, even if there is only a
small amount of data. When not set, data is buffered until there is a
sufficient amount to send out, thereby avoiding the frequent sending of
small packets.

# Examples

```no_run
use tokio::net::TcpStream;

# async fn dox() -> Result<(), Box<dyn std::error::Error>> {
let stream = TcpStream::connect("127.0.0.1:8080").await?;

stream.set_nodelay(true)?;
# Ok(())
# }
```

```rust
fn set_nodelay(self: &Self, nodelay: bool) -> io::Result<()>
```



## tokio::net::tcp::stream::TcpStream::set_quickack

*Function*

Enable or disable `TCP_QUICKACK`.

This flag causes Linux to eagerly send `ACK`s rather than delaying them.
Linux may reset this flag after further operations on the socket.

See [`man 7 tcp`](https://man7.org/linux/man-pages/man7/tcp.7.html) and
[TCP delayed acknowledgment](https://en.wikipedia.org/wiki/TCP_delayed_acknowledgment)
for more information.

# Examples

```no_run
use tokio::net::TcpStream;

# async fn dox() -> Result<(), Box<dyn std::error::Error>> {
let stream = TcpStream::connect("127.0.0.1:8080").await?;

stream.set_quickack(true)?;
# Ok(())
# }
```

```rust
fn set_quickack(self: &Self, quickack: bool) -> io::Result<()>
```



## tokio::net::tcp::stream::TcpStream::set_ttl

*Function*

Sets the value for the `IP_TTL` option on this socket.

This value sets the time-to-live field that is used in every packet sent
from this socket.

# Examples

```no_run
use tokio::net::TcpStream;

# async fn dox() -> Result<(), Box<dyn std::error::Error>> {
let stream = TcpStream::connect("127.0.0.1:8080").await?;

stream.set_ttl(123)?;
# Ok(())
# }
```

```rust
fn set_ttl(self: &Self, ttl: u32) -> io::Result<()>
```



## tokio::net::tcp::stream::TcpStream::set_zero_linger

*Function*

Sets a linger duration of zero on this socket by setting the `SO_LINGER` option.

This causes the connection to be forcefully aborted ("abortive close") when the socket
is dropped or closed. Instead of the normal TCP shutdown handshake (`FIN`/`ACK`), a TCP
`RST` (reset) segment is sent to the peer, and the socket immediately discards any
unsent data residing in the socket send buffer. This prevents the socket from entering
the `TIME_WAIT` state after closing it.

This is a destructive action. Any data currently buffered by the OS but not yet
transmitted will be lost. The peer will likely receive a "Connection Reset" error
rather than a clean end-of-stream.

See the documentation for [`set_linger`](Self::set_linger) for additional details on
how `SO_LINGER` works.

# Examples

```no_run
use std::time::Duration;
use tokio::net::TcpStream;

# async fn dox() -> Result<(), Box<dyn std::error::Error>> {
let stream = TcpStream::connect("127.0.0.1:8080").await?;

stream.set_zero_linger()?;
assert_eq!(stream.linger()?, Some(Duration::ZERO));
# Ok(())
# }
```

```rust
fn set_zero_linger(self: &Self) -> io::Result<()>
```



## tokio::net::tcp::stream::TcpStream::split

*Function*

Splits a `TcpStream` into a read half and a write half, which can be used
to read and write the stream concurrently.

This method is more efficient than [`into_split`], but the halves cannot be
moved into independently spawned tasks.

[`into_split`]: TcpStream::into_split()

```rust
fn split<'a>(self: &'a  mut Self) -> (ReadHalf<'a>, WriteHalf<'a>)
```



## tokio::net::tcp::stream::TcpStream::try_read

*Function*

Tries to read data from the stream into the provided buffer, returning how
many bytes were read.

Receives any pending data from the socket but does not wait for new data
to arrive. On success, returns the number of bytes read. Because
`try_read()` is non-blocking, the buffer does not have to be stored by
the async task and can exist entirely on the stack.

Usually, [`readable()`] or [`ready()`] is used with this function.

[`readable()`]: TcpStream::readable()
[`ready()`]: TcpStream::ready()

# Return

If data is successfully read, `Ok(n)` is returned, where `n` is the
number of bytes read. If `n` is `0`, then it can indicate one of two scenarios:

1. The stream's read half is closed and will no longer yield data.
2. The specified buffer was 0 bytes in length.

If the stream is not ready to read data,
`Err(io::ErrorKind::WouldBlock)` is returned.

# Examples

```no_run
use tokio::net::TcpStream;
use std::error::Error;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    let stream = TcpStream::connect("127.0.0.1:8080").await?;

    loop {
        // Wait for the socket to be readable
        stream.readable().await?;

        // Creating the buffer **after** the `await` prevents it from
        // being stored in the async task.
        let mut buf = [0; 4096];

        // Try to read data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match stream.try_read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                println!("read {} bytes", n);
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    Ok(())
}
```

```rust
fn try_read(self: &Self, buf: & mut [u8]) -> io::Result<usize>
```



## tokio::net::tcp::stream::TcpStream::try_write

*Function*

Try to write a buffer to the stream, returning how many bytes were
written.

The function will attempt to write the entire contents of `buf`, but
only part of the buffer may be written.

This function is usually paired with `writable()`.

# Return

If data is successfully written, `Ok(n)` is returned, where `n` is the
number of bytes written. If the stream is not ready to write data,
`Err(io::ErrorKind::WouldBlock)` is returned.

# Examples

```no_run
use tokio::net::TcpStream;
use std::error::Error;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    let stream = TcpStream::connect("127.0.0.1:8080").await?;

    loop {
        // Wait for the socket to be writable
        stream.writable().await?;

        // Try to write data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match stream.try_write(b"hello world") {
            Ok(n) => {
                break;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    Ok(())
}
```

```rust
fn try_write(self: &Self, buf: &[u8]) -> io::Result<usize>
```



## tokio::net::tcp::stream::TcpStream::writable

*Function*

Waits for the socket to become writable.

This function is equivalent to `ready(Interest::WRITABLE)` and is usually
paired with `try_write()`.

# Cancel safety

This method is cancel safe. Once a readiness event occurs, the method
will continue to return immediately until the readiness event is
consumed by an attempt to write that fails with `WouldBlock` or
`Poll::Pending`.

# Examples

```no_run
use tokio::net::TcpStream;
use std::error::Error;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    let stream = TcpStream::connect("127.0.0.1:8080").await?;

    loop {
        // Wait for the socket to be writable
        stream.writable().await?;

        // Try to write data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match stream.try_write(b"hello world") {
            Ok(n) => {
                break;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    Ok(())
}
```

```rust
fn writable(self: &Self) -> io::Result<()>
```



