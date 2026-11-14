**tokio > io > stdin**

# Module: io::stdin

## Contents

**Structs**

- [`Stdin`](#stdin) - A handle to the standard input stream of a process.

**Functions**

- [`stdin`](#stdin) - Constructs a new handle to the standard input of the current process.

---

## tokio::io::stdin::Stdin

*Struct*

A handle to the standard input stream of a process.

The handle implements the [`AsyncRead`] trait, but beware that concurrent
reads of `Stdin` must be executed with care.

This handle is best used for non-interactive uses, such as when a file
is piped into the application. For technical reasons, `stdin` is
implemented by using an ordinary blocking read on a separate thread, and
it is impossible to cancel that read. This can make shutdown of the
runtime hang until the user presses enter.

For interactive uses, it is recommended to spawn a thread dedicated to
user input and use blocking IO directly in that thread.

Created by the [`stdin`] function.

[`stdin`]: fn@stdin
[`AsyncRead`]: trait@AsyncRead

**Trait Implementations:**

- **AsyncRead**
  - `fn poll_read(self: Pin<& mut Self>, cx: & mut Context, buf: & mut ReadBuf) -> Poll<io::Result<()>>`
- **AsFd**
  - `fn as_fd(self: &Self) -> BorrowedFd`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **AsRawFd**
  - `fn as_raw_fd(self: &Self) -> RawFd`



## tokio::io::stdin::stdin

*Function*

Constructs a new handle to the standard input of the current process.

This handle is best used for non-interactive uses, such as when a file
is piped into the application. For technical reasons, `stdin` is
implemented by using an ordinary blocking read on a separate thread, and
it is impossible to cancel that read. This can make shutdown of the
runtime hang until the user presses enter.

For interactive uses, it is recommended to spawn a thread dedicated to
user input and use blocking IO directly in that thread.

```rust
fn stdin() -> Stdin
```



