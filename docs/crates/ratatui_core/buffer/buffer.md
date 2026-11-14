**ratatui_core > buffer > buffer**

# Module: buffer::buffer

## Contents

**Structs**

- [`Buffer`](#buffer) - A buffer that maps to the desired content of the terminal after the draw call

---

## ratatui_core::buffer::buffer::Buffer

*Struct*

A buffer that maps to the desired content of the terminal after the draw call

No widget in the library interacts directly with the terminal. Instead each of them is required
to draw their state to an intermediate buffer. It is basically a grid where each cell contains
a grapheme, a foreground color and a background color. This grid will then be used to output
the appropriate escape sequences and characters to draw the UI as the user has defined it.

# Examples:

```
use ratatui_core::buffer::{Buffer, Cell};
use ratatui_core::layout::{Position, Rect};
use ratatui_core::style::{Color, Style};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let mut buf = Buffer::empty(Rect {
    x: 0,
    y: 0,
    width: 10,
    height: 5,
});

// indexing using Position
buf[Position { x: 0, y: 0 }].set_symbol("A");
assert_eq!(buf[Position { x: 0, y: 0 }].symbol(), "A");

// indexing using (x, y) tuple (which is converted to Position)
buf[(0, 1)].set_symbol("B");
assert_eq!(buf[(0, 1)].symbol(), "B");

// getting an Option instead of panicking if the position is outside the buffer
let cell = buf
    .cell_mut(Position { x: 0, y: 2 })
    .ok_or("cell not found")?;
cell.set_symbol("C");

let cell = buf.cell(Position { x: 0, y: 2 }).ok_or("cell not found")?;
assert_eq!(cell.symbol(), "C");

buf.set_string(
    3,
    0,
    "string",
    Style::default().fg(Color::Red).bg(Color::White),
);
let cell = &buf[(5, 0)]; // cannot move out of buf, so we borrow it
assert_eq!(cell.symbol(), "r");
assert_eq!(cell.fg, Color::Red);
assert_eq!(cell.bg, Color::White);
# Ok(())
# }
```

**Fields:**
- `area: crate::layout::Rect` - The area represented by this buffer
- `content: alloc::vec::Vec<crate::buffer::Cell>` - The content of the buffer. The length of this Vec should always be equal to area.width *

**Methods:**

- `fn empty(area: Rect) -> Self` - Returns a Buffer with all cells set to the default one
- `fn filled(area: Rect, cell: Cell) -> Self` - Returns a Buffer with all cells initialized with the attributes of the given Cell
- `fn with_lines<'a, Iter>(lines: Iter) -> Self` - Returns a Buffer containing the given lines
- `fn content(self: &Self) -> &[Cell]` - Returns the content of the buffer as a slice
- `fn area(self: &Self) -> &Rect` - Returns the area covered by this buffer
- `fn get(self: &Self, x: u16, y: u16) -> &Cell` - Returns a reference to the [`Cell`] at the given coordinates
- `fn get_mut(self: & mut Self, x: u16, y: u16) -> & mut Cell` - Returns a mutable reference to the [`Cell`] at the given coordinates.
- `fn cell<P>(self: &Self, position: P) -> Option<&Cell>` - Returns a reference to the [`Cell`] at the given position or [`None`] if the position is
- `fn cell_mut<P>(self: & mut Self, position: P) -> Option<& mut Cell>` - Returns a mutable reference to the [`Cell`] at the given position or [`None`] if the
- `fn index_of(self: &Self, x: u16, y: u16) -> usize` - Returns the index in the `Vec<Cell>` for the given global (x, y) coordinates.
- `fn pos_of(self: &Self, index: usize) -> (u16, u16)` - Returns the (global) coordinates of a cell given its index.
- `fn set_string<T, S>(self: & mut Self, x: u16, y: u16, string: T, style: S)` - Print a string, starting at the position (x, y)
- `fn set_stringn<T, S>(self: & mut Self, x: u16, y: u16, string: T, max_width: usize, style: S) -> (u16, u16)` - Print at most the first n characters of a string if enough space is available
- `fn set_line(self: & mut Self, x: u16, y: u16, line: &Line, max_width: u16) -> (u16, u16)` - Print a line, starting at the position (x, y)
- `fn set_span(self: & mut Self, x: u16, y: u16, span: &Span, max_width: u16) -> (u16, u16)` - Print a span, starting at the position (x, y)
- `fn set_style<S>(self: & mut Self, area: Rect, style: S)` - Set the style of all cells in the given area.
- `fn resize(self: & mut Self, area: Rect)` - Resize the buffer so that the mapped area matches the given area and that the buffer
- `fn reset(self: & mut Self)` - Reset all cells in the buffer
- `fn merge(self: & mut Self, other: &Self)` - Merge an other buffer into this one
- `fn diff<'a>(self: &Self, other: &'a Self) -> Vec<(u16, u16, &'a Cell)>` - Collects the diff between `self` and `other` into a `Vec`.
- `fn diff_iter<'prev, 'next>(self: &'prev Self, other: &'next Self) -> BufferDiff<'prev, 'next>` - Builds a minimal sequence of coordinates and Cells necessary to update the UI from

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **IndexMut**
  - `fn index_mut(self: & mut Self, position: P) -> & mut <Self as >::Output` - Returns a mutable reference to the [`Cell`] at the given position.
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result` - Writes a debug representation of the buffer to the given formatter.
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Index**
  - `fn index(self: &Self, position: P) -> &<Self as >::Output` - Returns a reference to the [`Cell`] at the given position.



