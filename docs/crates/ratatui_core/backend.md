**ratatui_core > backend**

# Module: backend

## Contents

**Structs**

- [`WindowSize`](#windowsize) - The window size in characters (columns / rows) as well as pixels.

**Enums**

- [`ClearType`](#cleartype) - Defines which region of the terminal's visible display area is cleared.

**Traits**

- [`Backend`](#backend) - The `Backend` trait provides an abstraction over different terminal libraries. It defines the

---

## ratatui_core::backend::Backend

*Trait*

The `Backend` trait provides an abstraction over different terminal libraries. It defines the
methods required to draw content, manipulate the cursor, and clear the terminal screen.

Most applications should not need to interact with the `Backend` trait directly as the
[`Terminal`] struct provides a higher level interface for interacting with the terminal.

[`Terminal`]: https://docs.rs/ratatui/latest/ratatui/struct.Terminal.html

**Methods:**

- `Error`: Error type associated with this Backend.
- `draw`: Draw the given content to the terminal screen.
- `append_lines`: Insert `n` line breaks to the terminal screen.
- `hide_cursor`: Hide the cursor on the terminal screen.
- `show_cursor`: Show the cursor on the terminal screen.
- `get_cursor_position`: Get the current cursor position on the terminal screen.
- `set_cursor_position`: Set the cursor position on the terminal screen to the given x and y coordinates.
- `get_cursor`: Get the current cursor position on the terminal screen.
- `set_cursor`: Set the cursor position on the terminal screen to the given x and y coordinates.
- `clear`: Clears all character cells in the terminal's visible display area.
- `clear_region`: Clears a specific region of the terminal's visible display area, as defined by
- `size`: Get the size of the terminal screen in columns/rows as a [`Size`].
- `window_size`: Get the size of the terminal screen in columns/rows and pixels as a [`WindowSize`].
- `flush`: Flush any backend-buffered output to the terminal screen.



## ratatui_core::backend::ClearType

*Enum*

Defines which region of the terminal's visible display area is cleared.

Clearing operates on character cells in the active display surface. It does not move, hide, or
reset the cursor position. If the cursor lies inside the cleared region, the character cell at
the cursor position is cleared as well.

Clearing applies to the terminal's visible display area, not just content previously drawn by
Ratatui. No guarantees are made about scrollback, history, or off-screen buffers.

**Variants:**
- `All` - Clears all character cells in the visible display area.
- `AfterCursor` - Clears all character cells from the cursor position (inclusive) through the end of the
- `BeforeCursor` - Clears all character cells from the start of the display area through the cursor position
- `CurrentLine` - Clears all character cells in the cursor's current line.
- `UntilNewLine` - Clears all character cells from the cursor position (inclusive) to the end of the current

**Traits:** Eq, Copy

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **FromStr**
  - `fn from_str(s: &str) -> ::core::result::Result<ClearType, <Self as ::core::str::FromStr>::Err>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **TryFrom**
  - `fn try_from(s: &str) -> ::core::result::Result<ClearType, <Self as ::core::convert::TryFrom>::Error>`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::result::Result<(), ::core::fmt::Error>`



## ratatui_core::backend::WindowSize

*Struct*

The window size in characters (columns / rows) as well as pixels.

**Fields:**
- `columns_rows: crate::layout::Size` - Size of the window in characters (columns / rows).
- `pixels: crate::layout::Size` - Size of the window in pixels.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`



