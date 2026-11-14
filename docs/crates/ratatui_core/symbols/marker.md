**ratatui_core > symbols > marker**

# Module: symbols::marker

## Contents

**Enums**

- [`Marker`](#marker) - Marker to use when plotting data points

**Constants**

- [`DOT`](#dot)

---

## ratatui_core::symbols::marker::DOT

*Constant*: `&str`



## ratatui_core::symbols::marker::Marker

*Enum*

Marker to use when plotting data points

**Variants:**
- `Dot` - One point per cell in shape of dot (`•`)
- `Block` - One point per cell in shape of a block (`█`)
- `Bar` - One point per cell in the shape of a bar (`▄`)
- `Braille` - Use the [Unicode Braille Patterns](https://en.wikipedia.org/wiki/Braille_Patterns) block to
- `HalfBlock` - Use the unicode block and half block characters (`█`, `▄`, and `▀`) to represent points in
- `Quadrant` - Use quadrant characters to represent data points.
- `Sextant` - Use sextant characters from the [Unicode Symbols for Legacy Computing
- `Octant` - Use octant characters from the [Unicode Symbols for Legacy Computing
- `Custom(char)` - Custom marker where the supplied char is applied once per cell

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::result::Result<(), ::core::fmt::Error>`
- **FromStr**
  - `fn from_str(s: &str) -> ::core::result::Result<Marker, <Self as ::core::str::FromStr>::Err>`
- **TryFrom**
  - `fn try_from(s: &str) -> ::core::result::Result<Marker, <Self as ::core::convert::TryFrom>::Error>`
- **Default**
  - `fn default() -> Self`



