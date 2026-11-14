**tempfile > file**

# Module: file

## Contents

**Structs**

- [`NamedTempFile`](#namedtempfile) - A named temporary file.
- [`PathPersistError`](#pathpersisterror) - Error returned when persisting a temporary file path fails.
- [`PersistError`](#persisterror) - Error returned when persisting a temporary file fails.
- [`TempPath`](#temppath) - A path to a named temporary file without an open file handle.

**Functions**

- [`tempfile`](#tempfile) - Create a new temporary file. Also see [`tempfile_in`].
- [`tempfile_in`](#tempfile_in) - Create a new temporary file in the specified directory. Also see [`tempfile`].

---

## tempfile::file::NamedTempFile

*Struct*

A named temporary file.

The default constructor, [`NamedTempFile::new()`], creates files in
the location returned by [`env::temp_dir()`], but `NamedTempFile`
can be configured to manage a temporary file in any location
by constructing with [`NamedTempFile::new_in()`].

# Security

Most operating systems employ temporary file cleaners to delete old
temporary files. Unfortunately these temporary file cleaners don't always
reliably _detect_ whether the temporary file is still being used.

Specifically, the following sequence of events can happen:

1. A user creates a temporary file with `NamedTempFile::new()`.
2. Time passes.
3. The temporary file cleaner deletes (unlinks) the temporary file from the
   filesystem.
4. Some other program creates a new file to replace this deleted temporary
   file.
5. The user tries to re-open the temporary file (in the same program or in a
   different program) by path. Unfortunately, they'll end up opening the
   file created by the other program, not the original file.

## Operating System Specific Concerns

The behavior of temporary files and temporary file cleaners differ by
operating system.

### Windows

On Windows, temporary files are, by default, created in per-user temporary
file directories so only an application running as the same user would be
able to interfere (which they could do anyways). However, an application
running as the same user can still _accidentally_ re-create deleted
temporary files if the number of random bytes in the temporary file name is
too small.

### MacOS

Like on Windows, temporary files are created in per-user temporary file
directories by default so calling `NamedTempFile::new()` should be
relatively safe.

### Linux

Unfortunately, most _Linux_ distributions don't create per-user temporary
file directories. Worse, systemd's tmpfiles daemon (a common temporary file
cleaner) will happily remove open temporary files if they haven't been
modified within the last 10 days.

# Resource Leaking

If the program exits before the `NamedTempFile` destructor is
run, the temporary file will not be deleted. This can happen
if the process exits using [`std::process::exit()`], a segfault occurs,
receiving an interrupt signal like `SIGINT` that is not handled, or by using
a statically declared `NamedTempFile` instance (like with [`lazy_static`]).

Use the [`tempfile()`] function unless you need a named file path.

[`tempfile()`]: fn.tempfile.html
[`NamedTempFile::new()`]: #method.new
[`NamedTempFile::new_in()`]: #method.new_in
[`std::process::exit()`]: http://doc.rust-lang.org/std/process/fn.exit.html
[`lazy_static`]: https://github.com/rust-lang-nursery/lazy-static.rs/issues/62

**Generic Parameters:**
- F

**Methods:**

- `fn reopen(self: &Self) -> io::Result<File>` - Securely reopen the temporary file.
- `fn path(self: &Self) -> &Path` - Get the temporary file's path.
- `fn close(self: Self) -> io::Result<()>` - Close and remove the temporary file.
- `fn persist<P>(self: Self, new_path: P) -> Result<F, PersistError<F>>` - Persist the temporary file at the target path.
- `fn persist_noclobber<P>(self: Self, new_path: P) -> Result<F, PersistError<F>>` - Persist the temporary file at the target path if and only if no file exists there.
- `fn keep(self: Self) -> Result<(F, PathBuf), PersistError<F>>` - Keep the temporary file from being deleted. This function will turn the
- `fn disable_cleanup(self: & mut Self, disable_cleanup: bool)` - Disable cleanup of the temporary file. If `disable_cleanup` is `true`, the temporary file
- `fn as_file(self: &Self) -> &F` - Get a reference to the underlying file.
- `fn as_file_mut(self: & mut Self) -> & mut F` - Get a mutable reference to the underlying file.
- `fn into_file(self: Self) -> F` - Turn this named temporary file into an "unnamed" temporary file as if you
- `fn into_temp_path(self: Self) -> TempPath` - Closes the file, leaving only the temporary file path.
- `fn into_parts(self: Self) -> (F, TempPath)` - Converts the named temporary file into its constituent parts.
- `fn from_parts(file: F, path: TempPath) -> Self` - Creates a `NamedTempFile` from its constituent parts.
- `fn new() -> io::Result<NamedTempFile>` - Create a new named temporary file.
- `fn new_in<P>(dir: P) -> io::Result<NamedTempFile>` - Create a new named temporary file in the specified directory.
- `fn with_suffix<S>(suffix: S) -> io::Result<NamedTempFile>` - Create a new named temporary file with the specified filename suffix.
- `fn with_suffix_in<S, P>(suffix: S, dir: P) -> io::Result<NamedTempFile>` - Create a new named temporary file with the specified filename suffix,
- `fn with_prefix<S>(prefix: S) -> io::Result<NamedTempFile>` - Create a new named temporary file with the specified filename prefix.
- `fn with_prefix_in<S, P>(prefix: S, dir: P) -> io::Result<NamedTempFile>` - Create a new named temporary file with the specified filename prefix,

**Trait Implementations:**

- **Read**
  - `fn read(self: & mut Self, buf: & mut [u8]) -> io::Result<usize>`
  - `fn read_vectored(self: & mut Self, bufs: & mut [io::IoSliceMut]) -> io::Result<usize>`
  - `fn read_to_end(self: & mut Self, buf: & mut Vec<u8>) -> io::Result<usize>`
  - `fn read_to_string(self: & mut Self, buf: & mut String) -> io::Result<usize>`
  - `fn read_exact(self: & mut Self, buf: & mut [u8]) -> io::Result<()>`
- **Seek**
  - `fn seek(self: & mut Self, pos: SeekFrom) -> io::Result<u64>`
- **From**
  - `fn from(error: PersistError<F>) -> NamedTempFile<F>`
- **AsRef**
  - `fn as_ref(self: &Self) -> &Path`
- **Write**
  - `fn write(self: & mut Self, buf: &[u8]) -> io::Result<usize>`
  - `fn flush(self: & mut Self) -> io::Result<()>`
  - `fn write_vectored(self: & mut Self, bufs: &[io::IoSlice]) -> io::Result<usize>`
  - `fn write_all(self: & mut Self, buf: &[u8]) -> io::Result<()>`
  - `fn write_fmt(self: & mut Self, fmt: fmt::Arguments) -> io::Result<()>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **AsRawFd**
  - `fn as_raw_fd(self: &Self) -> RawFd`
- **AsFd**
  - `fn as_fd(self: &Self) -> BorrowedFd`



## tempfile::file::PathPersistError

*Struct*

Error returned when persisting a temporary file path fails.

**Fields:**
- `error: io::Error` - The underlying IO error.
- `path: TempPath` - The temporary file path that couldn't be persisted.

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Error**
  - `fn source(self: &Self) -> Option<&dyn error::Error>`



## tempfile::file::PersistError

*Struct*

Error returned when persisting a temporary file fails.

**Generic Parameters:**
- F

**Fields:**
- `error: io::Error` - The underlying IO error.
- `file: NamedTempFile<F>` - The temporary file that couldn't be persisted.

**Trait Implementations:**

- **Error**
  - `fn source(self: &Self) -> Option<&dyn error::Error>`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## tempfile::file::TempPath

*Struct*

A path to a named temporary file without an open file handle.

This is useful when the temporary file needs to be used by a child process,
for example.

When dropped, the temporary file is deleted unless `disable_cleanup(true)` was called on the
builder that constructed this temporary file and/or was called on either this `TempPath` or the
`NamedTempFile` from which this `TempPath` was constructed.

**Methods:**

- `fn close(self: Self) -> io::Result<()>` - Close and remove the temporary file.
- `fn persist<P>(self: Self, new_path: P) -> Result<(), PathPersistError>` - Persist the temporary file at the target path.
- `fn persist_noclobber<P>(self: Self, new_path: P) -> Result<(), PathPersistError>` - Persist the temporary file at the target path if and only if no file exists there.
- `fn keep(self: Self) -> Result<PathBuf, PathPersistError>` - Keep the temporary file from being deleted. This function will turn the
- `fn disable_cleanup(self: & mut Self, disable_cleanup: bool)` - Disable cleanup of the temporary file. If `disable_cleanup` is `true`, the temporary file
- `fn from_path<impl Into<PathBuf>>(path: impl Trait) -> Self` - Create a new TempPath from an existing path. This can be done even if no file exists at the
- `fn try_from_path<impl Into<PathBuf>>(path: impl Trait) -> io::Result<Self>` - Create a new TempPath from an existing path. This can be done even if no file exists at the

**Trait Implementations:**

- **From**
  - `fn from(error: PathPersistError) -> TempPath`
- **AsRef**
  - `fn as_ref(self: &Self) -> &OsStr`
- **AsRef**
  - `fn as_ref(self: &Self) -> &Path`
- **Deref**
  - `fn deref(self: &Self) -> &Path`
- **Drop**
  - `fn drop(self: & mut Self)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## tempfile::file::tempfile

*Function*

Create a new temporary file. Also see [`tempfile_in`].

The file will be created in the location returned by [`env::temp_dir()`].

# Security

This variant is secure/reliable in the presence of a pathological temporary file cleaner.

# Resource Leaking

The temporary file will be automatically removed by the OS when the last handle to it is closed.
This doesn't rely on Rust destructors being run, so will (almost) never fail to clean up the temporary file.

# Errors

If the file can not be created, `Err` is returned.

# Examples

```
use tempfile::tempfile;
use std::io::Write;

// Create a file inside of `env::temp_dir()`.
let mut file = tempfile()?;

writeln!(file, "Brian was here. Briefly.")?;
# Ok::<(), std::io::Error>(())
```

```rust
fn tempfile() -> io::Result<std::fs::File>
```



## tempfile::file::tempfile_in

*Function*

Create a new temporary file in the specified directory. Also see [`tempfile`].

# Security

This variant is secure/reliable in the presence of a pathological temporary file cleaner.
If the temporary file isn't created in [`env::temp_dir()`] then temporary file cleaners aren't an issue.

# Resource Leaking

The temporary file will be automatically removed by the OS when the last handle to it is closed.
This doesn't rely on Rust destructors being run, so will (almost) never fail to clean up the temporary file.

# Errors

If the file can not be created, `Err` is returned.

# Examples

```
use tempfile::tempfile_in;
use std::io::Write;

// Create a file inside of the current working directory
let mut file = tempfile_in("./")?;

writeln!(file, "Brian was here. Briefly.")?;
# Ok::<(), std::io::Error>(())
```

```rust
fn tempfile_in<P>(dir: P) -> io::Result<std::fs::File>
```



