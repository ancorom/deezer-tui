**tokio > io > read_buf**

# Module: io::read_buf

## Contents

**Structs**

- [`ReadBuf`](#readbuf) - A wrapper around a byte buffer that is incrementally filled and initialized.

---

## tokio::io::read_buf::ReadBuf

*Struct*

A wrapper around a byte buffer that is incrementally filled and initialized.

This type is a sort of "double cursor". It tracks three regions in the
buffer: a region at the beginning of the buffer that has been logically
filled with data, a region that has been initialized at some point but not
yet logically filled, and a region at the end that may be uninitialized.
The filled region is guaranteed to be a subset of the initialized region.

In summary, the contents of the buffer can be visualized as:

```not_rust
[             capacity              ]
[ filled |         unfilled         ]
[    initialized    | uninitialized ]
```

It is undefined behavior to de-initialize any bytes from the uninitialized
region, since it is merely unknown whether this region is uninitialized or
not, and if part of it turns out to be initialized, it must stay initialized.

**Generic Parameters:**
- 'a

**Methods:**

- `fn new(buf: &'a  mut [u8]) -> ReadBuf<'a>` - Creates a new `ReadBuf` from a fully initialized buffer.
- `fn uninit(buf: &'a  mut [MaybeUninit<u8>]) -> ReadBuf<'a>` - Creates a new `ReadBuf` from a buffer that may be uninitialized.
- `fn capacity(self: &Self) -> usize` - Returns the total capacity of the buffer.
- `fn filled(self: &Self) -> &[u8]` - Returns a shared reference to the filled portion of the buffer.
- `fn filled_mut(self: & mut Self) -> & mut [u8]` - Returns a mutable reference to the filled portion of the buffer.
- `fn take(self: & mut Self, n: usize) -> ReadBuf` - Returns a new `ReadBuf` comprised of the unfilled section up to `n`.
- `fn initialized(self: &Self) -> &[u8]` - Returns a shared reference to the initialized portion of the buffer.
- `fn initialized_mut(self: & mut Self) -> & mut [u8]` - Returns a mutable reference to the initialized portion of the buffer.
- `fn inner_mut(self: & mut Self) -> & mut [MaybeUninit<u8>]` - Returns a mutable reference to the entire buffer, without ensuring that it has been fully
- `fn unfilled_mut(self: & mut Self) -> & mut [MaybeUninit<u8>]` - Returns a mutable reference to the unfilled part of the buffer without ensuring that it has been fully
- `fn initialize_unfilled(self: & mut Self) -> & mut [u8]` - Returns a mutable reference to the unfilled part of the buffer, ensuring it is fully initialized.
- `fn initialize_unfilled_to(self: & mut Self, n: usize) -> & mut [u8]` - Returns a mutable reference to the first `n` bytes of the unfilled part of the buffer, ensuring it is
- `fn remaining(self: &Self) -> usize` - Returns the number of bytes at the end of the slice that have not yet been filled.
- `fn clear(self: & mut Self)` - Clears the buffer, resetting the filled region to empty.
- `fn advance(self: & mut Self, n: usize)` - Advances the size of the filled region of the buffer.
- `fn set_filled(self: & mut Self, n: usize)` - Sets the size of the filled region of the buffer.
- `fn assume_init(self: & mut Self, n: usize)` - Asserts that the first `n` unfilled bytes of the buffer are initialized.
- `fn put_slice(self: & mut Self, buf: &[u8])` - Appends data to the buffer, advancing the written position and possibly also the initialized position.

**Trait Implementations:**

- **BufMut**
  - `fn remaining_mut(self: &Self) -> usize`
  - `fn advance_mut(self: & mut Self, cnt: usize)`
  - `fn chunk_mut(self: & mut Self) -> & mut bytes::buf::UninitSlice`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



