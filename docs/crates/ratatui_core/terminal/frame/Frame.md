**ratatui_core > terminal > frame > Frame**

# Module: terminal::frame::Frame

## Contents

**Functions**

- [`area`](#area) - Returns the area of the current frame.
- [`buffer_mut`](#buffer_mut) - Gets the buffer that this `Frame` draws into as a mutable reference.
- [`render_widget`](#render_widget) - Render a [`Widget`] to the current buffer using [`Widget::render`].
- [`set_cursor_position`](#set_cursor_position) - After this frame is rendered, make the cursor visible and put it at the specified `(x, y)`

---

## ratatui_core::terminal::frame::Frame::area

*Function*

Returns the area of the current frame.

This is guaranteed not to change during rendering, so may be called multiple times.

If your app listens for a resize event from the backend, ignore that event's dimensions for
calculations performed during the current render callback and use this value instead. It is
the area of the buffer that is actually being rendered for this pass.

```rust
fn area(self: &Self) -> Rect
```



## ratatui_core::terminal::frame::Frame::buffer_mut

*Function*

Gets the buffer that this `Frame` draws into as a mutable reference.

This is an escape hatch for direct buffer manipulation. Normal applications should prefer
the widget rendering methods so layout and rendering intent stay visible at the call site.

Use this when tests, custom widgets, or specialized integrations need direct cell access
during a render pass.

Changes written here are not visible on the backend until the render pass is applied by
[`Terminal::flush`](crate::terminal::Terminal::flush) or a full
[`Terminal::draw`](crate::terminal::Terminal::draw) /
[`Terminal::try_draw`](crate::terminal::Terminal::try_draw) pass.

# Example

```rust
# use ratatui_core::{backend::TestBackend, terminal::Terminal};
# let backend = TestBackend::new(5, 1);
# let mut terminal = Terminal::new(backend).unwrap();
# let mut frame = terminal.get_frame();
frame.buffer_mut()[(0, 0)].set_symbol("h");
```

```rust
fn buffer_mut(self: & mut Self) -> & mut Buffer
```



## ratatui_core::terminal::frame::Frame::render_widget

*Function*

Render a [`Widget`] to the current buffer using [`Widget::render`].

Usually the area argument is the size of the current frame or a sub-area of the current
frame (which can be obtained using [`Layout`] to split the total area).

Rendering writes directly into the current frame buffer. If multiple widgets cover the same
cells, later renders win for those cells.

# Example

```rust
# use ratatui_core::{backend::TestBackend, terminal::Terminal};
# let backend = TestBackend::new(5, 5);
# let mut terminal = Terminal::new(backend).unwrap();
# let mut frame = terminal.get_frame();
use ratatui_core::layout::Rect;

let area = Rect::new(0, 0, 5, 5);
frame.render_widget("Hello", area);
```

[`Layout`]: crate::layout::Layout

```rust
fn render_widget<W>(self: & mut Self, widget: W, area: Rect)
```



## ratatui_core::terminal::frame::Frame::set_cursor_position

*Function*

After this frame is rendered, make the cursor visible and put it at the specified `(x, y)`
coordinates. If this method is not called, the cursor will be hidden.

The cursor is applied after Ratatui flushes the frame's buffer diff to the backend.

Note that this will interfere with calls to [`Terminal::hide_cursor`],
[`Terminal::show_cursor`], and [`Terminal::set_cursor_position`]. Pick one of the APIs and
stick with it.

[`Terminal::hide_cursor`]: crate::terminal::Terminal::hide_cursor
[`Terminal::show_cursor`]: crate::terminal::Terminal::show_cursor
[`Terminal::set_cursor_position`]: crate::terminal::Terminal::set_cursor_position

```rust
fn set_cursor_position<P>(self: & mut Self, position: P)
```



