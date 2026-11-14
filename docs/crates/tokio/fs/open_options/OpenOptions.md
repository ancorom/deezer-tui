**tokio > fs > open_options > OpenOptions**

# Module: fs::open_options::OpenOptions

## Contents

**Functions**

- [`append`](#append) - Sets the option for the append mode.
- [`create`](#create) - Sets the option for creating a new file.
- [`new`](#new) - Creates a blank new set of options ready for configuration.
- [`open`](#open) - Opens a file at `path` with the options specified by `self`.
- [`truncate`](#truncate) - Sets the option for truncating a previous file.
- [`write`](#write) - Sets the option for write access.

---

## tokio::fs::open_options::OpenOptions::append

*Function*

Sets the option for the append mode.

This option, when true, means that writes will append to a file instead
of overwriting previous contents.  Note that setting
`.write(true).append(true)` has the same effect as setting only
`.append(true)`.

For most filesystems, the operating system guarantees that all writes are
atomic: no writes get mangled because another process writes at the same
time.

One maybe obvious note when using append-mode: make sure that all data
that belongs together is written to the file in one operation. This
can be done by concatenating strings before passing them to [`write()`],
or using a buffered writer (with a buffer of adequate size),
and calling [`flush()`] when the message is complete.

If a file is opened with both read and append access, beware that after
opening, and after every write, the position for reading may be set at the
end of the file. So, before writing, save the current position (using
[`seek`]`(`[`SeekFrom`]`::`[`Current`]`(0))`), and restore it before the next read.

This is an async version of [`std::fs::OpenOptions::append`][std]

[std]: std::fs::OpenOptions::append

## Note

This function doesn't create the file if it doesn't exist. Use the [`create`]
method to do so.

[`write()`]: crate::io::AsyncWriteExt::write
[`flush()`]: crate::io::AsyncWriteExt::flush
[`seek`]: crate::io::AsyncSeekExt::seek
[`SeekFrom`]: std::io::SeekFrom
[`Current`]: std::io::SeekFrom::Current
[`create`]: OpenOptions::create

# Examples

```no_run
use tokio::fs::OpenOptions;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let file = OpenOptions::new()
        .append(true)
        .open("foo.txt")
        .await?;

    Ok(())
}
```

```rust
fn append(self: & mut Self, append: bool) -> & mut OpenOptions
```



## tokio::fs::open_options::OpenOptions::create

*Function*

Sets the option for creating a new file.

This option indicates whether a new file will be created if the file
does not yet already exist.

In order for the file to be created, [`write`] or [`append`] access must
be used.

This is an async version of [`std::fs::OpenOptions::create`][std]

[std]: std::fs::OpenOptions::create
[`write`]: OpenOptions::write
[`append`]: OpenOptions::append

# Examples

```no_run
use tokio::fs::OpenOptions;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("foo.txt")
        .await?;

    Ok(())
}
```

```rust
fn create(self: & mut Self, create: bool) -> & mut OpenOptions
```



## tokio::fs::open_options::OpenOptions::new

*Function*

Creates a blank new set of options ready for configuration.

All options are initially set to `false`.

This is an async version of [`std::fs::OpenOptions::new`][std]

[std]: std::fs::OpenOptions::new

# Examples

```no_run
use tokio::fs::OpenOptions;

let mut options = OpenOptions::new();
let future = options.read(true).open("foo.txt");
```

```rust
fn new() -> OpenOptions
```



## tokio::fs::open_options::OpenOptions::open

*Function*

Opens a file at `path` with the options specified by `self`.

This is an async version of [`std::fs::OpenOptions::open`][std]

[std]: std::fs::OpenOptions::open

# Errors

This function will return an error under a number of different
circumstances. Some of these error conditions are listed here, together
with their [`ErrorKind`]. The mapping to [`ErrorKind`]s is not part of
the compatibility contract of the function, especially the `Other` kind
might change to more specific kinds in the future.

* [`NotFound`]: The specified file does not exist and neither `create`
  or `create_new` is set.
* [`NotFound`]: One of the directory components of the file path does
  not exist.
* [`PermissionDenied`]: The user lacks permission to get the specified
  access rights for the file.
* [`PermissionDenied`]: The user lacks permission to open one of the
  directory components of the specified path.
* [`AlreadyExists`]: `create_new` was specified and the file already
  exists.
* [`InvalidInput`]: Invalid combinations of open options (truncate
  without write access, no access mode set, etc.).
* [`Other`]: One of the directory components of the specified file path
  was not, in fact, a directory.
* [`Other`]: Filesystem-level errors: full disk, write permission
  requested on a read-only file system, exceeded disk quota, too many
  open files, too long filename, too many symbolic links in the
  specified path (Unix-like systems only), etc.

# io_uring support

On Linux, you can also use `io_uring` for executing system calls.
To enable `io_uring`, you need to specify the `--cfg tokio_unstable`
flag at compile time, enable the `io-uring` cargo feature, and set the
`Builder::enable_io_uring` runtime option.

Support for `io_uring` is currently experimental, so its behavior may
change or it may be removed in future versions.

# Examples

```no_run
use tokio::fs::OpenOptions;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let file = OpenOptions::new().open("foo.txt").await?;
    Ok(())
}
```

[`ErrorKind`]: std::io::ErrorKind
[`AlreadyExists`]: std::io::ErrorKind::AlreadyExists
[`InvalidInput`]: std::io::ErrorKind::InvalidInput
[`NotFound`]: std::io::ErrorKind::NotFound
[`Other`]: std::io::ErrorKind::Other
[`PermissionDenied`]: std::io::ErrorKind::PermissionDenied

```rust
fn open<impl AsRef<Path>>(self: &Self, path: impl Trait) -> io::Result<File>
```



## tokio::fs::open_options::OpenOptions::truncate

*Function*

Sets the option for truncating a previous file.

If a file is successfully opened with this option set it will truncate
the file to 0 length if it already exists.

The file must be opened with write access for truncate to work.

This is an async version of [`std::fs::OpenOptions::truncate`][std]

[std]: std::fs::OpenOptions::truncate

# Examples

```no_run
use tokio::fs::OpenOptions;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("foo.txt")
        .await?;

    Ok(())
}
```

```rust
fn truncate(self: & mut Self, truncate: bool) -> & mut OpenOptions
```



## tokio::fs::open_options::OpenOptions::write

*Function*

Sets the option for write access.

This option, when true, will indicate that the file should be
`write`-able if opened.

This is an async version of [`std::fs::OpenOptions::write`][std]

[std]: std::fs::OpenOptions::write

# Examples

```no_run
use tokio::fs::OpenOptions;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .open("foo.txt")
        .await?;

    Ok(())
}
```

```rust
fn write(self: & mut Self, write: bool) -> & mut OpenOptions
```



