**tokio > io > util > buf_stream**

# Module: io::util::buf_stream

## Contents

**Structs**

- [`BufStream`](#bufstream) - Wraps a type that is [`AsyncWrite`] and [`AsyncRead`], and buffers its input and output.

---

## tokio::io::util::buf_stream::BufStream

*Struct*

Wraps a type that is [`AsyncWrite`] and [`AsyncRead`], and buffers its input and output.

It can be excessively inefficient to work directly with something that implements [`AsyncWrite`]
and [`AsyncRead`]. For example, every `write`, however small, has to traverse the syscall
interface, and similarly, every read has to do the same. The [`BufWriter`] and [`BufReader`]
types aid with these problems respectively, but do so in only one direction. `BufStream` wraps
one in the other so that both directions are buffered. See their documentation for details.

**Generic Parameters:**
- RW

**Methods:**

- `fn new(stream: RW) -> BufStream<RW>` - Wraps a type in both [`BufWriter`] and [`BufReader`].
- `fn with_capacity(reader_capacity: usize, writer_capacity: usize, stream: RW) -> BufStream<RW>` - Creates a `BufStream` with the specified [`BufReader`] capacity and [`BufWriter`]
- `fn get_ref(self: &Self) -> &RW` - Gets a reference to the underlying I/O object.
- `fn get_mut(self: & mut Self) -> & mut RW` - Gets a mutable reference to the underlying I/O object.
- `fn get_pin_mut(self: Pin<& mut Self>) -> Pin<& mut RW>` - Gets a pinned mutable reference to the underlying I/O object.
- `fn into_inner(self: Self) -> RW` - Consumes this `BufStream`, returning the underlying I/O object.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **AsyncSeek**
  - `fn start_seek(self: Pin<& mut Self>, position: SeekFrom) -> io::Result<()>`
  - `fn poll_complete(self: Pin<& mut Self>, cx: & mut Context) -> Poll<io::Result<u64>>`
- **AsyncWrite**
  - `fn poll_write(self: Pin<& mut Self>, cx: & mut Context, buf: &[u8]) -> Poll<io::Result<usize>>`
  - `fn poll_write_vectored(self: Pin<& mut Self>, cx: & mut Context, bufs: &[IoSlice]) -> Poll<io::Result<usize>>`
  - `fn is_write_vectored(self: &Self) -> bool`
  - `fn poll_flush(self: Pin<& mut Self>, cx: & mut Context) -> Poll<io::Result<()>>`
  - `fn poll_shutdown(self: Pin<& mut Self>, cx: & mut Context) -> Poll<io::Result<()>>`
- **From**
  - `fn from(b: BufReader<BufWriter<RW>>) -> Self`
- **AsyncBufRead**
  - `fn poll_fill_buf(self: Pin<& mut Self>, cx: & mut Context) -> Poll<io::Result<&[u8]>>`
  - `fn consume(self: Pin<& mut Self>, amt: usize)`
- **AsyncRead**
  - `fn poll_read(self: Pin<& mut Self>, cx: & mut Context, buf: & mut ReadBuf) -> Poll<io::Result<()>>`
- **From**
  - `fn from(b: BufWriter<BufReader<RW>>) -> Self`



