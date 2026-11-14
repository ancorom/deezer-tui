**tokio > net > unix > pipe > Receiver**

# Module: net::unix::pipe::Receiver

## Contents

**Functions**

- [`readable`](#readable) - Waits for the pipe to become readable.
- [`ready`](#ready) - Waits for any of the requested ready states.
- [`try_read`](#try_read) - Tries to read data from the pipe into the provided buffer, returning how

---

## tokio::net::unix::pipe::Receiver::readable

*Function*

Waits for the pipe to become readable.

This function is equivalent to `ready(Interest::READABLE)` and is usually
paired with [`try_read()`].

[`try_read()`]: Self::try_read()

# Examples

```no_run
use tokio::net::unix::pipe;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Open a reading end of a fifo
    let rx = pipe::OpenOptions::new().open_receiver("path/to/a/fifo")?;

    let mut msg = vec![0; 1024];

    loop {
        // Wait for the pipe to be readable
        rx.readable().await?;

        // Try to read data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match rx.try_read(&mut msg) {
            Ok(n) => {
                msg.truncate(n);
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

    println!("GOT = {:?}", msg);
    Ok(())
}
```

```rust
fn readable(self: &Self) -> io::Result<()>
```



## tokio::net::unix::pipe::Receiver::ready

*Function*

Waits for any of the requested ready states.

This function can be used instead of [`readable()`] to check the returned
ready set for [`Ready::READABLE`] and [`Ready::READ_CLOSED`] events.

The function may complete without the pipe being ready. This is a
false-positive and attempting an operation will return with
`io::ErrorKind::WouldBlock`. The function can also return with an empty
[`Ready`] set, so you should always check the returned value and possibly
wait again if the requested states are not set.

[`readable()`]: Self::readable

# Cancel safety

This method is cancel safe. Once a readiness event occurs, the method
will continue to return immediately until the readiness event is
consumed by an attempt to read that fails with `WouldBlock` or
`Poll::Pending`.

```rust
fn ready(self: &Self, interest: Interest) -> io::Result<Ready>
```



## tokio::net::unix::pipe::Receiver::try_read

*Function*

Tries to read data from the pipe into the provided buffer, returning how
many bytes were read.

Reads any pending data from the pipe but does not wait for new data
to arrive. On success, returns the number of bytes read. Because
`try_read()` is non-blocking, the buffer does not have to be stored by
the async task and can exist entirely on the stack.

Usually [`readable()`] is used with this function.

[`readable()`]: Self::readable()

# Return

If data is successfully read, `Ok(n)` is returned, where `n` is the
number of bytes read. If `n` is `0`, then it can indicate one of two scenarios:

1. The pipe's writing end is closed and will no longer write data.
2. The specified buffer was 0 bytes in length.

If the pipe is not ready to read data,
`Err(io::ErrorKind::WouldBlock)` is returned.

# Notes

To avoid unnecessary syscalls, this will only attempt the read
operation if the OS has informed Tokio that this pipe has become
readable. Because of this, `try_read()` may fail with a
[`WouldBlock`] error if Tokio has not yet heard from the OS that
this pipe has become readable.

# Examples

```no_run
use tokio::net::unix::pipe;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Open a reading end of a fifo
    let rx = pipe::OpenOptions::new().open_receiver("path/to/a/fifo")?;

    let mut msg = vec![0; 1024];

    loop {
        // Wait for the pipe to be readable
        rx.readable().await?;

        // Try to read data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match rx.try_read(&mut msg) {
            Ok(n) => {
                msg.truncate(n);
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

    println!("GOT = {:?}", msg);
    Ok(())
}
```

[`WouldBlock`]: std::io::ErrorKind::WouldBlock

```rust
fn try_read(self: &Self, buf: & mut [u8]) -> io::Result<usize>
```



