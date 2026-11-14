**tempfile > file > TempPath**

# Module: file::TempPath

## Contents

**Functions**

- [`keep`](#keep) - Keep the temporary file from being deleted. This function will turn the
- [`try_from_path`](#try_from_path) - Create a new TempPath from an existing path. This can be done even if no file exists at the

---

## tempfile::file::TempPath::keep

*Function*

Keep the temporary file from being deleted. This function will turn the
temporary file into a non-temporary file without moving it.

# Errors

On some platforms (e.g., Windows), we need to mark the file as
non-temporary. This operation could fail.

# Examples

```no_run
use std::io::Write;
use tempfile::NamedTempFile;

let mut file = NamedTempFile::new()?;
writeln!(file, "Brian was here. Briefly.")?;

let path = file.into_temp_path();
let path = path.keep()?;
# Ok::<(), std::io::Error>(())
```

[`PathPersistError`]: struct.PathPersistError.html

```rust
fn keep(self: Self) -> Result<PathBuf, PathPersistError>
```



## tempfile::file::TempPath::try_from_path

*Function*

Create a new TempPath from an existing path. This can be done even if no file exists at the
given path.

This is mostly useful for interacting with libraries and external components that provide
files to be consumed or expect a path with no existing file to be given.

Relative paths are resolved relative to the current working directory. If the passed path is
empty or if the current working directory cannot be determined, this function returns an
error.

**NOTE:** this function does not check if the target path exists.

```rust
fn try_from_path<impl Into<PathBuf>>(path: impl Trait) -> io::Result<Self>
```



