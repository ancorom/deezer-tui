**ratatui_core > layout > rect**

# Module: layout::rect

## Contents

**Structs**

- [`Rect`](#rect) - A rectangular area in the terminal.

---

## ratatui_core::layout::rect::Rect

*Struct*

A rectangular area in the terminal.

A `Rect` represents a rectangular region in the terminal coordinate system, defined by its
top-left corner position and dimensions. This is the fundamental building block for all layout
operations and widget rendering in Ratatui.

Rectangles are used throughout the layout system to define areas where widgets can be rendered.
They are typically created by [`Layout`] operations that divide terminal space, but can also be
manually constructed for specific positioning needs.

The coordinate system uses the top-left corner as the origin (0, 0), with x increasing to the
right and y increasing downward. All measurements are in character cells.

# Construction and Conversion

- [`new`](Self::new) - Create a new rectangle from coordinates and dimensions
- [`as_position`](Self::as_position) - Convert to a position at the top-left corner
- [`as_size`](Self::as_size) - Convert to a size representing the dimensions
- [`from((Position, Size))`](Self::from) - Create from `(Position, Size)` tuple
- [`from(((u16, u16), (u16, u16)))`](Self::from) - Create from `((u16, u16), (u16, u16))`
  coordinate and dimension tuples
- [`into((Position, Size))`] - Convert to `(Position, Size)` tuple
- [`default`](Self::default) - Create a zero-sized rectangle at origin

# Geometry and Properties

- [`area`](Self::area) - Calculate the total area in character cells
- [`is_empty`](Self::is_empty) - Check if the rectangle has zero area
- [`left`](Self::left), [`right`](Self::right), [`top`](Self::top), [`bottom`](Self::bottom) -
  Get edge coordinates

# Spatial Operations

- [`inner`](Self::inner), [`outer`](Self::outer) - Apply margins to shrink or expand
- [`offset`](Self::offset) - Move the rectangle by a relative amount
- [`resize`](Self::resize) - Change the rectangle size while keeping the bottom/right in range
- [`union`](Self::union) - Combine with another rectangle to create a bounding box
- [`intersection`](Self::intersection) - Find the overlapping area with another rectangle
- [`clamp`](Self::clamp) - Constrain the rectangle to fit within another

# Positioning and Centering

- [`centered_horizontally`](Self::centered_horizontally) - Center horizontally within a
  constraint
- [`centered_vertically`](Self::centered_vertically) - Center vertically within a constraint
- [`centered`](Self::centered) - Center both horizontally and vertically

# Testing and Iteration

- [`contains`](Self::contains) - Check if a position is within the rectangle
- [`intersects`](Self::intersects) - Check if it overlaps with another rectangle
- [`rows`](Self::rows) - Iterate over horizontal rows within the rectangle
- [`columns`](Self::columns) - Iterate over vertical columns within the rectangle
- [`positions`](Self::positions) - Iterate over all positions within the rectangle

# Examples

To create a new `Rect`, use [`Rect::new`]. The size of the `Rect` will be clamped to keep the
right and bottom coordinates within `u16`. Note that this clamping does not occur when creating
a `Rect` directly.

```rust
use ratatui_core::layout::Rect;

let rect = Rect::new(1, 2, 3, 4);
assert_eq!(
    rect,
    Rect {
        x: 1,
        y: 2,
        width: 3,
        height: 4
    }
);
```

You can also create a `Rect` from a [`Position`] and a [`Size`].

```rust
use ratatui_core::layout::{Position, Rect, Size};

let position = Position::new(1, 2);
let size = Size::new(3, 4);
let rect = Rect::from((position, size));
assert_eq!(
    rect,
    Rect {
        x: 1,
        y: 2,
        width: 3,
        height: 4
    }
);
```

To move a `Rect` without modifying its size, add or subtract an [`Offset`] to it.

```rust
use ratatui_core::layout::{Offset, Rect};

let rect = Rect::new(1, 2, 3, 4);
let offset = Offset::new(5, 6);
let moved_rect = rect + offset;
assert_eq!(moved_rect, Rect::new(6, 8, 3, 4));
```

To resize a `Rect` while ensuring it stays within bounds, use [`Rect::resize`]. The size is
clamped so that `right()` and `bottom()` do not exceed `u16::MAX`.

```rust
use ratatui_core::layout::{Rect, Size};

let rect = Rect::new(u16::MAX - 1, u16::MAX - 1, 1, 1).resize(Size::new(10, 10));
assert_eq!(rect, Rect::new(u16::MAX - 1, u16::MAX - 1, 1, 1));
```

For comprehensive layout documentation and examples, see the [`layout`](crate::layout) module.

**Fields:**
- `x: u16` - The x coordinate of the top left corner of the `Rect`.
- `y: u16` - The y coordinate of the top left corner of the `Rect`.
- `width: u16` - The width of the `Rect`.
- `height: u16` - The height of the `Rect`.

**Methods:**

- `fn new(x: u16, y: u16, width: u16, height: u16) -> Self` - Creates a new `Rect`, with width and height limited to keep both bounds within `u16`.
- `fn area(self: Self) -> u32` - The area of the `Rect`.
- `fn is_empty(self: Self) -> bool` - Returns true if the `Rect` has no area.
- `fn left(self: Self) -> u16` - Returns the left coordinate of the `Rect`.
- `fn right(self: Self) -> u16` - Returns the right coordinate of the `Rect`. This is the first coordinate outside of the
- `fn top(self: Self) -> u16` - Returns the top coordinate of the `Rect`.
- `fn bottom(self: Self) -> u16` - Returns the bottom coordinate of the `Rect`. This is the first coordinate outside of the
- `fn inner(self: Self, margin: Margin) -> Self` - Returns a new `Rect` inside the current one, with the given margin on each side.
- `fn outer(self: Self, margin: Margin) -> Self` - Returns a new `Rect` outside the current one, with the given margin applied on each side.
- `fn offset(self: Self, offset: Offset) -> Self` - Moves the `Rect` without modifying its size.
- `fn resize(self: Self, size: Size) -> Self` - Resizes the `Rect`, clamping to keep the right and bottom within `u16::MAX`.
- `fn union(self: Self, other: Self) -> Self` - Returns a new `Rect` that contains both the current one and the given one.
- `fn intersection(self: Self, other: Self) -> Self` - Returns a new `Rect` that is the intersection of the current one and the given one.
- `fn intersects(self: Self, other: Self) -> bool` - Returns true if the two `Rect`s intersect.
- `fn contains(self: Self, position: Position) -> bool` - Returns true if the given position is inside the `Rect`.
- `fn clamp(self: Self, other: Self) -> Self` - Clamp this `Rect` to fit inside the other `Rect`.
- `fn rows(self: Self) -> Rows` - An iterator over rows within the `Rect`.
- `fn columns(self: Self) -> Columns` - An iterator over columns within the `Rect`.
- `fn positions(self: Self) -> Positions` - An iterator over the positions within the `Rect`.
- `fn as_position(self: Self) -> Position` - Returns a [`Position`] with the same coordinates as this `Rect`.
- `fn as_size(self: Self) -> Size` - Converts the `Rect` into a size struct.
- `fn centered_horizontally(self: Self, constraint: Constraint) -> Self` - Returns a new Rect, centered horizontally based on the provided constraint.
- `fn centered_vertically(self: Self, constraint: Constraint) -> Self` - Returns a new Rect, centered vertically based on the provided constraint.
- `fn centered(self: Self, horizontal_constraint: Constraint, vertical_constraint: Constraint) -> Self` - Returns a new Rect, centered horizontally and vertically based on the provided constraints.
- `fn layout<const N>(self: Self, layout: &Layout) -> [Self; N]` - Split the rect into a number of sub-rects according to the given [`Layout`].
- `fn layout_vec(self: Self, layout: &Layout) -> alloc::vec::Vec<Self>` - Split the rect into a number of sub-rects according to the given [`Layout`].
- `fn try_layout<const N>(self: Self, layout: &Layout) -> Result<[Self; N], TryFromSliceError>` - Try to split the rect into a number of sub-rects according to the given [`Layout`].

**Traits:** Copy, Eq

**Trait Implementations:**

- **SubAssign**
  - `fn sub_assign(self: & mut Self, offset: Offset)` - Moves the rect by an offset in place without changing its size.
- **From**
  - `fn from(size: Size) -> Self` - Creates a new `Rect` with the given size at [`Position::ORIGIN`] (0, 0).
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, offset: Offset)` - Moves the rect by an offset in place without changing its size.
- **Add**
  - `fn add(self: Self, offset: Offset) -> Self` - Moves the rect by an offset without changing its size.
- **From**
  - `fn from((position, size): (Position, Size)) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Sub**
  - `fn sub(self: Self, offset: Offset) -> Self` - Subtracts an offset from the rect without changing its size.
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Default**
  - `fn default() -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



