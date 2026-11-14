**tempfile > dir > TempDir**

# Module: dir::TempDir

## Contents

**Functions**

- [`disable_cleanup`](#disable_cleanup) - Disable cleanup of the temporary directory. If `disable_cleanup` is `true`, the temporary
- [`keep`](#keep) - Persist the temporary directory to disk, returning the [`PathBuf`] where it is located.

---

## tempfile::dir::TempDir::disable_cleanup

*Function*

Disable cleanup of the temporary directory. If `disable_cleanup` is `true`, the temporary
directory will not be deleted when this `TempDir` is dropped. This method is equivalent to
calling [`Builder::disable_cleanup`] when creating the `TempDir`.

**NOTE:** this method is primarily useful for testing/debugging. If you want to simply turn
a temporary directory into a non-temporary directory, prefer [`TempDir::keep`].

```rust
fn disable_cleanup(self: & mut Self, disable_cleanup: bool)
```



## tempfile::dir::TempDir::keep

*Function*

Persist the temporary directory to disk, returning the [`PathBuf`] where it is located.

This consumes the [`TempDir`] without deleting directory on the filesystem, meaning that
the directory will no longer be automatically deleted.

If you want to disable automatic cleanup of the temporary directory in-place, keeping the
`TempDir` as-is, use [`TempDir::disable_cleanup`] instead.

[`TempDir`]: struct.TempDir.html
[`PathBuf`]: http://doc.rust-lang.org/std/path/struct.PathBuf.html

# Examples

```
use std::fs;
use tempfile::TempDir;

let tmp_dir = TempDir::new()?;

// Persist the temporary directory to disk,
// getting the path where it is.
let tmp_path = tmp_dir.keep();

// Delete the temporary directory ourselves.
fs::remove_dir_all(tmp_path)?;
# Ok::<(), std::io::Error>(())
```

```rust
fn keep(self: Self) -> PathBuf
```



