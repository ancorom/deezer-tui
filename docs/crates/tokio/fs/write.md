**tokio > fs > write**

# Module: fs::write

## Contents

**Functions**

- [`write`](#write) - Creates a future that will open a file for writing and write the entire

---

## tokio::fs::write::write

*Function*

Creates a future that will open a file for writing and write the entire
contents of `contents` to it.

This is the async equivalent of [`std::fs::write`][std].

This operation is implemented by running the equivalent blocking operation
on a separate thread pool using [`spawn_blocking`].

[`spawn_blocking`]: crate::task::spawn_blocking
[std]: fn@std::fs::write

# Examples

```no_run
use tokio::fs;

# async fn dox() -> std::io::Result<()> {
fs::write("foo.txt", b"Hello world!").await?;
# Ok(())
# }
```

```rust
fn write<impl AsRef<Path>, impl AsRef<[u8]>>(path: impl Trait, contents: impl Trait) -> io::Result<()>
```



