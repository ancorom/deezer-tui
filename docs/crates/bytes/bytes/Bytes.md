**bytes > bytes > Bytes**

# Module: bytes::Bytes

## Contents

**Functions**

- [`from_owner`](#from_owner) - Create [Bytes] with a buffer whose lifetime is controlled
- [`from_static`](#from_static) - Creates a new `Bytes` from a static slice.
- [`split_off`](#split_off) - Splits the bytes into two at the given index.

---

## bytes::bytes::Bytes::from_owner

*Function*

Create [Bytes] with a buffer whose lifetime is controlled
via an explicit owner.

A common use case is to zero-copy construct from mapped memory.

```
# struct File;
#
# impl File {
#     pub fn open(_: &str) -> Result<Self, ()> {
#         Ok(Self)
#     }
# }
#
# mod memmap2 {
#     pub struct Mmap;
#
#     impl Mmap {
#         pub unsafe fn map(_file: &super::File) -> Result<Self, ()> {
#             Ok(Self)
#         }
#     }
#
#     impl AsRef<[u8]> for Mmap {
#         fn as_ref(&self) -> &[u8] {
#             b"buf"
#         }
#     }
# }
use bytes::Bytes;
use memmap2::Mmap;

# fn main() -> Result<(), ()> {
let file = File::open("upload_bundle.tar.gz")?;
let mmap = unsafe { Mmap::map(&file) }?;
let b = Bytes::from_owner(mmap);
# Ok(())
# }
```

The `owner` will be transferred to the constructed [Bytes] object, which
will ensure it is dropped once all remaining clones of the constructed
object are dropped. The owner will then be responsible for dropping the
specified region of memory as part of its [Drop] implementation.

Note that converting [Bytes] constructed from an owner into a [BytesMut]
will always create a deep copy of the buffer into newly allocated memory.

```rust
fn from_owner<T>(owner: T) -> Self
```



## bytes::bytes::Bytes::from_static

*Function*

Creates a new `Bytes` from a static slice.

The returned `Bytes` will point directly to the static slice. There is
no allocating or copying.

# Examples

```
use bytes::Bytes;

let b = Bytes::from_static(b"hello");
assert_eq!(&b[..], b"hello");
```

```rust
fn from_static(bytes: &'static [u8]) -> Self
```



## bytes::bytes::Bytes::split_off

*Function*

Splits the bytes into two at the given index.

Afterwards `self` contains elements `[0, at)`, and the returned `Bytes`
contains elements `[at, len)`. It's guaranteed that the memory does not
move, that is, the address of `self` does not change, and the address of
the returned slice is `at` bytes after that.

This is an `O(1)` operation that just increases the reference count and
sets a few indices.

# Examples

```
use bytes::Bytes;

let mut a = Bytes::from(&b"hello world"[..]);
let b = a.split_off(5);

assert_eq!(&a[..], b"hello");
assert_eq!(&b[..], b" world");
```

# Panics

Panics if `at > len`.

```rust
fn split_off(self: & mut Self, at: usize) -> Self
```



