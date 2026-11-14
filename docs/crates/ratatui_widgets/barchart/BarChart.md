**ratatui_widgets > barchart > BarChart**

# Module: barchart::BarChart

## Contents

**Functions**

- [`bar_style`](#bar_style) - Set the default style of the bar.
- [`data`](#data) - Add group of bars to the `BarChart`
- [`direction`](#direction) - Set the direction of the bars.

---

## ratatui_widgets::barchart::BarChart::bar_style

*Function*

Set the default style of the bar.

`style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
your own type that implements [`Into<Style>`]).

It is also possible to set individually the style of each [`Bar`].
In this case the default style will be patched by the individual style

[`Color`]: ratatui_core::style::Color

```rust
fn bar_style<S>(self: Self, style: S) -> Self
```



## ratatui_widgets::barchart::BarChart::data

*Function*

Add group of bars to the `BarChart`

# Examples

The following example creates a `BarChart` with two groups of bars.
The first group is added by an array slice (`&[(&str, u64)]`).
The second group is added by a [`BarGroup`] instance.
```
use ratatui::widgets::{Bar, BarChart, BarGroup};

BarChart::default()
    .data(&[("B0", 0), ("B1", 2), ("B2", 4), ("B3", 3)])
    .data(BarGroup::new([
        Bar::with_label("A", 10),
        Bar::with_label("B", 20),
    ]));
```

```rust
fn data<impl Into<BarGroup<'a>>>(self: Self, data: impl Trait) -> Self
```



## ratatui_widgets::barchart::BarChart::direction

*Function*

Set the direction of the bars.

[`Vertical`](ratatui_core::layout::Direction::Vertical) bars are the default.

# Examples

Vertical bars
```plain
  █
█ █
f b
```

Horizontal bars
```plain
█foo██

█bar██
```

```rust
fn direction(self: Self, direction: Direction) -> Self
```



