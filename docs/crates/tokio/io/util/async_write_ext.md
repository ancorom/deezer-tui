**tokio > io > util > async_write_ext**

# Module: io::util::async_write_ext

## Contents

**Traits**

- [`AsyncWriteExt`](#asyncwriteext) - Writes bytes to a sink.

---

## tokio::io::util::async_write_ext::AsyncWriteExt

*Trait*

Writes bytes to a sink.

Implemented as an extension trait, adding utility methods to all
[`AsyncWrite`] types. Callers will tend to import this trait instead of
[`AsyncWrite`].

```no_run
# #[cfg(not(target_family = "wasm"))]
# {
use tokio::io::{self, AsyncWriteExt};
use tokio::fs::File;

#[tokio::main]
async fn main() -> io::Result<()> {
    let data = b"some bytes";

    let mut pos = 0;
    let mut buffer = File::create("foo.txt").await?;

    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..]).await?;
        pos += bytes_written;
    }

    Ok(())
}
# }
```

See [module][crate::io] documentation for more details.

[`AsyncWrite`]: AsyncWrite

**Methods:**

- `write`: Writes a buffer into this writer, returning how many bytes were
- `write_vectored`: Like [`write`], except that it writes from a slice of buffers.
- `write_buf`: Writes a buffer into this writer, advancing the buffer's internal
- `write_all_buf`: Attempts to write an entire buffer into this writer.
- `write_all`: Attempts to write an entire buffer into this writer.
- `write_u8`: Writes an unsigned 8-bit integer to the underlying writer.
- `write_i8`: Writes a signed 8-bit integer to the underlying writer.
- `write_u16`: Writes an unsigned 16-bit integer in big-endian order to the
- `write_i16`: Writes a signed 16-bit integer in big-endian order to the
- `write_u32`: Writes an unsigned 32-bit integer in big-endian order to the
- `write_i32`: Writes a signed 32-bit integer in big-endian order to the
- `write_u64`: Writes an unsigned 64-bit integer in big-endian order to the
- `write_i64`: Writes an signed 64-bit integer in big-endian order to the
- `write_u128`: Writes an unsigned 128-bit integer in big-endian order to the
- `write_i128`: Writes an signed 128-bit integer in big-endian order to the
- `write_f32`: Writes an 32-bit floating point type in big-endian order to the
- `write_f64`: Writes an 64-bit floating point type in big-endian order to the
- `write_u16_le`: Writes an unsigned 16-bit integer in little-endian order to the
- `write_i16_le`: Writes a signed 16-bit integer in little-endian order to the
- `write_u32_le`: Writes an unsigned 32-bit integer in little-endian order to the
- `write_i32_le`: Writes a signed 32-bit integer in little-endian order to the
- `write_u64_le`: Writes an unsigned 64-bit integer in little-endian order to the
- `write_i64_le`: Writes an signed 64-bit integer in little-endian order to the
- `write_u128_le`: Writes an unsigned 128-bit integer in little-endian order to the
- `write_i128_le`: Writes an signed 128-bit integer in little-endian order to the
- `write_f32_le`: Writes an 32-bit floating point type in little-endian order to the
- `write_f64_le`: Writes an 64-bit floating point type in little-endian order to the
- `flush`: Flushes this output stream, ensuring that all intermediately buffered
- `shutdown`: Shuts down the output stream, ensuring that the value can be dropped



