**tokio > io > util > copy_buf**

# Module: io::util::copy_buf

## Contents

**Functions**

- [`copy_buf`](#copy_buf) - Asynchronously copies the entire contents of a reader into a writer.

---

## tokio::io::util::copy_buf::copy_buf

*Function*

Asynchronously copies the entire contents of a reader into a writer.

This function returns a future that will continuously read data from
`reader` and then write it into `writer` in a streaming fashion until
`reader` returns EOF or fails.

On success, the total number of bytes that were copied from `reader` to
`writer` is returned.

This is a [`tokio::io::copy`] alternative for [`AsyncBufRead`] readers
with no extra buffer allocation, since [`AsyncBufRead`] allow access
to the reader's inner buffer.

# When to use async alternatives instead of `SyncIoBridge`

If you are looking to use [`std::io::copy`] with a synchronous consumer
(like a `hasher` or compressor), consider using async alternatives instead of
wrapping the reader with [`SyncIoBridge`]. See the [`SyncIoBridge`]
documentation for detailed examples and guidance on hashing, compression,
and data parsing.

[`tokio::io::copy`]: crate::io::copy
[`AsyncBufRead`]: crate::io::AsyncBufRead
[`SyncIoBridge`]: https://docs.rs/tokio-util/latest/tokio_util/io/struct.SyncIoBridge.html

# Errors

Errors from `poll_fill_buf` and `poll_write` are returned immediately,
except [`io::ErrorKind::Interrupted`], which is retried. Errors from
flushing the writer are returned without retrying.

# Examples

```
use tokio::io;

# async fn dox() -> std::io::Result<()> {
let mut reader: &[u8] = b"hello";
let mut writer: Vec<u8> = vec![];

io::copy_buf(&mut reader, &mut writer).await?;

assert_eq!(b"hello", &writer[..]);
# Ok(())
# }
```

```rust
fn copy_buf<'a, R, W>(reader: &'a  mut R, writer: &'a  mut W) -> io::Result<u64>
```



