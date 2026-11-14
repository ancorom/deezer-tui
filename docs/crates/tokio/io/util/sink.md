**tokio > io > util > sink**

# Module: io::util::sink

## Contents

**Structs**

- [`Sink`](#sink) - An async writer which will move data into the void.

**Functions**

- [`sink`](#sink) - Creates an instance of an async writer which will successfully consume all

---

## tokio::io::util::sink::Sink

*Struct*

An async writer which will move data into the void.

This struct is generally created by calling [`sink`][sink]. Please
see the documentation of `sink()` for more details.

This is an asynchronous version of [`std::io::Sink`][std].

[sink]: sink()
[std]: std::io::Sink

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **AsyncWrite**
  - `fn poll_write(self: Pin<& mut Self>, cx: & mut Context, buf: &[u8]) -> Poll<Result<usize, io::Error>>`
  - `fn is_write_vectored(self: &Self) -> bool`
  - `fn poll_write_vectored(self: Pin<& mut Self>, cx: & mut Context, bufs: &[io::IoSlice]) -> Poll<Result<usize, io::Error>>`
  - `fn poll_flush(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<(), io::Error>>`
  - `fn poll_shutdown(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<(), io::Error>>`



## tokio::io::util::sink::sink

*Function*

Creates an instance of an async writer which will successfully consume all
data.

All calls to [`poll_write`] on the returned instance will return
`Poll::Ready(Ok(buf.len()))` and the contents of the buffer will not be
inspected.

This is an asynchronous version of [`std::io::sink`][std].

[`poll_write`]: crate::io::AsyncWrite::poll_write()
[std]: std::io::sink

# Examples

```
use tokio::io::{self, AsyncWriteExt};

# #[tokio::main(flavor = "current_thread")]
# async fn main() -> io::Result<()> {
let buffer = vec![1, 2, 3, 5, 8];
let num_bytes = io::sink().write(&buffer).await?;
assert_eq!(num_bytes, 5);
Ok(())
# }
```

```rust
fn sink() -> Sink
```



