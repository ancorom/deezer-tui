**tokio > io > util > repeat**

# Module: io::util::repeat

## Contents

**Structs**

- [`Repeat`](#repeat) - An async reader which yields one byte over and over and over and over and

**Functions**

- [`repeat`](#repeat) - Creates an instance of an async reader that infinitely repeats one byte.

---

## tokio::io::util::repeat::Repeat

*Struct*

An async reader which yields one byte over and over and over and over and
over and...

This struct is generally created by calling [`repeat`][repeat]. Please
see the documentation of `repeat()` for more details.

This is an asynchronous version of [`std::io::Repeat`][std].

[repeat]: fn@repeat
[std]: std::io::Repeat

**Trait Implementations:**

- **AsyncRead**
  - `fn poll_read(self: Pin<& mut Self>, cx: & mut Context, buf: & mut ReadBuf) -> Poll<io::Result<()>>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## tokio::io::util::repeat::repeat

*Function*

Creates an instance of an async reader that infinitely repeats one byte.

All reads from this reader will succeed by filling the specified buffer with
the given byte.

This is an asynchronous version of [`std::io::repeat`][std].

[std]: std::io::repeat

# Examples

```
use tokio::io::{self, AsyncReadExt};

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let mut buffer = [0; 3];
io::repeat(0b101).read_exact(&mut buffer).await.unwrap();
assert_eq!(buffer, [0b101, 0b101, 0b101]);
# }
```

```rust
fn repeat(byte: u8) -> Repeat
```



