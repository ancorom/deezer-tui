**ratatui_core > layout > rect > Rect**

# Module: layout::rect::Rect

## Contents

**Functions**

- [`area`](#area) - The area of the `Rect`.
- [`as_position`](#as_position) - Returns a [`Position`] with the same coordinates as this `Rect`.
- [`as_size`](#as_size) - Converts the `Rect` into a size struct.
- [`bottom`](#bottom) - Returns the bottom coordinate of the `Rect`. This is the first coordinate outside of the
- [`centered`](#centered) - Returns a new Rect, centered horizontally and vertically based on the provided constraints.
- [`centered_horizontally`](#centered_horizontally) - Returns a new Rect, centered horizontally based on the provided constraint.
- [`centered_vertically`](#centered_vertically) - Returns a new Rect, centered vertically based on the provided constraint.
- [`clamp`](#clamp) - Clamp this `Rect` to fit inside the other `Rect`.
- [`columns`](#columns) - An iterator over columns within the `Rect`.
- [`contains`](#contains) - Returns true if the given position is inside the `Rect`.
- [`inner`](#inner) - Returns a new `Rect` inside the current one, with the given margin on each side.
- [`intersection`](#intersection) - Returns a new `Rect` that is the intersection of the current one and the given one.
- [`intersects`](#intersects) - Returns true if the two `Rect`s intersect.
- [`is_empty`](#is_empty) - Returns true if the `Rect` has no area.
- [`left`](#left) - Returns the left coordinate of the `Rect`.
- [`new`](#new) - Creates a new `Rect`, with width and height limited to keep both bounds within `u16`.
- [`offset`](#offset) - Moves the `Rect` without modifying its size.
- [`outer`](#outer) - Returns a new `Rect` outside the current one, with the given margin applied on each side.
- [`positions`](#positions) - An iterator over the positions within the `Rect`.
- [`resize`](#resize) - Resizes the `Rect`, clamping to keep the right and bottom within `u16::MAX`.
- [`right`](#right) - Returns the right coordinate of the `Rect`. This is the first coordinate outside of the
- [`rows`](#rows) - An iterator over rows within the `Rect`.
- [`top`](#top) - Returns the top coordinate of the `Rect`.
- [`union`](#union) - Returns a new `Rect` that contains both the current one and the given one.

---

## ratatui_core::layout::rect::Rect::area

*Function*

The area of the `Rect`.

```rust
fn area(self: Self) -> u32
```



## ratatui_core::layout::rect::Rect::as_position

*Function*

Returns a [`Position`] with the same coordinates as this `Rect`.

# Examples

```
use ratatui_core::layout::Rect;

let rect = Rect::new(1, 2, 3, 4);
let position = rect.as_position();
````

```rust
fn as_position(self: Self) -> Position
```



## ratatui_core::layout::rect::Rect::as_size

*Function*

Converts the `Rect` into a size struct.

```rust
fn as_size(self: Self) -> Size
```



## ratatui_core::layout::rect::Rect::bottom

*Function*

Returns the bottom coordinate of the `Rect`. This is the first coordinate outside of the
`Rect`.

If the bottom coordinate is larger than the maximum value of u16, it will be clamped to
`u16::MAX`.

```rust
fn bottom(self: Self) -> u16
```



## ratatui_core::layout::rect::Rect::centered

*Function*

Returns a new Rect, centered horizontally and vertically based on the provided constraints.

# Examples

```
use ratatui_core::layout::Constraint;
use ratatui_core::terminal::Frame;

fn render(frame: &mut Frame) {
    let area = frame
        .area()
        .centered(Constraint::Ratio(1, 2), Constraint::Ratio(1, 3));
}
```

```rust
fn centered(self: Self, horizontal_constraint: Constraint, vertical_constraint: Constraint) -> Self
```



## ratatui_core::layout::rect::Rect::centered_horizontally

*Function*

Returns a new Rect, centered horizontally based on the provided constraint.

# Examples

```
use ratatui_core::layout::Constraint;
use ratatui_core::terminal::Frame;

fn render(frame: &mut Frame) {
    let area = frame.area().centered_horizontally(Constraint::Ratio(1, 2));
}
```

```rust
fn centered_horizontally(self: Self, constraint: Constraint) -> Self
```



## ratatui_core::layout::rect::Rect::centered_vertically

*Function*

Returns a new Rect, centered vertically based on the provided constraint.

# Examples

```
use ratatui_core::layout::Constraint;
use ratatui_core::terminal::Frame;

fn render(frame: &mut Frame) {
    let area = frame.area().centered_vertically(Constraint::Ratio(1, 2));
}
```

```rust
fn centered_vertically(self: Self, constraint: Constraint) -> Self
```



## ratatui_core::layout::rect::Rect::clamp

*Function*

Clamp this `Rect` to fit inside the other `Rect`.

If the width or height of this `Rect` is larger than the other `Rect`, it will be clamped to
the other `Rect`'s width or height.

If the left or top coordinate of this `Rect` is smaller than the other `Rect`, it will be
clamped to the other `Rect`'s left or top coordinate.

If the right or bottom coordinate of this `Rect` is larger than the other `Rect`, it will be
clamped to the other `Rect`'s right or bottom coordinate.

This is different from [`Rect::intersection`] because it will move this `Rect` to fit inside
the other `Rect`, while [`Rect::intersection`] instead would keep this `Rect`'s position and
truncate its size to only that which is inside the other `Rect`.

# Examples

```rust
use ratatui_core::layout::Rect;

let area = Rect::new(0, 0, 100, 100);
let rect = Rect::new(80, 80, 30, 30).clamp(area);
assert_eq!(rect, Rect::new(70, 70, 30, 30));
```

```rust
fn clamp(self: Self, other: Self) -> Self
```



## ratatui_core::layout::rect::Rect::columns

*Function*

An iterator over columns within the `Rect`.

Each column is a full `Rect` region with width 1 that can be used for rendering widgets
or as input to further layout methods.

# Example

```
use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;
use ratatui_core::widgets::Widget;

fn render_columns(area: Rect, buf: &mut Buffer) {
    // Renders column indices (0-9 repeating) in each column
    for (i, column) in area.columns().enumerate() {
        format!("{}", i % 10).render(column, buf);
    }
}
```

```rust
fn columns(self: Self) -> Columns
```



## ratatui_core::layout::rect::Rect::contains

*Function*

Returns true if the given position is inside the `Rect`.

The position is considered inside the `Rect` if it is on the `Rect`'s border.

# Examples

```rust
use ratatui_core::layout::{Position, Rect};

let rect = Rect::new(1, 2, 3, 4);
assert!(rect.contains(Position { x: 1, y: 2 }));
````

```rust
fn contains(self: Self, position: Position) -> bool
```



## ratatui_core::layout::rect::Rect::inner

*Function*

Returns a new `Rect` inside the current one, with the given margin on each side.

If the margin is larger than the `Rect`, the returned `Rect` will have no area.

```rust
fn inner(self: Self, margin: Margin) -> Self
```



## ratatui_core::layout::rect::Rect::intersection

*Function*

Returns a new `Rect` that is the intersection of the current one and the given one.

If the two `Rect`s do not intersect, the returned `Rect` will have no area.

```rust
fn intersection(self: Self, other: Self) -> Self
```



## ratatui_core::layout::rect::Rect::intersects

*Function*

Returns true if the two `Rect`s intersect.

```rust
fn intersects(self: Self, other: Self) -> bool
```



## ratatui_core::layout::rect::Rect::is_empty

*Function*

Returns true if the `Rect` has no area.

```rust
fn is_empty(self: Self) -> bool
```



## ratatui_core::layout::rect::Rect::left

*Function*

Returns the left coordinate of the `Rect`.

```rust
fn left(self: Self) -> u16
```



## ratatui_core::layout::rect::Rect::new

*Function*

Creates a new `Rect`, with width and height limited to keep both bounds within `u16`.

If the width or height would cause the right or bottom coordinate to be larger than the
maximum value of `u16`, the width or height will be clamped to keep the right or bottom
coordinate within `u16`.

# Examples

```
use ratatui_core::layout::Rect;

let rect = Rect::new(1, 2, 3, 4);
```

```rust
fn new(x: u16, y: u16, width: u16, height: u16) -> Self
```



## ratatui_core::layout::rect::Rect::offset

*Function*

Moves the `Rect` without modifying its size.

Moves the `Rect` according to the given offset without modifying its [`width`](Rect::width)
or [`height`](Rect::height).
- Positive `x` moves the whole `Rect` to the right, negative to the left.
- Positive `y` moves the whole `Rect` to the bottom, negative to the top.

See [`Offset`] for details.

```rust
fn offset(self: Self, offset: Offset) -> Self
```



## ratatui_core::layout::rect::Rect::outer

*Function*

Returns a new `Rect` outside the current one, with the given margin applied on each side.

If the margin causes the `Rect`'s bounds to be outside the range of a `u16`, the `Rect` will
be truncated to keep the bounds within `u16`. This will cause the size of the `Rect` to
change.

The generated `Rect` may not fit inside the buffer or containing area, so it consider
constraining the resulting `Rect` with [`Rect::clamp`] before using it.

```rust
fn outer(self: Self, margin: Margin) -> Self
```



## ratatui_core::layout::rect::Rect::positions

*Function*

An iterator over the positions within the `Rect`.

The positions are returned in a row-major order (left-to-right, top-to-bottom).
Each position is a `Position` that represents a single cell coordinate.

# Example

```
use ratatui_core::buffer::Buffer;
use ratatui_core::layout::{Position, Rect};
use ratatui_core::widgets::Widget;

fn render_positions(area: Rect, buf: &mut Buffer) {
    // Renders position indices (0-9 repeating) at each cell position
    for (i, position) in area.positions().enumerate() {
        buf[position].set_symbol(&format!("{}", i % 10));
    }
}
```

```rust
fn positions(self: Self) -> Positions
```



## ratatui_core::layout::rect::Rect::resize

*Function*

Resizes the `Rect`, clamping to keep the right and bottom within `u16::MAX`.

The position is preserved. If the requested size would push the `Rect` beyond the bounds of
`u16`, the width or height is reduced so that [`right`](Self::right) and
[`bottom`](Self::bottom) remain within range.

```rust
fn resize(self: Self, size: Size) -> Self
```



## ratatui_core::layout::rect::Rect::right

*Function*

Returns the right coordinate of the `Rect`. This is the first coordinate outside of the
`Rect`.

If the right coordinate is larger than the maximum value of u16, it will be clamped to
`u16::MAX`.

```rust
fn right(self: Self) -> u16
```



## ratatui_core::layout::rect::Rect::rows

*Function*

An iterator over rows within the `Rect`.

Each row is a full `Rect` region with height 1 that can be used for rendering widgets
or as input to further layout methods.

# Example

```
use ratatui_core::buffer::Buffer;
use ratatui_core::layout::{Constraint, Layout, Rect};
use ratatui_core::widgets::Widget;

fn render_list(area: Rect, buf: &mut Buffer) {
    // Renders "Item 0", "Item 1", etc. in each row
    for (i, row) in area.rows().enumerate() {
        format!("Item {i}").render(row, buf);
    }
}

fn render_with_nested_layout(area: Rect, buf: &mut Buffer) {
    // Splits each row into left/right areas and renders labels and content
    for (i, row) in area.rows().take(3).enumerate() {
        let [left, right] =
            Layout::horizontal([Constraint::Percentage(30), Constraint::Fill(1)]).areas(row);

        format!("{i}:").render(left, buf);
        "Content".render(right, buf);
    }
}
```

```rust
fn rows(self: Self) -> Rows
```



## ratatui_core::layout::rect::Rect::top

*Function*

Returns the top coordinate of the `Rect`.

```rust
fn top(self: Self) -> u16
```



## ratatui_core::layout::rect::Rect::union

*Function*

Returns a new `Rect` that contains both the current one and the given one.

```rust
fn union(self: Self, other: Self) -> Self
```



