**ratatui_core > layout > size**

# Module: layout::size

## Contents

**Structs**

- [`Size`](#size) - A simple size struct for representing dimensions in the terminal.

---

## ratatui_core::layout::size::Size

*Struct*

A simple size struct for representing dimensions in the terminal.

The width and height are stored as `u16` values and represent the number of columns and rows
respectively. This is used throughout the layout system to represent dimensions of rectangular
areas and other layout elements.

Size can be created from tuples, extracted from rectangular areas, or constructed directly.
It's commonly used in conjunction with [`Position`](crate::layout::Position) to define
rectangular areas.

# Construction

- [`new`](Self::new) - Create a new size from width and height
- [`default`](Default::default) - Create with zero dimensions

# Conversion

- [`from((u16, u16))`](Self::from) - Create from `(u16, u16)` tuple
- [`from(Rect)`](Self::from) - Create from [`Rect`] (uses width and height)
- [`into((u16, u16))`] - Convert to `(u16, u16)` tuple

# Computation

- [`area`](Self::area) - Compute the total number of cells covered by the size

# Examples

```rust
use ratatui_core::layout::{Rect, Size};

let size = Size::new(80, 24);
assert_eq!(size.area(), 1920);
let size = Size::from((80, 24));
let size = Size::from(Rect::new(0, 0, 80, 24));
assert_eq!(size.area(), 1920);
```

For comprehensive layout documentation and examples, see the [`layout`](crate::layout) module.

**Fields:**
- `width: u16` - The width in columns
- `height: u16` - The height in rows

**Methods:**

- `fn new(width: u16, height: u16) -> Self` - Create a new `Size` struct
- `fn area(self: Self) -> u32` - Compute the total area of the size as a `u32`.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Default**
  - `fn default() -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **From**
  - `fn from(rect: Rect) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **From**
  - `fn from((width, height): (u16, u16)) -> Self`



