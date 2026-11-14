**tokio > io > util > take**

# Module: io::util::take

## Contents

**Structs**

- [`Take`](#take) - Stream for the [`take`](super::AsyncReadExt::take) method.

---

## tokio::io::util::take::Take

*Struct*

Stream for the [`take`](super::AsyncReadExt::take) method.

**Generic Parameters:**
- R

**Methods:**

- `fn limit(self: &Self) -> u64` - Returns the remaining number of bytes that can be
- `fn set_limit(self: & mut Self, limit: u64)` - Sets the number of bytes that can be read before this instance will
- `fn get_ref(self: &Self) -> &R` - Gets a reference to the underlying reader.
- `fn get_mut(self: & mut Self) -> & mut R` - Gets a mutable reference to the underlying reader.
- `fn get_pin_mut(self: Pin<& mut Self>) -> Pin<& mut R>` - Gets a pinned mutable reference to the underlying reader.
- `fn into_inner(self: Self) -> R` - Consumes the `Take`, returning the wrapped reader.

**Trait Implementations:**

- **AsyncBufRead**
  - `fn poll_fill_buf(self: Pin<& mut Self>, cx: & mut Context) -> Poll<io::Result<&[u8]>>`
  - `fn consume(self: Pin<& mut Self>, amt: usize)`
- **AsyncRead**
  - `fn poll_read(self: Pin<& mut Self>, cx: & mut Context, buf: & mut ReadBuf) -> Poll<Result<(), io::Error>>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



