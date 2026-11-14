**bytes > buf > uninit_slice**

# Module: buf::uninit_slice

## Contents

**Structs**

- [`UninitSlice`](#uninitslice) - Uninitialized byte slice.

---

## bytes::buf::uninit_slice::UninitSlice

*Struct*

Uninitialized byte slice.

Returned by `BufMut::chunk_mut()`, the referenced byte slice may be
uninitialized. The wrapper provides safe access without introducing
undefined behavior.

The safety invariants of this wrapper are:

 1. Reading from an `UninitSlice` is undefined behavior.
 2. Writing uninitialized bytes to an `UninitSlice` is undefined behavior.

The difference between `&mut UninitSlice` and `&mut [MaybeUninit<u8>]` is
that it is possible in safe code to write uninitialized bytes to an
`&mut [MaybeUninit<u8>]`, which this type prohibits.

**Tuple Struct**: `()`

**Methods:**

- `fn new(slice: & mut [u8]) -> & mut UninitSlice` - Creates a `&mut UninitSlice` wrapping a slice of initialised memory.
- `fn uninit(slice: & mut [MaybeUninit<u8>]) -> & mut UninitSlice` - Creates a `&mut UninitSlice` wrapping a slice of uninitialised memory.
- `fn from_raw_parts_mut<'a>(ptr: *mut u8, len: usize) -> &'a  mut UninitSlice` - Create a `&mut UninitSlice` from a pointer and a length.
- `fn write_byte(self: & mut Self, index: usize, byte: u8)` - Write a single byte at the specified offset.
- `fn copy_from_slice(self: & mut Self, src: &[u8])` - Copies bytes from `src` into `self`.
- `fn as_mut_ptr(self: & mut Self) -> *mut u8` - Return a raw pointer to the slice's buffer.
- `fn as_uninit_slice_mut(self: & mut Self) -> & mut [MaybeUninit<u8>]` - Return a `&mut [MaybeUninit<u8>]` to this slice's buffer.
- `fn len(self: &Self) -> usize` - Returns the number of bytes in the slice.

**Trait Implementations:**

- **Index**
  - `fn index(self: &Self, index: RangeToInclusive<usize>) -> &UninitSlice`
- **IndexMut**
  - `fn index_mut(self: & mut Self, index: RangeTo<usize>) -> & mut UninitSlice`
- **Index**
  - `fn index(self: &Self, index: RangeInclusive<usize>) -> &UninitSlice`
- **IndexMut**
  - `fn index_mut(self: & mut Self, index: RangeFull) -> & mut UninitSlice`
- **Index**
  - `fn index(self: &Self, index: RangeFrom<usize>) -> &UninitSlice`
- **Index**
  - `fn index(self: &Self, index: Range<usize>) -> &UninitSlice`
- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **IndexMut**
  - `fn index_mut(self: & mut Self, index: RangeToInclusive<usize>) -> & mut UninitSlice`
- **IndexMut**
  - `fn index_mut(self: & mut Self, index: Range<usize>) -> & mut UninitSlice`
- **Index**
  - `fn index(self: &Self, index: RangeTo<usize>) -> &UninitSlice`
- **IndexMut**
  - `fn index_mut(self: & mut Self, index: RangeInclusive<usize>) -> & mut UninitSlice`
- **Index**
  - `fn index(self: &Self, index: RangeFull) -> &UninitSlice`
- **IndexMut**
  - `fn index_mut(self: & mut Self, index: RangeFrom<usize>) -> & mut UninitSlice`



