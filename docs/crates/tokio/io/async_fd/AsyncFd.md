**tokio > io > async_fd > AsyncFd**

# Module: io::async_fd::AsyncFd

## Contents

**Functions**

- [`async_io`](#async_io) - Reads or writes from the file descriptor using a user-provided IO operation.
- [`into_inner`](#into_inner) - Deregisters this file descriptor and returns ownership of the backing
- [`poll_read_ready`](#poll_read_ready) - Polls for read readiness.
- [`poll_read_ready_mut`](#poll_read_ready_mut) - Polls for read readiness.
- [`poll_write_ready`](#poll_write_ready) - Polls for write readiness.
- [`poll_write_ready_mut`](#poll_write_ready_mut) - Polls for write readiness.
- [`readable`](#readable) - Waits for the file descriptor to become readable, returning a
- [`readable_mut`](#readable_mut) - Waits for the file descriptor to become readable, returning a
- [`ready`](#ready) - Waits for any of the requested ready states, returning a
- [`ready_mut`](#ready_mut) - Waits for any of the requested ready states, returning a
- [`register`](#register) - Creates an `AsyncFd` backed by (and taking ownership of) an object
- [`register_with_interest`](#register_with_interest) - Creates an `AsyncFd` backed by (and taking ownership of) an object
- [`try_io`](#try_io) - Tries to read or write from the file descriptor using a user-provided IO operation.
- [`writable`](#writable) - Waits for the file descriptor to become writable, returning a
- [`writable_mut`](#writable_mut) - Waits for the file descriptor to become writable, returning a

---

## tokio::io::async_fd::AsyncFd::async_io

*Function*

Reads or writes from the file descriptor using a user-provided IO operation.

The `async_io` method is a convenience utility that waits for the file
descriptor to become ready, and then executes the provided IO operation.
Since file descriptors may be marked ready spuriously, the closure will
be called repeatedly until it returns something other than a
[`WouldBlock`] error. This is done using the following loop:

```no_run
# use std::io::{self, Result};
# struct Dox<T> { inner: T }
# impl<T> Dox<T> {
#     async fn writable(&self) -> Result<&Self> {
#         Ok(self)
#     }
#     fn try_io<R>(&self, _: impl FnMut(&T) -> Result<R>) -> Result<Result<R>> {
#         panic!()
#     }
async fn async_io<R>(&self, mut f: impl FnMut(&T) -> io::Result<R>) -> io::Result<R> {
    loop {
        // or `readable` if called with the read interest.
        let guard = self.writable().await?;

        match guard.try_io(&mut f) {
            Ok(result) => return result,
            Err(_would_block) => continue,
        }
    }
}
# }
```

The closure should only return a [`WouldBlock`] error if it has performed
an IO operation on the file descriptor that failed due to the file descriptor not being
ready. Returning a [`WouldBlock`] error in any other situation will
incorrectly clear the readiness flag, which can cause the file descriptor to
behave incorrectly.

The closure should not perform the IO operation using any of the methods
defined on the Tokio [`AsyncFd`] type, as this will mess with the
readiness flag and can cause the file descriptor to behave incorrectly.

This method is not intended to be used with combined interests.
The closure should perform only one type of IO operation, so it should not
require more than one ready state. This method may panic or sleep forever
if it is called with a combined interest.

# Examples

This example sends some bytes on the inner [`std::net::UdpSocket`]. The `async_io`
method waits for readiness, and retries if the send operation does block. This example
is equivalent to the one given for [`try_io`].

```no_run
use tokio::io::{Interest, unix::AsyncFd};

use std::io;
use std::net::UdpSocket;

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:8080")?;
    socket.set_nonblocking(true)?;
    // SAFETY: `UdpSocket` owns a valid file descriptor and we do not
    // replace or close it while owned by `AsyncFd`.
    let async_fd = unsafe { AsyncFd::register(socket)? };

    let written = async_fd
        .async_io(Interest::WRITABLE, |inner| inner.send(&[1, 2]))
        .await?;

    println!("wrote {written} bytes");

    Ok(())
}
```

[`try_io`]: AsyncFdReadyGuard::try_io
[`WouldBlock`]: std::io::ErrorKind::WouldBlock

```rust
fn async_io<R, impl FnMut(&T) -> io::Result<R>>(self: &Self, interest: Interest, f: impl Trait) -> io::Result<R>
```



## tokio::io::async_fd::AsyncFd::into_inner

*Function*

Deregisters this file descriptor and returns ownership of the backing
object.

```rust
fn into_inner(self: Self) -> T
```



## tokio::io::async_fd::AsyncFd::poll_read_ready

*Function*

Polls for read readiness.

If the file descriptor is not currently ready for reading, this method
will store a clone of the [`Waker`] from the provided [`Context`]. When the
file descriptor becomes ready for reading, [`Waker::wake`] will be called.

Note that on multiple calls to [`poll_read_ready`] or
[`poll_read_ready_mut`], only the `Waker` from the `Context` passed to the
most recent call is scheduled to receive a wakeup. (However,
[`poll_write_ready`] retains a second, independent waker).

This method is intended for cases where creating and pinning a future
via [`readable`] is not feasible. Where possible, using [`readable`] is
preferred, as this supports polling from multiple tasks at once.

This method takes `&self`, so it is possible to call this method
concurrently with other methods on this struct. This method only
provides shared access to the inner IO resource when handling the
[`AsyncFdReadyGuard`].

[`poll_read_ready`]: method@Self::poll_read_ready
[`poll_read_ready_mut`]: method@Self::poll_read_ready_mut
[`poll_write_ready`]: method@Self::poll_write_ready
[`readable`]: method@Self::readable
[`Context`]: struct@std::task::Context
[`Waker`]: struct@std::task::Waker
[`Waker::wake`]: method@std::task::Waker::wake

```rust
fn poll_read_ready<'a>(self: &'a Self, cx: & mut Context) -> Poll<io::Result<AsyncFdReadyGuard<'a, T>>>
```



## tokio::io::async_fd::AsyncFd::poll_read_ready_mut

*Function*

Polls for read readiness.

If the file descriptor is not currently ready for reading, this method
will store a clone of the [`Waker`] from the provided [`Context`]. When the
file descriptor becomes ready for reading, [`Waker::wake`] will be called.

Note that on multiple calls to [`poll_read_ready`] or
[`poll_read_ready_mut`], only the `Waker` from the `Context` passed to the
most recent call is scheduled to receive a wakeup. (However,
[`poll_write_ready`] retains a second, independent waker).

This method is intended for cases where creating and pinning a future
via [`readable`] is not feasible. Where possible, using [`readable`] is
preferred, as this supports polling from multiple tasks at once.

This method takes `&mut self`, so it is possible to access the inner IO
resource mutably when handling the [`AsyncFdReadyMutGuard`].

[`poll_read_ready`]: method@Self::poll_read_ready
[`poll_read_ready_mut`]: method@Self::poll_read_ready_mut
[`poll_write_ready`]: method@Self::poll_write_ready
[`readable`]: method@Self::readable
[`Context`]: struct@std::task::Context
[`Waker`]: struct@std::task::Waker
[`Waker::wake`]: method@std::task::Waker::wake

```rust
fn poll_read_ready_mut<'a>(self: &'a  mut Self, cx: & mut Context) -> Poll<io::Result<AsyncFdReadyMutGuard<'a, T>>>
```



## tokio::io::async_fd::AsyncFd::poll_write_ready

*Function*

Polls for write readiness.

If the file descriptor is not currently ready for writing, this method
will store a clone of the [`Waker`] from the provided [`Context`]. When the
file descriptor becomes ready for writing, [`Waker::wake`] will be called.

Note that on multiple calls to [`poll_write_ready`] or
[`poll_write_ready_mut`], only the `Waker` from the `Context` passed to the
most recent call is scheduled to receive a wakeup. (However,
[`poll_read_ready`] retains a second, independent waker).

This method is intended for cases where creating and pinning a future
via [`writable`] is not feasible. Where possible, using [`writable`] is
preferred, as this supports polling from multiple tasks at once.

This method takes `&self`, so it is possible to call this method
concurrently with other methods on this struct. This method only
provides shared access to the inner IO resource when handling the
[`AsyncFdReadyGuard`].

[`poll_read_ready`]: method@Self::poll_read_ready
[`poll_write_ready`]: method@Self::poll_write_ready
[`poll_write_ready_mut`]: method@Self::poll_write_ready_mut
[`writable`]: method@Self::writable
[`Context`]: struct@std::task::Context
[`Waker`]: struct@std::task::Waker
[`Waker::wake`]: method@std::task::Waker::wake

```rust
fn poll_write_ready<'a>(self: &'a Self, cx: & mut Context) -> Poll<io::Result<AsyncFdReadyGuard<'a, T>>>
```



## tokio::io::async_fd::AsyncFd::poll_write_ready_mut

*Function*

Polls for write readiness.

If the file descriptor is not currently ready for writing, this method
will store a clone of the [`Waker`] from the provided [`Context`]. When the
file descriptor becomes ready for writing, [`Waker::wake`] will be called.

Note that on multiple calls to [`poll_write_ready`] or
[`poll_write_ready_mut`], only the `Waker` from the `Context` passed to the
most recent call is scheduled to receive a wakeup. (However,
[`poll_read_ready`] retains a second, independent waker).

This method is intended for cases where creating and pinning a future
via [`writable`] is not feasible. Where possible, using [`writable`] is
preferred, as this supports polling from multiple tasks at once.

This method takes `&mut self`, so it is possible to access the inner IO
resource mutably when handling the [`AsyncFdReadyMutGuard`].

[`poll_read_ready`]: method@Self::poll_read_ready
[`poll_write_ready`]: method@Self::poll_write_ready
[`poll_write_ready_mut`]: method@Self::poll_write_ready_mut
[`writable`]: method@Self::writable
[`Context`]: struct@std::task::Context
[`Waker`]: struct@std::task::Waker
[`Waker::wake`]: method@std::task::Waker::wake

```rust
fn poll_write_ready_mut<'a>(self: &'a  mut Self, cx: & mut Context) -> Poll<io::Result<AsyncFdReadyMutGuard<'a, T>>>
```



## tokio::io::async_fd::AsyncFd::readable

*Function*

Waits for the file descriptor to become readable, returning a
[`AsyncFdReadyGuard`] that must be dropped to resume read-readiness
polling.

This method takes `&self`, so it is possible to call this method
concurrently with other methods on this struct. This method only
provides shared access to the inner IO resource when handling the
[`AsyncFdReadyGuard`].

# Cancel safety

This method is cancel safe. Once a readiness event occurs, the method
will continue to return immediately until the readiness event is
consumed by an attempt to read or write that fails with `WouldBlock` or
`Poll::Pending`.

```rust
fn readable<'a>(self: &'a Self) -> io::Result<AsyncFdReadyGuard<'a, T>>
```



## tokio::io::async_fd::AsyncFd::readable_mut

*Function*

Waits for the file descriptor to become readable, returning a
[`AsyncFdReadyMutGuard`] that must be dropped to resume read-readiness
polling.

This method takes `&mut self`, so it is possible to access the inner IO
resource mutably when handling the [`AsyncFdReadyMutGuard`].

# Cancel safety

This method is cancel safe. Once a readiness event occurs, the method
will continue to return immediately until the readiness event is
consumed by an attempt to read or write that fails with `WouldBlock` or
`Poll::Pending`.

```rust
fn readable_mut<'a>(self: &'a  mut Self) -> io::Result<AsyncFdReadyMutGuard<'a, T>>
```



## tokio::io::async_fd::AsyncFd::ready

*Function*

Waits for any of the requested ready states, returning a
[`AsyncFdReadyGuard`] that must be dropped to resume
polling for the requested ready states.

The function may complete without the file descriptor being ready. This is a
false-positive and attempting an operation will return with
`io::ErrorKind::WouldBlock`. The function can also return with an empty
[`Ready`] set, so you should always check the returned value and possibly
wait again if the requested states are not set.

When an IO operation does return `io::ErrorKind::WouldBlock`, the readiness must be cleared.
When a combined interest is used, it is important to clear only the readiness
that is actually observed to block. For instance when the combined
interest `Interest::READABLE | Interest::WRITABLE` is used, and a read blocks, only
read readiness should be cleared using the [`AsyncFdReadyGuard::clear_ready_matching`] method:
`guard.clear_ready_matching(Ready::READABLE)`.
Also clearing the write readiness in this case would be incorrect. The [`AsyncFdReadyGuard::clear_ready`]
method clears all readiness flags.

This method takes `&self`, so it is possible to call this method
concurrently with other methods on this struct. This method only
provides shared access to the inner IO resource when handling the
[`AsyncFdReadyGuard`].

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
fn ready(self: &Self, interest: Interest) -> io::Result<AsyncFdReadyGuard<T>>
```



## tokio::io::async_fd::AsyncFd::ready_mut

*Function*

Waits for any of the requested ready states, returning a
[`AsyncFdReadyMutGuard`] that must be dropped to resume
polling for the requested ready states.

The function may complete without the file descriptor being ready. This is a
false-positive and attempting an operation will return with
`io::ErrorKind::WouldBlock`. The function can also return with an empty
[`Ready`] set, so you should always check the returned value and possibly
wait again if the requested states are not set.

When an IO operation does return `io::ErrorKind::WouldBlock`, the readiness must be cleared.
When a combined interest is used, it is important to clear only the readiness
that is actually observed to block. For instance when the combined
interest `Interest::READABLE | Interest::WRITABLE` is used, and a read blocks, only
read readiness should be cleared using the [`AsyncFdReadyMutGuard::clear_ready_matching`] method:
`guard.clear_ready_matching(Ready::READABLE)`.
Also clearing the write readiness in this case would be incorrect.
The [`AsyncFdReadyMutGuard::clear_ready`] method clears all readiness flags.

This method takes `&mut self`, so it is possible to access the inner IO
resource mutably when handling the [`AsyncFdReadyMutGuard`].

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
    let mut stream = unsafe { AsyncFd::register(stream)? };

    loop {
        let mut guard = stream
            .ready_mut(Interest::READABLE | Interest::WRITABLE)
            .await?;

        if guard.ready().is_readable() {
            let mut data = vec![0; 1024];
            // Try to read data, this may still fail with `WouldBlock`
            // if the readiness event is a false positive.
            match guard.get_inner_mut().read(&mut data) {
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
            match guard.get_inner_mut().write(b"hello world") {
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
fn ready_mut(self: & mut Self, interest: Interest) -> io::Result<AsyncFdReadyMutGuard<T>>
```



## tokio::io::async_fd::AsyncFd::register

*Function*

Creates an `AsyncFd` backed by (and taking ownership of) an object
implementing [`AsRawFd`].

Only configures the [`Interest::READABLE`] and [`Interest::WRITABLE`] interests. For more
control, use [`AsyncFd::register_with_interest`].

This method must be called in the context of a tokio runtime.

In the case of failure, it returns an [`AsyncFdRegisterError`] that contains the original
object passed to this function.

# Safety

The file descriptor returned by [`inner.as_raw_fd()`](AsRawFd::as_raw_fd) must be a valid,
open file descriptor, and it must remain open and refer to the same underlying file
description for the entire lifetime of the `AsyncFd` (i.e., until the `AsyncFd` is
dropped, or [`AsyncFd::into_inner`] is called, or the `AsyncFd` is
[forgotten](std::mem::forget)). While owned by this `AsyncFd`, every call to
[`inner.as_raw_fd()`](AsRawFd::as_raw_fd) must return that same file descriptor.

# Panics

This function panics if there is no current reactor set, or if the `rt`
feature flag is not enabled.

```rust
fn register(inner: T) -> Result<Self, AsyncFdRegisterError<T>>
```



## tokio::io::async_fd::AsyncFd::register_with_interest

*Function*

Creates an `AsyncFd` backed by (and taking ownership of) an object
implementing [`AsRawFd`], with a specific [`Interest`].

This method must be called in the context of a tokio runtime.

In the case of failure, it returns an [`AsyncFdRegisterError`] that contains the original
object passed to this function.

# Safety

The file descriptor returned by [`inner.as_raw_fd()`](AsRawFd::as_raw_fd) must be a valid,
open file descriptor, and it must remain open and refer to the same underlying file
description for the entire lifetime of the `AsyncFd` (i.e., until the `AsyncFd` is
dropped, or [`AsyncFd::into_inner`] is called, or the `AsyncFd` is
[forgotten](std::mem::forget)). While owned by this `AsyncFd`, every call to
[`inner.as_raw_fd()`](AsRawFd::as_raw_fd) must return that same file descriptor.

# Panics

This function panics if there is no current reactor set, or if the `rt`
feature flag is not enabled.

```rust
fn register_with_interest(inner: T, interest: Interest) -> Result<Self, AsyncFdRegisterError<T>>
```



## tokio::io::async_fd::AsyncFd::try_io

*Function*

Tries to read or write from the file descriptor using a user-provided IO operation.

If the file descriptor is ready, the provided closure is called. The closure
should attempt to perform IO operation on the file descriptor by manually
calling the appropriate syscall. If the operation fails because the
file descriptor is not actually ready, then the closure should return a
`WouldBlock` error and the readiness flag is cleared. The return value
of the closure is then returned by `try_io`.

If the file descriptor is not ready, then the closure is not called
and a `WouldBlock` error is returned.

The closure should only return a `WouldBlock` error if it has performed
an IO operation on the file descriptor that failed due to the file descriptor not being
ready. Returning a `WouldBlock` error in any other situation will
incorrectly clear the readiness flag, which can cause the file descriptor to
behave incorrectly.

The closure should not perform the IO operation using any of the methods
defined on the Tokio `AsyncFd` type, as this will mess with the
readiness flag and can cause the file descriptor to behave incorrectly.

This method is not intended to be used with combined interests.
The closure should perform only one type of IO operation, so it should not
require more than one ready state. This method may panic or sleep forever
if it is called with a combined interest.

```rust
fn try_io<R, impl FnOnce(&T) -> io::Result<R>>(self: &Self, interest: Interest, f: impl Trait) -> io::Result<R>
```



## tokio::io::async_fd::AsyncFd::writable

*Function*

Waits for the file descriptor to become writable, returning a
[`AsyncFdReadyGuard`] that must be dropped to resume write-readiness
polling.

This method takes `&self`, so it is possible to call this method
concurrently with other methods on this struct. This method only
provides shared access to the inner IO resource when handling the
[`AsyncFdReadyGuard`].

# Cancel safety

This method is cancel safe. Once a readiness event occurs, the method
will continue to return immediately until the readiness event is
consumed by an attempt to read or write that fails with `WouldBlock` or
`Poll::Pending`.

```rust
fn writable<'a>(self: &'a Self) -> io::Result<AsyncFdReadyGuard<'a, T>>
```



## tokio::io::async_fd::AsyncFd::writable_mut

*Function*

Waits for the file descriptor to become writable, returning a
[`AsyncFdReadyMutGuard`] that must be dropped to resume write-readiness
polling.

This method takes `&mut self`, so it is possible to access the inner IO
resource mutably when handling the [`AsyncFdReadyMutGuard`].

# Cancel safety

This method is cancel safe. Once a readiness event occurs, the method
will continue to return immediately until the readiness event is
consumed by an attempt to read or write that fails with `WouldBlock` or
`Poll::Pending`.

```rust
fn writable_mut<'a>(self: &'a  mut Self) -> io::Result<AsyncFdReadyMutGuard<'a, T>>
```



