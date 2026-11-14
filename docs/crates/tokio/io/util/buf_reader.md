**tokio > io > util > buf_reader**

# Module: io::util::buf_reader

## Contents

**Structs**

- [`BufReader`](#bufreader) - The `BufReader` struct adds buffering to any reader.

---

## tokio::io::util::buf_reader::BufReader

*Struct*

The `BufReader` struct adds buffering to any reader.

It can be excessively inefficient to work directly with a [`AsyncRead`]
instance. A `BufReader` performs large, infrequent reads on the underlying
[`AsyncRead`] and maintains an in-memory buffer of the results.

`BufReader` can improve the speed of programs that make *small* and
*repeated* read calls to the same file or network socket. It does not
help when reading very large amounts at once, or reading just one or a few
times. It also provides no advantage when reading from a source that is
already in memory, like a `Vec<u8>`.

When the `BufReader` is dropped, the contents of its buffer will be
discarded. Creating multiple instances of a `BufReader` on the same
stream can cause data loss.

**Generic Parameters:**
- R

**Methods:**

- `fn new(inner: R) -> Self` - Creates a new `BufReader` with a default buffer capacity. The default is currently 8 KB,
- `fn with_capacity(capacity: usize, inner: R) -> Self` - Creates a new `BufReader` with the specified buffer capacity.
- `fn get_ref(self: &Self) -> &R` - Gets a reference to the underlying reader.
- `fn get_mut(self: & mut Self) -> & mut R` - Gets a mutable reference to the underlying reader.
- `fn get_pin_mut(self: Pin<& mut Self>) -> Pin<& mut R>` - Gets a pinned mutable reference to the underlying reader.
- `fn into_inner(self: Self) -> R` - Consumes this `BufReader`, returning the underlying reader.
- `fn buffer(self: &Self) -> &[u8]` - Returns a reference to the internally buffered data.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **AsyncSeek**
  - `fn start_seek(self: Pin<& mut Self>, pos: SeekFrom) -> io::Result<()>`
  - `fn poll_complete(self: Pin<& mut Self>, cx: & mut Context) -> Poll<io::Result<u64>>`
- **AsyncWrite**
  - `fn poll_write(self: Pin<& mut Self>, cx: & mut Context, buf: &[u8]) -> Poll<io::Result<usize>>`
  - `fn poll_write_vectored(self: Pin<& mut Self>, cx: & mut Context, bufs: &[IoSlice]) -> Poll<io::Result<usize>>`
  - `fn is_write_vectored(self: &Self) -> bool`
  - `fn poll_flush(self: Pin<& mut Self>, cx: & mut Context) -> Poll<io::Result<()>>`
  - `fn poll_shutdown(self: Pin<& mut Self>, cx: & mut Context) -> Poll<io::Result<()>>`
- **AsyncRead**
  - `fn poll_read(self: Pin<& mut Self>, cx: & mut Context, buf: & mut ReadBuf) -> Poll<io::Result<()>>`
- **AsyncBufRead**
  - `fn poll_fill_buf(self: Pin<& mut Self>, cx: & mut Context) -> Poll<io::Result<&[u8]>>`
  - `fn consume(self: Pin<& mut Self>, amt: usize)`



