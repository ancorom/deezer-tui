**ratatui_widgets > scrollbar > ScrollbarState**

# Module: scrollbar::ScrollbarState

## Contents

**Functions**

- [`content_length`](#content_length) - Sets the length of the scrollable content.
- [`scroll`](#scroll) - Changes the scroll position based on the provided [`ScrollDirection`].

---

## ratatui_widgets::scrollbar::ScrollbarState::content_length

*Function*

Sets the length of the scrollable content.

This is the number of scrollable items. If items have a length of one, then this is the
same as the number of scrollable cells.

This is a fluent setter method which must be chained or used as it consumes self

```rust
fn content_length(self: Self, content_length: usize) -> Self
```



## ratatui_widgets::scrollbar::ScrollbarState::scroll

*Function*

Changes the scroll position based on the provided [`ScrollDirection`].

```rust
fn scroll(self: & mut Self, direction: ScrollDirection)
```



