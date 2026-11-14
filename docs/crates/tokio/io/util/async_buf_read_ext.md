**tokio > io > util > async_buf_read_ext**

# Module: io::util::async_buf_read_ext

## Contents

**Traits**

- [`AsyncBufReadExt`](#asyncbufreadext) - An extension trait which adds utility methods to [`AsyncBufRead`] types.

---

## tokio::io::util::async_buf_read_ext::AsyncBufReadExt

*Trait*

An extension trait which adds utility methods to [`AsyncBufRead`] types.

[`AsyncBufRead`]: crate::io::AsyncBufRead

**Methods:**

- `read_until`: Reads all bytes into `buf` until the delimiter `byte` or EOF is reached.
- `read_line`: Reads all bytes until a newline (the 0xA byte) is reached, and append
- `split`: Returns a stream of the contents of this reader split on the byte
- `fill_buf`: Returns the contents of the internal buffer, filling it with more
- `consume`: Tells this buffer that `amt` bytes have been consumed from the
- `lines`: Returns a stream over the lines of this reader.



