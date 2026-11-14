**tempfile > Builder**

# Module: Builder

## Contents

**Functions**

- [`append`](#append) - Configure the file to be opened in append-only mode.
- [`disable_cleanup`](#disable_cleanup) - Disable cleanup of the file/folder to even when the [`NamedTempFile`]/[`TempDir`] goes out
- [`make`](#make) - Attempts to create a temporary file (or file-like object) using the
- [`make_in`](#make_in) - This is the same as [`Builder::make`], except `dir` is used as the base
- [`permissions`](#permissions) - Set the permissions for the new temporary file/directory.
- [`rand_bytes`](#rand_bytes) - Set the number of random bytes.
- [`tempfile`](#tempfile) - Create the named temporary file.

---

## tempfile::Builder::append

*Function*

Configure the file to be opened in append-only mode.

Default: `false`.

# Examples

```
use tempfile::Builder;

let named_tempfile = Builder::new()
    .append(true)
    .tempfile()?;
# Ok::<(), std::io::Error>(())
```

```rust
fn append(self: & mut Self, append: bool) -> & mut Self
```



## tempfile::Builder::disable_cleanup

*Function*

Disable cleanup of the file/folder to even when the [`NamedTempFile`]/[`TempDir`] goes out
of scope. Prefer [`NamedTempFile::keep`] and [`TempDir::keep`] where possible;
`disable_cleanup` is provided for testing & debugging.

By default, the file/folder is automatically cleaned up in the destructor of
[`NamedTempFile`]/[`TempDir`]. When `disable_cleanup` is set to `true`, this behavior is
suppressed. If you wish to disable cleanup after creating a temporary file/directory, call
[`NamedTempFile::disable_cleanup`] or [`TempDir::disable_cleanup`].

# Warnings

On some platforms (for now, only Windows), temporary files are marked with a special
"temporary file" (`FILE_ATTRIBUTE_TEMPORARY`) attribute. Disabling cleanup _will not_ unset
this attribute while calling [`NamedTempFile::keep`] will.

# Examples

```
use tempfile::Builder;

let named_tempfile = Builder::new()
    .disable_cleanup(true)
    .tempfile()?;
# Ok::<(), std::io::Error>(())
```

```rust
fn disable_cleanup(self: & mut Self, disable_cleanup: bool) -> & mut Self
```



## tempfile::Builder::make

*Function*

Attempts to create a temporary file (or file-like object) using the
provided closure. The closure is passed a temporary file path and
returns an [`std::io::Result`]. The path provided to the closure will be
inside of [`env::temp_dir()`]. Use [`Builder::make_in`] to provide
a custom temporary directory. If the closure returns one of the
following errors, then another randomized file path is tried:
 - [`std::io::ErrorKind::AlreadyExists`]
 - [`std::io::ErrorKind::AddrInUse`]

This can be helpful for taking full control over the file creation, but
leaving the temporary file path construction up to the library. This
also enables creating a temporary UNIX domain socket, since it is not
possible to bind to a socket that already exists.

Note that [`Builder::append`] is ignored when using [`Builder::make`].

# Security

This has the same [security implications][security] as
[`NamedTempFile`], but with additional caveats. Specifically, it is up
to the closure to ensure that the file does not exist and that such a
check is *atomic*. Otherwise, a [time-of-check to time-of-use
bug][TOCTOU] could be introduced.

For example, the following is **not** secure:

```
use std::fs::File;
use tempfile::Builder;

// This is NOT secure!
let tempfile = Builder::new().make(|path| {
    if path.is_file() {
        return Err(std::io::ErrorKind::AlreadyExists.into());
    }

    // Between the check above and the usage below, an attacker could
    // have replaced `path` with another file, which would get truncated
    // by `File::create`.

    File::create(path)
})?;
# Ok::<(), std::io::Error>(())
```

Note that simply using [`std::fs::File::create`] alone is not correct
because it does not fail if the file already exists:

```
use tempfile::Builder;
use std::fs::File;

// This could overwrite an existing file!
let tempfile = Builder::new().make(|path| File::create(path))?;
# Ok::<(), std::io::Error>(())
```
For creating regular temporary files, use [`Builder::tempfile`] instead
to avoid these problems. This function is meant to enable more exotic
use-cases.

# Resource leaking

See [the resource leaking][resource-leaking] docs on `NamedTempFile`.

# Errors

If the closure returns any error besides
[`std::io::ErrorKind::AlreadyExists`] or
[`std::io::ErrorKind::AddrInUse`], then `Err` is returned.

# Examples
```
# #[cfg(unix)]
# {
use std::os::unix::net::UnixListener;
use tempfile::Builder;

let tempsock = Builder::new().make(|path| UnixListener::bind(path))?;
# }
# Ok::<(), std::io::Error>(())
```

[TOCTOU]: https://en.wikipedia.org/wiki/Time-of-check_to_time-of-use
[security]: struct.NamedTempFile.html#security
[resource-leaking]: struct.NamedTempFile.html#resource-leaking

```rust
fn make<F, R>(self: &Self, f: F) -> io::Result<NamedTempFile<R>>
```



## tempfile::Builder::make_in

*Function*

This is the same as [`Builder::make`], except `dir` is used as the base
directory for the temporary file path.

See [`Builder::make`] for more details and security implications.

# Examples
```
# #[cfg(unix)]
# {
use tempfile::Builder;
use std::os::unix::net::UnixListener;

let tempsock = Builder::new().make_in("./", |path| UnixListener::bind(path))?;
# }
# Ok::<(), std::io::Error>(())
```

```rust
fn make_in<F, R, P>(self: &Self, dir: P, f: F) -> io::Result<NamedTempFile<R>>
```



## tempfile::Builder::permissions

*Function*

Set the permissions for the new temporary file/directory.

# Platform Notes

## Windows

This setting is only fully-supported on unix-like platforms. On Windows, if this method is
called with a [`Permissions`] object where `permissions.readonly` returns true, creating
temporary files and directories will fail with an error.

## Unix

On unix-like systems, the actual permission bits set on the tempfile or tempdir will be
affected by the `umask` applied by the underlying syscall. The actual permission bits are
calculated via `permissions & !umask`. In other words, depending on your umask, the
permissions of the created file may be more restrictive (but never more permissive) than the
ones you specified.

Permissions default to `0o600` for tempfiles and `0o777` for tempdirs. Note, this doesn't
include effects of the current `umask`. For example, combined with the standard umask
`0o022`, the defaults yield `0o600` for tempfiles and `0o755` for tempdirs.

## WASI

While custom permissions are allowed on WASI, they will be ignored as the platform has no
concept of permissions or file modes (or multiple users for that matter).

# Examples

Create a named temporary file that is world-readable.

```
# #[cfg(unix)]
# {
use tempfile::Builder;
use std::os::unix::fs::PermissionsExt;

let all_read_write = std::fs::Permissions::from_mode(0o666);
let tempfile = Builder::new().permissions(all_read_write).tempfile()?;

// Check that this worked and that the file is world-readable.
//
// NOTE: the file likely won't actually be created with 0o666 permissions because it's
// restricted by the user's umask.
//
// NOTE: This test will fail if the user's umask is, e.g., 0o066.
let actual_permissions = tempfile.path().metadata()?.permissions();
assert_eq!(actual_permissions.mode() & 0o044, 0o044);
# }
# Ok::<(), std::io::Error>(())
```

Create a named temporary directory that is restricted to the owner.

```
# #[cfg(unix)]
# {
use tempfile::Builder;
use std::os::unix::fs::PermissionsExt;

let owner_rwx = std::fs::Permissions::from_mode(0o700);
let tempdir = Builder::new().permissions(owner_rwx).tempdir()?;
let actual_permissions = tempdir.path().metadata()?.permissions();
assert_eq!(
    actual_permissions.mode() & !0o170000,
    0o700,
    "we get the narrow permissions we asked for"
);
# }
# Ok::<(), std::io::Error>(())
```

```rust
fn permissions(self: & mut Self, permissions: Permissions) -> & mut Self
```



## tempfile::Builder::rand_bytes

*Function*

Set the number of random bytes.

Default: `6`.

# Examples

```
use tempfile::Builder;

let named_tempfile = Builder::new()
    .rand_bytes(5)
    .tempfile()?;
# Ok::<(), std::io::Error>(())
```

```rust
fn rand_bytes(self: & mut Self, rand: usize) -> & mut Self
```



## tempfile::Builder::tempfile

*Function*

Create the named temporary file.

# Security

See [the security][security] docs on `NamedTempFile`.

# Resource leaking

See [the resource leaking][resource-leaking] docs on `NamedTempFile`.

# Errors

If the file cannot be created, `Err` is returned.

# Examples

```
use tempfile::Builder;

let tempfile = Builder::new().tempfile()?;
# Ok::<(), std::io::Error>(())
```

[security]: struct.NamedTempFile.html#security
[resource-leaking]: struct.NamedTempFile.html#resource-leaking

```rust
fn tempfile(self: &Self) -> io::Result<NamedTempFile>
```



