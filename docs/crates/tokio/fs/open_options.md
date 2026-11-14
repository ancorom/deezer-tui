**tokio > fs > open_options**

# Module: fs::open_options

## Contents

**Structs**

- [`OpenOptions`](#openoptions) - Options and flags which can be used to configure how a file is opened.

---

## tokio::fs::open_options::OpenOptions

*Struct*

Options and flags which can be used to configure how a file is opened.

This builder exposes the ability to configure how a [`File`] is opened and
what operations are permitted on the open file. The [`File::open`] and
[`File::create`] methods are aliases for commonly used options using this
builder.

Generally speaking, when using `OpenOptions`, you'll first call [`new`],
then chain calls to methods to set each option, then call [`open`], passing
the path of the file you're trying to open. This will give you a
[`io::Result`] with a [`File`] inside that you can further operate
on.

This is a specialized version of [`std::fs::OpenOptions`] for usage from
the Tokio runtime.

`From<std::fs::OpenOptions>` is implemented for more advanced configuration
than the methods provided here.

[`new`]: OpenOptions::new
[`open`]: OpenOptions::open
[`File`]: File
[`File::open`]: File::open
[`File::create`]: File::create

# Examples

Opening a file to read:

```no_run
use tokio::fs::OpenOptions;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .open("foo.txt")
        .await?;

    Ok(())
}
```

Opening a file for both reading and writing, as well as creating it if it
doesn't exist:

```no_run
use tokio::fs::OpenOptions;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("foo.txt")
        .await?;

    Ok(())
}
```

**Methods:**

- `fn new() -> OpenOptions` - Creates a blank new set of options ready for configuration.
- `fn read(self: & mut Self, read: bool) -> & mut OpenOptions` - Sets the option for read access.
- `fn write(self: & mut Self, write: bool) -> & mut OpenOptions` - Sets the option for write access.
- `fn append(self: & mut Self, append: bool) -> & mut OpenOptions` - Sets the option for the append mode.
- `fn truncate(self: & mut Self, truncate: bool) -> & mut OpenOptions` - Sets the option for truncating a previous file.
- `fn create(self: & mut Self, create: bool) -> & mut OpenOptions` - Sets the option for creating a new file.
- `fn create_new(self: & mut Self, create_new: bool) -> & mut OpenOptions` - Sets the option to always create a new file.
- `fn open<impl AsRef<Path>>(self: &Self, path: impl Trait) -> io::Result<File>` - Opens a file at `path` with the options specified by `self`.
- `fn mode(self: & mut Self, mode: u32) -> & mut OpenOptions` - Sets the mode bits that a new file will be created with.
- `fn custom_flags(self: & mut Self, flags: i32) -> & mut OpenOptions` - Passes custom flags to the `flags` argument of `open`.

**Trait Implementations:**

- **From**
  - `fn from(options: StdOpenOptions) -> OpenOptions`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



