**tokio > net > unix > pipe > OpenOptions**

# Module: net::unix::pipe::OpenOptions

## Contents

**Functions**

- [`new`](#new) - Creates a blank new set of options ready for configuration.
- [`open_receiver`](#open_receiver) - Creates a [`Receiver`] from a FIFO file with the options specified by `self`.
- [`open_sender`](#open_sender) - Creates a [`Sender`] from a FIFO file with the options specified by `self`.

---

## tokio::net::unix::pipe::OpenOptions::new

*Function*

Creates a blank new set of options ready for configuration.

All options are initially set to `false`.

```rust
fn new() -> OpenOptions
```



## tokio::net::unix::pipe::OpenOptions::open_receiver

*Function*

Creates a [`Receiver`] from a FIFO file with the options specified by `self`.

This function will open the FIFO file at the specified path, possibly
check if it is a pipe, and associate the pipe with the default event
loop for reading.

# Errors

If the file type check fails, this function will fail with `io::ErrorKind::InvalidInput`.
This function may also fail with other standard OS errors.

# Panics

This function panics if it is not called from within a runtime with
IO enabled.

The runtime is usually set implicitly when this function is called
from a future driven by a tokio runtime, otherwise runtime can be set
explicitly with [`Runtime::enter`](crate::runtime::Runtime::enter) function.

```rust
fn open_receiver<P>(self: &Self, path: P) -> io::Result<Receiver>
```



## tokio::net::unix::pipe::OpenOptions::open_sender

*Function*

Creates a [`Sender`] from a FIFO file with the options specified by `self`.

This function will open the FIFO file at the specified path, possibly
check if it is a pipe, and associate the pipe with the default event
loop for writing.

# Errors

If the file type check fails, this function will fail with `io::ErrorKind::InvalidInput`.
If the file is not opened in read-write access mode and the file is not
currently open for reading, this function will fail with `ENXIO`.
This function may also fail with other standard OS errors.

# Panics

This function panics if it is not called from within a runtime with
IO enabled.

The runtime is usually set implicitly when this function is called
from a future driven by a tokio runtime, otherwise runtime can be set
explicitly with [`Runtime::enter`](crate::runtime::Runtime::enter) function.

```rust
fn open_sender<P>(self: &Self, path: P) -> io::Result<Sender>
```



