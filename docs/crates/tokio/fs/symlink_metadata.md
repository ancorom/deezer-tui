**tokio > fs > symlink_metadata**

# Module: fs::symlink_metadata

## Contents

**Functions**

- [`symlink_metadata`](#symlink_metadata) - Queries the file system metadata for a path.

---

## tokio::fs::symlink_metadata::symlink_metadata

*Function*

Queries the file system metadata for a path.

This is an async version of [`std::fs::symlink_metadata`][std]

[std]: fn@std::fs::symlink_metadata

```rust
fn symlink_metadata<impl AsRef<Path>>(path: impl Trait) -> io::Result<std::fs::Metadata>
```



