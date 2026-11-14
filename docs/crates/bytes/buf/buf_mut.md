**bytes > buf > buf_mut**

# Module: buf::buf_mut

## Contents

**Traits**

- [`BufMut`](#bufmut) - A trait for values that provide sequential write access to bytes.

---

## bytes::buf::buf_mut::BufMut

*Trait*

A trait for values that provide sequential write access to bytes.

Write bytes to a buffer

A buffer stores bytes in memory such that write operations are infallible.
The underlying storage may or may not be in contiguous memory. A `BufMut`
value is a cursor into the buffer. Writing to `BufMut` advances the cursor
position.

The simplest `BufMut` is a `Vec<u8>`.

```
use bytes::BufMut;

let mut buf = vec![];

buf.put(&b"hello world"[..]);

assert_eq!(buf, b"hello world");
```

**Methods:**

- `remaining_mut`: Returns the number of bytes that can be written from the current
- `advance_mut`: Advance the internal cursor of the BufMut
- `has_remaining_mut`: Returns true if there is space in `self` for more bytes.
- `chunk_mut`: Returns a mutable slice starting at the current BufMut position and of
- `put`: Transfer bytes into `self` from `src` and advance the cursor by the
- `put_slice`: Transfer bytes into `self` from `src` and advance the cursor by the
- `put_bytes`: Put `cnt` bytes `val` into `self`.
- `put_u8`: Writes an unsigned 8 bit integer to `self`.
- `put_i8`: Writes a signed 8 bit integer to `self`.
- `put_u16`: Writes an unsigned 16 bit integer to `self` in big-endian byte order.
- `put_u16_le`: Writes an unsigned 16 bit integer to `self` in little-endian byte order.
- `put_u16_ne`: Writes an unsigned 16 bit integer to `self` in native-endian byte order.
- `put_i16`: Writes a signed 16 bit integer to `self` in big-endian byte order.
- `put_i16_le`: Writes a signed 16 bit integer to `self` in little-endian byte order.
- `put_i16_ne`: Writes a signed 16 bit integer to `self` in native-endian byte order.
- `put_u32`: Writes an unsigned 32 bit integer to `self` in big-endian byte order.
- `put_u32_le`: Writes an unsigned 32 bit integer to `self` in little-endian byte order.
- `put_u32_ne`: Writes an unsigned 32 bit integer to `self` in native-endian byte order.
- `put_i32`: Writes a signed 32 bit integer to `self` in big-endian byte order.
- `put_i32_le`: Writes a signed 32 bit integer to `self` in little-endian byte order.
- `put_i32_ne`: Writes a signed 32 bit integer to `self` in native-endian byte order.
- `put_u64`: Writes an unsigned 64 bit integer to `self` in the big-endian byte order.
- `put_u64_le`: Writes an unsigned 64 bit integer to `self` in little-endian byte order.
- `put_u64_ne`: Writes an unsigned 64 bit integer to `self` in native-endian byte order.
- `put_i64`: Writes a signed 64 bit integer to `self` in the big-endian byte order.
- `put_i64_le`: Writes a signed 64 bit integer to `self` in little-endian byte order.
- `put_i64_ne`: Writes a signed 64 bit integer to `self` in native-endian byte order.
- `put_u128`: Writes an unsigned 128 bit integer to `self` in the big-endian byte order.
- `put_u128_le`: Writes an unsigned 128 bit integer to `self` in little-endian byte order.
- `put_u128_ne`: Writes an unsigned 128 bit integer to `self` in native-endian byte order.
- `put_i128`: Writes a signed 128 bit integer to `self` in the big-endian byte order.
- `put_i128_le`: Writes a signed 128 bit integer to `self` in little-endian byte order.
- `put_i128_ne`: Writes a signed 128 bit integer to `self` in native-endian byte order.
- `put_uint`: Writes an unsigned n-byte integer to `self` in big-endian byte order.
- `put_uint_le`: Writes an unsigned n-byte integer to `self` in the little-endian byte order.
- `put_uint_ne`: Writes an unsigned n-byte integer to `self` in the native-endian byte order.
- `put_int`: Writes low `nbytes` of a signed integer to `self` in big-endian byte order.
- `put_int_le`: Writes low `nbytes` of a signed integer to `self` in little-endian byte order.
- `put_int_ne`: Writes low `nbytes` of a signed integer to `self` in native-endian byte order.
- `put_f32`: Writes an IEEE754 single-precision (4 bytes) floating point number to
- `put_f32_le`: Writes an IEEE754 single-precision (4 bytes) floating point number to
- `put_f32_ne`: Writes an IEEE754 single-precision (4 bytes) floating point number to
- `put_f64`: Writes an IEEE754 double-precision (8 bytes) floating point number to
- `put_f64_le`: Writes an IEEE754 double-precision (8 bytes) floating point number to
- `put_f64_ne`: Writes an IEEE754 double-precision (8 bytes) floating point number to
- `limit`: Creates an adaptor which can write at most `limit` bytes to `self`.
- `writer`: Creates an adaptor which implements the `Write` trait for `self`.
- `chain_mut`: Creates an adapter which will chain this buffer with another.



