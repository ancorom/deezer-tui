**tokio > io > async_fd > AsyncFdReadyGuard**

# Module: io::async_fd::AsyncFdReadyGuard

## Contents

**Functions**

- [`clear_ready`](#clear_ready) - Indicates to tokio that the file descriptor is no longer ready. All
- [`clear_ready_matching`](#clear_ready_matching) - Indicates to tokio that the file descriptor no longer has a specific readiness.
- [`try_io`](#try_io) - Performs the provided IO operation.

---

## tokio::io::async_fd::AsyncFdReadyGuard::clear_ready

*Function*

Indicates to tokio that the file descriptor is no longer ready. All
internal readiness flags will be cleared, and tokio will wait for the
next edge-triggered readiness notification from the OS.

This function is commonly used with guards returned by [`AsyncFd::readable`] and
[`AsyncFd::writable`].

It is critical that this function not be called unless your code
_actually observes_ that the file descriptor is _not_ ready. Do not call
it simply because, for example, a read succeeded; it should be called
when a read is observed to block.

This method only clears readiness events that happened before the creation of this guard.
In other words, if the IO resource becomes ready between the creation of the guard and
this call to `clear_ready`, then the readiness is not actually cleared.

```rust
fn clear_ready(self: & mut Self)
```



## tokio::io::async_fd::AsyncFdReadyGuard::clear_ready_matching

*Function*

Indicates to tokio that the file descriptor no longer has a specific readiness.
The internal readiness flag will be cleared, and tokio will wait for the
next edge-triggered readiness notification from the OS.

This function is useful in combination with the [`AsyncFd::ready`] method when a
combined interest like `Interest::READABLE | Interest::WRITABLE` is used.

It is critical that this function not be called unless your code
_actually observes_ that the file descriptor is _not_ ready for the provided `Ready`.
Do not call it simply because, for example, a read succeeded; it should be called
when a read is observed to block. Only clear the specific readiness that is observed to
block. For example when a read blocks when using a combined interest,
only clear `Ready::READABLE`.

This method only clears readiness events that happened before the creation of this guard.
In other words, if the IO resource becomes ready between the creation of the guard and
this call to `clear_ready_matching`, then the readiness is not actually cleared.

# Examples

Concurrently read and write to a [`std::net::TcpStream`] on the same task without
splitting.

```no_run
use std::error::Error;
use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use tokio::io::unix::AsyncFd;
use tokio::io::{Interest, Ready};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let stream = TcpStream::connect("127.0.0.1:8080")?;
    stream.set_nonblocking(true)?;
    // SAFETY: `TcpStream` owns a valid file descriptor and we do not
    // replace or close it while owned by `AsyncFd`.
    let stream = unsafe { AsyncFd::register(stream)? };

    loop {
        let mut guard = stream
            .ready(Interest::READABLE | Interest::WRITABLE)
            .await?;

        if guard.ready().is_readable() {
            let mut data = vec![0; 1024];
            // Try to read data, this may still fail with `WouldBlock`
            // if the readiness event is a false positive.
            match stream.get_ref().read(&mut data) {
                Ok(n) => {
                    println!("read {} bytes", n);
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // a read has blocked, but a write might still succeed.
                    // clear only the read readiness.
                    guard.clear_ready_matching(Ready::READABLE);
                    continue;
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }

        if guard.ready().is_writable() {
            // Try to write data, this may still fail with `WouldBlock`
            // if the readiness event is a false positive.
            match stream.get_ref().write(b"hello world") {
                Ok(n) => {
                    println!("write {} bytes", n);
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // a write has blocked, but a read might still succeed.
                    // clear only the write readiness.
                    guard.clear_ready_matching(Ready::WRITABLE);
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
fn clear_ready_matching(self: & mut Self, ready: Ready)
```



## tokio::io::async_fd::AsyncFdReadyGuard::try_io

*Function*

Performs the provided IO operation.

If `f` returns a [`WouldBlock`] error, the readiness state associated
with this file descriptor is cleared, and the method returns
`Err(TryIoError::WouldBlock)`. You will typically need to poll the
`AsyncFd` again when this happens.

This method helps ensure that the readiness state of the underlying file
descriptor remains in sync with the tokio-side readiness state, by
clearing the tokio-side state only when a [`WouldBlock`] condition
occurs. It is the responsibility of the caller to ensure that `f`
returns [`WouldBlock`] only if the file descriptor that originated this
`AsyncFdReadyGuard` no longer expresses the readiness state that was queried to
create this `AsyncFdReadyGuard`.

# Examples

This example sends some bytes to the inner [`std::net::UdpSocket`]. Waiting
for write-readiness and retrying when the send operation does block are explicit.
This example can be written more succinctly using [`AsyncFd::async_io`].

```no_run
use tokio::io::unix::AsyncFd;

use std::io;
use std::net::UdpSocket;

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:8080")?;
    socket.set_nonblocking(true)?;
    // SAFETY: `UdpSocket` owns a valid file descriptor and we do not
    // replace or close it while owned by `AsyncFd`.
    let async_fd = unsafe { AsyncFd::register(socket)? };

    let written = loop {
        let mut guard = async_fd.writable().await?;
        match guard.try_io(|inner| inner.get_ref().send(&[1, 2])) {
            Ok(result) => {
                break result?;
            }
            Err(_would_block) => {
                // try_io already cleared the file descriptor's readiness state
                continue;
            }
        }
    };

    println!("wrote {written} bytes");

    Ok(())
}
```

[`WouldBlock`]: std::io::ErrorKind::WouldBlock

```rust
fn try_io<R, impl FnOnce(&'a AsyncFd<Inner>) -> io::Result<R>>(self: & mut Self, f: impl Trait) -> Result<io::Result<R>, TryIoError>
```



