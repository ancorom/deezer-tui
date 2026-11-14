**tokio > io > util > lines**

# Module: io::util::lines

## Contents

**Structs**

- [`Lines`](#lines) - Reads lines from an [`AsyncBufRead`].

---

## tokio::io::util::lines::Lines

*Struct*

Reads lines from an [`AsyncBufRead`].

A `Lines` can be turned into a `Stream` with [`LinesStream`].

This type is usually created using the [`lines`] method.

[`AsyncBufRead`]: crate::io::AsyncBufRead
[`LinesStream`]: https://docs.rs/tokio-stream/0.1/tokio_stream/wrappers/struct.LinesStream.html
[`lines`]: crate::io::AsyncBufReadExt::lines

**Generic Parameters:**
- R

**Methods:**

- `fn next_line(self: & mut Self) -> io::Result<Option<String>>` - Returns the next line in the stream.
- `fn get_mut(self: & mut Self) -> & mut R` - Obtains a mutable reference to the underlying reader.
- `fn get_ref(self: & mut Self) -> &R` - Obtains a reference to the underlying reader.
- `fn into_inner(self: Self) -> R` - Unwraps this `Lines<R>`, returning the underlying reader.
- `fn poll_next_line(self: Pin<& mut Self>, cx: & mut Context) -> Poll<io::Result<Option<String>>>` - Polls for the next line in the stream.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



