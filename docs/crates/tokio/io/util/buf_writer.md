**tokio > io > util > buf_writer**

# Module: io::util::buf_writer

## Contents

**Structs**

- [`BufWriter`](#bufwriter) - Wraps a writer and buffers its output.

---

## tokio::io::util::buf_writer::BufWriter

*Struct*

Wraps a writer and buffers its output.

It can be excessively inefficient to work directly with something that
implements [`AsyncWrite`]. A `BufWriter` keeps an in-memory buffer of data and
writes it to an underlying writer in large, infrequent batches.

`BufWriter` can improve the speed of programs that make *small* and
*repeated* write calls to the same file or network socket. It does not
help when writing very large amounts at once, or writing just one or a few
times. It also provides no advantage when writing to a destination that is
in memory, like a `Vec<u8>`.

When the `BufWriter` is dropped, the contents of its buffer will be
discarded. Creating multiple instances of a `BufWriter` on the same
stream can cause data loss. If you need to write out the contents of its
buffer, you must manually call flush before the writer is dropped.

[`AsyncWrite`]: AsyncWrite
[`flush`]: super::AsyncWriteExt::flush


**Generic Parameters:**
- W

**Methods:**

- `fn new(inner: W) -> Self` - Creates a new `BufWriter` with a default buffer capacity. The default is currently 8 KB,
- `fn with_capacity(cap: usize, inner: W) -> Self` - Creates a new `BufWriter` with the specified buffer capacity.
- `fn get_ref(self: &Self) -> &W` - Gets a reference to the underlying writer.
- `fn get_mut(self: & mut Self) -> & mut W` - Gets a mutable reference to the underlying writer.
- `fn get_pin_mut(self: Pin<& mut Self>) -> Pin<& mut W>` - Gets a pinned mutable reference to the underlying writer.
- `fn into_inner(self: Self) -> W` - Consumes this `BufWriter`, returning the underlying writer.
- `fn buffer(self: &Self) -> &[u8]` - Returns a reference to the internally buffered data.

**Trait Implementations:**

- **AsyncRead**
  - `fn poll_read(self: Pin<& mut Self>, cx: & mut Context, buf: & mut ReadBuf) -> Poll<io::Result<()>>`
- **AsyncSeek**
  - `fn start_seek(self: Pin<& mut Self>, pos: SeekFrom) -> io::Result<()>`
  - `fn poll_complete(self: Pin<& mut Self>, cx: & mut Context) -> Poll<io::Result<u64>>`
- **AsyncBufRead**
  - `fn poll_fill_buf(self: Pin<& mut Self>, cx: & mut Context) -> Poll<io::Result<&[u8]>>`
  - `fn consume(self: Pin<& mut Self>, amt: usize)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **AsyncWrite**
  - `fn poll_write(self: Pin<& mut Self>, cx: & mut Context, buf: &[u8]) -> Poll<io::Result<usize>>`
  - `fn poll_write_vectored(self: Pin<& mut Self>, cx: & mut Context, bufs: &[IoSlice]) -> Poll<io::Result<usize>>`
  - `fn is_write_vectored(self: &Self) -> bool`
  - `fn poll_flush(self: Pin<& mut Self>, cx: & mut Context) -> Poll<io::Result<()>>`
  - `fn poll_shutdown(self: Pin<& mut Self>, cx: & mut Context) -> Poll<io::Result<()>>`



