**tempfile > spooled**

# Module: spooled

## Contents

**Structs**

- [`SpooledTempFile`](#spooledtempfile) - An object that behaves like a regular temporary file, but keeps data in

**Enums**

- [`SpooledData`](#spooleddata) - A wrapper for the two states of a [`SpooledTempFile`]. Either:

**Functions**

- [`spooled_tempfile`](#spooled_tempfile) - Create a new [`SpooledTempFile`]. Also see [`spooled_tempfile_in`].
- [`spooled_tempfile_in`](#spooled_tempfile_in) - Construct a new [`SpooledTempFile`], backed by a file in the specified directory. Use this when,

---

## tempfile::spooled::SpooledData

*Enum*

A wrapper for the two states of a [`SpooledTempFile`]. Either:

1. An in-memory [`Cursor`] representing the state of the file.
2. A temporary [`File`].

**Variants:**
- `InMemory(std::io::Cursor<Vec<u8>>)`
- `OnDisk(std::fs::File)`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## tempfile::spooled::SpooledTempFile

*Struct*

An object that behaves like a regular temporary file, but keeps data in
memory until it reaches a configured size, at which point the data is
written to a temporary file on disk, and further operations use the file
on disk.

**Methods:**

- `fn new(max_size: usize) -> SpooledTempFile` - Construct a new [`SpooledTempFile`].
- `fn new_in<P>(max_size: usize, dir: P) -> SpooledTempFile` - Construct a new [`SpooledTempFile`], backed by a file in the specified directory.
- `fn is_rolled(self: &Self) -> bool` - Returns true if the file has been rolled over to disk.
- `fn roll(self: & mut Self) -> io::Result<()>` - Rolls over to a file on disk, regardless of current size. Does nothing
- `fn set_len(self: & mut Self, size: u64) -> Result<(), io::Error>` - Truncate the file to the specified size.
- `fn into_inner(self: Self) -> SpooledData` - Consumes and returns the inner `SpooledData` type.
- `fn into_file(self: Self) -> io::Result<File>` - Convert into a regular unnamed temporary file, writing it to disk if necessary.

**Trait Implementations:**

- **Write**
  - `fn write(self: & mut Self, buf: &[u8]) -> io::Result<usize>`
  - `fn write_vectored(self: & mut Self, bufs: &[io::IoSlice]) -> io::Result<usize>`
  - `fn flush(self: & mut Self) -> io::Result<()>`
- **Read**
  - `fn read(self: & mut Self, buf: & mut [u8]) -> io::Result<usize>`
  - `fn read_vectored(self: & mut Self, bufs: & mut [io::IoSliceMut]) -> io::Result<usize>`
  - `fn read_to_end(self: & mut Self, buf: & mut Vec<u8>) -> io::Result<usize>`
  - `fn read_to_string(self: & mut Self, buf: & mut String) -> io::Result<usize>`
  - `fn read_exact(self: & mut Self, buf: & mut [u8]) -> io::Result<()>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Seek**
  - `fn seek(self: & mut Self, pos: SeekFrom) -> io::Result<u64>`



## tempfile::spooled::spooled_tempfile

*Function*

Create a new [`SpooledTempFile`]. Also see [`spooled_tempfile_in`].

# Security

This variant is secure/reliable in the presence of a pathological temporary
file cleaner.

# Backing Storage

By default, the underlying temporary file will be created in your operating system's temporary
file directory which is _often_ an in-memory filesystem. You may want to consider using
[`spooled_tempfile_in`] instead, passing a storage-backed filesystem (e.g., `/var/tmp` on
Linux).

# Resource Leaking

The temporary file will be automatically removed by the OS when the last
handle to it is closed. This doesn't rely on Rust destructors being run, so
will (almost) never fail to clean up the temporary file.

# Examples

```
use tempfile::spooled_tempfile;
use std::io::Write;

let mut file = spooled_tempfile(15);

writeln!(file, "short line")?;
assert!(!file.is_rolled());

// as a result of this write call, the size of the data will exceed
// `max_size` (15), so it will be written to a temporary file on disk,
// and the in-memory buffer will be dropped
writeln!(file, "marvin gardens")?;
assert!(file.is_rolled());
# Ok::<(), std::io::Error>(())
```

```rust
fn spooled_tempfile(max_size: usize) -> SpooledTempFile
```



## tempfile::spooled::spooled_tempfile_in

*Function*

Construct a new [`SpooledTempFile`], backed by a file in the specified directory. Use this when,
e.g., you need the temporary file to be backed by a specific filesystem (e.g., when your default
temporary directory is in-memory). Also see [`spooled_tempfile`].

**NOTE:** The specified path isn't checked until the temporary file is "rolled over" into a real
temporary file. If the specified directory isn't writable, writes to the temporary file will
fail once the `max_size` is reached.

```rust
fn spooled_tempfile_in<P>(max_size: usize, dir: P) -> SpooledTempFile
```



