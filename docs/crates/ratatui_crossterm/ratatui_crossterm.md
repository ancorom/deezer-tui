**ratatui_crossterm**

# Module: ratatui_crossterm

## Contents

**Structs**

- [`CrosstermBackend`](#crosstermbackend) - A [`Backend`] implementation that uses [Crossterm] to render to the terminal.

**Traits**

- [`FromCrossterm`](#fromcrossterm) - A trait for converting a Crossterm type to a Ratatui type.
- [`IntoCrossterm`](#intocrossterm) - A trait for converting a Ratatui type to a Crossterm type.

---

## ratatui_crossterm::CrosstermBackend

*Struct*

A [`Backend`] implementation that uses [Crossterm] to render to the terminal.

The `CrosstermBackend` struct is a wrapper around a writer implementing [`Write`], which is
used to send commands to the terminal. It provides methods for drawing content, manipulating
the cursor, and clearing the terminal screen.

Most applications should not call the methods on `CrosstermBackend` directly, but will instead
use the [`Terminal`] struct, which provides a more ergonomic interface.

Usually applications will enable raw mode and switch to alternate screen mode after creating
a `CrosstermBackend`. This is done by calling [`crossterm::terminal::enable_raw_mode`] and
[`crossterm::terminal::EnterAlternateScreen`] (and the corresponding disable/leave functions
when the application exits). This is not done automatically by the backend because it is
possible that the application may want to use the terminal for other purposes (like showing
help text) before entering alternate screen mode.

# Example

```rust,ignore
use std::io::{stderr, stdout};

use crossterm::ExecutableCommand;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

let mut backend = CrosstermBackend::new(stdout());
// or
let backend = CrosstermBackend::new(stderr());
let mut terminal = Terminal::new(backend)?;

enable_raw_mode()?;
stdout().execute(EnterAlternateScreen)?;

terminal.clear()?;
terminal.draw(|frame| {
    // -- snip --
})?;

stdout().execute(LeaveAlternateScreen)?;
disable_raw_mode()?;

# std::io::Result::Ok(())
```

See the [Examples] directory for more examples. See the [`backend`] module documentation
for more details on raw mode and alternate screen.

[`Write`]: std::io::Write
[`Terminal`]: https://docs.rs/ratatui/latest/ratatui/struct.Terminal.html
[`backend`]: ratatui_core::backend
[Crossterm]: https://crates.io/crates/crossterm
[Examples]: https://github.com/ratatui/ratatui/tree/main/ratatui/examples/README.md

**Generic Parameters:**
- W

**Methods:**

- `fn new(writer: W) -> Self` - Creates a new `CrosstermBackend` with the given writer.
- `fn writer(self: &Self) -> &W` - Gets the writer.
- `fn writer_mut(self: & mut Self) -> & mut W` - Gets the writer as a mutable reference.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Default**
  - `fn default() -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Backend**
  - `fn draw<'a, I>(self: & mut Self, content: I) -> io::Result<()>`
  - `fn hide_cursor(self: & mut Self) -> io::Result<()>`
  - `fn show_cursor(self: & mut Self) -> io::Result<()>`
  - `fn get_cursor_position(self: & mut Self) -> io::Result<Position>`
  - `fn set_cursor_position<P>(self: & mut Self, position: P) -> io::Result<()>`
  - `fn clear(self: & mut Self) -> io::Result<()>`
  - `fn clear_region(self: & mut Self, clear_type: ClearType) -> io::Result<()>`
  - `fn append_lines(self: & mut Self, n: u16) -> io::Result<()>`
  - `fn size(self: &Self) -> io::Result<Size>`
  - `fn window_size(self: & mut Self) -> io::Result<WindowSize>`
  - `fn flush(self: & mut Self) -> io::Result<()>`
- **Write**
  - `fn write(self: & mut Self, buf: &[u8]) -> io::Result<usize>` - Writes a buffer of bytes to the underlying buffer.
  - `fn flush(self: & mut Self) -> io::Result<()>` - Flushes the underlying buffer.



## ratatui_crossterm::FromCrossterm

*Trait*

A trait for converting a Crossterm type to a Ratatui type.

This trait is needed for avoiding the orphan rule when implementing `From` for crossterm types
once these are moved to a separate crate.

**Methods:**

- `from_crossterm`: Converts the crossterm type to a ratatui type.



## ratatui_crossterm::IntoCrossterm

*Trait*

A trait for converting a Ratatui type to a Crossterm type.

This trait is needed for avoiding the orphan rule when implementing `From` for crossterm types
once these are moved to a separate crate.

**Methods:**

- `into_crossterm`: Converts the ratatui type to a crossterm type.



