**ratatui_core > terminal > frame**

# Module: terminal::frame

## Contents

**Structs**

- [`CompletedFrame`](#completedframe) - `CompletedFrame` represents the state of the terminal after the last successful
- [`Frame`](#frame) - A consistent view into the terminal state for rendering a single frame.

---

## ratatui_core::terminal::frame::CompletedFrame

*Struct*

`CompletedFrame` represents the state of the terminal after the last successful
[`Terminal::draw`] / [`Terminal::try_draw`] render pass has been applied. Therefore, it is only
valid until the next successful draw call.

This lifetime follows Ratatui's double-buffering model: the next render pass swaps buffers via
[`Terminal::swap_buffers`], so the previously completed buffer is no longer the current output.

[`Terminal::draw`]: crate::terminal::Terminal::draw
[`Terminal::swap_buffers`]: crate::terminal::Terminal::swap_buffers
[`Terminal::try_draw`]: crate::terminal::Terminal::try_draw

**Generic Parameters:**
- 'a

**Fields:**
- `buffer: &'a crate::buffer::Buffer` - The buffer that was used to draw the last frame.
- `area: crate::layout::Rect` - The size of the last frame.
- `count: usize` - The frame count indicating the sequence number of this frame.

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## ratatui_core::terminal::frame::Frame

*Struct*

A consistent view into the terminal state for rendering a single frame.

You usually get a `Frame` from the closure argument of [`Terminal::draw`] /
[`Terminal::try_draw`]. For manual rendering, use
[`Terminal::get_frame`](crate::terminal::Terminal::get_frame).

A `Frame` is used to render widgets into Ratatui's current buffer and request the cursor state
for the end of the render pass.

The changes drawn to the frame are applied only to the current [`Buffer`]. After the closure
returns, the current buffer is compared to the previous buffer and only the changed cells are
sent to the backend. This avoids drawing redundant cells.

[`Buffer`]: crate::buffer::Buffer
[`Terminal::draw`]: crate::terminal::Terminal::draw
[`Terminal::try_draw`]: crate::terminal::Terminal::try_draw

**Generic Parameters:**
- 'a

**Methods:**

- `fn area(self: &Self) -> Rect` - Returns the area of the current frame.
- `fn size(self: &Self) -> Rect` - Returns the area of the current frame.
- `fn render_widget<W>(self: & mut Self, widget: W, area: Rect)` - Render a [`Widget`] to the current buffer using [`Widget::render`].
- `fn render_stateful_widget<W>(self: & mut Self, widget: W, area: Rect, state: & mut <W as >::State)` - Render a [`StatefulWidget`] to the current buffer using [`StatefulWidget::render`].
- `fn set_cursor_position<P>(self: & mut Self, position: P)` - After this frame is rendered, make the cursor visible and put it at the specified `(x, y)`
- `fn set_cursor(self: & mut Self, x: u16, y: u16)` - After this frame is rendered, make the cursor visible and put it at the specified `(x, y)`
- `fn buffer_mut(self: & mut Self) -> & mut Buffer` - Gets the buffer that this `Frame` draws into as a mutable reference.
- `fn count(self: &Self) -> usize` - Returns the current frame count.

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



