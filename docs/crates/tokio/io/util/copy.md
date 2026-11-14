**tokio > io > util > copy**

# Module: io::util::copy

## Contents

**Functions**

- [`copy`](#copy) - Asynchronously copies the entire contents of a reader into a writer.

---

## tokio::io::util::copy::copy

*Function*

Asynchronously copies the entire contents of a reader into a writer.

This function returns a future that will continuously read data from
`reader` and then write it into `writer` in a streaming fashion until
`reader` returns EOF or fails.

On success, the total number of bytes that were copied from `reader` to
`writer` is returned.

This is an asynchronous version of [`std::io::copy`][std].

A heap-allocated copy buffer with 8 KB is created to take data from the
reader to the writer, check [`copy_buf`] if you want an alternative for
[`AsyncBufRead`]. You can use `copy_buf` with [`BufReader`] to change the
buffer capacity.

# When to use async alternatives instead of `SyncIoBridge`

If you are looking to use [`std::io::copy`] with a synchronous consumer
(like a `hasher` or compressor), consider using async alternatives instead of
wrapping the reader with [`SyncIoBridge`].
See the [`SyncIoBridge`] documentation for detailed examples and guidance.

[std]: std::io::copy
[`copy_buf`]: crate::io::copy_buf
[`AsyncBufRead`]: crate::io::AsyncBufRead
[`BufReader`]: crate::io::BufReader
[`SyncIoBridge`]: https://docs.rs/tokio-util/latest/tokio_util/io/struct.SyncIoBridge.html

# Errors

The returned future will return an error immediately if any call to
`poll_read` or `poll_write` returns an error other than
[`io::ErrorKind::Interrupted`], which is retried. Errors from flushing
the writer are returned without retrying.

# Examples

```
use tokio::io;

# async fn dox() -> std::io::Result<()> {
let mut reader: &[u8] = b"hello";
let mut writer: Vec<u8> = vec![];

io::copy(&mut reader, &mut writer).await?;

assert_eq!(&b"hello"[..], &writer[..]);
# Ok(())
# }
```

```rust
fn copy<'a, R, W>(reader: &'a  mut R, writer: &'a  mut W) -> io::Result<u64>
```



