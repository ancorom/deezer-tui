**tempfile > file > NamedTempFile**

# Module: file::NamedTempFile

## Contents

**Functions**

- [`disable_cleanup`](#disable_cleanup) - Disable cleanup of the temporary file. If `disable_cleanup` is `true`, the temporary file
- [`into_parts`](#into_parts) - Converts the named temporary file into its constituent parts.
- [`keep`](#keep) - Keep the temporary file from being deleted. This function will turn the
- [`new`](#new) - Create a new named temporary file.

---

## tempfile::file::NamedTempFile::disable_cleanup

*Function*

Disable cleanup of the temporary file. If `disable_cleanup` is `true`, the temporary file
will not be deleted when this `TempPath` is dropped. This method is equivalent to calling
[`Builder::disable_cleanup`] when creating the original `NamedTempFile`, which see for
relevant warnings.

**NOTE:** this method is primarily useful for testing/debugging. If you want to simply turn
a temporary file into a non-temporary file, prefer [`NamedTempFile::keep`].

```rust
fn disable_cleanup(self: & mut Self, disable_cleanup: bool)
```



## tempfile::file::NamedTempFile::into_parts

*Function*

Converts the named temporary file into its constituent parts.

Note: When the path is dropped, the underlying file will be removed from the filesystem but
the returned [`File`] can still be read/written.

```rust
fn into_parts(self: Self) -> (F, TempPath)
```



## tempfile::file::NamedTempFile::keep

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

let (file, path) = file.keep()?;
# Ok::<(), std::io::Error>(())
```

[`PathPersistError`]: struct.PathPersistError.html

```rust
fn keep(self: Self) -> Result<(F, PathBuf), PersistError<F>>
```



## tempfile::file::NamedTempFile::new

*Function*

Create a new named temporary file.

See [`Builder`] for more configuration.

# Security

This will create a temporary file in the default temporary file
directory (platform dependent). This has security implications on many
platforms so please read the security section of this type's
documentation.

Reasons to use this method:

  1. The file has a short lifetime and your temporary file cleaner is
     sane (doesn't delete recently accessed files).

  2. You trust every user on your system (i.e. you are the only user).

  3. You have disabled your system's temporary file cleaner or verified
     that your system doesn't have a temporary file cleaner.

Reasons not to use this method:

  1. You'll fix it later. No you won't.

  2. You don't care about the security of the temporary file. If none of
     the "reasons to use this method" apply, referring to a temporary
     file by name may allow an attacker to create/overwrite your
     non-temporary files. There are exceptions but if you don't already
     know them, don't use this method.

# Errors

If the file can not be created, `Err` is returned.

# Examples

Create a named temporary file and write some data to it:

```no_run
use std::io::Write;
use tempfile::NamedTempFile;

let mut file = NamedTempFile::new()?;

writeln!(file, "Brian was here. Briefly.")?;
# Ok::<(), std::io::Error>(())
```

[`Builder`]: struct.Builder.html

```rust
fn new() -> io::Result<NamedTempFile>
```



