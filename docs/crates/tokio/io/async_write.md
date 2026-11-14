**tokio > io > async_write**

# Module: io::async_write

## Contents

**Traits**

- [`AsyncWrite`](#asyncwrite) - Writes bytes asynchronously.

---

## tokio::io::async_write::AsyncWrite

*Trait*

Writes bytes asynchronously.

This trait is analogous to the [`std::io::Write`] trait, but integrates with
the asynchronous task system. In particular, the [`poll_write`] method,
unlike [`Write::write`], will automatically queue the current task for wakeup
and return if data is not yet available, rather than blocking the calling
thread.

Specifically, this means that the [`poll_write`] function will return one of
the following:

* `Poll::Ready(Ok(n))` means that `n` bytes of data was immediately
  written.

* `Poll::Pending` means that no data was written from the buffer
  provided. The I/O object is not currently writable but may become writable
  in the future. Most importantly, **the current future's task is scheduled
  to get unparked when the object is writable**. This means that like
  `Future::poll` you'll receive a notification when the I/O object is
  writable again.

* `Poll::Ready(Err(e))` for other errors are standard I/O errors coming from the
  underlying object.

Utilities for working with `AsyncWrite` values are provided by
[`AsyncWriteExt`]. Most users will interact with `AsyncWrite` types through
these extension methods, which provide ergonomic async functions such as
`write_all` and `flush`.

[`std::io::Write`]: std::io::Write
[`Write::write`]: std::io::Write::write()
[`poll_write`]: AsyncWrite::poll_write()
[`AsyncWriteExt`]: crate::io::AsyncWriteExt

**Methods:**

- `poll_write`: Attempt to write bytes from `buf` into the object.
- `poll_flush`: Attempts to flush the object, ensuring that any buffered data reach
- `poll_shutdown`: Initiates or attempts to shut down this writer, returning success when
- `poll_write_vectored`: Like [`poll_write`], except that it writes from a slice of buffers.
- `is_write_vectored`: Determines if this writer has an efficient [`poll_write_vectored`]



