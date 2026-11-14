**crossterm > style > types > colored**

# Module: style::types::colored

## Contents

**Enums**

- [`Colored`](#colored) - Represents a foreground or background color.

---

## crossterm::style::types::colored::Colored

*Enum*

Represents a foreground or background color.

This can be converted to a [Colors](struct.Colors.html) by calling `into()` and applied
using the [SetColors](struct.SetColors.html) command.

**Variants:**
- `ForegroundColor(crate::style::Color)` - A foreground color.
- `BackgroundColor(crate::style::Color)` - A background color.
- `UnderlineColor(crate::style::Color)` - An underline color.

**Methods:**

- `fn parse_ansi(ansi: &str) -> Option<Self>` - Parse an ANSI foreground or background color.
- `fn ansi_color_disabled() -> bool` - Checks whether ansi color sequences are disabled by setting of NO_COLOR
- `fn ansi_color_disabled_memoized() -> bool`
- `fn set_ansi_color_disabled(val: bool)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> Self`



