**tracing_appender > rolling > builder**

# Module: rolling::builder

## Contents

**Structs**

- [`Builder`](#builder) - A [builder] for configuring [`RollingFileAppender`]s.
- [`InitError`](#initerror) - Errors returned by [`Builder::build`].

---

## tracing_appender::rolling::builder::Builder

*Struct*

A [builder] for configuring [`RollingFileAppender`]s.

[builder]: https://rust-unofficial.github.io/patterns/patterns/creational/builder.html

**Methods:**

- `fn new() -> Self` - Returns a new `Builder` for configuring a [`RollingFileAppender`], with
- `fn rotation(self: Self, rotation: Rotation) -> Self` - Sets the [rotation strategy] for log files.
- `fn filename_prefix<impl Into<String>>(self: Self, prefix: impl Trait) -> Self` - Sets the prefix for log filenames. The prefix is output before the
- `fn filename_suffix<impl Into<String>>(self: Self, suffix: impl Trait) -> Self` - Sets the suffix for log filenames. The suffix is output after the
- `fn max_log_files(self: Self, n: usize) -> Self` - Keeps the last `n` log files on disk.
- `fn latest_symlink<impl Into<String>>(self: Self, name: impl Trait) -> Self` - Create a symbolic link that points to the latest log file.
- `fn build<impl AsRef<Path>>(self: &Self, directory: impl Trait) -> Result<RollingFileAppender, InitError>` - Builds a new [`RollingFileAppender`] with the configured parameters,

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## tracing_appender::rolling::builder::InitError

*Struct*

Errors returned by [`Builder::build`].

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, __formatter: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Error**
  - `fn source(self: &Self) -> ::core::option::Option<&dyn ::thiserror::__private21::Error>`



