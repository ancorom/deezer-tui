**tokio > io > util > chain**

# Module: io::util::chain

## Contents

**Structs**

- [`Chain`](#chain) - Stream for the [`chain`](super::AsyncReadExt::chain) method.

---

## tokio::io::util::chain::Chain

*Struct*

Stream for the [`chain`](super::AsyncReadExt::chain) method.

**Generic Parameters:**
- T
- U

**Methods:**

- `fn get_ref(self: &Self) -> (&T, &U)` - Gets references to the underlying readers in this `Chain`.
- `fn get_mut(self: & mut Self) -> (& mut T, & mut U)` - Gets mutable references to the underlying readers in this `Chain`.
- `fn get_pin_mut(self: Pin<& mut Self>) -> (Pin<& mut T>, Pin<& mut U>)` - Gets pinned mutable references to the underlying readers in this `Chain`.
- `fn into_inner(self: Self) -> (T, U)` - Consumes the `Chain`, returning the wrapped readers.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **AsyncBufRead**
  - `fn poll_fill_buf(self: Pin<& mut Self>, cx: & mut Context) -> Poll<io::Result<&[u8]>>`
  - `fn consume(self: Pin<& mut Self>, amt: usize)`
- **AsyncRead**
  - `fn poll_read(self: Pin<& mut Self>, cx: & mut Context, buf: & mut ReadBuf) -> Poll<io::Result<()>>`



