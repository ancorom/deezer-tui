**tokio > fs > create_dir**

# Module: fs::create_dir

## Contents

**Functions**

- [`create_dir`](#create_dir) - Creates a new, empty directory at the provided path.

---

## tokio::fs::create_dir::create_dir

*Function*

Creates a new, empty directory at the provided path.

This is an async version of [`std::fs::create_dir`].

# Platform-specific behavior

This function currently corresponds to the `mkdir` function on Unix
and the `CreateDirectory` function on Windows.
Note that, this [may change in the future][changes].

[changes]: https://doc.rust-lang.org/std/io/index.html#platform-specific-behavior

**NOTE**: If a parent of the given path doesn't exist, this function will
return an error. To create a directory and all its missing parents at the
same time, use the [`create_dir_all`] function.

# Errors

This function will return an error in the following situations, but is not
limited to just these cases:

* User lacks permissions to create directory at `path`.
* A parent of the given path doesn't exist. (To create a directory and all
  its missing parents at the same time, use the [`create_dir_all`]
  function.)
* `path` already exists.

[`create_dir_all`]: super::create_dir_all()

# Examples

```no_run
use tokio::fs;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    fs::create_dir("/some/dir").await?;
    Ok(())
}
```

```rust
fn create_dir<impl AsRef<Path>>(path: impl Trait) -> io::Result<()>
```



