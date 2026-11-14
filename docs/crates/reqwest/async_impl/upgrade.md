**reqwest > async_impl > upgrade**

# Module: async_impl::upgrade

## Contents

**Structs**

- [`Upgraded`](#upgraded) - An upgraded HTTP connection.

---

## reqwest::async_impl::upgrade::Upgraded

*Struct*

An upgraded HTTP connection.

**Trait Implementations:**

- **AsyncWrite**
  - `fn poll_write(self: Pin<& mut Self>, cx: & mut task::Context, buf: &[u8]) -> Poll<io::Result<usize>>`
  - `fn poll_write_vectored(self: Pin<& mut Self>, cx: & mut task::Context, bufs: &[io::IoSlice]) -> Poll<io::Result<usize>>`
  - `fn poll_flush(self: Pin<& mut Self>, cx: & mut task::Context) -> Poll<io::Result<()>>`
  - `fn poll_shutdown(self: Pin<& mut Self>, cx: & mut task::Context) -> Poll<io::Result<()>>`
  - `fn is_write_vectored(self: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **From**
  - `fn from(inner: hyper::upgrade::Upgraded) -> Self`
- **AsyncRead**
  - `fn poll_read(self: Pin<& mut Self>, cx: & mut task::Context, buf: & mut ReadBuf) -> Poll<io::Result<()>>`



