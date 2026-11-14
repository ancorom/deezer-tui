**tokio > io > stderr**

# Module: io::stderr

## Contents

**Structs**

- [`Stderr`](#stderr) - A handle to the standard error stream of a process.

**Functions**

- [`stderr`](#stderr) - Constructs a new handle to the standard error of the current process.

---

## tokio::io::stderr::Stderr

*Struct*

A handle to the standard error stream of a process.

Concurrent writes to stderr must be executed with care: Only individual
writes to this [`AsyncWrite`] are guaranteed to be intact. In particular
you should be aware that writes using [`write_all`] are not guaranteed
to occur as a single write, so multiple threads writing data with
[`write_all`] may result in interleaved output.

Created by the [`stderr`] function.

[`stderr`]: stderr()
[`AsyncWrite`]: AsyncWrite
[`write_all`]: crate::io::AsyncWriteExt::write_all()

# Examples

```
use tokio::io::{self, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut stderr = io::stderr();
    stderr.write_all(b"Print some error here.").await?;
    Ok(())
}
```

**Trait Implementations:**

- **AsFd**
  - `fn as_fd(self: &Self) -> BorrowedFd`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **AsyncWrite**
  - `fn poll_write(self: Pin<& mut Self>, cx: & mut Context, buf: &[u8]) -> Poll<io::Result<usize>>`
  - `fn poll_flush(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<(), io::Error>>`
  - `fn poll_shutdown(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<(), io::Error>>`
- **AsRawFd**
  - `fn as_raw_fd(self: &Self) -> RawFd`



## tokio::io::stderr::stderr

*Function*

Constructs a new handle to the standard error of the current process.

The returned handle allows writing to standard error from the within the
Tokio runtime.

Concurrent writes to stderr must be executed with care: Only individual
writes to this [`AsyncWrite`] are guaranteed to be intact. In particular
you should be aware that writes using [`write_all`] are not guaranteed
to occur as a single write, so multiple threads writing data with
[`write_all`] may result in interleaved output.

Note that unlike [`std::io::stderr`], each call to this `stderr()`
produces a new writer, so for example, this program does **not** flush stderr:

```no_run
# use tokio::io::AsyncWriteExt;
# #[tokio::main]
# async fn main() -> std::io::Result<()> {
tokio::io::stderr().write_all(b"aa").await?;
tokio::io::stderr().flush().await?;
# Ok(())
# }
```

[`std::io::stderr`]: std::io::stderr
[`AsyncWrite`]: AsyncWrite
[`write_all`]: crate::io::AsyncWriteExt::write_all()

# Examples

```
use tokio::io::{self, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut stderr = io::stderr();
    stderr.write_all(b"Print some error here.").await?;
    Ok(())
}
```

```rust
fn stderr() -> Stderr
```



