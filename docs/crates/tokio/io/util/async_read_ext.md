**tokio > io > util > async_read_ext**

# Module: io::util::async_read_ext

## Contents

**Traits**

- [`AsyncReadExt`](#asyncreadext) - Reads bytes from a source.

---

## tokio::io::util::async_read_ext::AsyncReadExt

*Trait*

Reads bytes from a source.

Implemented as an extension trait, adding utility methods to all
[`AsyncRead`] types. Callers will tend to import this trait instead of
[`AsyncRead`].

```no_run
# #[cfg(not(target_family = "wasm"))]
# {
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buffer = [0; 10];

    // The `read` method is defined by this trait.
    let n = f.read(&mut buffer[..]).await?;

    Ok(())
}
# }
```

See [module][crate::io] documentation for more details.

[`AsyncRead`]: AsyncRead

**Methods:**

- `chain`: Creates a new `AsyncRead` instance that chains this stream with
- `read`: Pulls some bytes from this source into the specified buffer,
- `read_buf`: Pulls some bytes from this source into the specified buffer,
- `read_exact`: Reads the exact number of bytes required to fill `buf`.
- `read_u8`: Reads an unsigned 8 bit integer from the underlying reader.
- `read_i8`: Reads a signed 8 bit integer from the underlying reader.
- `read_u16`: Reads an unsigned 16-bit integer in big-endian order from the
- `read_i16`: Reads a signed 16-bit integer in big-endian order from the
- `read_u32`: Reads an unsigned 32-bit integer in big-endian order from the
- `read_i32`: Reads a signed 32-bit integer in big-endian order from the
- `read_u64`: Reads an unsigned 64-bit integer in big-endian order from the
- `read_i64`: Reads an signed 64-bit integer in big-endian order from the
- `read_u128`: Reads an unsigned 128-bit integer in big-endian order from the
- `read_i128`: Reads an signed 128-bit integer in big-endian order from the
- `read_f32`: Reads an 32-bit floating point type in big-endian order from the
- `read_f64`: Reads an 64-bit floating point type in big-endian order from the
- `read_u16_le`: Reads an unsigned 16-bit integer in little-endian order from the
- `read_i16_le`: Reads a signed 16-bit integer in little-endian order from the
- `read_u32_le`: Reads an unsigned 32-bit integer in little-endian order from the
- `read_i32_le`: Reads a signed 32-bit integer in little-endian order from the
- `read_u64_le`: Reads an unsigned 64-bit integer in little-endian order from the
- `read_i64_le`: Reads an signed 64-bit integer in little-endian order from the
- `read_u128_le`: Reads an unsigned 128-bit integer in little-endian order from the
- `read_i128_le`: Reads an signed 128-bit integer in little-endian order from the
- `read_f32_le`: Reads an 32-bit floating point type in little-endian order from the
- `read_f64_le`: Reads an 64-bit floating point type in little-endian order from the
- `read_to_end`: Reads all bytes until EOF in this source, placing them into `buf`.
- `read_to_string`: Reads all bytes until EOF in this source, appending them to `buf`.
- `take`: Creates an adaptor which reads at most `limit` bytes from it.



