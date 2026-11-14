**bytes > bytes_mut > BytesMut**

# Module: bytes_mut::BytesMut

## Contents

**Functions**

- [`set_len`](#set_len) - Sets the length of the buffer.
- [`split_off`](#split_off) - Splits the bytes into two at the given index.

---

## bytes::bytes_mut::BytesMut::set_len

*Function*

Sets the length of the buffer.

This will explicitly set the size of the buffer without actually
modifying the data, so it is up to the caller to ensure that the data
has been initialized.

# Examples

```
use bytes::BytesMut;

let mut b = BytesMut::from(&b"hello world"[..]);

unsafe {
    b.set_len(5);
}

assert_eq!(&b[..], b"hello");

unsafe {
    b.set_len(11);
}

assert_eq!(&b[..], b"hello world");
```

```rust
fn set_len(self: & mut Self, len: usize)
```



## bytes::bytes_mut::BytesMut::split_off

*Function*

Splits the bytes into two at the given index.

Afterwards `self` contains elements `[0, at)`, and the returned
`BytesMut` contains elements `[at, capacity)`. It's guaranteed that the
memory does not move, that is, the address of `self` does not change,
and the address of the returned slice is `at` bytes after that.

This is an `O(1)` operation that just increases the reference count
and sets a few indices.

# Examples

```
use bytes::BytesMut;

let mut a = BytesMut::from(&b"hello world"[..]);
let mut b = a.split_off(5);

a[0] = b'j';
b[0] = b'!';

assert_eq!(&a[..], b"jello");
assert_eq!(&b[..], b"!world");
```

# Panics

Panics if `at > capacity`.

```rust
fn split_off(self: & mut Self, at: usize) -> BytesMut
```



