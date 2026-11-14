**tokio > io > async_fd**

# Module: io::async_fd

## Contents

**Structs**

- [`AsyncFd`](#asyncfd) - Associates an IO object backed by a Unix file descriptor with the tokio
- [`AsyncFdReadyGuard`](#asyncfdreadyguard) - Represents an IO-ready event detected on a particular file descriptor that
- [`AsyncFdReadyMutGuard`](#asyncfdreadymutguard) - Represents an IO-ready event detected on a particular file descriptor that
- [`AsyncFdRegisterError`](#asyncfdregistererror) - Error returned by [`register`] or [`register_with_interest`].
- [`TryIoError`](#tryioerror) - The error type returned by [`try_io`].

---

## tokio::io::async_fd::AsyncFd

*Struct*

Associates an IO object backed by a Unix file descriptor with the tokio
reactor, allowing for readiness to be polled. The file descriptor must be of
a type that can be used with the OS polling facilities (ie, `poll`, `epoll`,
`kqueue`, etc), such as a network socket or pipe, and the file descriptor
must have the nonblocking mode set to true.

Creating an [`AsyncFd`] registers the file descriptor with the current tokio
Reactor, allowing you to directly await the file descriptor being readable
or writable. Once registered, the file descriptor remains registered until
the [`AsyncFd`] is dropped.

The [`AsyncFd`] takes ownership of an arbitrary object to represent the IO
object. It is intended that the inner object will handle closing the file
descriptor when it is dropped, avoiding resource leaks and ensuring that the
[`AsyncFd`] can clean up the registration before closing the file descriptor.
The [`AsyncFd::into_inner`] function can be used to extract the inner object
to retake control from the tokio IO reactor. The [`OwnedFd`] type is often
used as the inner object, as it is the simplest type that closes the fd on
drop.

The inner object is required to implement [`AsRawFd`]. This file descriptor
must remain open and must not change while [`AsyncFd`] owns the inner
object, i.e. the [`AsRawFd::as_raw_fd`] method on the inner type must always
return the same valid file descriptor referring to the same underlying file
description when called multiple times. Violating this requirement leads to
undefined behavior.

Polling for readiness is done by calling the async functions [`readable`]
and [`writable`]. These functions complete when the associated readiness
condition is observed. Any number of tasks can query the same `AsyncFd` in
parallel, on the same or different conditions.

On some platforms, the readiness detecting mechanism relies on
edge-triggered notifications. This means that the OS will only notify Tokio
when the file descriptor transitions from not-ready to ready. For this to
work you should first try to read or write and only poll for readiness
if that fails with an error of [`std::io::ErrorKind::WouldBlock`].

Tokio internally tracks when it has received a ready notification, and when
readiness checking functions like [`readable`] and [`writable`] are called,
if the readiness flag is set, these async functions will complete
immediately. This however does mean that it is critical to ensure that this
ready flag is cleared when (and only when) the file descriptor ceases to be
ready. The [`AsyncFdReadyGuard`] returned from readiness checking functions
serves this function; after calling a readiness-checking async function,
you must use this [`AsyncFdReadyGuard`] to signal to tokio whether the file
descriptor is no longer in a ready state.

## Use with to a poll-based API

In some cases it may be desirable to use `AsyncFd` from APIs similar to
[`TcpStream::poll_read_ready`]. The [`AsyncFd::poll_read_ready`] and
[`AsyncFd::poll_write_ready`] functions are provided for this purpose.
Because these functions don't create a future to hold their state, they have
the limitation that only one task can wait on each direction (read or write)
at a time.

# Examples

This example shows how to turn [`std::net::TcpStream`] asynchronous using
`AsyncFd`.  It implements the read/write operations both as an `async fn`
and using the IO traits [`AsyncRead`] and [`AsyncWrite`].

```no_run
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::pin::Pin;
use std::task::{ready, Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::io::unix::AsyncFd;

pub struct AsyncTcpStream {
    inner: AsyncFd<TcpStream>,
}

impl AsyncTcpStream {
    pub fn new(tcp: TcpStream) -> io::Result<Self> {
        tcp.set_nonblocking(true)?;
        // SAFETY: `TcpStream` owns a valid file descriptor and we do not
        // replace or close it while owned by `AsyncFd`.
        Ok(Self {
            inner: unsafe { AsyncFd::register(tcp)? },
        })
    }

    pub async fn read(&self, out: &mut [u8]) -> io::Result<usize> {
        loop {
            let mut guard = self.inner.readable().await?;

            match guard.try_io(|inner| inner.get_ref().read(out)) {
                Ok(result) => return result,
                Err(_would_block) => continue,
            }
        }
    }

    pub async fn write(&self, buf: &[u8]) -> io::Result<usize> {
        loop {
            let mut guard = self.inner.writable().await?;

            match guard.try_io(|inner| inner.get_ref().write(buf)) {
                Ok(result) => return result,
                Err(_would_block) => continue,
            }
        }
    }
}

impl AsyncRead for AsyncTcpStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>
    ) -> Poll<io::Result<()>> {
        loop {
            let mut guard = ready!(self.inner.poll_read_ready(cx))?;

            let unfilled = buf.initialize_unfilled();
            match guard.try_io(|inner| inner.get_ref().read(unfilled)) {
                Ok(Ok(len)) => {
                    buf.advance(len);
                    return Poll::Ready(Ok(()));
                },
                Ok(Err(err)) => return Poll::Ready(Err(err)),
                Err(_would_block) => continue,
            }
        }
    }
}

impl AsyncWrite for AsyncTcpStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8]
    ) -> Poll<io::Result<usize>> {
        loop {
            let mut guard = ready!(self.inner.poll_write_ready(cx))?;

            match guard.try_io(|inner| inner.get_ref().write(buf)) {
                Ok(result) => return Poll::Ready(result),
                Err(_would_block) => continue,
            }
        }
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<()>> {
        // tcp flush is a no-op
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<()>> {
        self.inner.get_ref().shutdown(std::net::Shutdown::Write)?;
        Poll::Ready(Ok(()))
    }
}
```

[`readable`]: method@Self::readable
[`writable`]: method@Self::writable
[`AsyncFdReadyGuard`]: struct@self::AsyncFdReadyGuard
[`TcpStream::poll_read_ready`]: method@crate::net::TcpStream::poll_read_ready
[`AsyncRead`]: trait@crate::io::AsyncRead
[`AsyncWrite`]: trait@crate::io::AsyncWrite
[`OwnedFd`]: struct@std::os::fd::OwnedFd

**Generic Parameters:**
- T

**Methods:**

- `fn register(inner: T) -> Result<Self, AsyncFdRegisterError<T>>` - Creates an `AsyncFd` backed by (and taking ownership of) an object
- `fn register_with_interest(inner: T, interest: Interest) -> Result<Self, AsyncFdRegisterError<T>>` - Creates an `AsyncFd` backed by (and taking ownership of) an object
- `fn get_ref(self: &Self) -> &T` - Returns a shared reference to the backing object of this [`AsyncFd`].
- `fn get_mut(self: & mut Self) -> & mut T` - Returns a mutable reference to the backing object of this `AsyncFd`.
- `fn into_inner(self: Self) -> T` - Deregisters this file descriptor and returns ownership of the backing
- `fn poll_read_ready<'a>(self: &'a Self, cx: & mut Context) -> Poll<io::Result<AsyncFdReadyGuard<'a, T>>>` - Polls for read readiness.
- `fn poll_read_ready_mut<'a>(self: &'a  mut Self, cx: & mut Context) -> Poll<io::Result<AsyncFdReadyMutGuard<'a, T>>>` - Polls for read readiness.
- `fn poll_write_ready<'a>(self: &'a Self, cx: & mut Context) -> Poll<io::Result<AsyncFdReadyGuard<'a, T>>>` - Polls for write readiness.
- `fn poll_write_ready_mut<'a>(self: &'a  mut Self, cx: & mut Context) -> Poll<io::Result<AsyncFdReadyMutGuard<'a, T>>>` - Polls for write readiness.
- `fn ready(self: &Self, interest: Interest) -> io::Result<AsyncFdReadyGuard<T>>` - Waits for any of the requested ready states, returning a
- `fn ready_mut(self: & mut Self, interest: Interest) -> io::Result<AsyncFdReadyMutGuard<T>>` - Waits for any of the requested ready states, returning a
- `fn readable<'a>(self: &'a Self) -> io::Result<AsyncFdReadyGuard<'a, T>>` - Waits for the file descriptor to become readable, returning a
- `fn readable_mut<'a>(self: &'a  mut Self) -> io::Result<AsyncFdReadyMutGuard<'a, T>>` - Waits for the file descriptor to become readable, returning a
- `fn writable<'a>(self: &'a Self) -> io::Result<AsyncFdReadyGuard<'a, T>>` - Waits for the file descriptor to become writable, returning a
- `fn writable_mut<'a>(self: &'a  mut Self) -> io::Result<AsyncFdReadyMutGuard<'a, T>>` - Waits for the file descriptor to become writable, returning a
- `fn async_io<R, impl FnMut(&T) -> io::Result<R>>(self: &Self, interest: Interest, f: impl Trait) -> io::Result<R>` - Reads or writes from the file descriptor using a user-provided IO operation.
- `fn async_io_mut<R, impl FnMut(&mut T) -> io::Result<R>>(self: & mut Self, interest: Interest, f: impl Trait) -> io::Result<R>` - Reads or writes from the file descriptor using a user-provided IO operation.
- `fn try_io<R, impl FnOnce(&T) -> io::Result<R>>(self: &Self, interest: Interest, f: impl Trait) -> io::Result<R>` - Tries to read or write from the file descriptor using a user-provided IO operation.
- `fn try_io_mut<R, impl FnOnce(&mut T) -> io::Result<R>>(self: & mut Self, interest: Interest, f: impl Trait) -> io::Result<R>` - Tries to read or write from the file descriptor using a user-provided IO operation.
- `fn new(inner: T) -> io::Result<Self>` - Deprecated. Use [`AsyncFd::register`] instead. See [#8587] for more information.
- `fn with_interest(inner: T, interest: Interest) -> io::Result<Self>` - Deprecated. Use [`AsyncFd::register_with_interest`] instead. See [#8587] for more information.
- `fn try_new(inner: T) -> Result<Self, AsyncFdRegisterError<T>>` - Deprecated. Use [`AsyncFd::register`] instead. See [#8587] for more information.
- `fn try_with_interest(inner: T, interest: Interest) -> Result<Self, AsyncFdRegisterError<T>>` - Deprecated. Use [`AsyncFd::register_with_interest`] instead. See [#8587] for more information.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut std::fmt::Formatter) -> std::fmt::Result`
- **AsRawFd**
  - `fn as_raw_fd(self: &Self) -> RawFd`
- **Drop**
  - `fn drop(self: & mut Self)`
- **AsFd**
  - `fn as_fd(self: &Self) -> std::os::unix::io::BorrowedFd`



## tokio::io::async_fd::AsyncFdReadyGuard

*Struct*

Represents an IO-ready event detected on a particular file descriptor that
has not yet been acknowledged. This is a `must_use` structure to help ensure
that you do not forget to explicitly clear (or not clear) the event.

This type exposes an immutable reference to the underlying IO object.

**Generic Parameters:**
- 'a
- T

**Methods:**

- `fn clear_ready(self: & mut Self)` - Indicates to tokio that the file descriptor is no longer ready. All
- `fn clear_ready_matching(self: & mut Self, ready: Ready)` - Indicates to tokio that the file descriptor no longer has a specific readiness.
- `fn retain_ready(self: & mut Self)` - This method should be invoked when you intentionally want to keep the
- `fn ready(self: &Self) -> Ready` - Get the [`Ready`] value associated with this guard.
- `fn try_io<R, impl FnOnce(&'a AsyncFd<Inner>) -> io::Result<R>>(self: & mut Self, f: impl Trait) -> Result<io::Result<R>, TryIoError>` - Performs the provided IO operation.
- `fn get_ref(self: &Self) -> &'a AsyncFd<Inner>` - Returns a shared reference to the inner [`AsyncFd`].
- `fn get_inner(self: &Self) -> &'a Inner` - Returns a shared reference to the backing object of the inner [`AsyncFd`].

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut std::fmt::Formatter) -> std::fmt::Result`



## tokio::io::async_fd::AsyncFdReadyMutGuard

*Struct*

Represents an IO-ready event detected on a particular file descriptor that
has not yet been acknowledged. This is a `must_use` structure to help ensure
that you do not forget to explicitly clear (or not clear) the event.

This type exposes a mutable reference to the underlying IO object.

**Generic Parameters:**
- 'a
- T

**Methods:**

- `fn clear_ready(self: & mut Self)` - Indicates to tokio that the file descriptor is no longer ready. All
- `fn clear_ready_matching(self: & mut Self, ready: Ready)` - Indicates to tokio that the file descriptor no longer has a specific readiness.
- `fn retain_ready(self: & mut Self)` - This method should be invoked when you intentionally want to keep the
- `fn ready(self: &Self) -> Ready` - Get the [`Ready`] value associated with this guard.
- `fn try_io<R, impl FnOnce(&mut AsyncFd<Inner>) -> io::Result<R>>(self: & mut Self, f: impl Trait) -> Result<io::Result<R>, TryIoError>` - Performs the provided IO operation.
- `fn get_ref(self: &Self) -> &AsyncFd<Inner>` - Returns a shared reference to the inner [`AsyncFd`].
- `fn get_mut(self: & mut Self) -> & mut AsyncFd<Inner>` - Returns a mutable reference to the inner [`AsyncFd`].
- `fn get_inner(self: &Self) -> &Inner` - Returns a shared reference to the backing object of the inner [`AsyncFd`].
- `fn get_inner_mut(self: & mut Self) -> & mut Inner` - Returns a mutable reference to the backing object of the inner [`AsyncFd`].

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut std::fmt::Formatter) -> std::fmt::Result`



## tokio::io::async_fd::AsyncFdRegisterError

*Struct*

Error returned by [`register`] or [`register_with_interest`].

[`register`]: AsyncFd::register
[`register_with_interest`]: AsyncFd::register_with_interest

**Generic Parameters:**
- T

**Methods:**

- `fn into_parts(self: Self) -> (T, io::Error)` - Returns the original object passed to [`register`] or [`register_with_interest`]

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Error**
  - `fn source(self: &Self) -> Option<&dyn Error>`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## tokio::io::async_fd::TryIoError

*Struct*

The error type returned by [`try_io`].

This error indicates that the IO resource returned a [`WouldBlock`] error.

[`WouldBlock`]: std::io::ErrorKind::WouldBlock
[`try_io`]: method@AsyncFdReadyGuard::try_io

**Tuple Struct**: `()`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



