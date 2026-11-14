**ratatui_core > style > color**

# Module: style::color

## Contents

**Structs**

- [`ParseColorError`](#parsecolorerror) - Error type indicating a failure to parse a color string.

**Enums**

- [`Color`](#color) - ANSI Color

---

## ratatui_core::style::color::Color

*Enum*

ANSI Color

All colors from the [ANSI color table] are supported (though some names are not exactly the
same).

| Color Name     | Color                   | Foreground | Background |
|----------------|-------------------------|------------|------------|
| `black`        | [`Color::Black`]        | 30         | 40         |
| `red`          | [`Color::Red`]          | 31         | 41         |
| `green`        | [`Color::Green`]        | 32         | 42         |
| `yellow`       | [`Color::Yellow`]       | 33         | 43         |
| `blue`         | [`Color::Blue`]         | 34         | 44         |
| `magenta`      | [`Color::Magenta`]      | 35         | 45         |
| `cyan`         | [`Color::Cyan`]         | 36         | 46         |
| `gray`*        | [`Color::Gray`]         | 37         | 47         |
| `darkgray`*    | [`Color::DarkGray`]     | 90         | 100        |
| `lightred`     | [`Color::LightRed`]     | 91         | 101        |
| `lightgreen`   | [`Color::LightGreen`]   | 92         | 102        |
| `lightyellow`  | [`Color::LightYellow`]  | 93         | 103        |
| `lightblue`    | [`Color::LightBlue`]    | 94         | 104        |
| `lightmagenta` | [`Color::LightMagenta`] | 95         | 105        |
| `lightcyan`    | [`Color::LightCyan`]    | 96         | 106        |
| `white`*       | [`Color::White`]        | 97         | 107        |

- `gray` is sometimes called `white` - this is not supported as we use `white` for bright white
- `gray` is sometimes called `silver` - this is supported
- `darkgray` is sometimes called `light black` or `bright black` (both are supported)
- `white` is sometimes called `light white` or `bright white` (both are supported)
- we support `bright` and `light` prefixes for all colors
- we support `-` and `_` and ` ` as separators for all colors
- we support both `gray` and `grey` spellings

`From<Color> for Style` is implemented by creating a style with the foreground color set to the
given color. This allows you to use colors anywhere that accepts `Into<Style>`.

# Example

```
use std::str::FromStr;

use ratatui_core::style::Color;

assert_eq!(Color::from_str("red"), Ok(Color::Red));
assert_eq!("red".parse(), Ok(Color::Red));
assert_eq!("lightred".parse(), Ok(Color::LightRed));
assert_eq!("light red".parse(), Ok(Color::LightRed));
assert_eq!("light-red".parse(), Ok(Color::LightRed));
assert_eq!("light_red".parse(), Ok(Color::LightRed));
assert_eq!("lightRed".parse(), Ok(Color::LightRed));
assert_eq!("bright red".parse(), Ok(Color::LightRed));
assert_eq!("bright-red".parse(), Ok(Color::LightRed));
assert_eq!("silver".parse(), Ok(Color::Gray));
assert_eq!("dark-grey".parse(), Ok(Color::DarkGray));
assert_eq!("dark gray".parse(), Ok(Color::DarkGray));
assert_eq!("light-black".parse(), Ok(Color::DarkGray));
assert_eq!("white".parse(), Ok(Color::White));
assert_eq!("bright white".parse(), Ok(Color::White));
```

[ANSI color table]: https://en.wikipedia.org/wiki/ANSI_escape_code#Colors

**Variants:**
- `Reset` - Resets the foreground or background color
- `Black` - ANSI Color: Black. Foreground: 30, Background: 40
- `Red` - ANSI Color: Red. Foreground: 31, Background: 41
- `Green` - ANSI Color: Green. Foreground: 32, Background: 42
- `Yellow` - ANSI Color: Yellow. Foreground: 33, Background: 43
- `Blue` - ANSI Color: Blue. Foreground: 34, Background: 44
- `Magenta` - ANSI Color: Magenta. Foreground: 35, Background: 45
- `Cyan` - ANSI Color: Cyan. Foreground: 36, Background: 46
- `Gray` - ANSI Color: White. Foreground: 37, Background: 47
- `DarkGray` - ANSI Color: Bright Black. Foreground: 90, Background: 100
- `LightRed` - ANSI Color: Bright Red. Foreground: 91, Background: 101
- `LightGreen` - ANSI Color: Bright Green. Foreground: 92, Background: 102
- `LightYellow` - ANSI Color: Bright Yellow. Foreground: 93, Background: 103
- `LightBlue` - ANSI Color: Bright Blue. Foreground: 94, Background: 104
- `LightMagenta` - ANSI Color: Bright Magenta. Foreground: 95, Background: 105
- `LightCyan` - ANSI Color: Bright Cyan. Foreground: 96, Background: 106
- `White` - ANSI Color: Bright White. Foreground: 97, Background: 107
- `Rgb(u8, u8, u8)` - An RGB color.
- `Indexed(u8)` - An 8-bit 256 color.

**Methods:**

- `fn from_u32(u: u32) -> Self` - Convert a u32 to a Color

**Traits:** Eq, Copy

**Trait Implementations:**

- **From**
  - `fn from([r, g, b]: [u8; 3]) -> Self` - Converts an array of 3 u8 values to a `Color::Rgb` instance.
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **From**
  - `fn from((r, g, b, _): (u8, u8, u8, u8)) -> Self` - Converts a tuple of 4 u8 values to a `Color::Rgb` instance (ignoring the alpha value).
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **From**
  - `fn from([r, g, b, _]: [u8; 4]) -> Self` - Converts an array of 4 u8 values to a `Color::Rgb` instance (ignoring the alpha value).
- **FromStr**
  - `fn from_str(s: &str) -> Result<Self, <Self as >::Err>`
- **Default**
  - `fn default() -> Self`
- **From**
  - `fn from((r, g, b): (u8, u8, u8)) -> Self` - Converts a tuple of 3 u8 values to a `Color::Rgb` instance.
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ratatui_core::style::color::ParseColorError

*Struct*

Error type indicating a failure to parse a color string.

**Unit Struct**

**Traits:** Copy, Error, Eq

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



