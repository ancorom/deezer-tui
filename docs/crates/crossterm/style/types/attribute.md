**crossterm > style > types > attribute**

# Module: style::types::attribute

## Contents

**Enums**

- [`Attribute`](#attribute) - Represents an attribute.

---

## crossterm::style::types::attribute::Attribute

*Enum*

Represents an attribute.

# Platform-specific Notes

* Only UNIX and Windows 10 terminals do support text attributes.
* Keep in mind that not all terminals support all attributes.
* Crossterm implements almost all attributes listed in the
  [SGR parameters](https://en.wikipedia.org/wiki/ANSI_escape_code#Select_Graphic_Rendition_parameters).

| Attribute | Windows | UNIX | Notes |
| :-- | :--: | :--: | :-- |
| `Reset` | ✓ | ✓ | |
| `Bold` | ✓ | ✓ | |
| `Dim` | ✓ | ✓ | |
| `Italic` | ? | ? | Not widely supported, sometimes treated as inverse. |
| `Underlined` | ✓ | ✓ | |
| `SlowBlink` | ? | ? | Not widely supported, sometimes treated as inverse. |
| `RapidBlink` | ? | ? | Not widely supported. MS-DOS ANSI.SYS; 150+ per minute. |
| `Reverse` | ✓ | ✓ | |
| `Hidden` | ✓ | ✓ | Also known as Conceal. |
| `Fraktur` | ✗ | ✓ | Legible characters, but marked for deletion. |
| `DefaultForegroundColor` | ? | ? | Implementation specific (according to standard). |
| `DefaultBackgroundColor` | ? | ? | Implementation specific (according to standard). |
| `Framed` | ? | ? | Not widely supported. |
| `Encircled` | ? | ? | This should turn on the encircled attribute. |
| `OverLined` | ? | ? | This should draw a line at the top of the text. |

# Examples

Basic usage:

```no_run
use crossterm::style::Attribute;

println!(
    "{} Underlined {} No Underline",
    Attribute::Underlined,
    Attribute::NoUnderline
);
```

Style existing text:

```no_run
use crossterm::style::Stylize;

println!("{}", "Bold text".bold());
println!("{}", "Underlined text".underlined());
println!("{}", "Negative text".negative());
```

**Variants:**
- `Reset` - Resets all the attributes.
- `Bold` - Increases the text intensity.
- `Dim` - Decreases the text intensity.
- `Italic` - Emphasises the text.
- `Underlined` - Underlines the text.
- `DoubleUnderlined` - Double underlines the text.
- `Undercurled` - Undercurls the text.
- `Underdotted` - Underdots the text.
- `Underdashed` - Underdashes the text.
- `SlowBlink` - Makes the text blinking (< 150 per minute).
- `RapidBlink` - Makes the text blinking (>= 150 per minute).
- `Reverse` - Swaps foreground and background colors.
- `Hidden` - Hides the text (also known as Conceal).
- `CrossedOut` - Crosses the text.
- `Fraktur` - Sets the [Fraktur](https://en.wikipedia.org/wiki/Fraktur) typeface.
- `NoBold` - Turns off the `Bold` attribute. - Inconsistent - Prefer to use NormalIntensity
- `NormalIntensity` - Switches the text back to normal intensity (no bold, italic).
- `NoItalic` - Turns off the `Italic` attribute.
- `NoUnderline` - Turns off the `Underlined` attribute.
- `NoBlink` - Turns off the text blinking (`SlowBlink` or `RapidBlink`).
- `NoReverse` - Turns off the `Reverse` attribute.
- `NoHidden` - Turns off the `Hidden` attribute.
- `NotCrossedOut` - Turns off the `CrossedOut` attribute.
- `Framed` - Makes the text framed.
- `Encircled` - Makes the text encircled.
- `OverLined` - Draws a line at the top of the text.
- `NotFramedOrEncircled` - Turns off the `Frame` and `Encircled` attributes.
- `NotOverLined` - Turns off the `OverLined` attribute.

**Methods:**

- `fn iterator() -> impl Trait` - Iterates over all the variants of the Attribute enum.
- `fn bytes(self: Self) -> u32` - Returns a u32 with one bit set, which is the
- `fn sgr(self: Self) -> String` - Returns the SGR attribute value.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::std::fmt::Formatter) -> std::fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



