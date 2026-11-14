**ratatui_core > buffer > cell**

# Module: buffer::cell

## Contents

**Structs**

- [`Cell`](#cell) - A buffer cell

**Enums**

- [`CellDiffOption`](#celldiffoption) - Cell diffing options

---

## ratatui_core::buffer::cell::Cell

*Struct*

A buffer cell

**Fields:**
- `fg: crate::style::Color` - The foreground color of the cell.
- `bg: crate::style::Color` - The background color of the cell.
- `underline_color: crate::style::Color` - The underline color of the cell.
- `modifier: crate::style::Modifier` - The modifier of the cell.
- `diff_option: CellDiffOption` - Special option applied when copying (diffing) the buffer to the screen (or another buffer).
- `skip: bool` - Whether the cell should be skipped when copying (diffing) the buffer to the screen.

**Methods:**

- `fn new(symbol: &'static str) -> Self` - Creates a new `Cell` with the given symbol.
- `fn symbol(self: &Self) -> &str` - Gets the symbol of the cell.
- `fn merge_symbol(self: & mut Self, symbol: &str, strategy: MergeStrategy) -> & mut Self` - Merges the symbol of the cell with the one already on the cell, using the provided
- `fn set_symbol(self: & mut Self, symbol: &str) -> & mut Self` - Sets the symbol of the cell.
- `fn set_char(self: & mut Self, ch: char) -> & mut Self` - Sets the symbol of the cell to a single character.
- `fn set_fg(self: & mut Self, color: Color) -> & mut Self` - Sets the foreground color of the cell.
- `fn set_bg(self: & mut Self, color: Color) -> & mut Self` - Sets the background color of the cell.
- `fn set_style<S>(self: & mut Self, style: S) -> & mut Self` - Sets the style of the cell.
- `fn style(self: &Self) -> Style` - Returns the style of the cell.
- `fn set_skip(self: & mut Self, skip: bool) -> & mut Self` - Sets the cell to be skipped when copying (diffing) the buffer to the screen.
- `fn set_diff_option(self: & mut Self, diff_option: CellDiffOption) -> & mut Self` - Sets cell [`CellDiffOption`].
- `fn reset(self: & mut Self)` - Resets the cell to the empty state.

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Default**
  - `fn default() -> Self`
- **CellWidth**
  - `fn cell_width(self: &Self) -> u16` - Returns [`CellDiffOption::ForcedWidth`] when set, otherwise computes the width from the
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(ch: char) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool` - Compares two `Cell`s for equality.
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)` - Hashes the cell.



## ratatui_core::buffer::cell::CellDiffOption

*Enum*

Cell diffing options

**Variants:**
- `None` - No special option.
- `Skip` - Skip this cell when diffing.
- `AlwaysUpdate` - Always update this cell when diffing.
- `ForcedWidth(core::num::NonZeroU16)` - Force a width regardless of the symbol text width.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Default**
  - `fn default() -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



