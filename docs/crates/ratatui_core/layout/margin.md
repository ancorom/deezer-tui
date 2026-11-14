**ratatui_core > layout > margin**

# Module: layout::margin

## Contents

**Structs**

- [`Margin`](#margin) - Represents spacing around rectangular areas.

---

## ratatui_core::layout::margin::Margin

*Struct*

Represents spacing around rectangular areas.

`Margin` defines the horizontal and vertical spacing that should be applied around a rectangular
area. It's commonly used with [`Layout`](crate::layout::Layout) to add space between the
layout's boundaries and its contents, or with [`Rect::inner`](crate::layout::Rect::inner) and
[`Rect::outer`](crate::layout::Rect::outer) to create padded areas.

The margin values represent the number of character cells to add on each side. For horizontal
margin, the space is applied to both the left and right sides. For vertical margin, the space
is applied to both the top and bottom sides.

# Construction

- [`new`](Self::new) - Create a new margin with horizontal and vertical spacing
- [`default`](Default::default) - Create with zero margin

# Examples

```rust
use ratatui_core::layout::{Constraint, Layout, Margin, Rect};

// Create a margin of 2 cells horizontally and 1 cell vertically
let margin = Margin::new(2, 1);

// Apply directly to a rectangle
let area = Rect::new(0, 0, 80, 24);
let inner_area = area.inner(margin);

// Or use with a layout (which only accepts uniform margins)
let layout = Layout::vertical([Constraint::Fill(1)]).margin(2);
```

For comprehensive layout documentation and examples, see the [`layout`](crate::layout) module.

**Fields:**
- `horizontal: u16`
- `vertical: u16`

**Methods:**

- `fn new(horizontal: u16, vertical: u16) -> Self`

**Traits:** Copy, Eq

**Trait Implementations:**

- **From**
  - `fn from(value: u16) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Default**
  - `fn default() -> Self`



