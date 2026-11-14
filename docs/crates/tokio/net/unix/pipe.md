**tokio > net > unix > pipe**

# Module: net::unix::pipe

## Contents

**Structs**

- [`OpenOptions`](#openoptions) - Options and flags which can be used to configure how a FIFO file is opened.
- [`Receiver`](#receiver) - Reading end of a Unix pipe.
- [`Sender`](#sender) - Writing end of a Unix pipe.

**Functions**

- [`pipe`](#pipe) - Creates a new anonymous Unix pipe.

---

## tokio::net::unix::pipe::OpenOptions

*Struct*

Options and flags which can be used to configure how a FIFO file is opened.

This builder allows configuring how to create a pipe end from a FIFO file.
Generally speaking, when using `OpenOptions`, you'll first call [`new`],
then chain calls to methods to set each option, then call either
[`open_receiver`] or [`open_sender`], passing the path of the FIFO file you
are trying to open. This will give you a [`io::Result`] with a pipe end
inside that you can further operate on.

[`new`]: OpenOptions::new
[`open_receiver`]: OpenOptions::open_receiver
[`open_sender`]: OpenOptions::open_sender

# Examples

Opening a pair of pipe ends from a FIFO file:

```no_run
use tokio::net::unix::pipe;
# use std::error::Error;

const FIFO_NAME: &str = "path/to/a/fifo";

# async fn dox() -> Result<(), Box<dyn Error>> {
let rx = pipe::OpenOptions::new().open_receiver(FIFO_NAME)?;
let tx = pipe::OpenOptions::new().open_sender(FIFO_NAME)?;
# Ok(())
# }
```

Opening a [`Sender`] on Linux when you are sure the file is a FIFO:

```ignore
use tokio::net::unix::pipe;
use nix::{unistd::mkfifo, sys::stat::Mode};
# use std::error::Error;

// Our program has exclusive access to this path.
const FIFO_NAME: &str = "path/to/a/new/fifo";

# async fn dox() -> Result<(), Box<dyn Error>> {
mkfifo(FIFO_NAME, Mode::S_IRWXU)?;
let tx = pipe::OpenOptions::new()
    .read_write(true)
    .unchecked(true)
    .open_sender(FIFO_NAME)?;
# Ok(())
# }
```

**Methods:**

- `fn new() -> OpenOptions` - Creates a blank new set of options ready for configuration.
- `fn read_write(self: & mut Self, value: bool) -> & mut Self` - Sets the option for read-write access.
- `fn unchecked(self: & mut Self, value: bool) -> & mut Self` - Sets the option to skip the check for FIFO file type.
- `fn open_receiver<P>(self: &Self, path: P) -> io::Result<Receiver>` - Creates a [`Receiver`] from a FIFO file with the options specified by `self`.
- `fn open_sender<P>(self: &Self, path: P) -> io::Result<Sender>` - Creates a [`Sender`] from a FIFO file with the options specified by `self`.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> OpenOptions`



## tokio::net::unix::pipe::Receiver

*Struct*

Reading end of a Unix pipe.

It can be constructed from a FIFO file with [`OpenOptions::open_receiver`].

# Examples

Receiving messages from a named pipe in a loop:

```no_run
use tokio::net::unix::pipe;
use tokio::io::{self, AsyncReadExt};

const FIFO_NAME: &str = "path/to/a/fifo";

# async fn dox() -> Result<(), Box<dyn std::error::Error>> {
let mut rx = pipe::OpenOptions::new().open_receiver(FIFO_NAME)?;
loop {
    let mut msg = vec![0; 256];
    match rx.read_exact(&mut msg).await {
        Ok(_) => {
            /* handle the message */
        }
        Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
            // Writing end has been closed, we should reopen the pipe.
            rx = pipe::OpenOptions::new().open_receiver(FIFO_NAME)?;
        }
        Err(e) => return Err(e.into()),
    }
}
# }
```

On Linux, you can use a `Receiver` in read-write access mode to implement
resilient reading from a named pipe. Unlike `Receiver` opened in read-only
mode, read from a pipe in read-write mode will not fail with `UnexpectedEof`
when the writing end is closed. This way, a `Receiver` can asynchronously
wait for the next writer to open the pipe.

You should not use functions waiting for EOF such as [`read_to_end`] with
a `Receiver` in read-write access mode, since it **may wait forever**.
`Receiver` in this mode also holds an open writing end, which prevents
receiving EOF.

To set the read-write access mode you can use `OpenOptions::read_write`.
Note that using read-write access mode with FIFO files is not defined by
the POSIX standard and it is only guaranteed to work on Linux.

```ignore
use tokio::net::unix::pipe;
use tokio::io::AsyncReadExt;
# use std::error::Error;

const FIFO_NAME: &str = "path/to/a/fifo";

# async fn dox() -> Result<(), Box<dyn Error>> {
let mut rx = pipe::OpenOptions::new()
    .read_write(true)
    .open_receiver(FIFO_NAME)?;
loop {
    let mut msg = vec![0; 256];
    rx.read_exact(&mut msg).await?;
    /* handle the message */
}
# }
```

[`read_to_end`]: crate::io::AsyncReadExt::read_to_end

**Methods:**

- `fn from_file(file: File) -> io::Result<Receiver>` - Creates a new `Receiver` from a [`File`].
- `fn from_owned_fd(owned_fd: OwnedFd) -> io::Result<Receiver>` - Creates a new `Receiver` from an [`OwnedFd`].
- `fn from_file_unchecked(file: File) -> io::Result<Receiver>` - Creates a new `Receiver` from a [`File`] without checking pipe properties.
- `fn from_owned_fd_unchecked(owned_fd: OwnedFd) -> io::Result<Receiver>` - Creates a new `Receiver` from an [`OwnedFd`] without checking pipe properties.
- `fn ready(self: &Self, interest: Interest) -> io::Result<Ready>` - Waits for any of the requested ready states.
- `fn readable(self: &Self) -> io::Result<()>` - Waits for the pipe to become readable.
- `fn poll_read_ready(self: &Self, cx: & mut Context) -> Poll<io::Result<()>>` - Polls for read readiness.
- `fn try_read(self: &Self, buf: & mut [u8]) -> io::Result<usize>` - Tries to read data from the pipe into the provided buffer, returning how
- `fn try_read_vectored(self: &Self, bufs: & mut [io::IoSliceMut]) -> io::Result<usize>` - Tries to read data from the pipe into the provided buffers, returning
- `fn try_io<R, impl FnOnce() -> io::Result<R>>(self: &Self, f: impl Trait) -> io::Result<R>` - Tries to read to the socket using a user-provided IO operation.
- `fn try_read_buf<B>(self: &Self, buf: & mut B) -> io::Result<usize>` - Tries to read data from the pipe into the provided buffer, advancing the
- `fn into_blocking_fd(self: Self) -> io::Result<OwnedFd>` - Converts the pipe into an [`OwnedFd`] in blocking mode.
- `fn into_nonblocking_fd(self: Self) -> io::Result<OwnedFd>` - Converts the pipe into an [`OwnedFd`] in nonblocking mode.

**Trait Implementations:**

- **AsyncRead**
  - `fn poll_read(self: Pin<& mut Self>, cx: & mut Context, buf: & mut ReadBuf) -> Poll<io::Result<()>>`
- **AsFd**
  - `fn as_fd(self: &Self) -> BorrowedFd`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **AsRawFd**
  - `fn as_raw_fd(self: &Self) -> RawFd`



## tokio::net::unix::pipe::Sender

*Struct*

Writing end of a Unix pipe.

It can be constructed from a FIFO file with [`OpenOptions::open_sender`].

Opening a named pipe for writing involves a few steps.
Call to [`OpenOptions::open_sender`] might fail with an error indicating
different things:

* [`io::ErrorKind::NotFound`] - There is no file at the specified path.
* [`io::ErrorKind::InvalidInput`] - The file exists, but it is not a FIFO.
* [`ENXIO`] - The file is a FIFO, but no process has it open for reading.
  Sleep for a while and try again.
* Other OS errors not specific to opening FIFO files.

Opening a `Sender` from a FIFO file should look like this:

```no_run
use tokio::net::unix::pipe;
use tokio::time::{self, Duration};

const FIFO_NAME: &str = "path/to/a/fifo";

# async fn dox() -> Result<(), Box<dyn std::error::Error>> {
// Wait for a reader to open the file.
let tx = loop {
    match pipe::OpenOptions::new().open_sender(FIFO_NAME) {
        Ok(tx) => break tx,
        Err(e) if e.raw_os_error() == Some(libc::ENXIO) => {},
        Err(e) => return Err(e.into()),
    }

    time::sleep(Duration::from_millis(50)).await;
};
# Ok(())
# }
```

On Linux, it is possible to create a `Sender` without waiting in a sleeping
loop. This is done by opening a named pipe in read-write access mode with
`OpenOptions::read_write`. This way, a `Sender` can at the same time hold
both a writing end and a reading end, and the latter allows to open a FIFO
without [`ENXIO`] error since the pipe is open for reading as well.

`Sender` cannot be used to read from a pipe, so in practice the read access
is only used when a FIFO is opened. However, using a `Sender` in read-write
mode **may lead to lost data**, because written data will be dropped by the
system as soon as all pipe ends are closed. To avoid lost data you have to
make sure that a reading end has been opened before dropping a `Sender`.

Note that using read-write access mode with FIFO files is not defined by
the POSIX standard and it is only guaranteed to work on Linux.

```ignore
use tokio::io::AsyncWriteExt;
use tokio::net::unix::pipe;

const FIFO_NAME: &str = "path/to/a/fifo";

# async fn dox() -> Result<(), Box<dyn std::error::Error>> {
let mut tx = pipe::OpenOptions::new()
    .read_write(true)
    .open_sender(FIFO_NAME)?;

// Asynchronously write to the pipe before a reader.
tx.write_all(b"hello world").await?;
# Ok(())
# }
```

[`ENXIO`]: https://docs.rs/libc/latest/libc/constant.ENXIO.html

**Methods:**

- `fn from_file(file: File) -> io::Result<Sender>` - Creates a new `Sender` from a [`File`].
- `fn from_owned_fd(owned_fd: OwnedFd) -> io::Result<Sender>` - Creates a new `Sender` from an [`OwnedFd`].
- `fn from_file_unchecked(file: File) -> io::Result<Sender>` - Creates a new `Sender` from a [`File`] without checking pipe properties.
- `fn from_owned_fd_unchecked(owned_fd: OwnedFd) -> io::Result<Sender>` - Creates a new `Sender` from an [`OwnedFd`] without checking pipe properties.
- `fn ready(self: &Self, interest: Interest) -> io::Result<Ready>` - Waits for any of the requested ready states.
- `fn writable(self: &Self) -> io::Result<()>` - Waits for the pipe to become writable.
- `fn poll_write_ready(self: &Self, cx: & mut Context) -> Poll<io::Result<()>>` - Polls for write readiness.
- `fn try_write(self: &Self, buf: &[u8]) -> io::Result<usize>` - Tries to write a buffer to the pipe, returning how many bytes were
- `fn try_write_vectored(self: &Self, buf: &[io::IoSlice]) -> io::Result<usize>` - Tries to write several buffers to the pipe, returning how many bytes
- `fn try_io<R, impl FnOnce() -> io::Result<R>>(self: &Self, f: impl Trait) -> io::Result<R>` - Tries to write from the socket using a user-provided IO operation.
- `fn into_blocking_fd(self: Self) -> io::Result<OwnedFd>` - Converts the pipe into an [`OwnedFd`] in blocking mode.
- `fn into_nonblocking_fd(self: Self) -> io::Result<OwnedFd>` - Converts the pipe into an [`OwnedFd`] in nonblocking mode.

**Trait Implementations:**

- **AsFd**
  - `fn as_fd(self: &Self) -> BorrowedFd`
- **AsyncWrite**
  - `fn poll_write(self: Pin<& mut Self>, cx: & mut Context, buf: &[u8]) -> Poll<io::Result<usize>>`
  - `fn poll_write_vectored(self: Pin<& mut Self>, cx: & mut Context, bufs: &[io::IoSlice]) -> Poll<io::Result<usize>>`
  - `fn is_write_vectored(self: &Self) -> bool`
  - `fn poll_flush(self: Pin<& mut Self>, _: & mut Context) -> Poll<io::Result<()>>`
  - `fn poll_shutdown(self: Pin<& mut Self>, _: & mut Context) -> Poll<io::Result<()>>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **AsRawFd**
  - `fn as_raw_fd(self: &Self) -> RawFd`



## tokio::net::unix::pipe::pipe

*Function*

Creates a new anonymous Unix pipe.

This function will open a new pipe and associate both pipe ends with the default
event loop.

If you need to create a pipe for communication with a spawned process, you can
use [`Stdio::piped()`] instead.

[`Stdio::piped()`]: std::process::Stdio::piped

# Errors

If creating a pipe fails, this function will return with the related OS error.

# Examples

Create a pipe and pass the writing end to a spawned process.

```no_run
use tokio::net::unix::pipe;
use tokio::process::Command;
# use tokio::io::AsyncReadExt;
# use std::error::Error;

# async fn dox() -> Result<(), Box<dyn Error>> {
let (tx, mut rx) = pipe::pipe()?;
let mut buffer = String::new();

let status = Command::new("echo")
    .arg("Hello, world!")
    .stdout(tx.into_blocking_fd()?)
    .status();
rx.read_to_string(&mut buffer).await?;

assert!(status.await?.success());
assert_eq!(buffer, "Hello, world!\n");
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
fn pipe() -> io::Result<(Sender, Receiver)>
```



