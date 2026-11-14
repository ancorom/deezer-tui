**tokio > fs > hard_link**

# Module: fs::hard_link

## Contents

**Functions**

- [`hard_link`](#hard_link) - Creates a new hard link on the filesystem.

---

## tokio::fs::hard_link::hard_link

*Function*

Creates a new hard link on the filesystem.

This is an async version of [`std::fs::hard_link`].

The `link` path will be a link pointing to the `original` path. Note that systems
often require these two paths to both be located on the same filesystem.

# Platform-specific behavior

This function currently corresponds to the `link` function on Unix
and the `CreateHardLink` function on Windows.
Note that, this [may change in the future][changes].

[changes]: https://doc.rust-lang.org/std/io/index.html#platform-specific-behavior

# Errors

This function will return an error in the following situations, but is not
limited to just these cases:

* The `original` path is not a file or doesn't exist.

# Examples

```no_run
use tokio::fs;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    fs::hard_link("a.txt", "b.txt").await?; // Hard link a.txt to b.txt
    Ok(())
}
```

```rust
fn hard_link<impl AsRef<Path>, impl AsRef<Path>>(original: impl Trait, link: impl Trait) -> io::Result<()>
```



