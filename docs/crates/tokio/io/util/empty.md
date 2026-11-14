**tokio > io > util > empty**

# Module: io::util::empty

## Contents

**Structs**

- [`Empty`](#empty) - `Empty` ignores any data written via [`AsyncWrite`], and will always be empty

**Functions**

- [`empty`](#empty) - Creates a value that is always at EOF for reads, and ignores all data written.

---

## tokio::io::util::empty::Empty

*Struct*

`Empty` ignores any data written via [`AsyncWrite`], and will always be empty
(returning zero bytes) when read via [`AsyncRead`].

This struct is generally created by calling [`empty`]. Please see
the documentation of [`empty()`][`empty`] for more details.

This is an asynchronous version of [`std::io::empty`][std].

[`empty`]: fn@empty
[std]: std::io::empty

**Trait Implementations:**

- **AsyncBufRead**
  - `fn poll_fill_buf(self: Pin<& mut Self>, cx: & mut Context) -> Poll<io::Result<&[u8]>>`
  - `fn consume(self: Pin<& mut Self>, _: usize)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **AsyncSeek**
  - `fn start_seek(self: Pin<& mut Self>, _position: SeekFrom) -> io::Result<()>`
  - `fn poll_complete(self: Pin<& mut Self>, cx: & mut Context) -> Poll<io::Result<u64>>`
- **AsyncWrite**
  - `fn poll_write(self: Pin<& mut Self>, cx: & mut Context, buf: &[u8]) -> Poll<io::Result<usize>>`
  - `fn poll_flush(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<(), io::Error>>`
  - `fn poll_shutdown(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<(), io::Error>>`
  - `fn is_write_vectored(self: &Self) -> bool`
  - `fn poll_write_vectored(self: Pin<& mut Self>, cx: & mut Context, bufs: &[io::IoSlice]) -> Poll<Result<usize, io::Error>>`
- **AsyncRead**
  - `fn poll_read(self: Pin<& mut Self>, cx: & mut Context, _: & mut ReadBuf) -> Poll<io::Result<()>>`



## tokio::io::util::empty::empty

*Function*

Creates a value that is always at EOF for reads, and ignores all data written.

All writes on the returned instance will return `Poll::Ready(Ok(buf.len()))`
and the contents of the buffer will not be inspected.

All reads from the returned instance will return `Poll::Ready(Ok(0))`.

This is an asynchronous version of [`std::io::empty`][std].

[std]: std::io::empty

# Examples

A slightly sad example of not reading anything into a buffer:

```
use tokio::io::{self, AsyncReadExt};

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
    let mut buffer = String::new();
    io::empty().read_to_string(&mut buffer).await.unwrap();
    assert!(buffer.is_empty());
# }
```

A convoluted way of getting the length of a buffer:

```
use tokio::io::{self, AsyncWriteExt};

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let buffer = vec![1, 2, 3, 5, 8];
let num_bytes = io::empty().write(&buffer).await.unwrap();
assert_eq!(num_bytes, 5);
# }
```

```rust
fn empty() -> Empty
```



