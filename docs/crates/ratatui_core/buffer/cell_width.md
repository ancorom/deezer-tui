**ratatui_core > buffer > cell_width**

# Module: buffer::cell_width

## Contents

**Traits**

- [`CellWidth`](#cellwidth) - Returns the display width of a value in terminal cells.

---

## ratatui_core::buffer::cell_width::CellWidth

*Trait*

Returns the display width of a value in terminal cells.

This trait provides a unified way to compute cell widths for both string content
and [`Cell`](super::Cell)s:

- **`str`**: width is derived from [`UnicodeWidthStr`], with a fast path for single-byte ASCII
  characters and a terminal-compatibility adjustment for halfwidth katakana dakuten/handakuten
  (`U+FF9E`/`U+FF9F`).
- **[`Cell`](super::Cell)**: returns the
  [`CellDiffOption::ForcedWidth`](super::CellDiffOption::ForcedWidth) when set, otherwise falls
  back to the width of the cell's symbol.

**Methods:**

- `cell_width`: Returns the display width in terminal cells.



