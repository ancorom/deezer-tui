**tokio > net > unix > stream > UnixStream**

# Module: net::unix::stream::UnixStream

## Contents

**Functions**

- [`connect`](#connect) - Connects to the socket named by `path`.
- [`from_std`](#from_std) - Creates new [`UnixStream`] from a [`std::os::unix::net::UnixStream`].
- [`into_split`](#into_split) - Splits a `UnixStream` into a read half and a write half, which can be used
- [`readable`](#readable) - Waits for the socket to become readable.
- [`ready`](#ready) - Waits for any of the requested ready states.
- [`split`](#split) - Splits a `UnixStream` into a read half and a write half, which can be used
- [`try_read`](#try_read) - Try to read data from the stream into the provided buffer, returning how
- [`try_write`](#try_write) - Tries to write a buffer to the stream, returning how many bytes were
- [`writable`](#writable) - Waits for the socket to become writable.

---

## tokio::net::unix::stream::UnixStream::connect

*Function*

Connects to the socket named by `path`.

This function will create a new Unix socket and connect to the path
specified, associating the returned stream with the default event loop's
handle.

# Panics

This function panics if it is not called from within a runtime with
IO enabled.

The runtime is usually set implicitly when this function is called
from a future driven by a tokio runtime, otherwise runtime can be set
explicitly with [`Runtime::enter`](crate::runtime::Runtime::enter) function.

```rust
fn connect<P>(path: P) -> io::Result<UnixStream>
```



## tokio::net::unix::stream::UnixStream::from_std

*Function*

Creates new [`UnixStream`] from a [`std::os::unix::net::UnixStream`].

This function is intended to be used to wrap a `UnixStream` from the
standard library in the Tokio equivalent.

# Notes

The caller is responsible for ensuring that the stream is in
non-blocking mode. Otherwise all I/O operations on the stream
will block the thread, which will cause unexpected behavior.
Non-blocking mode can be set using [`set_nonblocking`].

Passing a listener in blocking mode is always erroneous,
and the behavior in that case may change in the future.
For example, it could panic.

[`set_nonblocking`]: std::os::unix::net::UnixStream::set_nonblocking

# Examples

```no_run
use tokio::net::UnixStream;
use std::os::unix::net::UnixStream as StdUnixStream;
# use std::error::Error;

# async fn dox() -> Result<(), Box<dyn Error>> {
let std_stream = StdUnixStream::connect("/path/to/the/socket")?;
std_stream.set_nonblocking(true)?;
let stream = UnixStream::from_std(std_stream)?;
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
fn from_std(stream: net::UnixStream) -> io::Result<UnixStream>
```



## tokio::net::unix::stream::UnixStream::into_split

*Function*

Splits a `UnixStream` into a read half and a write half, which can be used
to read and write the stream concurrently.

Unlike [`split`], the owned halves can be moved to separate tasks, however
this comes at the cost of a heap allocation.

**Note:** Dropping the write half will only shut down the write half of the
stream. This is equivalent to calling [`shutdown()`] on the `UnixStream`.

[`split`]: Self::split()
[`shutdown()`]: fn@crate::io::AsyncWriteExt::shutdown

```rust
fn into_split(self: Self) -> (OwnedReadHalf, OwnedWriteHalf)
```



## tokio::net::unix::stream::UnixStream::readable

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
use tokio::net::UnixStream;
use std::error::Error;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    let dir = tempfile::tempdir().unwrap();
    let bind_path = dir.path().join("bind_path");
    let stream = UnixStream::connect(bind_path).await?;

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



## tokio::net::unix::stream::UnixStream::ready

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
use tokio::net::UnixStream;
use std::error::Error;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let dir = tempfile::tempdir().unwrap();
    let bind_path = dir.path().join("bind_path");
    let stream = UnixStream::connect(bind_path).await?;

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
                    continue;
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



## tokio::net::unix::stream::UnixStream::split

*Function*

Splits a `UnixStream` into a read half and a write half, which can be used
to read and write the stream concurrently.

This method is more efficient than [`into_split`], but the halves cannot be
moved into independently spawned tasks.

[`into_split`]: Self::into_split()

```rust
fn split<'a>(self: &'a  mut Self) -> (ReadHalf<'a>, WriteHalf<'a>)
```



## tokio::net::unix::stream::UnixStream::try_read

*Function*

Try to read data from the stream into the provided buffer, returning how
many bytes were read.

Receives any pending data from the socket but does not wait for new data
to arrive. On success, returns the number of bytes read. Because
`try_read()` is non-blocking, the buffer does not have to be stored by
the async task and can exist entirely on the stack.

Usually, [`readable()`] or [`ready()`] is used with this function.

[`readable()`]: UnixStream::readable()
[`ready()`]: UnixStream::ready()

# Return

If data is successfully read, `Ok(n)` is returned, where `n` is the
number of bytes read. If `n` is `0`, then it can indicate one of two scenarios:

1. The stream's read half is closed and will no longer yield data.
2. The specified buffer was 0 bytes in length.

If the stream is not ready to read data,
`Err(io::ErrorKind::WouldBlock)` is returned.

# Examples

```no_run
use tokio::net::UnixStream;
use std::error::Error;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    let dir = tempfile::tempdir().unwrap();
    let bind_path = dir.path().join("bind_path");
    let stream = UnixStream::connect(bind_path).await?;

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



## tokio::net::unix::stream::UnixStream::try_write

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

# Examples

```no_run
use tokio::net::UnixStream;
use std::error::Error;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    let dir = tempfile::tempdir().unwrap();
    let bind_path = dir.path().join("bind_path");
    let stream = UnixStream::connect(bind_path).await?;

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



## tokio::net::unix::stream::UnixStream::writable

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
use tokio::net::UnixStream;
use std::error::Error;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    let dir = tempfile::tempdir().unwrap();
    let bind_path = dir.path().join("bind_path");
    let stream = UnixStream::connect(bind_path).await?;

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



