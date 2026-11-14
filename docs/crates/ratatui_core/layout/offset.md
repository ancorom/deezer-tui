**ratatui_core > layout > offset**

# Module: layout::offset

## Contents

**Structs**

- [`Offset`](#offset) - Amounts by which to move a [`Rect`](crate::layout::Rect).

---

## ratatui_core::layout::offset::Offset

*Struct*

Amounts by which to move a [`Rect`](crate::layout::Rect).

Positive numbers move to the right/bottom and negative to the left/top.

See [`Rect::offset`](crate::layout::Rect::offset) for usage.

**Fields:**
- `x: i32` - How much to move on the X axis
- `y: i32` - How much to move on the Y axis

**Methods:**

- `fn new(x: i32, y: i32) -> Self` - Creates a new `Offset` with the given values.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Add**
  - `fn add(self: Self, position: Position) -> Position` - Moves the position by the given offset.
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Default**
  - `fn default() -> Self`
- **Add**
  - `fn add(self: Self, rect: Rect) -> Rect` - Moves the rect by an offset without changing its size.
- **From**
  - `fn from(position: Position) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Neg**
  - `fn neg(self: Self) -> Self` - Negates the offset.
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



