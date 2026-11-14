**tokio > fs > file**

# Module: fs::file

## Contents

**Structs**

- [`File`](#file) - A reference to an open file on the filesystem.

---

## tokio::fs::file::File

*Struct*

A reference to an open file on the filesystem.

This is a specialized version of [`std::fs::File`] for usage from the
Tokio runtime.

An instance of a `File` can be read and/or written depending on what options
it was opened with. Files also implement [`AsyncSeek`] to alter the logical
cursor that the file contains internally.

A file will not be closed immediately when it goes out of scope if there
are any IO operations that have not yet completed. To ensure that a file is
closed immediately when it is dropped, you should call [`flush`] before
dropping it. Note that this does not ensure that the file has been fully
written to disk; the operating system might keep the changes around in an
in-memory buffer. See the [`sync_all`] method for telling the OS to write
the data to disk.

Reading and writing to a `File` is usually done using the convenience
methods found on the [`AsyncReadExt`] and [`AsyncWriteExt`] traits.

[`AsyncSeek`]: trait@crate::io::AsyncSeek
[`flush`]: fn@crate::io::AsyncWriteExt::flush
[`sync_all`]: fn@crate::fs::File::sync_all
[`AsyncReadExt`]: trait@crate::io::AsyncReadExt
[`AsyncWriteExt`]: trait@crate::io::AsyncWriteExt

# Examples

Create a new file and asynchronously write bytes to it:

```no_run
use tokio::fs::File;
use tokio::io::AsyncWriteExt; // for write_all()

# async fn dox() -> std::io::Result<()> {
let mut file = File::create("foo.txt").await?;
file.write_all(b"hello, world!").await?;
# Ok(())
# }
```

Read the contents of a file into a buffer:

```no_run
use tokio::fs::File;
use tokio::io::AsyncReadExt; // for read_to_end()

# async fn dox() -> std::io::Result<()> {
let mut file = File::open("foo.txt").await?;

let mut contents = vec![];
file.read_to_end(&mut contents).await?;

println!("len = {}", contents.len());
# Ok(())
# }
```

**Methods:**

- `fn open<impl AsRef<Path>>(path: impl Trait) -> io::Result<File>` - Attempts to open a file in read-only mode.
- `fn create<impl AsRef<Path>>(path: impl Trait) -> io::Result<File>` - Opens a file in write-only mode.
- `fn create_new<P>(path: P) -> std::io::Result<File>` - Opens a file in read-write mode.
- `fn options() -> OpenOptions` - Returns a new [`OpenOptions`] object.
- `fn from_std(std: StdFile) -> File` - Converts a [`std::fs::File`] to a [`tokio::fs::File`](File).
- `fn sync_all(self: &Self) -> io::Result<()>` - Attempts to sync all OS-internal metadata to disk.
- `fn sync_data(self: &Self) -> io::Result<()>` - This function is similar to `sync_all`, except that it may not
- `fn set_len(self: &Self, size: u64) -> io::Result<()>` - Truncates or extends the underlying file, updating the size of this file to become size.
- `fn metadata(self: &Self) -> io::Result<Metadata>` - Queries metadata about the underlying file.
- `fn try_clone(self: &Self) -> io::Result<File>` - Creates a new `File` instance that shares the same underlying file handle
- `fn into_std(self: Self) -> StdFile` - Destructures `File` into a [`std::fs::File`]. This function is
- `fn try_into_std(self: Self) -> Result<StdFile, Self>` - Tries to immediately destructure `File` into a [`std::fs::File`].
- `fn set_permissions(self: &Self, perm: Permissions) -> io::Result<()>` - Changes the permissions on the underlying file.
- `fn set_times(self: &Self, times: FileTimes) -> io::Result<()>` - Changes the timestamps on the underlying file.
- `fn set_modified(self: &Self, time: std::time::SystemTime) -> io::Result<()>` - Changes the modification timestamp on the underlying file.
- `fn set_max_buf_size(self: & mut Self, max_buf_size: usize)` - Set the maximum buffer size for the underlying [`AsyncRead`] / [`AsyncWrite`] operation.
- `fn max_buf_size(self: &Self) -> usize` - Get the maximum buffer size for the underlying [`AsyncRead`] / [`AsyncWrite`] operation.

**Trait Implementations:**

- **AsyncSeek**
  - `fn start_seek(self: Pin<& mut Self>, pos: SeekFrom) -> io::Result<()>`
  - `fn poll_complete(self: Pin<& mut Self>, cx: & mut Context) -> Poll<io::Result<u64>>`
- **AsFd**
  - `fn as_fd(self: &Self) -> std::os::unix::io::BorrowedFd`
- **FromRawFd**
  - `fn from_raw_fd(fd: std::os::unix::io::RawFd) -> Self`
- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **AsyncRead**
  - `fn poll_read(self: Pin<& mut Self>, cx: & mut Context, dst: & mut ReadBuf) -> Poll<io::Result<()>>`
- **From**
  - `fn from(fd: std::os::fd::OwnedFd) -> Self`
- **AsRawFd**
  - `fn as_raw_fd(self: &Self) -> std::os::unix::io::RawFd`
- **AsyncWrite**
  - `fn poll_write(self: Pin<& mut Self>, cx: & mut Context, src: &[u8]) -> Poll<io::Result<usize>>`
  - `fn poll_write_vectored(self: Pin<& mut Self>, cx: & mut Context, bufs: &[io::IoSlice]) -> Poll<Result<usize, io::Error>>`
  - `fn is_write_vectored(self: &Self) -> bool`
  - `fn poll_flush(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<(), io::Error>>`
  - `fn poll_shutdown(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<(), io::Error>>`
- **From**
  - `fn from(std: StdFile) -> Self`



