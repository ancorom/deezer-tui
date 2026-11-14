**ratatui_core > terminal > Terminal**

# Module: terminal::Terminal

## Contents

**Functions**

- [`apply_buffer_with_cursor`](#apply_buffer_with_cursor) - A low-level function that applies and flushes the current buffer to the backend and
- [`autoresize`](#autoresize) - Queries the backend for size and resizes if it doesn't match the previous size.
- [`clear`](#clear) - Clear the terminal and force a full redraw on the next draw call.
- [`draw`](#draw) - Draws a single frame to the terminal.
- [`flush`](#flush) - Applies the current buffer diff to the backend's active display surface.
- [`get_frame`](#get_frame) - Returns a [`Frame`] for manual rendering.
- [`hide_cursor`](#hide_cursor) - Hides the cursor.
- [`insert_before`](#insert_before) - Insert some content before the current inline viewport. This has no effect when the
- [`new`](#new) - Creates a new [`Terminal`] with the given [`Backend`] with a full screen viewport.
- [`resize`](#resize) - Updates the Terminal so that internal buffers match the requested area.
- [`set_cursor_position`](#set_cursor_position) - Sets the cursor position.
- [`show_cursor`](#show_cursor) - Shows the cursor.
- [`swap_buffers`](#swap_buffers) - Clears the inactive buffer and swaps it with the current buffer.
- [`try_draw`](#try_draw) - Tries to draw a single frame to the terminal.
- [`with_options`](#with_options) - Creates a new [`Terminal`] with the given [`Backend`] and [`TerminalOptions`].

---

## ratatui_core::terminal::Terminal::apply_buffer_with_cursor

*Function*

A low-level function that applies and flushes the current buffer to the backend and
re-positions the cursor. This function is useful if you need to manage your own custom
draw lifecycle and buffer.

Returns [`Result::Ok`] containing a [`CompletedFrame`] if successful, otherwise
[`Result::Err`] containing the backend error (`B::Error`) that caused the failure.

This method will:

- show/hide the cursor based on `cursor_position` ([`None`] will hide the cursor)
- call [`Terminal::swap_buffers`] to prepare for the next render pass
- call [`Backend::flush`] to flush any buffered backend output
- return a [`CompletedFrame`] with the current buffer and the area used for rendering

The [`CompletedFrame`] returned by this method can be useful for debugging or testing
purposes, but it is often not used in regular applications.

# Examples

```rust,no_run
# #![allow(unexpected_cfgs)]
# #[cfg(feature = "crossterm")]
# {
use std::io;

use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::buffer::Buffer;
use ratatui::widgets::Widget;

let backend = CrosstermBackend::new(io::stdout());
let mut terminal = Terminal::new(backend)?;

terminal.autoresize()?;

let mut custom_buffer = Buffer::default();
custom_buffer.resize(terminal.get_frame().area());
custom_buffer.reset();

"Hello World!".render(custom_buffer.area, &mut custom_buffer);

terminal.current_buffer_mut().merge(&custom_buffer);
terminal.apply_buffer_with_cursor(None)?;
# }
```

```rust
fn apply_buffer_with_cursor(self: & mut Self, cursor_position: Option<Position>) -> Result<CompletedFrame, <B as >::Error>
```



## ratatui_core::terminal::Terminal::autoresize

*Function*

Queries the backend for size and resizes if it doesn't match the previous size.

This is called automatically during [`Terminal::draw`] / [`Terminal::try_draw`] for
fullscreen and inline viewports. Fixed viewports are not automatically resized.

If the size changed, this calls [`Terminal::resize`] and therefore clears the affected
region before the next frame is rendered.

```rust
fn autoresize(self: & mut Self) -> Result<(), <B as >::Error>
```



## ratatui_core::terminal::Terminal::clear

*Function*

Clear the terminal and force a full redraw on the next draw call.

What gets cleared depends on the active [`Viewport`]:

- [`Viewport::Fullscreen`]: clears the entire terminal.
- [`Viewport::Fixed`]: clears only the viewport region.
- [`Viewport::Inline`]: clears after the viewport's origin, leaving any content above the
  viewport untouched.

Current behavior: for [`Viewport::Inline`], clearing runs from the viewport origin through
the end of the visible display area, not just the viewport's rectangle. This is an
implementation detail rather than a contract; do not rely on it.

This preserves the backend's current cursor position.

This also resets the "previous" buffer so the next [`Terminal::flush`] redraws the full
viewport.

[`Terminal::resize`]: crate::terminal::Terminal::resize

Implementation note: this uses [`ClearType::AfterCursor`] starting at the viewport origin.

```rust
fn clear(self: & mut Self) -> Result<(), <B as >::Error>
```



## ratatui_core::terminal::Terminal::draw

*Function*

Draws a single frame to the terminal.

Returns a [`CompletedFrame`] if successful, otherwise a backend error (`B::Error`).

If the render callback passed to this method can fail, use [`try_draw`] instead.

Applications should call `draw` or [`try_draw`] in a loop to continuously render the
terminal. These methods are the main entry points for drawing to the terminal.

[`try_draw`]: Terminal::try_draw

The [`Frame`] passed to the render callback represents the currently configured
[`Viewport`] (see [`Frame::area`] and [`Terminal::with_options`]).

Build layout relative to the [`Rect`] returned by [`Frame::area`] rather than assuming the
origin is `(0, 0)`, so the same rendering code works for fixed and inline viewports.

[`Frame::area`]: crate::terminal::Frame::area
[`Rect`]: crate::layout::Rect
[`Viewport`]: crate::terminal::Viewport

This method will:

- call [`Terminal::autoresize`] if necessary
- call the render callback, passing it a [`Frame`] reference to render to
- call [`Terminal::flush`] to apply the current buffer diff to the backend
- show/hide the cursor based on [`Frame::set_cursor_position`]
- call [`Terminal::swap_buffers`] to prepare for the next render pass
- call [`Backend::flush`] to flush any buffered backend output
- return a [`CompletedFrame`] with the current buffer and the area used for rendering

If any backend step fails, the error is returned immediately and later steps in the render
pass are skipped.

The [`CompletedFrame`] returned by this method can be useful for debugging or testing
purposes, but it is often not used in regular applications.

The render callback should fully render the entire frame when called, including areas that
are unchanged from the previous frame. This is because each frame is compared to the
previous frame to determine what has changed, and only the changes are written to the
terminal. If the render callback does not fully render the frame, the terminal will not be
in a consistent state.

# Examples

```rust,no_run
# mod ratatui {
#     pub use ratatui_core::backend;
#     pub use ratatui_core::layout;
#     pub use ratatui_core::terminal::{Frame, Terminal};
# }
use ratatui::backend::TestBackend;
use ratatui::layout::Position;
use ratatui::{Frame, Terminal};

let backend = TestBackend::new(10, 10);
let mut terminal = Terminal::new(backend)?;

// With a closure.
terminal.draw(|frame| {
    let area = frame.area();
    frame.render_widget("Hello World!", area);
    frame.set_cursor_position(Position { x: 0, y: 0 });
})?;

// Or with a function.
terminal.draw(render)?;

fn render(frame: &mut Frame<'_>) {
    frame.render_widget("Hello World!", frame.area());
}
# Ok::<(), Box<dyn std::error::Error>>(())
```

[`Backend::flush`]: crate::backend::Backend::flush

```rust
fn draw<F>(self: & mut Self, render_callback: F) -> Result<CompletedFrame, <B as >::Error>
```



## ratatui_core::terminal::Terminal::flush

*Function*

Applies the current buffer diff to the backend's active display surface.

This compares the current buffer with the previous buffer and passes only the changed cells
to [`Backend::draw`]. It is one of the building blocks used by [`Terminal::draw`] /
[`Terminal::try_draw`].

This method does not swap buffers, does not update cursor visibility or position, and does
not call [`Backend::flush`]. See [`Terminal::swap_buffers`] and [`Backend::flush`].

`Terminal::flush` only reasons about Ratatui's internal buffers. It does not know whether
the backend's display surface changed since the last render pass. For example, if you leave
the alternate screen and then call `Terminal::flush`, Ratatui may replay a diff that was
computed for the alternate screen onto the main screen. In normal applications, prefer
[`Terminal::draw`] / [`Terminal::try_draw`] unless you are intentionally managing the whole
render pipeline yourself.

Implementation note: when there are updates, Ratatui records the position of the last
updated cell as the "last known cursor position". Inline viewports use this to preserve the
cursor's relative position within the viewport across resizes.

[`Backend::flush`]: crate::backend::Backend::flush

```rust
fn flush(self: & mut Self) -> Result<(), <B as >::Error>
```



## ratatui_core::terminal::Terminal::get_frame

*Function*

Returns a [`Frame`] for manual rendering.

Most applications should render via [`Terminal::draw`] / [`Terminal::try_draw`]. This is an
escape hatch that exposes the frame construction step used by [`Terminal::try_draw`] so
tests and advanced callers can render without running the full draw pipeline.

This is primarily useful for tests, backend adapters, and specialized integrations that
intentionally manage presentation themselves.

Unlike `draw` / `try_draw`, this does not call [`Terminal::autoresize`], does not write
updates to the backend, and does not apply any cursor changes. After rendering, you
typically call [`Terminal::flush`], [`Terminal::swap_buffers`], and [`Backend::flush`].

For the full render-pass behavior that also handles resizing, cursor updates, buffer
swapping, and backend flushing, see [`Terminal::draw`] and [`Terminal::try_draw`].

The returned `Frame` mutably borrows the current buffer, so it must be dropped before you
can call methods like [`Terminal::flush`]. The example below uses a scope to make that
explicit.

# Example

```rust,no_run
# mod ratatui {
#     pub use ratatui_core::backend;
#     pub use ratatui_core::terminal::Terminal;
# }
use ratatui::Terminal;
use ratatui::backend::{Backend, TestBackend};

let backend = TestBackend::new(30, 5);
let mut terminal = Terminal::new(backend)?;
{
    let mut frame = terminal.get_frame();
    frame.render_widget("Hello", frame.area());
}
// When not using `draw`, present the buffer manually:
terminal.flush()?;
terminal.swap_buffers();
terminal.backend_mut().flush()?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

[`Backend::flush`]: crate::backend::Backend::flush

```rust
fn get_frame(self: & mut Self) -> Frame
```



## ratatui_core::terminal::Terminal::hide_cursor

*Function*

Hides the cursor.

When using [`Terminal::draw`] / [`Terminal::try_draw`], prefer controlling the cursor with
[`Frame::set_cursor_position`]. A later successful [`Terminal::draw`] /
[`Terminal::try_draw`] call may overwrite this change.

[`Frame::set_cursor_position`]: crate::terminal::Frame::set_cursor_position
[`Terminal::draw`]: crate::terminal::Terminal::draw
[`Terminal::try_draw`]: crate::terminal::Terminal::try_draw

```rust
fn hide_cursor(self: & mut Self) -> Result<(), <B as >::Error>
```



## ratatui_core::terminal::Terminal::insert_before

*Function*

Insert some content before the current inline viewport. This has no effect when the
viewport is not inline.

This is intended for inline UIs that want to print output (e.g. logs or status messages)
above the UI without breaking it. See [`Viewport::Inline`] for how inline viewports are
anchored.

The `draw_fn` closure will be called to draw into a writable `Buffer` that is `height`
lines tall. The content of that `Buffer` will then be inserted before the viewport.

When Ratatui is built with the `scrolling-regions` feature, this can be done without
clearing and redrawing the viewport. Without `scrolling-regions`, Ratatui falls back to a
more portable approach and clears the viewport so the next [`Terminal::draw`] /
[`Terminal::try_draw`] repaints it.

If the viewport isn't yet at the bottom of the screen, inserted lines will push it towards
the bottom. Once the viewport is at the bottom of the screen, inserted lines will scroll
the area of the screen above the viewport upwards.

Before:
```text
+---------------------+
| pre-existing line 1 |
| pre-existing line 2 |
+---------------------+
|       viewport      |
+---------------------+
|                     |
|                     |
+---------------------+
```

After inserting 2 lines:
```text
+---------------------+
| pre-existing line 1 |
| pre-existing line 2 |
|   inserted line 1   |
|   inserted line 2   |
+---------------------+
|       viewport      |
+---------------------+
+---------------------+
```

After inserting 2 more lines:
```text
+---------------------+
| pre-existing line 2 |
|   inserted line 1   |
|   inserted line 2   |
|   inserted line 3   |
|   inserted line 4   |
+---------------------+
|       viewport      |
+---------------------+
```

If more lines are inserted than there is space on the screen, then the top lines will go
directly into the terminal's scrollback buffer. At the limit, if the viewport takes up the
whole screen, all lines will be inserted directly into the scrollback buffer.

# Examples

## Insert a single line before the current viewport

```rust,no_run
# mod ratatui {
#     pub use ratatui_core::backend;
#     pub use ratatui_core::layout;
#     pub use ratatui_core::style;
#     pub use ratatui_core::terminal::{Terminal, TerminalOptions, Viewport};
#     pub use ratatui_core::text;
#     pub use ratatui_core::widgets;
# }
use ratatui::backend::{Backend, TestBackend};
use ratatui::layout::Position;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Widget;
use ratatui::{Terminal, TerminalOptions, Viewport};

let mut backend = TestBackend::new(10, 10);
// Simulate existing output above the inline UI.
backend.set_cursor_position(Position::new(0, 3))?;
let mut terminal = Terminal::with_options(
    backend,
    TerminalOptions {
        viewport: Viewport::Inline(4),
    },
)?;

terminal.insert_before(1, |buf| {
    Line::from(vec![
        Span::raw("This line will be added "),
        Span::styled("before", Style::default().fg(Color::Blue)),
        Span::raw(" the current viewport"),
    ])
    .render(buf.area, buf);
})?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

```rust
fn insert_before<F>(self: & mut Self, height: u16, draw_fn: F) -> Result<(), <B as >::Error>
```



## ratatui_core::terminal::Terminal::new

*Function*

Creates a new [`Terminal`] with the given [`Backend`] with a full screen viewport.

This is a convenience for [`Terminal::with_options`] with [`Viewport::Fullscreen`].
Ratatui initializes two empty buffers sized to the backend's current screen area and treats
future backend size changes as redraw-triggering resizes during render passes.

After creating a terminal, call [`Terminal::draw`] (or [`Terminal::try_draw`]) in a loop to
render your UI.

Note that unlike [`ratatui::init`], this does not install a panic hook, so it is
recommended to do that manually when using this function, otherwise any panic messages will
be printed to the alternate screen and the terminal may be left in an unusable state.

See [how to set up panic hooks](https://ratatui.rs/recipes/apps/panic-hooks/) and
[`better-panic` example](https://ratatui.rs/recipes/apps/better-panic/) for more
information.

# Example

```rust,no_run
# #![allow(unexpected_cfgs)]
# #[cfg(feature = "crossterm")]
# {
use std::io::stdout;

use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

let backend = CrosstermBackend::new(stdout());
let _terminal = Terminal::new(backend)?;

// Optionally set up a panic hook to restore the terminal on panic.
let old_hook = std::panic::take_hook();
std::panic::set_hook(Box::new(move |info| {
    ratatui::restore();
    old_hook(info);
}));
# }
# #[cfg(not(feature = "crossterm"))]
# {
# use ratatui_core::{backend::TestBackend, terminal::Terminal};
# let backend = TestBackend::new(10, 10);
# let _terminal = Terminal::new(backend)?;
# }
# Ok::<(), Box<dyn std::error::Error>>(())
```

[`ratatui::init`]: https://docs.rs/ratatui/latest/ratatui/fn.init.html

```rust
fn new(backend: B) -> Result<Self, <B as >::Error>
```



## ratatui_core::terminal::Terminal::resize

*Function*

Updates the Terminal so that internal buffers match the requested area.

This updates the buffer size used for rendering and triggers a full clear so the next
[`Terminal::draw`] / [`Terminal::try_draw`] paints into a consistent area.

When the viewport is [`Viewport::Inline`], the `area` argument is treated as the new
terminal size and the viewport origin is recomputed relative to the current cursor position.
Ratatui attempts to keep the cursor at the same relative row within the viewport across
resizes.

See also: [`Terminal::autoresize`] (automatic resizing during [`Terminal::draw`] /
[`Terminal::try_draw`]).

For [`Viewport::Fixed`] and [`Viewport::Fullscreen`], `area` becomes the new viewport area.
For [`Viewport::Inline`], `area` is interpreted as the backend's new terminal size and the
viewport origin may move to preserve the cursor's relative row within the inline UI.

```rust
fn resize(self: & mut Self, area: Rect) -> Result<(), <B as >::Error>
```



## ratatui_core::terminal::Terminal::set_cursor_position

*Function*

Sets the cursor position.

This updates the backend cursor and Ratatui's internal cursor tracking. Inline viewports
use that tracking when recomputing the viewport on resize.

When using [`Terminal::draw`] / [`Terminal::try_draw`], consider using
[`Frame::set_cursor_position`] instead so the cursor is updated as part of the normal
rendering flow. A later successful
[`Terminal::draw`] / [`Terminal::try_draw`] call may overwrite a direct cursor move.

[`Frame::set_cursor_position`]: crate::terminal::Frame::set_cursor_position
[`Terminal::draw`]: crate::terminal::Terminal::draw
[`Terminal::try_draw`]: crate::terminal::Terminal::try_draw

```rust
fn set_cursor_position<P>(self: & mut Self, position: P) -> Result<(), <B as >::Error>
```



## ratatui_core::terminal::Terminal::show_cursor

*Function*

Shows the cursor.

When using [`Terminal::draw`] / [`Terminal::try_draw`], prefer controlling the cursor with
[`Frame::set_cursor_position`]. A later successful [`Terminal::draw`] /
[`Terminal::try_draw`] call may overwrite this change.

[`Frame::set_cursor_position`]: crate::terminal::Frame::set_cursor_position
[`Terminal::draw`]: crate::terminal::Terminal::draw
[`Terminal::try_draw`]: crate::terminal::Terminal::try_draw

```rust
fn show_cursor(self: & mut Self) -> Result<(), <B as >::Error>
```



## ratatui_core::terminal::Terminal::swap_buffers

*Function*

Clears the inactive buffer and swaps it with the current buffer.

This is part of the standard rendering flow (see [`Terminal::try_draw`]). If you render
manually using [`Terminal::get_frame`] and [`Terminal::flush`], call this immediately
afterward so the next flush can compute diffs against the correct "previous" buffer.

```rust
fn swap_buffers(self: & mut Self)
```



## ratatui_core::terminal::Terminal::try_draw

*Function*

Tries to draw a single frame to the terminal.

Returns [`Result::Ok`] containing a [`CompletedFrame`] if successful, otherwise
[`Result::Err`] containing the backend error (`B::Error`) that caused the failure.

This is the equivalent of [`Terminal::draw`] but the render callback is a function or
closure that returns a `Result` instead of nothing.

Applications should call `try_draw` or [`draw`] in a loop to continuously render the
terminal. These methods are the main entry points for drawing to the terminal.

[`draw`]: Terminal::draw

The [`Frame`] passed to the render callback represents the currently configured
[`Viewport`] (see [`Frame::area`] and [`Terminal::with_options`]).

Build layout relative to the [`Rect`] returned by [`Frame::area`] rather than assuming the
origin is `(0, 0)`, so the same rendering code works for fixed and inline viewports.

[`Frame::area`]: crate::terminal::Frame::area
[`Rect`]: crate::layout::Rect
[`Viewport`]: crate::terminal::Viewport

This method will:

- call [`Terminal::autoresize`] if necessary
- call the render callback, passing it a [`Frame`] reference to render to
- call [`Terminal::flush`] to apply the current buffer diff to the backend
- show/hide the cursor based on [`Frame::set_cursor_position`]
- call [`Terminal::swap_buffers`] to prepare for the next render pass
- call [`Backend::flush`] to flush any buffered backend output
- return a [`CompletedFrame`] with the current buffer and the area used for rendering

If the render callback returns an error, Ratatui leaves the backend, buffers, cursor state,
and frame count unchanged.

The render callback passed to `try_draw` can return any [`Result`] with an error type that
can be converted into `B::Error` using the [`Into`] trait. This makes it possible to use the
`?` operator to propagate errors that occur during rendering. If the render callback returns
an error, the error will be returned from `try_draw` and the terminal will not be updated.

The [`CompletedFrame`] returned by this method can be useful for debugging or testing
purposes, but it is often not used in regular applications.

The render callback should fully render the entire frame when called, including areas that
are unchanged from the previous frame. This is because each frame is compared to the
previous frame to determine what has changed, and only the changes are written to the
terminal. If the render function does not fully render the frame, the terminal will not be
in a consistent state.

# Examples

```rust,no_run
# #![allow(unexpected_cfgs)]
# #[cfg(feature = "crossterm")]
# {
use std::io;

use ratatui::backend::CrosstermBackend;
use ratatui::layout::Position;
use ratatui::{Frame, Terminal};

let backend = CrosstermBackend::new(std::io::stdout());
let mut terminal = Terminal::new(backend)?;

// With a closure that returns `Result`.
terminal.try_draw(|frame| -> io::Result<()> {
    let _value: u8 = "42".parse().map_err(io::Error::other)?;
    let area = frame.area();
    frame.render_widget("Hello World!", area);
    frame.set_cursor_position(Position { x: 0, y: 0 });
    Ok(())
})?;

// Or with a function.
terminal.try_draw(render)?;

fn render(frame: &mut Frame<'_>) -> io::Result<()> {
    frame.render_widget("Hello World!", frame.area());
    Ok(())
}
# }
# #[cfg(not(feature = "crossterm"))]
# {
# use ratatui_core::{backend::TestBackend, terminal::Terminal};
# let backend = TestBackend::new(10, 10);
# let mut terminal = Terminal::new(backend)?;
# terminal
#     .try_draw(|frame| {
#         frame.render_widget("Hello World!", frame.area());
#         Ok::<(), core::convert::Infallible>(())
#     })
#     ?;
# }
# Ok::<(), Box<dyn std::error::Error>>(())
```

[`Backend::flush`]: crate::backend::Backend::flush

```rust
fn try_draw<F, E>(self: & mut Self, render_callback: F) -> Result<CompletedFrame, <B as >::Error>
```



## ratatui_core::terminal::Terminal::with_options

*Function*

Creates a new [`Terminal`] with the given [`Backend`] and [`TerminalOptions`].

The viewport determines what area is exposed to widgets via [`Frame::area`] and how Ratatui
keeps its internal buffers synchronized with the backend. See [`Viewport`] for an overview
of the available modes.

For viewport behavior after initialization, see [`Terminal::resize`] and
[`Terminal::autoresize`].

[`Frame::area`]: crate::terminal::Frame::area
[`Terminal::autoresize`]: crate::terminal::Terminal::autoresize
[`Terminal::resize`]: crate::terminal::Terminal::resize

After creating a terminal, call [`Terminal::draw`] (or [`Terminal::try_draw`]) in a loop to
render your UI.

Resize behavior depends on the selected viewport:

- [`Viewport::Fullscreen`] and [`Viewport::Inline`] are automatically resized during
  [`Terminal::draw`] / [`Terminal::try_draw`] (via [`Terminal::autoresize`]).
- [`Viewport::Fixed`] is not automatically resized; call [`Terminal::resize`] if the region
  should change.

# Example

```rust,no_run
# #![allow(unexpected_cfgs)]
# #[cfg(feature = "crossterm")]
# {
use std::io::stdout;

use ratatui::backend::CrosstermBackend;
use ratatui::layout::Rect;
use ratatui::{Terminal, TerminalOptions, Viewport};

let backend = CrosstermBackend::new(stdout());
let viewport = Viewport::Fixed(Rect::new(0, 0, 10, 10));
let _terminal = Terminal::with_options(backend, TerminalOptions { viewport })?;
# }
# #[cfg(not(feature = "crossterm"))]
# {
# use ratatui_core::{
#     backend::TestBackend,
#     layout::Rect,
#     terminal::{Terminal, TerminalOptions, Viewport},
# };
# let backend = TestBackend::new(10, 10);
# let viewport = Viewport::Fixed(Rect::new(0, 0, 10, 10));
# let _terminal = Terminal::with_options(backend, TerminalOptions { viewport })?;
# }
# Ok::<(), Box<dyn std::error::Error>>(())
```

When the viewport is [`Viewport::Inline`], Ratatui anchors the viewport to the current
cursor row at initialization time (always starting at column 0). Ratatui may append lines
and thereby scroll the terminal to make enough room for the requested height so the
viewport stays fully visible.

```rust
fn with_options(backend: B, options: TerminalOptions) -> Result<Self, <B as >::Error>
```



