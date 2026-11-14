**tokio > fs > metadata**

# Module: fs::metadata

## Contents

**Functions**

- [`metadata`](#metadata) - Given a path, queries the file system to get information about a file,

---

## tokio::fs::metadata::metadata

*Function*

Given a path, queries the file system to get information about a file,
directory, etc.

This is an async version of [`std::fs::metadata`].

This function will traverse symbolic links to query information about the
destination file.

# Platform-specific behavior

This function currently corresponds to the `stat` function on Unix and the
`GetFileAttributesEx` function on Windows.  Note that, this [may change in
the future][changes].

[changes]: https://doc.rust-lang.org/std/io/index.html#platform-specific-behavior

# Errors

This function will return an error in the following situations, but is not
limited to just these cases:

* The user lacks permissions to perform `metadata` call on `path`.
* `path` does not exist.

# Examples

```rust,no_run
use tokio::fs;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let attr = fs::metadata("/some/file/path.txt").await?;
    // inspect attr ...
    Ok(())
}
```

```rust
fn metadata<impl AsRef<Path>>(path: impl Trait) -> io::Result<std::fs::Metadata>
```



