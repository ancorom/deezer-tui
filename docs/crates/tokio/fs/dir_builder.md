**tokio > fs > dir_builder**

# Module: fs::dir_builder

## Contents

**Structs**

- [`DirBuilder`](#dirbuilder) - A builder for creating directories in various manners.

---

## tokio::fs::dir_builder::DirBuilder

*Struct*

A builder for creating directories in various manners.

This is a specialized version of [`std::fs::DirBuilder`] for usage on
the Tokio runtime.

**Methods:**

- `fn mode(self: & mut Self, mode: u32) -> & mut Self` - Sets the mode to create new directories with.
- `fn new() -> Self` - Creates a new set of options with default mode/security settings for all
- `fn recursive(self: & mut Self, recursive: bool) -> & mut Self` - Indicates whether to create directories recursively (including all parent directories).
- `fn create<impl AsRef<Path>>(self: &Self, path: impl Trait) -> io::Result<()>` - Creates the specified directory with the configured options.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`



