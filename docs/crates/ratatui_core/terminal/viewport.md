**ratatui_core > terminal > viewport**

# Module: terminal::viewport

## Contents

**Enums**

- [`Viewport`](#viewport) - The area of the terminal that Ratatui draws into.

---

## ratatui_core::terminal::viewport::Viewport

*Enum*

The area of the terminal that Ratatui draws into.

A [`Viewport`] controls where widgets render and what [`Frame::area`] returns.

For a higher-level overview of viewports in the context of an application (including
examples), see [`Terminal`].

Choose a viewport based on how the Ratatui UI should fit into the terminal:

- [`Viewport::Fullscreen`] for the standard case: your app owns the whole terminal surface.
- [`Viewport::Inline`] when the UI should live inside a larger CLI flow, with normal terminal
  output above it.
- [`Viewport::Fixed`] when Ratatui should render into one region of a terminal layout managed
  elsewhere.

In fullscreen mode, the viewport starts at (0, 0). In inline and fixed mode, the viewport may
have a non-zero `x`/`y` origin; prefer using `Frame::area()` as your root layout rectangle.
Code that assumes `(0, 0)` as the origin is therefore only correct for fullscreen viewports.

See [`Terminal::with_options`] for how to select a viewport, and [`Terminal::resize`] /
[`Terminal::autoresize`] for resize behavior.

# Example

```rust,no_run
# #![allow(unexpected_cfgs)]
# #[cfg(feature = "crossterm")]
# {
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::{Terminal, TerminalOptions, Viewport};

let mut terminal = Terminal::with_options(
    CrosstermBackend::new(std::io::stdout()),
    TerminalOptions {
        viewport: Viewport::Fixed(Rect::new(10, 5, 20, 4)),
    },
)?;

terminal.draw(|frame| {
    // `frame.area()` is `Rect::new(10, 5, 20, 4)`, not `(0, 0, 20, 4)`.
    let [title, body] =
        Layout::vertical([Constraint::Length(1), Constraint::Min(0)]).areas(frame.area());

    frame.render_widget("panel title", title);
    frame.render_widget("render the body relative to the fixed viewport", body);
})?;
# }
# Ok::<(), Box<dyn std::error::Error>>(())
```

[`Frame::area`]: crate::terminal::Frame::area
[`Terminal`]: crate::terminal::Terminal
[`Terminal::with_options`]: crate::terminal::Terminal::with_options
[`Terminal::resize`]: crate::terminal::Terminal::resize
[`Terminal::autoresize`]: crate::terminal::Terminal::autoresize

**Variants:**
- `Fullscreen` - Draw into the entire terminal.
- `Inline(u16)` - Draw the application inline with the rest of the terminal output.
- `Fixed(crate::layout::Rect)` - Draw into a fixed region of the terminal.

**Traits:** Eq

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Default**
  - `fn default() -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



