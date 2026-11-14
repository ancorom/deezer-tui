**symphonia_core > io > media_source_stream**

# Module: io::media_source_stream

## Contents

**Structs**

- [`MediaSourceStream`](#mediasourcestream) - `MediaSourceStream` is the main reader type for Symphonia.
- [`MediaSourceStreamOptions`](#mediasourcestreamoptions) - `MediaSourceStreamOptions` specifies the buffering behaviour of a `MediaSourceStream`.

---

## symphonia_core::io::media_source_stream::MediaSourceStream

*Struct*

`MediaSourceStream` is the main reader type for Symphonia.

By using type erasure and dynamic dispatch, `MediaSourceStream` wraps and hides the inner
reader from the consumer, allowing any typical `Read`er to be used with Symphonia in a generic
way, selectable at runtime.

`MediaSourceStream` is designed to provide speed and flexibility in a number of challenging I/O
scenarios.

First, to minimize system call and dynamic dispatch overhead on the inner reader, and to
amortize that overhead over many bytes, `MediaSourceStream` implements an exponentially growing
read-ahead buffer. The read-ahead length starts at 1kB, and doubles in length as more sequential
reads are performed until it reaches 32kB. Growing the read-ahead length over time reduces the
excess data buffered on consecutive `seek()` calls.

Second, to better support non-seekable sources, `MediaSourceStream` implements a configurable
length buffer. By default, the buffer allows backtracking by up-to the minimum of either
`buffer_len - 32kB` or the total number of bytes read since instantiation or the last buffer
invalidation. Note that regular a `seek()` will invalidate the buffer.

**Generic Parameters:**
- 's

**Methods:**

- `fn new(source: Box<dyn MediaSource>, options: MediaSourceStreamOptions) -> Self`

**Trait Implementations:**

- **MediaSource**
  - `fn is_seekable(self: &Self) -> bool`
  - `fn byte_len(self: &Self) -> Option<u64>`
- **Seek**
  - `fn seek(self: & mut Self, pos: io::SeekFrom) -> io::Result<u64>`
- **Read**
  - `fn read(self: & mut Self, buf: & mut [u8]) -> io::Result<usize>`
- **ReadBytes**
  - `fn read_byte(self: & mut Self) -> io::Result<u8>`
  - `fn read_double_bytes(self: & mut Self) -> io::Result<[u8; 2]>`
  - `fn read_triple_bytes(self: & mut Self) -> io::Result<[u8; 3]>`
  - `fn read_quad_bytes(self: & mut Self) -> io::Result<[u8; 4]>`
  - `fn read_buf(self: & mut Self, buf: & mut [u8]) -> io::Result<usize>`
  - `fn read_buf_exact(self: & mut Self, buf: & mut [u8]) -> io::Result<()>`
  - `fn scan_bytes_aligned<'a>(self: & mut Self, _: &[u8], _: usize, _: &'a  mut [u8]) -> io::Result<&'a  mut [u8]>`
  - `fn ignore_bytes(self: & mut Self, count: u64) -> io::Result<()>`
  - `fn pos(self: &Self) -> u64`
- **SeekBuffered**
  - `fn ensure_seekback_buffer(self: & mut Self, len: usize)`
  - `fn unread_buffer_len(self: &Self) -> usize`
  - `fn read_buffer_len(self: &Self) -> usize`
  - `fn seek_buffered(self: & mut Self, pos: u64) -> u64`
  - `fn seek_buffered_rel(self: & mut Self, delta: isize) -> u64`



## symphonia_core::io::media_source_stream::MediaSourceStreamOptions

*Struct*

`MediaSourceStreamOptions` specifies the buffering behaviour of a `MediaSourceStream`.

**Fields:**
- `buffer_len: usize` - The maximum buffer size. Must be a power of 2. Must be > 32kB.

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`



