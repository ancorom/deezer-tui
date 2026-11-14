**bytes > bytes_mut**

# Module: bytes_mut

## Contents

**Structs**

- [`BytesMut`](#bytesmut) - A unique reference to a contiguous slice of memory.

---

## bytes::bytes_mut::BytesMut

*Struct*

A unique reference to a contiguous slice of memory.

`BytesMut` represents a unique view into a potentially shared memory region.
Given the uniqueness guarantee, owners of `BytesMut` handles are able to
mutate the memory.

`BytesMut` can be thought of as containing a `buf: Arc<Vec<u8>>`, an offset
into `buf`, a slice length, and a guarantee that no other `BytesMut` for the
same `buf` overlaps with its slice. That guarantee means that a write lock
is not required.

# Growth

`BytesMut`'s `BufMut` implementation will implicitly grow its buffer as
necessary. However, explicitly reserving the required space up-front before
a series of inserts will be more efficient.

# Examples

```
use bytes::{BytesMut, BufMut};

let mut buf = BytesMut::with_capacity(64);

buf.put_u8(b'h');
buf.put_u8(b'e');
buf.put(&b"llo"[..]);

assert_eq!(&buf[..], b"hello");

// Freeze the buffer so that it can be shared
let a = buf.freeze();

// This does not allocate, instead `b` points to the same memory.
let b = a.clone();

assert_eq!(&a[..], b"hello");
assert_eq!(&b[..], b"hello");
```

**Methods:**

- `fn with_capacity(capacity: usize) -> BytesMut` - Creates a new `BytesMut` with the specified capacity.
- `fn new() -> BytesMut` - Creates a new `BytesMut` with default capacity.
- `fn len(self: &Self) -> usize` - Returns the number of bytes contained in this `BytesMut`.
- `fn is_empty(self: &Self) -> bool` - Returns true if the `BytesMut` has a length of 0.
- `fn capacity(self: &Self) -> usize` - Returns the number of bytes the `BytesMut` can hold without reallocating.
- `fn freeze(self: Self) -> Bytes` - Converts `self` into an immutable `Bytes`.
- `fn zeroed(len: usize) -> BytesMut` - Creates a new `BytesMut` containing `len` zeros.
- `fn split_off(self: & mut Self, at: usize) -> BytesMut` - Splits the bytes into two at the given index.
- `fn split(self: & mut Self) -> BytesMut` - Removes the bytes from the current view, returning them in a new
- `fn split_to(self: & mut Self, at: usize) -> BytesMut` - Splits the buffer into two at the given index.
- `fn truncate(self: & mut Self, len: usize)` - Shortens the buffer, keeping the first `len` bytes and dropping the
- `fn clear(self: & mut Self)` - Clears the buffer, removing all data. Existing capacity is preserved.
- `fn resize(self: & mut Self, new_len: usize, value: u8)` - Resizes the buffer so that `len` is equal to `new_len`.
- `fn set_len(self: & mut Self, len: usize)` - Sets the length of the buffer.
- `fn reserve(self: & mut Self, additional: usize)` - Reserves capacity for at least `additional` more bytes to be inserted
- `fn try_reclaim(self: & mut Self, additional: usize) -> bool` - Attempts to cheaply reclaim already allocated capacity for at least `additional` more
- `fn extend_from_slice(self: & mut Self, extend: &[u8])` - Appends given bytes to this `BytesMut`.
- `fn extend_from_within<impl core::ops::RangeBounds<usize>>(self: & mut Self, range: impl Trait)` - Clones the elements in the given `range` within this `BytesMut` and
- `fn unsplit(self: & mut Self, other: BytesMut)` - Absorbs a `BytesMut` that was previously split off if they are
- `fn try_unsplit(self: & mut Self, other: BytesMut) -> Result<(), BytesMut>` - Absorbs a `BytesMut` that was previously split off.
- `fn spare_capacity_mut(self: & mut Self) -> & mut [MaybeUninit<u8>]` - Returns the remaining spare capacity of the buffer as a slice of `MaybeUninit<u8>`.

**Traits:** Sync, Send, Eq

**Trait Implementations:**

- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8]`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Vec<u8>) -> Option<cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &BytesMut) -> cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &BytesMut) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &str) -> Option<cmp::Ordering>`
- **From**
  - `fn from(src: &'a [u8]) -> BytesMut`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &[u8]) -> Option<cmp::Ordering>`
- **Deref**
  - `fn deref(self: &Self) -> &[u8]`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut Formatter) -> Result`
- **Borrow**
  - `fn borrow(self: &Self) -> &[u8]`
- **FromIterator**
  - `fn from_iter<T>(into_iter: T) -> Self`
- **Default**
  - `fn default() -> BytesMut`
- **Extend**
  - `fn extend<T>(self: & mut Self, iter: T)`
- **BufMut**
  - `fn remaining_mut(self: &Self) -> usize`
  - `fn advance_mut(self: & mut Self, cnt: usize)`
  - `fn chunk_mut(self: & mut Self) -> & mut UninitSlice`
  - `fn put<T>(self: & mut Self, src: T)`
  - `fn put_slice(self: & mut Self, src: &[u8])`
  - `fn put_bytes(self: & mut Self, val: u8, cnt: usize)`
- **PartialEq**
  - `fn eq(self: &Self, other: &&'a T) -> bool`
- **PartialEq**
  - `fn eq(self: &Self, other: &String) -> bool`
- **Drop**
  - `fn drop(self: & mut Self)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Vec<u8>) -> bool`
- **PartialEq**
  - `fn eq(self: &Self, other: &Bytes) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &BytesMut) -> Option<cmp::Ordering>`
- **From**
  - `fn from(src: &'a str) -> BytesMut`
- **PartialEq**
  - `fn eq(self: &Self, other: &str) -> bool`
- **Write**
  - `fn write_str(self: & mut Self, s: &str) -> fmt::Result`
  - `fn write_fmt(self: & mut Self, args: fmt::Arguments) -> fmt::Result`
- **Buf**
  - `fn remaining(self: &Self) -> usize`
  - `fn chunk(self: &Self) -> &[u8]`
  - `fn advance(self: & mut Self, cnt: usize)`
  - `fn copy_to_bytes(self: & mut Self, len: usize) -> Bytes`
- **AsMut**
  - `fn as_mut(self: & mut Self) -> & mut [u8]`
- **PartialEq**
  - `fn eq(self: &Self, other: &[u8]) -> bool`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut Formatter) -> Result`
- **BorrowMut**
  - `fn borrow_mut(self: & mut Self) -> & mut [u8]`
- **FromIterator**
  - `fn from_iter<T>(into_iter: T) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut Formatter) -> Result`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Extend**
  - `fn extend<T>(self: & mut Self, iter: T)`
- **Extend**
  - `fn extend<T>(self: & mut Self, iter: T)`
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &&'a T) -> Option<cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> BytesMut`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut [u8]`
- **From**
  - `fn from(bytes: Bytes) -> Self` - Convert self into `BytesMut`.
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &String) -> Option<cmp::Ordering>`



