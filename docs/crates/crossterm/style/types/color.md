**crossterm > style > types > color**

# Module: style::types::color

## Contents

**Enums**

- [`Color`](#color) - Represents a color.

---

## crossterm::style::types::color::Color

*Enum*

Represents a color.

# Platform-specific Notes

The following list of 16 base colors are available for almost all terminals (Windows 7 and 8 included).

| Light      | Dark          |
| :--------- | :------------ |
| `DarkGrey` | `Black`       |
| `Red`      | `DarkRed`     |
| `Green`    | `DarkGreen`   |
| `Yellow`   | `DarkYellow`  |
| `Blue`     | `DarkBlue`    |
| `Magenta`  | `DarkMagenta` |
| `Cyan`     | `DarkCyan`    |
| `White`    | `Grey`        |

Most UNIX terminals and Windows 10 consoles support additional colors.
See [`Color::Rgb`] or [`Color::AnsiValue`] for more info.

**Variants:**
- `Reset` - Resets the terminal color.
- `Black` - Black color.
- `DarkGrey` - Dark grey color.
- `Red` - Light red color.
- `DarkRed` - Dark red color.
- `Green` - Light green color.
- `DarkGreen` - Dark green color.
- `Yellow` - Light yellow color.
- `DarkYellow` - Dark yellow color.
- `Blue` - Light blue color.
- `DarkBlue` - Dark blue color.
- `Magenta` - Light magenta color.
- `DarkMagenta` - Dark magenta color.
- `Cyan` - Light cyan color.
- `DarkCyan` - Dark cyan color.
- `White` - White color.
- `Grey` - Grey color.
- `Rgb{ r: u8, g: u8, b: u8 }` - An RGB color. See [RGB color model](https://en.wikipedia.org/wiki/RGB_color_model) for more info.
- `AnsiValue(u8)` - An ANSI color. See [256 colors - cheat sheet](https://jonasjacek.github.io/colors/) for more info.

**Methods:**

- `fn parse_ansi(ansi: &str) -> Option<Self>` - Parses an ANSI color sequence.

**Traits:** Eq, Copy

**Trait Implementations:**

- **TryFrom**
  - `fn try_from(src: &str) -> Result<Self, <Self as >::Error>` - Try to create a `Color` from the string representation. This returns an error if the string does not match.
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(val: (u8, u8, u8)) -> Self` - Creates a 'Color' from the tuple representation.
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **FromStr**
  - `fn from_str(src: &str) -> Result<Self, <Self as >::Err>` - Creates a `Color` from the string representation.
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



