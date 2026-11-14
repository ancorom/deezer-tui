**ratatui_widgets > sparkline > SparklineBar**

# Module: sparkline::SparklineBar

## Contents

**Functions**

- [`style`](#style) - Sets the style of the bar.

---

## ratatui_widgets::sparkline::SparklineBar::style

*Function*

Sets the style of the bar.

`style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
your own type that implements [`Into<Style>`]).

If not set, the default style of the sparkline will be used.

As well as the style of the sparkline, each [`SparklineBar`] may optionally set its own
style.  If set, the style of the bar will be the style of the sparkline combined with
the style of the bar.

[`Color`]: ratatui_core::style::Color

```rust
fn style<S>(self: Self, style: S) -> Self
```



