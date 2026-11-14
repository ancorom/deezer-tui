**tokio > fs > remove_file**

# Module: fs::remove_file

## Contents

**Functions**

- [`remove_file`](#remove_file) - Removes a file from the filesystem.

---

## tokio::fs::remove_file::remove_file

*Function*

Removes a file from the filesystem.

Note that there is no guarantee that the file is immediately deleted (e.g.
depending on platform, other open file descriptors may prevent immediate
removal).

This is an async version of [`std::fs::remove_file`].

```rust
fn remove_file<impl AsRef<Path>>(path: impl Trait) -> io::Result<()>
```



