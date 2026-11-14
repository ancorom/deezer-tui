**tokio > fs > file > File**

# Module: fs::file::File

## Contents

**Functions**

- [`create`](#create) - Opens a file in write-only mode.
- [`open`](#open) - Attempts to open a file in read-only mode.
- [`set_max_buf_size`](#set_max_buf_size) - Set the maximum buffer size for the underlying [`AsyncRead`] / [`AsyncWrite`] operation.
- [`set_times`](#set_times) - Changes the timestamps on the underlying file.
- [`sync_all`](#sync_all) - Attempts to sync all OS-internal metadata to disk.

---

## tokio::fs::file::File::create

*Function*

Opens a file in write-only mode.

This function will create a file if it does not exist, and will truncate
it if it does.

See [`OpenOptions`] for more details.

# Errors

Results in an error if called from outside of the Tokio runtime or if
the underlying [`create`] call results in an error.

[`create`]: std::fs::File::create

# Examples

```no_run
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

# async fn dox() -> std::io::Result<()> {
let mut file = File::create("foo.txt").await?;
file.write_all(b"hello, world!").await?;
# Ok(())
# }
```

The [`write_all`] method is defined on the [`AsyncWriteExt`] trait.

[`write_all`]: fn@crate::io::AsyncWriteExt::write_all
[`AsyncWriteExt`]: trait@crate::io::AsyncWriteExt

```rust
fn create<impl AsRef<Path>>(path: impl Trait) -> io::Result<File>
```



## tokio::fs::file::File::open

*Function*

Attempts to open a file in read-only mode.

See [`OpenOptions`] for more details.

# Errors

This function will return an error if called from outside of the Tokio
runtime or if path does not already exist. Other errors may also be
returned according to `OpenOptions::open`.

# Examples

```no_run
use tokio::fs::File;
use tokio::io::AsyncReadExt;

# async fn dox() -> std::io::Result<()> {
let mut file = File::open("foo.txt").await?;

let mut contents = vec![];
file.read_to_end(&mut contents).await?;

println!("len = {}", contents.len());
# Ok(())
# }
```

The [`read_to_end`] method is defined on the [`AsyncReadExt`] trait.

[`read_to_end`]: fn@crate::io::AsyncReadExt::read_to_end
[`AsyncReadExt`]: trait@crate::io::AsyncReadExt

```rust
fn open<impl AsRef<Path>>(path: impl Trait) -> io::Result<File>
```



## tokio::fs::file::File::set_max_buf_size

*Function*

Set the maximum buffer size for the underlying [`AsyncRead`] / [`AsyncWrite`] operation.

Although Tokio uses a sensible default value for this buffer size, this function would be
useful for changing that default depending on the situation.

# Panics

This function panics if `max_buf_size` is 0.

# Examples

```no_run
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

# async fn dox() -> std::io::Result<()> {
let mut file = File::open("foo.txt").await?;

// Set maximum buffer size to 8 MiB
file.set_max_buf_size(8 * 1024 * 1024);

let mut buf = vec![1; 1024 * 1024 * 1024];

// Write the 1 GiB buffer in chunks up to 8 MiB each.
file.write_all(&mut buf).await?;
# Ok(())
# }
```

```rust
fn set_max_buf_size(self: & mut Self, max_buf_size: usize)
```



## tokio::fs::file::File::set_times

*Function*

Changes the timestamps on the underlying file.

# Errors

This function will return an error if the user lacks permission to change
timestamps on the underlying file. It may also return an error in other
os-specific unspecified cases.

# Examples

```no_run
use tokio::fs::{File, FileTimes};

# async fn dox() -> std::io::Result<()> {
let file = File::open("foo.txt").await?;
let times = FileTimes::new()
    .set_accessed(std::time::SystemTime::now())
    .set_modified(std::time::SystemTime::now());
file.set_times(times).await?;
# Ok(())
# }
```

```rust
fn set_times(self: &Self, times: FileTimes) -> io::Result<()>
```



## tokio::fs::file::File::sync_all

*Function*

Attempts to sync all OS-internal metadata to disk.

This function will attempt to ensure that all in-core data reaches the
filesystem before returning.

# Examples

```no_run
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

# async fn dox() -> std::io::Result<()> {
let mut file = File::create("foo.txt").await?;
file.write_all(b"hello, world!").await?;
file.sync_all().await?;
# Ok(())
# }
```

The [`write_all`] method is defined on the [`AsyncWriteExt`] trait.

[`write_all`]: fn@crate::io::AsyncWriteExt::write_all
[`AsyncWriteExt`]: trait@crate::io::AsyncWriteExt

```rust
fn sync_all(self: &Self) -> io::Result<()>
```



