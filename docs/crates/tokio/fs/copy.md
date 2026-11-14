**tokio > fs > copy**

# Module: fs::copy

## Contents

**Functions**

- [`copy`](#copy) - Copies the contents of one file to another. This function will also copy the permission bits

---

## tokio::fs::copy::copy

*Function*

Copies the contents of one file to another. This function will also copy the permission bits
of the original file to the destination file.
This function will overwrite the contents of to.

This is the async equivalent of [`std::fs::copy`].

# Examples

```no_run
use tokio::fs;

# async fn dox() -> std::io::Result<()> {
fs::copy("foo.txt", "bar.txt").await?;
# Ok(())
# }
```

```rust
fn copy<impl AsRef<Path>, impl AsRef<Path>>(from: impl Trait, to: impl Trait) -> Result<u64, std::io::Error>
```



