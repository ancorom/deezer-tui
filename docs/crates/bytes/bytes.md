**bytes > bytes**

# Module: bytes

## Contents

**Structs**

- [`Bytes`](#bytes) - A cheaply cloneable and sliceable chunk of contiguous memory.

---

## bytes::bytes::Bytes

*Struct*

A cheaply cloneable and sliceable chunk of contiguous memory.

`Bytes` is an efficient container for storing and operating on contiguous
slices of memory. It is intended for use primarily in networking code, but
could have applications elsewhere as well.

`Bytes` values facilitate zero-copy network programming by allowing multiple
`Bytes` objects to point to the same underlying memory.

`Bytes` does not have a single implementation. It is an interface, whose
exact behavior is implemented through dynamic dispatch in several underlying
implementations of `Bytes`.

All `Bytes` implementations must fulfill the following requirements:
- They are cheaply cloneable and thereby shareable between an unlimited amount
  of components, for example by modifying a reference count.
- Instances can be sliced to refer to a subset of the original buffer.

```
use bytes::Bytes;

let mut mem = Bytes::from("Hello world");
let a = mem.slice(0..5);

assert_eq!(a, "Hello");

let b = mem.split_to(6);

assert_eq!(mem, "world");
assert_eq!(b, "Hello ");
```

# Memory layout

The `Bytes` struct itself is fairly small, limited to 4 `usize` fields used
to track information about which segment of the underlying memory the
`Bytes` handle has access to.

`Bytes` keeps both a pointer to the shared state containing the full memory
slice and a pointer to the start of the region visible by the handle.
`Bytes` also tracks the length of its view into the memory.

# Sharing

`Bytes` contains a vtable, which allows implementations of `Bytes` to define
how sharing/cloning is implemented in detail.
When `Bytes::clone()` is called, `Bytes` will call the vtable function for
cloning the backing storage in order to share it behind multiple `Bytes`
instances.

For `Bytes` implementations which refer to constant memory (e.g. created
via `Bytes::from_static()`) the cloning implementation will be a no-op.

For `Bytes` implementations which point to a reference counted shared storage
(e.g. an `Arc<[u8]>`), sharing will be implemented by increasing the
reference count.

Due to this mechanism, multiple `Bytes` instances may point to the same
shared memory region.
Each `Bytes` instance can point to different sections within that
memory region, and `Bytes` instances may or may not have overlapping views
into the memory.

The following diagram visualizes a scenario where 2 `Bytes` instances make
use of an `Arc`-based backing storage, and provide access to different views:

```text

   Arc ptrs                   ┌─────────┐
   ________________________ / │ Bytes 2 │
  /                           └─────────┘
 /          ┌───────────┐     |         |
|_________/ │  Bytes 1  │     |         |
|           └───────────┘     |         |
|           |           | ___/ data     | tail
|      data |      tail |/              |
v           v           v               v
┌─────┬─────┬───────────┬───────────────┬─────┐
│ Arc │     │           │               │     │
└─────┴─────┴───────────┴───────────────┴─────┘
```

**Methods:**

- `fn new() -> Self` - Creates a new empty `Bytes`.
- `fn from_static(bytes: &'static [u8]) -> Self` - Creates a new `Bytes` from a static slice.
- `fn from_owner<T>(owner: T) -> Self` - Create [Bytes] with a buffer whose lifetime is controlled
- `fn len(self: &Self) -> usize` - Returns the number of bytes contained in this `Bytes`.
- `fn is_empty(self: &Self) -> bool` - Returns true if the `Bytes` has a length of 0.
- `fn is_unique(self: &Self) -> bool` - Returns true if this is the only reference to the data and
- `fn copy_from_slice(data: &[u8]) -> Self` - Creates `Bytes` instance from slice, by copying it.
- `fn slice<impl RangeBounds<usize>>(self: &Self, range: impl Trait) -> Self` - Returns a slice of self for the provided range.
- `fn slice_ref(self: &Self, subset: &[u8]) -> Self` - Returns a slice of self that is equivalent to the given `subset`.
- `fn split_off(self: & mut Self, at: usize) -> Self` - Splits the bytes into two at the given index.
- `fn split_to(self: & mut Self, at: usize) -> Self` - Splits the bytes into two at the given index.
- `fn truncate(self: & mut Self, len: usize)` - Shortens the buffer, keeping the first `len` bytes and dropping the
- `fn clear(self: & mut Self)` - Clears the buffer, removing all data.
- `fn try_into_mut(self: Self) -> Result<BytesMut, Bytes>` - Try to convert self into `BytesMut`.

**Traits:** Sync, Eq, Send

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &String) -> bool`
- **Buf**
  - `fn remaining(self: &Self) -> usize`
  - `fn chunk(self: &Self) -> &[u8]`
  - `fn advance(self: & mut Self, cnt: usize)`
  - `fn copy_to_bytes(self: & mut Self, len: usize) -> Self`
- **From**
  - `fn from(slice: &'static [u8]) -> Bytes`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut Formatter) -> Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &BytesMut) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Bytes`
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`
- **Borrow**
  - `fn borrow(self: &Self) -> &[u8]`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &&'a T) -> Option<cmp::Ordering>`
- **Deref**
  - `fn deref(self: &Self) -> &[u8]`
- **PartialEq**
  - `fn eq(self: &Self, other: &Vec<u8>) -> bool`
- **From**
  - `fn from(src: BytesMut) -> Bytes`
- **FromIterator**
  - `fn from_iter<T>(into_iter: T) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &str) -> bool`
- **From**
  - `fn from(s: String) -> Bytes`
- **From**
  - `fn from(slice: Box<[u8]>) -> Bytes`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &String) -> Option<cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &[u8]) -> bool`
- **From**
  - `fn from(slice: &'static str) -> Bytes`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut Formatter) -> Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Bytes) -> Option<cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Bytes) -> bool`
- **Drop**
  - `fn drop(self: & mut Self)`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut Formatter) -> Result`
- **Ord**
  - `fn cmp(self: &Self, other: &Bytes) -> cmp::Ordering`
- **Default**
  - `fn default() -> Bytes`
- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8]`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Vec<u8>) -> Option<cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &&'a T) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &str) -> Option<cmp::Ordering>`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &[u8]) -> Option<cmp::Ordering>`
- **From**
  - `fn from(vec: Vec<u8>) -> Bytes`



