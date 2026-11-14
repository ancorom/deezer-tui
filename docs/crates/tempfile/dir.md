**tempfile > dir**

# Module: dir

## Contents

**Structs**

- [`TempDir`](#tempdir) - A directory in the filesystem that is automatically deleted when

**Functions**

- [`tempdir`](#tempdir) - Create a new temporary directory. Also see [`tempdir_in`].
- [`tempdir_in`](#tempdir_in) - Create a new temporary directory in a specific directory. Also see [`tempdir`].

---

## tempfile::dir::TempDir

*Struct*

A directory in the filesystem that is automatically deleted when
it goes out of scope.

The [`TempDir`] type creates a directory on the file system that
is deleted once it goes out of scope. At construction, the
`TempDir` creates a new directory with a randomly generated name.

The default constructor, [`TempDir::new()`], creates directories in
the location returned by [`env::temp_dir()`], but `TempDir`
can be configured to manage a temporary directory in any location
by constructing with a [`Builder`].

After creating a `TempDir`, work with the file system by doing
standard [`std::fs`] file system operations on its [`Path`],
which can be retrieved with [`TempDir::path()`]. Once the `TempDir`
value is dropped, the directory at the path will be deleted, along
with any files and directories it contains. It is your responsibility
to ensure that no further file system operations are attempted
inside the temporary directory once it has been deleted.

# Resource Leaking

Various platform-specific conditions may cause `TempDir` to fail
to delete the underlying directory. It's important to ensure that
handles (like [`File`] and [`ReadDir`]) to files inside the
directory are dropped before the `TempDir` goes out of scope. The
`TempDir` destructor will silently ignore any errors in deleting
the directory; to instead handle errors call [`TempDir::close()`].

Note that if the program exits before the `TempDir` destructor is
run, such as via [`std::process::exit()`], by segfaulting, or by
receiving a signal like `SIGINT`, then the temporary directory
will not be deleted.

# Examples

Create a temporary directory with a generated name:

```
use std::fs::File;
use std::io::Write;
use tempfile::TempDir;

// Create a directory inside of `env::temp_dir()`
let tmp_dir = TempDir::new()?;
# Ok::<(), std::io::Error>(())
```

Create a temporary directory with a prefix in its name:

```
use std::fs::File;
use std::io::Write;
use tempfile::Builder;

// Create a directory inside of `env::temp_dir()`,
// whose name will begin with 'example'.
let tmp_dir = Builder::new().prefix("example").tempdir()?;
# Ok::<(), std::io::Error>(())
```

[`File`]: http://doc.rust-lang.org/std/fs/struct.File.html
[`Path`]: http://doc.rust-lang.org/std/path/struct.Path.html
[`ReadDir`]: http://doc.rust-lang.org/std/fs/struct.ReadDir.html
[`Builder`]: struct.Builder.html
[`TempDir::close()`]: struct.TempDir.html#method.close
[`TempDir::new()`]: struct.TempDir.html#method.new
[`TempDir::path()`]: struct.TempDir.html#method.path
[`TempDir`]: struct.TempDir.html
[`std::fs`]: http://doc.rust-lang.org/std/fs/index.html
[`std::process::exit()`]: http://doc.rust-lang.org/std/process/fn.exit.html

**Methods:**

- `fn new() -> io::Result<TempDir>` - Attempts to make a temporary directory inside of `env::temp_dir()`.
- `fn new_in<P>(dir: P) -> io::Result<TempDir>` - Attempts to make a temporary directory inside of `dir`.
- `fn with_prefix<S>(prefix: S) -> io::Result<TempDir>` - Attempts to make a temporary directory with the specified prefix inside of
- `fn with_suffix<S>(suffix: S) -> io::Result<TempDir>` - Attempts to make a temporary directory with the specified suffix inside of
- `fn with_suffix_in<S, P>(suffix: S, dir: P) -> io::Result<TempDir>` - Attempts to make a temporary directory with the specified prefix inside
- `fn with_prefix_in<S, P>(prefix: S, dir: P) -> io::Result<TempDir>` - Attempts to make a temporary directory with the specified prefix inside
- `fn path(self: &Self) -> &path::Path` - Accesses the [`Path`] to the temporary directory.
- `fn into_path(self: Self) -> PathBuf` - Deprecated alias for [`TempDir::keep`].
- `fn keep(self: Self) -> PathBuf` - Persist the temporary directory to disk, returning the [`PathBuf`] where it is located.
- `fn disable_cleanup(self: & mut Self, disable_cleanup: bool)` - Disable cleanup of the temporary directory. If `disable_cleanup` is `true`, the temporary
- `fn close(self: Self) -> io::Result<()>` - Closes and removes the temporary directory, returning a `Result`.

**Trait Implementations:**

- **AsRef**
  - `fn as_ref(self: &Self) -> &Path`
- **Drop**
  - `fn drop(self: & mut Self)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## tempfile::dir::tempdir

*Function*

Create a new temporary directory. Also see [`tempdir_in`].

The `tempdir` function creates a directory in the file system and returns a
[`TempDir`]. The directory will be automatically deleted when the `TempDir`'s
destructor is run.

# Resource Leaking

See [the resource leaking][resource-leaking] docs on `TempDir`.

# Security

Temporary directories are created with the default permissions unless otherwise
specified via [`Builder::permissions`]. Depending on your platform, this may make
them world-readable.

# Errors

If the directory can not be created, `Err` is returned.

# Examples

```
use tempfile::tempdir;
use std::fs::File;
use std::io::Write;

// Create a directory inside of `env::temp_dir()`
let tmp_dir = tempdir()?;

let file_path = tmp_dir.path().join("my-temporary-note.txt");
let mut tmp_file = File::create(file_path)?;
writeln!(tmp_file, "Brian was here. Briefly.")?;

// `tmp_dir` goes out of scope, the directory as well as
// `tmp_file` will be deleted here.
drop(tmp_file);
tmp_dir.close()?;
# Ok::<(), std::io::Error>(())
```

[`TempDir`]: struct.TempDir.html
[resource-leaking]: struct.TempDir.html#resource-leaking

```rust
fn tempdir() -> io::Result<TempDir>
```



## tempfile::dir::tempdir_in

*Function*

Create a new temporary directory in a specific directory. Also see [`tempdir`].

The `tempdir_in` function creates a directory in the specified directory
and returns a [`TempDir`].
The directory will be automatically deleted when the `TempDir`s
destructor is run.

# Resource Leaking

See [the resource leaking][resource-leaking] docs on `TempDir`.

# Errors

If the directory can not be created, `Err` is returned.

# Examples

```
use tempfile::tempdir_in;
use std::fs::File;
use std::io::Write;

// Create a directory inside of the current directory.
let tmp_dir = tempdir_in(".")?;

let file_path = tmp_dir.path().join("my-temporary-note.txt");
let mut tmp_file = File::create(file_path)?;
writeln!(tmp_file, "Brian was here. Briefly.")?;

// `tmp_dir` goes out of scope, the directory as well as
// `tmp_file` will be deleted here.
drop(tmp_file);
tmp_dir.close()?;
# Ok::<(), std::io::Error>(())
```

[`TempDir`]: struct.TempDir.html
[resource-leaking]: struct.TempDir.html#resource-leaking

```rust
fn tempdir_in<P>(dir: P) -> io::Result<TempDir>
```



