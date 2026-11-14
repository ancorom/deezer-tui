**ratatui_image > errors**

# Module: errors

## Contents

**Enums**

- [`Errors`](#errors)

---

## ratatui_image::errors::Errors

*Enum*

**Variants:**
- `NoFontSize`
- `NoCap`
- `NoStdinResponse`
- `Sixel(String)`
- `Tmux(&'static str)`
- `Io(std::io::Error)`
- `Image(image::error::ImageError)`

**Trait Implementations:**

- **From**
  - `fn from(errno: rustix::io::Errno) -> Self`
- **From**
  - `fn from(source: std::io::Error) -> Self`
- **Error**
  - `fn source(self: &Self) -> ::core::option::Option<&dyn std::error::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(source: image::error::ImageError) -> Self`
- **Display**
  - `fn fmt(self: &Self, __formatter: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`



