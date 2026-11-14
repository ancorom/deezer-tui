**tempfile > env**

# Module: env

## Contents

**Functions**

- [`override_temp_dir`](#override_temp_dir) - Override the default temporary directory (defaults to [`std::env::temp_dir`]). This function
- [`temp_dir`](#temp_dir) - Returns the default temporary directory, used for both temporary directories and files if no

---

## tempfile::env::override_temp_dir

*Function*

Override the default temporary directory (defaults to [`std::env::temp_dir`]). This function
changes the _global_ default temporary directory for the entire program and should not be called
except in exceptional cases where it's not configured correctly by the platform. Applications
should first check if the path returned by [`env::temp_dir`] is acceptable.

If you're writing a library and want to control where your temporary files are placed, you
should instead use the `_in` variants of the various temporary file/directory constructors
([`tempdir_in`], [`tempfile_in`], the so-named functions on [`Builder`], etc.).

Only the first call to this function will succeed. All further calls will fail with `Err(path)`
where `path` is previously set default temporary directory override.

**NOTE:** This function does not check if the specified directory exists and/or is writable.

```rust
fn override_temp_dir(path: &std::path::Path) -> Result<(), std::path::PathBuf>
```



## tempfile::env::temp_dir

*Function*

Returns the default temporary directory, used for both temporary directories and files if no
directory is explicitly specified.

This function simply delegates to [`std::env::temp_dir`] unless the default temporary directory
has been overridden by a call to [`override_temp_dir`].

**NOTE:** This function does not check if the returned directory exists and/or is writable.

```rust
fn temp_dir() -> std::path::PathBuf
```



