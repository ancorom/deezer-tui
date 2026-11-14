**ratatui_core > backend > test**

# Module: backend::test

## Contents

**Structs**

- [`TestBackend`](#testbackend) - A [`Backend`] implementation used for integration testing that renders to an memory buffer.

---

## ratatui_core::backend::test::TestBackend

*Struct*

A [`Backend`] implementation used for integration testing that renders to an memory buffer.

Note: that although many of the integration and unit tests in ratatui are written using this
backend, it is preferable to write unit tests for widgets directly against the buffer rather
than using this backend. This backend is intended for integration tests that test the entire
terminal UI.

# Example

```rust,ignore
use ratatui::backend::{Backend, TestBackend};

let mut backend = TestBackend::new(10, 2);
backend.clear()?;
backend.assert_buffer_lines(["          "; 2]);
# Result::Ok(())
```

**Methods:**

- `fn new(width: u16, height: u16) -> Self` - Creates a new `TestBackend` with the specified width and height.
- `fn with_lines<'line, Lines>(lines: Lines) -> Self` - Creates a new `TestBackend` with the specified lines as the initial screen state.
- `fn buffer(self: &Self) -> &Buffer` - Returns a reference to the internal buffer of the `TestBackend`.
- `fn cursor_visible(self: &Self) -> bool` - Returns whether the cursor is visible.
- `fn cursor_position(self: &Self) -> Position` - Returns the current cursor position.
- `fn scrollback(self: &Self) -> &Buffer` - Returns a reference to the internal scrollback buffer of the `TestBackend`.
- `fn resize(self: & mut Self, width: u16, height: u16)` - Resizes the `TestBackend` to the specified width and height.
- `fn assert_buffer(self: &Self, expected: &Buffer)` - Asserts that the `TestBackend`'s buffer is equal to the expected buffer.
- `fn assert_scrollback(self: &Self, expected: &Buffer)` - Asserts that the `TestBackend`'s scrollback buffer is equal to the expected buffer.
- `fn assert_scrollback_empty(self: &Self)` - Asserts that the `TestBackend`'s scrollback buffer is empty.
- `fn assert_buffer_lines<'line, Lines>(self: &Self, expected: Lines)` - Asserts that the `TestBackend`'s buffer is equal to the expected lines.
- `fn assert_scrollback_lines<'line, Lines>(self: &Self, expected: Lines)` - Asserts that the `TestBackend`'s scrollback buffer is equal to the expected lines.
- `fn assert_cursor_position<P>(self: & mut Self, position: P)` - Asserts that the `TestBackend`'s cursor position is equal to the expected one.

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result` - Formats the `TestBackend` for display by calling the `buffer_view` function
- **Backend**
  - `fn draw<'a, I>(self: & mut Self, content: I) -> core::result::Result<(), core::convert::Infallible>`
  - `fn hide_cursor(self: & mut Self) -> core::result::Result<(), core::convert::Infallible>`
  - `fn show_cursor(self: & mut Self) -> core::result::Result<(), core::convert::Infallible>`
  - `fn get_cursor_position(self: & mut Self) -> core::result::Result<Position, core::convert::Infallible>`
  - `fn set_cursor_position<P>(self: & mut Self, position: P) -> core::result::Result<(), core::convert::Infallible>`
  - `fn clear(self: & mut Self) -> core::result::Result<(), core::convert::Infallible>`
  - `fn clear_region(self: & mut Self, clear_type: ClearType) -> core::result::Result<(), core::convert::Infallible>`
  - `fn append_lines(self: & mut Self, line_count: u16) -> core::result::Result<(), core::convert::Infallible>` - Inserts n line breaks at the current cursor position.
  - `fn size(self: &Self) -> core::result::Result<Size, core::convert::Infallible>`
  - `fn window_size(self: & mut Self) -> core::result::Result<WindowSize, core::convert::Infallible>`
  - `fn flush(self: & mut Self) -> core::result::Result<(), core::convert::Infallible>`



