**tokio > fs > remove_dir**

# Module: fs::remove_dir

## Contents

**Functions**

- [`remove_dir`](#remove_dir) - Removes an existing, empty directory.

---

## tokio::fs::remove_dir::remove_dir

*Function*

Removes an existing, empty directory.

This is an async version of [`std::fs::remove_dir`].

```rust
fn remove_dir<impl AsRef<Path>>(path: impl Trait) -> io::Result<()>
```



