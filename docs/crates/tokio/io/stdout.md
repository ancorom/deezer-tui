**tokio > io > stdout**

# Module: io::stdout

## Contents

**Structs**

- [`Stdout`](#stdout) - A handle to the standard output stream of a process.

**Functions**

- [`stdout`](#stdout) - Constructs a new handle to the standard output of the current process.

---

## tokio::io::stdout::Stdout

*Struct*

A handle to the standard output stream of a process.

Concurrent writes to stdout must be executed with care: Only individual
writes to this [`AsyncWrite`] are guaranteed to be intact. In particular
you should be aware that writes using [`write_all`] are not guaranteed
to occur as a single write, so multiple threads writing data with
[`write_all`] may result in interleaved output.

# Warning

Each call to [`stdout()`] creates a **new** handle with its own
internal state. Writes through different handles are not
coordinated, so creating a new handle in a loop can cause output
to appear out of order:

```no_run
# use tokio::io::{self, AsyncWriteExt};
# #[tokio::main]
# async fn main() -> std::io::Result<()> {
// WRONG: creates a new handle each iteration
for i in 0..10 {
    let mut out = io::stdout();
    out.write_all(b"data").await?;
    out.write_all(b"\n").await?;
    // out is dropped here; its last write may still be
    // running when the next iteration starts
}
# Ok(())
# }
```

To preserve order, create one handle outside the loop and
reuse it:

```no_run
# use tokio::io::{self, AsyncWriteExt};
# #[tokio::main]
# async fn main() -> std::io::Result<()> {
let mut out = io::stdout();
for i in 0..10 {
    out.write_all(b"data").await?;
    out.write_all(b"\n").await?;
}
# Ok(())
# }
```

Created by the [`stdout`] function.

[`stdout`]: stdout()
[`AsyncWrite`]: AsyncWrite
[`write_all`]: crate::io::AsyncWriteExt::write_all()

# Examples

```
use tokio::io::{self, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.write_all(b"Hello world!").await?;
    Ok(())
}
```

The following is an example of using `stdio` with loop.

```
use tokio::io::{self, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let messages = vec!["hello", " world\n"];

    // When you use `stdio` in a loop, it is recommended to create
    // a single `stdio` instance outside the loop and call a write
    // operation against that instance on each loop.
    //
    // Repeatedly creating `stdout` instances inside the loop and
    // writing to that handle could result in mangled output since
    // each write operation is handled by a different blocking thread.
    let mut stdout = io::stdout();

    for message in &messages {
        stdout.write_all(message.as_bytes()).await.unwrap();
        stdout.flush().await.unwrap();
    }
}
```

**Trait Implementations:**

- **AsRawFd**
  - `fn as_raw_fd(self: &Self) -> RawFd`
- **AsFd**
  - `fn as_fd(self: &Self) -> BorrowedFd`
- **AsyncWrite**
  - `fn poll_write(self: Pin<& mut Self>, cx: & mut Context, buf: &[u8]) -> Poll<io::Result<usize>>`
  - `fn poll_flush(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<(), io::Error>>`
  - `fn poll_shutdown(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<(), io::Error>>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## tokio::io::stdout::stdout

*Function*

Constructs a new handle to the standard output of the current process.

The returned handle allows writing to standard out from the within the
Tokio runtime.

Concurrent writes to stdout must be executed with care: Only individual
writes to this [`AsyncWrite`] are guaranteed to be intact. In particular
you should be aware that writes using [`write_all`] are not guaranteed
to occur as a single write, so multiple threads writing data with
[`write_all`] may result in interleaved output.

Note that unlike [`std::io::stdout`], each call to this `stdout()`
produces a new writer, so for example, this program does **not** flush stdout:

```no_run
# use tokio::io::AsyncWriteExt;
# #[tokio::main]
# async fn main() -> std::io::Result<()> {
tokio::io::stdout().write_all(b"aa").await?;
tokio::io::stdout().flush().await?;
# Ok(())
# }
```

[`std::io::stdout`]: std::io::stdout
[`AsyncWrite`]: AsyncWrite
[`write_all`]: crate::io::AsyncWriteExt::write_all()

# Examples

```
use tokio::io::{self, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.write_all(b"Hello world!").await?;
    Ok(())
}
```

The following is an example of using `stdio` with loop.

```
use tokio::io::{self, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let messages = vec!["hello", " world\n"];

    // When you use `stdio` in a loop, it is recommended to create
    // a single `stdio` instance outside the loop and call a write
    // operation against that instance on each loop.
    //
    // Repeatedly creating `stdout` instances inside the loop and
    // writing to that handle could result in mangled output since
    // each write operation is handled by a different blocking thread.
    let mut stdout = io::stdout();

    for message in &messages {
        stdout.write_all(message.as_bytes()).await.unwrap();
        stdout.flush().await.unwrap();
    }
}
```

```rust
fn stdout() -> Stdout
```



