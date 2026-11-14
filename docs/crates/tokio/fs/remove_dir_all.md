**tokio > fs > remove_dir_all**

# Module: fs::remove_dir_all

## Contents

**Functions**

- [`remove_dir_all`](#remove_dir_all) - Removes a directory at this path, after removing all its contents. Use carefully!

---

## tokio::fs::remove_dir_all::remove_dir_all

*Function*

Removes a directory at this path, after removing all its contents. Use carefully!

This is an async version of [`std::fs::remove_dir_all`][std]

[std]: fn@std::fs::remove_dir_all

```rust
fn remove_dir_all<impl AsRef<Path>>(path: impl Trait) -> io::Result<()>
```



