**ratatui_core > buffer > buffer > Buffer**

# Module: buffer::buffer::Buffer

## Contents

**Functions**

- [`cell`](#cell) - Returns a reference to the [`Cell`] at the given position or [`None`] if the position is
- [`cell_mut`](#cell_mut) - Returns a mutable reference to the [`Cell`] at the given position or [`None`] if the
- [`diff_iter`](#diff_iter) - Builds a minimal sequence of coordinates and Cells necessary to update the UI from
- [`set_string`](#set_string) - Print a string, starting at the position (x, y)

---

## ratatui_core::buffer::buffer::Buffer::cell

*Function*

Returns a reference to the [`Cell`] at the given position or [`None`] if the position is
outside the `Buffer`'s area.

This method accepts any value that can be converted to [`Position`] (e.g. `(x, y)` or
`Position::new(x, y)`).

For a method that panics when the position is outside the buffer instead of returning
`None`, use [`Buffer[]`](Self::index).

# Examples

```rust
use ratatui_core::buffer::{Buffer, Cell};
use ratatui_core::layout::{Position, Rect};

let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 10));

assert_eq!(buffer.cell(Position::new(0, 0)), Some(&Cell::default()));
assert_eq!(buffer.cell(Position::new(10, 10)), None);
assert_eq!(buffer.cell((0, 0)), Some(&Cell::default()));
assert_eq!(buffer.cell((10, 10)), None);
```

```rust
fn cell<P>(self: &Self, position: P) -> Option<&Cell>
```



## ratatui_core::buffer::buffer::Buffer::cell_mut

*Function*

Returns a mutable reference to the [`Cell`] at the given position or [`None`] if the
position is outside the `Buffer`'s area.

This method accepts any value that can be converted to [`Position`] (e.g. `(x, y)` or
`Position::new(x, y)`).

For a method that panics when the position is outside the buffer instead of returning
`None`, use [`Buffer[]`](Self::index_mut).

# Examples

```rust
use ratatui_core::buffer::{Buffer, Cell};
use ratatui_core::layout::{Position, Rect};
use ratatui_core::style::{Color, Style};
let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 10));

if let Some(cell) = buffer.cell_mut(Position::new(0, 0)) {
    cell.set_symbol("A");
}
if let Some(cell) = buffer.cell_mut((0, 0)) {
    cell.set_style(Style::default().fg(Color::Red));
}
```

```rust
fn cell_mut<P>(self: & mut Self, position: P) -> Option<& mut Cell>
```



## ratatui_core::buffer::buffer::Buffer::diff_iter

*Function*

Builds a minimal sequence of coordinates and Cells necessary to update the UI from
self to other.

We're assuming that buffers are well-formed, that is no double-width cell is followed by
a non-blank cell.

# Multi-width characters handling:

```text
(Index:) `01`
Prev:    `コ`
Next:    `aa`
Updates: `0: a, 1: a'
```

```text
(Index:) `01`
Prev:    `a `
Next:    `コ`
Updates: `0: コ` (double width symbol at index 0 - skip index 1)
```

```text
(Index:) `012`
Prev:    `aaa`
Next:    `aコ`
Updates: `0: a, 1: コ` (double width symbol at index 1 - skip index 2)
```
# Panics

Panics if the two buffers have different `x`, `y`, or `width` values.

```rust
fn diff_iter<'prev, 'next>(self: &'prev Self, other: &'next Self) -> BufferDiff<'prev, 'next>
```



## ratatui_core::buffer::buffer::Buffer::set_string

*Function*

Print a string, starting at the position (x, y)

```rust
fn set_string<T, S>(self: & mut Self, x: u16, y: u16, string: T, style: S)
```



