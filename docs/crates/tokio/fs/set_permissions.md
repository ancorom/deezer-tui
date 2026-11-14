**tokio > fs > set_permissions**

# Module: fs::set_permissions

## Contents

**Functions**

- [`set_permissions`](#set_permissions) - Changes the permissions found on a file or a directory.

---

## tokio::fs::set_permissions::set_permissions

*Function*

Changes the permissions found on a file or a directory.

This is an async version of [`std::fs::set_permissions`][std]

[std]: fn@std::fs::set_permissions

```rust
fn set_permissions<impl AsRef<Path>>(path: impl Trait, perm: std::fs::Permissions) -> io::Result<()>
```



