**tokio > io > async_seek**

# Module: io::async_seek

## Contents

**Traits**

- [`AsyncSeek`](#asyncseek) - Seek bytes asynchronously.

---

## tokio::io::async_seek::AsyncSeek

*Trait*

Seek bytes asynchronously.

This trait is analogous to the [`std::io::Seek`] trait, but integrates
with the asynchronous task system. In particular, the `start_seek`
method, unlike [`Seek::seek`], will not block the calling thread.

Utilities for working with `AsyncSeek` values are provided by
[`AsyncSeekExt`].

[`std::io::Seek`]: std::io::Seek
[`Seek::seek`]: std::io::Seek::seek()
[`AsyncSeekExt`]: crate::io::AsyncSeekExt

**Methods:**

- `start_seek`: Attempts to seek to an offset, in bytes, in a stream.
- `poll_complete`: Waits for a seek operation to complete.



