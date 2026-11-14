# bytes

Provides abstractions for working with bytes.

The `bytes` crate provides an efficient byte buffer structure
([`Bytes`]) and traits for working with buffer
implementations ([`Buf`], [`BufMut`]).

# `Bytes`

`Bytes` is an efficient container for storing and operating on contiguous
slices of memory. It is intended for use primarily in networking code, but
could have applications elsewhere as well.

`Bytes` values facilitate zero-copy network programming by allowing multiple
`Bytes` objects to point to the same underlying memory. This is managed by
using a reference count to track when the memory is no longer needed and can
be freed.

A `Bytes` handle can be created directly from an existing byte store (such as `&[u8]`
or `Vec<u8>`), but usually a `BytesMut` is used first and written to. For
example:

```rust
use bytes::{BytesMut, BufMut};

let mut buf = BytesMut::with_capacity(1024);
buf.put(&b"hello world"[..]);
buf.put_u16(1234);

let a = buf.split();
assert_eq!(a, b"hello world\x04\xD2"[..]);

buf.put(&b"goodbye world"[..]);

let b = buf.split();
assert_eq!(b, b"goodbye world"[..]);

assert_eq!(buf.capacity(), 998);
```

In the above example, only a single buffer of 1024 is allocated. The handles
`a` and `b` will share the underlying buffer and maintain indices tracking
the view into the buffer represented by the handle.

See the [struct docs](`Bytes`) for more details.

# `Buf`, `BufMut`

These two traits provide read and write access to buffers. The underlying
storage may or may not be in contiguous memory. For example, `Bytes` is a
buffer that guarantees contiguous memory, but a [rope] stores the bytes in
disjoint chunks. `Buf` and `BufMut` maintain cursors tracking the current
position in the underlying byte storage. When bytes are read or written, the
cursor is advanced.

[rope]: https://en.wikipedia.org/wiki/Rope_(data_structure)

## Relation with `Read` and `Write`

At first glance, it may seem that `Buf` and `BufMut` overlap in
functionality with [`std::io::Read`] and [`std::io::Write`]. However, they
serve different purposes. A buffer is the value that is provided as an
argument to `Read::read` and `Write::write`. `Read` and `Write` may then
perform a syscall, which has the potential of failing. Operations on `Buf`
and `BufMut` are infallible.

## Modules

### [`bytes`](bytes.md)

*1 module, 1 struct*

### [`buf::buf_impl`](buf/buf_impl.md)

*1 trait*

### [`buf::buf_mut`](buf/buf_mut.md)

*1 trait*

### [`buf::chain`](buf/chain.md)

*1 struct*

### [`buf::iter`](buf/iter.md)

*1 struct*

### [`buf::limit`](buf/limit.md)

*1 struct*

### [`buf::reader`](buf/reader.md)

*1 struct*

### [`buf::take`](buf/take.md)

*1 struct*

### [`buf::uninit_slice`](buf/uninit_slice.md)

*1 struct*

### [`buf::writer`](buf/writer.md)

*1 struct*

### [`bytes`](bytes.md)

*1 struct*

### [`bytes::Bytes`](bytes/Bytes.md)

*3 functions*

### [`bytes_mut`](bytes_mut.md)

*1 struct*

### [`bytes_mut::BytesMut`](bytes_mut/BytesMut.md)

*2 functions*

