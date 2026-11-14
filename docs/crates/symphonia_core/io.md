**symphonia_core > io**

# Module: io

## Contents

**Structs**

- [`ReadOnlySource`](#readonlysource) - `ReadOnlySource` wraps any source implementing [`std::io::Read`] in an unseekable

**Traits**

- [`FiniteStream`](#finitestream) - A `FiniteStream` is a stream that has a known length in bytes.
- [`MediaSource`](#mediasource) - `MediaSource` is a composite trait of [`std::io::Read`] and [`std::io::Seek`]. A source *must*
- [`ReadBytes`](#readbytes) - `ReadBytes` provides methods to read bytes and interpret them as little- or big-endian
- [`SeekBuffered`](#seekbuffered) - `SeekBuffered` provides methods to seek within the buffered portion of a stream.

---

## symphonia_core::io::FiniteStream

*Trait*

A `FiniteStream` is a stream that has a known length in bytes.

**Methods:**

- `byte_len`: Returns the length of the the stream in bytes.
- `bytes_read`: Returns the number of bytes that have been read.
- `bytes_available`: Returns the number of bytes available for reading.



## symphonia_core::io::MediaSource

*Trait*

`MediaSource` is a composite trait of [`std::io::Read`] and [`std::io::Seek`]. A source *must*
implement this trait to be used by [`MediaSourceStream`].

Despite requiring the [`std::io::Seek`] trait, seeking is an optional capability that can be
queried at runtime.

**Methods:**

- `is_seekable`: Returns if the source is seekable. This may be an expensive operation.
- `byte_len`: Returns the length in bytes, if available. This may be an expensive operation.



## symphonia_core::io::ReadBytes

*Trait*

`ReadBytes` provides methods to read bytes and interpret them as little- or big-endian
unsigned integers or floating-point values of standard widths.

**Methods:**

- `read_byte`: Reads a single byte from the stream and returns it or an error.
- `read_double_bytes`: Reads two bytes from the stream and returns them in read-order or an error.
- `read_triple_bytes`: Reads three bytes from the stream and returns them in read-order or an error.
- `read_quad_bytes`: Reads four bytes from the stream and returns them in read-order or an error.
- `read_buf`: Reads up-to the number of bytes required to fill buf or returns an error.
- `read_buf_exact`: Reads exactly the number of bytes required to fill be provided buffer or returns an error.
- `read_u8`: Reads a single unsigned byte from the stream and returns it or an error.
- `read_i8`: Reads a single signed byte from the stream and returns it or an error.
- `read_u16`: Reads two bytes from the stream and interprets them as an unsigned 16-bit little-endian
- `read_i16`: Reads two bytes from the stream and interprets them as an signed 16-bit little-endian
- `read_be_u16`: Reads two bytes from the stream and interprets them as an unsigned 16-bit big-endian
- `read_be_i16`: Reads two bytes from the stream and interprets them as an signed 16-bit big-endian
- `read_u24`: Reads three bytes from the stream and interprets them as an unsigned 24-bit little-endian
- `read_i24`: Reads three bytes from the stream and interprets them as an signed 24-bit little-endian
- `read_be_u24`: Reads three bytes from the stream and interprets them as an unsigned 24-bit big-endian
- `read_be_i24`: Reads three bytes from the stream and interprets them as an signed 24-bit big-endian
- `read_u32`: Reads four bytes from the stream and interprets them as an unsigned 32-bit little-endian
- `read_i32`: Reads four bytes from the stream and interprets them as an signed 32-bit little-endian
- `read_be_u32`: Reads four bytes from the stream and interprets them as an unsigned 32-bit big-endian
- `read_be_i32`: Reads four bytes from the stream and interprets them as a signed 32-bit big-endian
- `read_u64`: Reads eight bytes from the stream and interprets them as an unsigned 64-bit little-endian
- `read_i64`: Reads eight bytes from the stream and interprets them as an signed 64-bit little-endian
- `read_be_u64`: Reads eight bytes from the stream and interprets them as an unsigned 64-bit big-endian
- `read_be_i64`: Reads eight bytes from the stream and interprets them as an signed 64-bit big-endian
- `read_f32`: Reads four bytes from the stream and interprets them as a 32-bit little-endian IEEE-754
- `read_be_f32`: Reads four bytes from the stream and interprets them as a 32-bit big-endian IEEE-754
- `read_f64`: Reads four bytes from the stream and interprets them as a 64-bit little-endian IEEE-754
- `read_be_f64`: Reads four bytes from the stream and interprets them as a 64-bit big-endian IEEE-754
- `read_boxed_slice`: Reads up-to the number of bytes requested, and returns a boxed slice of the data or an
- `read_boxed_slice_exact`: Reads exactly the number of bytes requested, and returns a boxed slice of the data or an
- `scan_bytes`: Reads bytes from the stream into a supplied buffer until a byte pattern is matched. Returns
- `scan_bytes_aligned`: Reads bytes from a stream into a supplied buffer until a byte patter is matched on an
- `ignore_bytes`: Ignores the specified number of bytes from the stream or returns an error.
- `pos`: Gets the position of the stream.



## symphonia_core::io::ReadOnlySource

*Struct*

`ReadOnlySource` wraps any source implementing [`std::io::Read`] in an unseekable
[`MediaSource`].

**Generic Parameters:**
- R

**Methods:**

- `fn new(inner: R) -> Self` - Instantiates a new `ReadOnlySource<R>` by taking ownership and wrapping the provided
- `fn get_ref(self: &Self) -> &R` - Gets a reference to the underlying reader.
- `fn get_mut(self: & mut Self) -> & mut R` - Gets a mutable reference to the underlying reader.
- `fn into_inner(self: Self) -> R` - Unwraps this `ReadOnlySource<R>`, returning the underlying reader.

**Trait Implementations:**

- **Read**
  - `fn read(self: & mut Self, buf: & mut [u8]) -> io::Result<usize>`
- **MediaSource**
  - `fn is_seekable(self: &Self) -> bool`
  - `fn byte_len(self: &Self) -> Option<u64>`
- **Seek**
  - `fn seek(self: & mut Self, _: io::SeekFrom) -> io::Result<u64>`



## symphonia_core::io::SeekBuffered

*Trait*

`SeekBuffered` provides methods to seek within the buffered portion of a stream.

**Methods:**

- `ensure_seekback_buffer`: Ensures that `len` bytes will be available for backwards seeking if `len` bytes have been
- `unread_buffer_len`: Get the number of bytes buffered but not yet read.
- `read_buffer_len`: Gets the number of bytes buffered and read.
- `seek_buffered`: Seek within the buffered data to an absolute position in the stream. Returns the position
- `seek_buffered_rel`: Seek within the buffered data relative to the current position in the stream. Returns the
- `seek_buffered_rev`: Seek backwards within the buffered data.



