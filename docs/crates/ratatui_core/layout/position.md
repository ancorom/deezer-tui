**ratatui_core > layout > position**

# Module: layout::position

## Contents

**Structs**

- [`Position`](#position) - Position in the terminal coordinate system.

---

## ratatui_core::layout::position::Position

*Struct*

Position in the terminal coordinate system.

The position is relative to the top left corner of the terminal window, with the top left corner
being (0, 0). The x axis is horizontal increasing to the right, and the y axis is vertical
increasing downwards.

`Position` is used throughout the layout system to represent specific points in the terminal.
It can be created from coordinates, tuples, or extracted from rectangular areas.

# Construction

- [`new`](Self::new) - Create a new position from x and y coordinates
- [`default`](Default::default) - Create at origin (0, 0)

# Conversion

- [`from((u16, u16))`](Self::from) - Create from `(u16, u16)` tuple
- [`from(Rect)`](Self::from) - Create from [`Rect`] (uses top-left corner)
- [`into((u16, u16))`] - Convert to `(u16, u16)` tuple

# Movement

- [`offset`](Self::offset) - Move by an [`Offset`]
- [`Add<Offset>`](core::ops::Add) and [`Sub<Offset>`](core::ops::Sub) - Shift by offsets with
  clamping
- [`AddAssign<Offset>`](core::ops::AddAssign) and [`SubAssign<Offset>`](core::ops::SubAssign) -
  In-place shifting

# Examples

```
use ratatui_core::layout::{Offset, Position, Rect};

// the following are all equivalent
let position = Position { x: 1, y: 2 };
let position = Position::new(1, 2);
let position = Position::from((1, 2));
let position = Position::from(Rect::new(1, 2, 3, 4));

// position can be converted back into the components when needed
let (x, y) = position.into();

// movement by offsets
let position = Position::new(5, 5) + Offset::new(2, -3);
assert_eq!(position, Position::new(7, 2));
```

For comprehensive layout documentation and examples, see the [`layout`](crate::layout) module.

**Fields:**
- `x: u16` - The x coordinate of the position
- `y: u16` - The y coordinate of the position

**Methods:**

- `fn new(x: u16, y: u16) -> Self` - Create a new position
- `fn offset(self: Self, offset: Offset) -> Self` - Moves the position by the given offset.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Sub**
  - `fn sub(self: Self, offset: Offset) -> Self` - Moves the position by the inverse of the given offset.
- **AddAssign**
  - `fn add_assign(self: & mut Self, offset: Offset)` - Moves the position in place by the given offset.
- **Default**
  - `fn default() -> Self`
- **From**
  - `fn from(rect: Rect) -> Self`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, offset: Offset)` - Moves the position in place by the inverse of the given offset.
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Add**
  - `fn add(self: Self, offset: Offset) -> Self` - Moves the position by the given offset.
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> $crate::cmp::Ordering`
- **From**
  - `fn from((x, y): (u16, u16)) -> Self`



