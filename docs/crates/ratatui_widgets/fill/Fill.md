**ratatui_widgets > fill > Fill**

# Module: fill::Fill

## Contents

**Functions**

- [`style`](#style) - Set the style used to paint each cell.

---

## ratatui_widgets::fill::Fill::style

*Function*

Set the style used to paint each cell.

`style` accepts any value convertible into a [`Style`] (e.g. [`Style`],
[`Color`](ratatui_core::style::Color), or your own type implementing
[`Into<Style>`]).

This is a fluent setter method which must be chained or used as it consumes self

```rust
fn style<S>(self: Self, style: S) -> Self
```



