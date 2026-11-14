**tempfile**

# Module: tempfile

## Contents

**Modules**

- [`env`](#env)

**Structs**

- [`Builder`](#builder) - Create a new temporary file or directory with custom options.

---

## tempfile::Builder

*Struct*

Create a new temporary file or directory with custom options.

**Generic Parameters:**
- 'a
- 'b

**Methods:**

- `fn new() -> Self` - Create a new `Builder`.
- `fn prefix<S>(self: & mut Self, prefix: &'a S) -> & mut Self` - Set a custom filename prefix.
- `fn suffix<S>(self: & mut Self, suffix: &'b S) -> & mut Self` - Set a custom filename suffix.
- `fn rand_bytes(self: & mut Self, rand: usize) -> & mut Self` - Set the number of random bytes.
- `fn append(self: & mut Self, append: bool) -> & mut Self` - Configure the file to be opened in append-only mode.
- `fn permissions(self: & mut Self, permissions: Permissions) -> & mut Self` - Set the permissions for the new temporary file/directory.
- `fn disable_cleanup(self: & mut Self, disable_cleanup: bool) -> & mut Self` - Disable cleanup of the file/folder to even when the [`NamedTempFile`]/[`TempDir`] goes out
- `fn keep(self: & mut Self, keep: bool) -> & mut Self` - Deprecated alias for [`Builder::disable_cleanup`].
- `fn tempfile(self: &Self) -> io::Result<NamedTempFile>` - Create the named temporary file.
- `fn tempfile_in<P>(self: &Self, dir: P) -> io::Result<NamedTempFile>` - Create the named temporary file in the specified directory.
- `fn tempdir(self: &Self) -> io::Result<TempDir>` - Attempts to make a temporary directory inside of [`env::temp_dir()`] whose
- `fn tempdir_in<P>(self: &Self, dir: P) -> io::Result<TempDir>` - Attempts to make a temporary directory inside of `dir`.
- `fn make<F, R>(self: &Self, f: F) -> io::Result<NamedTempFile<R>>` - Attempts to create a temporary file (or file-like object) using the
- `fn make_in<F, R, P>(self: &Self, dir: P, f: F) -> io::Result<NamedTempFile<R>>` - This is the same as [`Builder::make`], except `dir` is used as the base

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## Module: env



