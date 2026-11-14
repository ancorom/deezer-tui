**ratatui_image > protocol > Protocol**

# Module: protocol::Protocol

## Contents

**Functions**

- [`needs_placeholder`](#needs_placeholder) - Returns a placeholder area, if the image will not render into the given area, or `None`.

---

## ratatui_image::protocol::Protocol::needs_placeholder

*Function*

Returns a placeholder area, if the image will not render into the given area, or `None`.

The returned [`ratatui::layout::Rect`] is the area the image would cover, constrained by
the size of `area` argument, if the image does not fit.

Kitty and Halfblocks can always render partially, so they always return `None`.

```rust
fn needs_placeholder(self: &Self, area: Rect) -> Option<Rect>
```



