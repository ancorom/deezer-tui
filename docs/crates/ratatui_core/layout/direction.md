**ratatui_core > layout > direction**

# Module: layout::direction

## Contents

**Enums**

- [`Direction`](#direction) - Defines the direction of a layout.

---

## ratatui_core::layout::direction::Direction

*Enum*

Defines the direction of a layout.

This enumeration is used with [`Layout`](crate::layout::Layout) to specify whether layout
segments should be arranged horizontally or vertically.

- `Horizontal`: Layout segments are arranged side by side (left to right)
- `Vertical`: Layout segments are arranged top to bottom (default)

For comprehensive layout documentation and examples, see the [`layout`](crate::layout) module.

**Variants:**
- `Horizontal` - Layout segments are arranged side by side (left to right).
- `Vertical` - Layout segments are arranged top to bottom (default).

**Methods:**

- `fn perpendicular(self: Self) -> Self` - The perpendicular direction to this direction.

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
  - `fn from_str(s: &str) -> ::core::result::Result<Direction, <Self as ::core::str::FromStr>::Err>`
- **TryFrom**
  - `fn try_from(s: &str) -> ::core::result::Result<Direction, <Self as ::core::convert::TryFrom>::Error>`
- **Default**
  - `fn default() -> Self`



