**ratatui_core > layout > alignment**

# Module: layout::alignment

## Contents

**Enums**

- [`HorizontalAlignment`](#horizontalalignment) - Horizontal content alignment within a layout area.
- [`VerticalAlignment`](#verticalalignment) - Vertical content alignment within a layout area.

**Type Aliases**

- [`Alignment`](#alignment) - A type alias for `HorizontalAlignment`.

---

## ratatui_core::layout::alignment::Alignment

*Type Alias*: `HorizontalAlignment`

A type alias for `HorizontalAlignment`.

Prior to Ratatui 0.30.0, [`HorizontalAlignment`] was named `Alignment`. This alias is provided
for backwards compatibility. Because this type is used almost everywhere in Ratatui related apps
and libraries, it's unlikely that this alias will be removed in the future.



## ratatui_core::layout::alignment::HorizontalAlignment

*Enum*

Horizontal content alignment within a layout area.

Prior to Ratatui 0.30.0, this type was named `Alignment`. In Ratatui 0.30.0, the name was
changed to `HorizontalAlignment` to make it more descriptive. The old name is still available as
an alias for backwards compatibility.

This type is used throughout Ratatui to control how content is positioned horizontally within
available space. It's commonly used with widgets to control text alignment, but can also be
used in layout calculations.

For comprehensive layout documentation and examples, see the [`layout`](crate::layout) module.

**Variants:**
- `Left`
- `Center`
- `Right`

**Traits:** Copy, Eq

**Trait Implementations:**

- **FromStr**
  - `fn from_str(s: &str) -> ::core::result::Result<HorizontalAlignment, <Self as ::core::str::FromStr>::Err>`
- **TryFrom**
  - `fn try_from(s: &str) -> ::core::result::Result<HorizontalAlignment, <Self as ::core::convert::TryFrom>::Error>`
- **Default**
  - `fn default() -> Self`
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



## ratatui_core::layout::alignment::VerticalAlignment

*Enum*

Vertical content alignment within a layout area.

This type is used to control how content is positioned vertically within available space.
It complements [`HorizontalAlignment`] to provide full 2D positioning control.

For comprehensive layout documentation and examples, see the [`layout`](crate::layout) module.

**Variants:**
- `Top`
- `Center`
- `Bottom`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::result::Result<(), ::core::fmt::Error>`
- **FromStr**
  - `fn from_str(s: &str) -> ::core::result::Result<VerticalAlignment, <Self as ::core::str::FromStr>::Err>`
- **TryFrom**
  - `fn try_from(s: &str) -> ::core::result::Result<VerticalAlignment, <Self as ::core::convert::TryFrom>::Error>`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



