**tokio > net > unix > pipe > Sender**

# Module: net::unix::pipe::Sender

## Contents

**Functions**

- [`ready`](#ready) - Waits for any of the requested ready states.
- [`try_write`](#try_write) - Tries to write a buffer to the pipe, returning how many bytes were
- [`writable`](#writable) - Waits for the pipe to become writable.

---

## tokio::net::unix::pipe::Sender::ready

*Function*

Waits for any of the requested ready states.

This function can be used instead of [`writable()`] to check the returned
ready set for [`Ready::WRITABLE`] and [`Ready::WRITE_CLOSED`] events.

The function may complete without the pipe being ready. This is a
false-positive and attempting an operation will return with
`io::ErrorKind::WouldBlock`. The function can also return with an empty
[`Ready`] set, so you should always check the returned value and possibly
wait again if the requested states are not set.

[`writable()`]: Self::writable

# Cancel safety

This method is cancel safe. Once a readiness event occurs, the method
will continue to return immediately until the readiness event is
consumed by an attempt to write that fails with `WouldBlock` or
`Poll::Pending`.

```rust
fn ready(self: &Self, interest: Interest) -> io::Result<Ready>
```



## tokio::net::unix::pipe::Sender::try_write

*Function*

Tries to write a buffer to the pipe, returning how many bytes were
written.

The function will attempt to write the entire contents of `buf`, but
only part of the buffer may be written. If the length of `buf` is not
greater than `PIPE_BUF` (an OS constant, 4096 under Linux), then the
write is guaranteed to be atomic, i.e. either the entire content of
`buf` will be written or this method will fail with `WouldBlock`. There
is no such guarantee if `buf` is larger than `PIPE_BUF`.

This function is usually paired with [`writable`].

[`writable`]: Self::writable

# Return

If data is successfully written, `Ok(n)` is returned, where `n` is the
number of bytes written. If the pipe is not ready to write data,
`Err(io::ErrorKind::WouldBlock)` is returned.

# Notes

To avoid unnecessary syscalls, this will only attempt the write
operation if the OS has informed Tokio that this pipe has become
writable. Because of this, `try_write()` may fail with a
[`WouldBlock`] error if Tokio has not yet heard from the OS that
this pipe has become writable.

# Examples

```no_run
use tokio::net::unix::pipe;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Open a writing end of a fifo
    let tx = pipe::OpenOptions::new().open_sender("path/to/a/fifo")?;

    loop {
        // Wait for the pipe to be writable
        tx.writable().await?;

        // Try to write data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match tx.try_write(b"hello world") {
            Ok(n) => {
                break;
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
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

[`WouldBlock`]: std::io::ErrorKind::WouldBlock

```rust
fn try_write(self: &Self, buf: &[u8]) -> io::Result<usize>
```



## tokio::net::unix::pipe::Sender::writable

*Function*

Waits for the pipe to become writable.

This function is equivalent to `ready(Interest::WRITABLE)` and is usually
paired with [`try_write()`].

[`try_write()`]: Self::try_write

# Examples

```no_run
use tokio::net::unix::pipe;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Open a writing end of a fifo
    let tx = pipe::OpenOptions::new().open_sender("path/to/a/fifo")?;

    loop {
        // Wait for the pipe to be writable
        tx.writable().await?;

        // Try to write data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match tx.try_write(b"hello world") {
            Ok(n) => {
                break;
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
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



