**tokio > fs > rename**

# Module: fs::rename

## Contents

**Functions**

- [`rename`](#rename) - Renames a file or directory to a new name, replacing the original file if

---

## tokio::fs::rename::rename

*Function*

Renames a file or directory to a new name, replacing the original file if
`to` already exists.

This will not work if the new name is on a different mount point.

This is an async version of [`std::fs::rename`].

```rust
fn rename<impl AsRef<Path>, impl AsRef<Path>>(from: impl Trait, to: impl Trait) -> io::Result<()>
```



