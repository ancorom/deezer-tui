**tokio > io > util > async_seek_ext**

# Module: io::util::async_seek_ext

## Contents

**Traits**

- [`AsyncSeekExt`](#asyncseekext) - An extension trait that adds utility methods to [`AsyncSeek`] types.

---

## tokio::io::util::async_seek_ext::AsyncSeekExt

*Trait*

An extension trait that adds utility methods to [`AsyncSeek`] types.

# Examples

```
use std::io::{self, Cursor, SeekFrom};
use tokio::io::{AsyncSeekExt, AsyncReadExt};

# #[tokio::main(flavor = "current_thread")]
# async fn main() -> io::Result<()> {
let mut cursor = Cursor::new(b"abcdefg");

// the `seek` method is defined by this trait
cursor.seek(SeekFrom::Start(3)).await?;

let mut buf = [0; 1];
let n = cursor.read(&mut buf).await?;
assert_eq!(n, 1);
assert_eq!(buf, [b'd']);

Ok(())
# }
```

See [module][crate::io] documentation for more details.

[`AsyncSeek`]: AsyncSeek

**Methods:**

- `seek`: Creates a future which will seek an IO object, and then yield the
- `rewind`: Creates a future which will rewind to the beginning of the stream.
- `stream_position`: Creates a future which will return the current seek position from the



