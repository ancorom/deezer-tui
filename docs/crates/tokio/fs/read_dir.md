**tokio > fs > read_dir**

# Module: fs::read_dir

## Contents

**Structs**

- [`DirEntry`](#direntry) - Entries returned by the [`ReadDir`] stream.
- [`ReadDir`](#readdir) - Reads the entries in a directory.

**Functions**

- [`read_dir`](#read_dir) - Returns a stream over the entries within a directory.

---

## tokio::fs::read_dir::DirEntry

*Struct*

Entries returned by the [`ReadDir`] stream.

[`ReadDir`]: struct@ReadDir

This is a specialized version of [`std::fs::DirEntry`] for usage from the
Tokio runtime.

An instance of `DirEntry` represents an entry inside of a directory on the
filesystem. Each entry can be inspected via methods to learn about the full
path or possibly other metadata through per-platform extension traits.

**Methods:**

- `fn ino(self: &Self) -> u64` - Returns the underlying `d_ino` field in the contained `dirent`
- `fn path(self: &Self) -> PathBuf` - Returns the full path to the file that this entry represents.
- `fn file_name(self: &Self) -> OsString` - Returns the bare file name of this directory entry without any other
- `fn metadata(self: &Self) -> io::Result<Metadata>` - Returns the metadata for the file that this entry points at.
- `fn file_type(self: &Self) -> io::Result<FileType>` - Returns the file type for the file that this entry points at.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## tokio::fs::read_dir::ReadDir

*Struct*

Reads the entries in a directory.

This struct is returned from the [`read_dir`] function of this module and
will yield instances of [`DirEntry`]. Through a [`DirEntry`] information
like the entry's path and possibly other metadata can be learned.

A `ReadDir` can be turned into a `Stream` with [`ReadDirStream`].

[`ReadDirStream`]: https://docs.rs/tokio-stream/0.1/tokio_stream/wrappers/struct.ReadDirStream.html

# Errors

This stream will return an [`Err`] if there's some sort of intermittent
IO error during iteration.

[`read_dir`]: read_dir
[`DirEntry`]: DirEntry
[`Err`]: std::result::Result::Err

**Tuple Struct**: `()`

**Methods:**

- `fn next_entry(self: & mut Self) -> io::Result<Option<DirEntry>>` - Returns the next entry in the directory stream.
- `fn poll_next_entry(self: & mut Self, cx: & mut Context) -> Poll<io::Result<Option<DirEntry>>>` - Polls for the next directory entry in the stream.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## tokio::fs::read_dir::read_dir

*Function*

Returns a stream over the entries within a directory.

This is an async version of [`std::fs::read_dir`].

This operation is implemented by running the equivalent blocking
operation on a separate thread pool using [`spawn_blocking`].

[`spawn_blocking`]: crate::task::spawn_blocking

```rust
fn read_dir<impl AsRef<Path>>(path: impl Trait) -> io::Result<ReadDir>
```



