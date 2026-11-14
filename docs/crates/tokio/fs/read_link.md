**tokio > fs > read_link**

# Module: fs::read_link

## Contents

**Functions**

- [`read_link`](#read_link) - Reads a symbolic link, returning the file that the link points to.

---

## tokio::fs::read_link::read_link

*Function*

Reads a symbolic link, returning the file that the link points to.

This is an async version of [`std::fs::read_link`].

```rust
fn read_link<impl AsRef<Path>>(path: impl Trait) -> io::Result<std::path::PathBuf>
```



