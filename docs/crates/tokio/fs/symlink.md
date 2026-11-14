**tokio > fs > symlink**

# Module: fs::symlink

## Contents

**Functions**

- [`symlink`](#symlink) - Creates a new symbolic link on the filesystem.

---

## tokio::fs::symlink::symlink

*Function*

Creates a new symbolic link on the filesystem.

The `link` path will be a symbolic link pointing to the `original` path.

This is an async version of [`std::os::unix::fs::symlink`].

```rust
fn symlink<impl AsRef<Path>, impl AsRef<Path>>(original: impl Trait, link: impl Trait) -> io::Result<()>
```



