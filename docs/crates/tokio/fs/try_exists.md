**tokio > fs > try_exists**

# Module: fs::try_exists

## Contents

**Functions**

- [`try_exists`](#try_exists) - Returns `Ok(true)` if the path points at an existing entity.

---

## tokio::fs::try_exists::try_exists

*Function*

Returns `Ok(true)` if the path points at an existing entity.

This function will traverse symbolic links to query information about the
destination file. In case of broken symbolic links this will return `Ok(false)`.

This is the async equivalent of [`std::path::Path::try_exists`][std].

[std]: fn@std::path::Path::try_exists

# Examples

```no_run
use tokio::fs;

# async fn dox() -> std::io::Result<()> {
fs::try_exists("foo.txt").await?;
# Ok(())
# }
```

```rust
fn try_exists<impl AsRef<Path>>(path: impl Trait) -> io::Result<bool>
```



