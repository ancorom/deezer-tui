**bytes > buf > buf_impl**

# Module: buf::buf_impl

## Contents

**Traits**

- [`Buf`](#buf) - Read bytes from a buffer.

---

## bytes::buf::buf_impl::Buf

*Trait*

Read bytes from a buffer.

A buffer stores bytes in memory such that read operations are infallible.
The underlying storage may or may not be in contiguous memory. A `Buf` value
is a cursor into the buffer. Reading from `Buf` advances the cursor
position. It can be thought of as an efficient `Iterator` for collections of
bytes.

The simplest `Buf` is a `&[u8]`.

```
use bytes::Buf;

let mut buf = &b"hello world"[..];

assert_eq!(b'h', buf.get_u8());
assert_eq!(b'e', buf.get_u8());
assert_eq!(b'l', buf.get_u8());

let mut rest = [0; 8];
buf.copy_to_slice(&mut rest);

assert_eq!(&rest[..], &b"lo world"[..]);
```

**Methods:**

- `remaining`: Returns the number of bytes between the current position and the end of
- `chunk`: Returns a slice starting at the current position and of length between 0
- `chunks_vectored`: Fills `dst` with potentially multiple slices starting at `self`'s
- `advance`: Advance the internal cursor of the Buf
- `has_remaining`: Returns true if there are any more bytes to consume
- `copy_to_slice`: Copies bytes from `self` into `dst`.
- `get_u8`: Gets an unsigned 8 bit integer from `self`.
- `get_i8`: Gets a signed 8 bit integer from `self`.
- `get_u16`: Gets an unsigned 16 bit integer from `self` in big-endian byte order.
- `get_u16_le`: Gets an unsigned 16 bit integer from `self` in little-endian byte order.
- `get_u16_ne`: Gets an unsigned 16 bit integer from `self` in native-endian byte order.
- `get_i16`: Gets a signed 16 bit integer from `self` in big-endian byte order.
- `get_i16_le`: Gets a signed 16 bit integer from `self` in little-endian byte order.
- `get_i16_ne`: Gets a signed 16 bit integer from `self` in native-endian byte order.
- `get_u32`: Gets an unsigned 32 bit integer from `self` in the big-endian byte order.
- `get_u32_le`: Gets an unsigned 32 bit integer from `self` in the little-endian byte order.
- `get_u32_ne`: Gets an unsigned 32 bit integer from `self` in native-endian byte order.
- `get_i32`: Gets a signed 32 bit integer from `self` in big-endian byte order.
- `get_i32_le`: Gets a signed 32 bit integer from `self` in little-endian byte order.
- `get_i32_ne`: Gets a signed 32 bit integer from `self` in native-endian byte order.
- `get_u64`: Gets an unsigned 64 bit integer from `self` in big-endian byte order.
- `get_u64_le`: Gets an unsigned 64 bit integer from `self` in little-endian byte order.
- `get_u64_ne`: Gets an unsigned 64 bit integer from `self` in native-endian byte order.
- `get_i64`: Gets a signed 64 bit integer from `self` in big-endian byte order.
- `get_i64_le`: Gets a signed 64 bit integer from `self` in little-endian byte order.
- `get_i64_ne`: Gets a signed 64 bit integer from `self` in native-endian byte order.
- `get_u128`: Gets an unsigned 128 bit integer from `self` in big-endian byte order.
- `get_u128_le`: Gets an unsigned 128 bit integer from `self` in little-endian byte order.
- `get_u128_ne`: Gets an unsigned 128 bit integer from `self` in native-endian byte order.
- `get_i128`: Gets a signed 128 bit integer from `self` in big-endian byte order.
- `get_i128_le`: Gets a signed 128 bit integer from `self` in little-endian byte order.
- `get_i128_ne`: Gets a signed 128 bit integer from `self` in native-endian byte order.
- `get_uint`: Gets an unsigned n-byte integer from `self` in big-endian byte order.
- `get_uint_le`: Gets an unsigned n-byte integer from `self` in little-endian byte order.
- `get_uint_ne`: Gets an unsigned n-byte integer from `self` in native-endian byte order.
- `get_int`: Gets a signed n-byte integer from `self` in big-endian byte order.
- `get_int_le`: Gets a signed n-byte integer from `self` in little-endian byte order.
- `get_int_ne`: Gets a signed n-byte integer from `self` in native-endian byte order.
- `get_f32`: Gets an IEEE754 single-precision (4 bytes) floating point number from
- `get_f32_le`: Gets an IEEE754 single-precision (4 bytes) floating point number from
- `get_f32_ne`: Gets an IEEE754 single-precision (4 bytes) floating point number from
- `get_f64`: Gets an IEEE754 double-precision (8 bytes) floating point number from
- `get_f64_le`: Gets an IEEE754 double-precision (8 bytes) floating point number from
- `get_f64_ne`: Gets an IEEE754 double-precision (8 bytes) floating point number from
- `try_copy_to_slice`: Copies bytes from `self` into `dst`.
- `try_get_u8`: Gets an unsigned 8 bit integer from `self`.
- `try_get_i8`: Gets a signed 8 bit integer from `self`.
- `try_get_u16`: Gets an unsigned 16 bit integer from `self` in big-endian byte order.
- `try_get_u16_le`: Gets an unsigned 16 bit integer from `self` in little-endian byte order.
- `try_get_u16_ne`: Gets an unsigned 16 bit integer from `self` in native-endian byte order.
- `try_get_i16`: Gets a signed 16 bit integer from `self` in big-endian byte order.
- `try_get_i16_le`: Gets an signed 16 bit integer from `self` in little-endian byte order.
- `try_get_i16_ne`: Gets a signed 16 bit integer from `self` in native-endian byte order.
- `try_get_u32`: Gets an unsigned 32 bit integer from `self` in big-endian byte order.
- `try_get_u32_le`: Gets an unsigned 32 bit integer from `self` in little-endian byte order.
- `try_get_u32_ne`: Gets an unsigned 32 bit integer from `self` in native-endian byte order.
- `try_get_i32`: Gets a signed 32 bit integer from `self` in big-endian byte order.
- `try_get_i32_le`: Gets a signed 32 bit integer from `self` in little-endian byte order.
- `try_get_i32_ne`: Gets a signed 32 bit integer from `self` in native-endian byte order.
- `try_get_u64`: Gets an unsigned 64 bit integer from `self` in big-endian byte order.
- `try_get_u64_le`: Gets an unsigned 64 bit integer from `self` in little-endian byte order.
- `try_get_u64_ne`: Gets an unsigned 64 bit integer from `self` in native-endian byte order.
- `try_get_i64`: Gets a signed 64 bit integer from `self` in big-endian byte order.
- `try_get_i64_le`: Gets a signed 64 bit integer from `self` in little-endian byte order.
- `try_get_i64_ne`: Gets a signed 64 bit integer from `self` in native-endian byte order.
- `try_get_u128`: Gets an unsigned 128 bit integer from `self` in big-endian byte order.
- `try_get_u128_le`: Gets an unsigned 128 bit integer from `self` in little-endian byte order.
- `try_get_u128_ne`: Gets an unsigned 128 bit integer from `self` in native-endian byte order.
- `try_get_i128`: Gets a signed 128 bit integer from `self` in big-endian byte order.
- `try_get_i128_le`: Gets a signed 128 bit integer from `self` in little-endian byte order.
- `try_get_i128_ne`: Gets a signed 128 bit integer from `self` in native-endian byte order.
- `try_get_uint`: Gets an unsigned n-byte integer from `self` in big-endian byte order.
- `try_get_uint_le`: Gets an unsigned n-byte integer from `self` in little-endian byte order.
- `try_get_uint_ne`: Gets an unsigned n-byte integer from `self` in native-endian byte order.
- `try_get_int`: Gets a signed n-byte integer from `self` in big-endian byte order.
- `try_get_int_le`: Gets a signed n-byte integer from `self` in little-endian byte order.
- `try_get_int_ne`: Gets a signed n-byte integer from `self` in native-endian byte order.
- `try_get_f32`: Gets an IEEE754 single-precision (4 bytes) floating point number from
- `try_get_f32_le`: Gets an IEEE754 single-precision (4 bytes) floating point number from
- `try_get_f32_ne`: Gets an IEEE754 single-precision (4 bytes) floating point number from
- `try_get_f64`: Gets an IEEE754 double-precision (8 bytes) floating point number from
- `try_get_f64_le`: Gets an IEEE754 double-precision (8 bytes) floating point number from
- `try_get_f64_ne`: Gets an IEEE754 double-precision (8 bytes) floating point number from
- `copy_to_bytes`: Consumes `len` bytes inside self and returns new instance of `Bytes`
- `take`: Creates an adaptor which will read at most `limit` bytes from `self`.
- `chain`: Creates an adaptor which will chain this buffer with another.
- `reader`: Creates an adaptor which implements the `Read` trait for `self`.



