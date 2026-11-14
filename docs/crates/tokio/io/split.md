**tokio > io > split**

# Module: io::split

## Contents

**Structs**

- [`ReadHalf`](#readhalf) - The readable half of a value returned from [`split`](split()).
- [`WriteHalf`](#writehalf) - The writable half of a value returned from [`split`](split()).

**Functions**

- [`split`](#split) - Splits a single value implementing `AsyncRead + AsyncWrite` into separate

---

## tokio::io::split::ReadHalf

*Struct*

The readable half of a value returned from [`split`](split()).

**Generic Parameters:**
- T

**Methods:**

- `fn is_pair_of(self: &Self, other: &WriteHalf<T>) -> bool` - Checks if this `ReadHalf` and some `WriteHalf` were split from the same
- `fn unsplit(self: Self, wr: WriteHalf<T>) -> T` - Reunites with a previously split `WriteHalf`.

**Traits:** Sync, Send

**Trait Implementations:**

- **AsyncRead**
  - `fn poll_read(self: Pin<& mut Self>, cx: & mut Context, buf: & mut ReadBuf) -> Poll<io::Result<()>>`
- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`



## tokio::io::split::WriteHalf

*Struct*

The writable half of a value returned from [`split`](split()).

**Generic Parameters:**
- T

**Methods:**

- `fn is_pair_of(self: &Self, other: &ReadHalf<T>) -> bool` - Checks if this `WriteHalf` and some `ReadHalf` were split from the same

**Traits:** Sync, Send

**Trait Implementations:**

- **AsyncWrite**
  - `fn poll_write(self: Pin<& mut Self>, cx: & mut Context, buf: &[u8]) -> Poll<Result<usize, io::Error>>`
  - `fn poll_flush(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<(), io::Error>>`
  - `fn poll_shutdown(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<(), io::Error>>`
  - `fn poll_write_vectored(self: Pin<& mut Self>, cx: & mut Context, bufs: &[io::IoSlice]) -> Poll<Result<usize, io::Error>>`
  - `fn is_write_vectored(self: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`



## tokio::io::split::split

*Function*

Splits a single value implementing `AsyncRead + AsyncWrite` into separate
`AsyncRead` and `AsyncWrite` handles.

To restore this read/write object from its `ReadHalf` and
`WriteHalf` use [`unsplit`](ReadHalf::unsplit()).

```rust
fn split<T>(stream: T) -> (ReadHalf<T>, WriteHalf<T>)
```



