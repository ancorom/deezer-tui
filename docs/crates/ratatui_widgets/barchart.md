**ratatui_widgets > barchart**

# Module: barchart

## Contents

**Structs**

- [`BarChart`](#barchart) - A chart showing values as [bars](Bar).

---

## ratatui_widgets::barchart::BarChart

*Struct*

A chart showing values as [bars](Bar).

Here is a possible `BarChart` output.
```plain
┌─────────────────────────────────┐
│                             ████│
│                        ▅▅▅▅ ████│
│            ▇▇▇▇        ████ ████│
│     ▄▄▄▄   ████ ████   ████ ████│
│▆10▆ █20█   █50█ █40█   █60█ █90█│
│ B1   B2     B1   B2     B1   B2 │
│ Group1      Group2      Group3  │
└─────────────────────────────────┘
```

A `BarChart` is composed of a set of [`Bar`] which can be set via [`BarChart::data`].
Bars can be styled globally ([`BarChart::bar_style`]) or individually ([`Bar::style`]).
There are other methods available to style even more precisely. See [`Bar`] to find out about
each bar component.

The `BarChart` widget can also show groups of bars via [`BarGroup`].
A [`BarGroup`] is a set of [`Bar`], multiple can be added to a `BarChart` using
[`BarChart::data`] multiple time as demonstrated in the example below.

The chart can have a [`Direction`] (by default the bars are [`Vertical`](Direction::Vertical)).
This is set using [`BarChart::direction`].

Note: this is the only widget that doesn't implement `Widget` for `&T` because the current
implementation modifies the internal state of self. This will be fixed in the future.

# Examples

The following example creates a `BarChart` with two groups of bars.
The first group is added by an array slice (`&[(&str, u64)]`).
The second group is added by a [`BarGroup`] instance.
```
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Bar, BarChart, BarGroup, Block};

BarChart::default()
    .block(Block::bordered().title("BarChart"))
    .bar_width(3)
    .bar_gap(1)
    .group_gap(3)
    .bar_style(Style::new().yellow().on_red())
    .value_style(Style::new().red().bold())
    .label_style(Style::new().white())
    .data(&[("A0", 0), ("A1", 2), ("A2", 4), ("A3", 3)])
    .data(BarGroup::new([
        Bar::with_label("B0", 10),
        Bar::with_label("B2", 20),
    ]))
    .max(4);
```

For simpler usages, you can also create a `BarChart` simply by

```rust
use ratatui::widgets::{Bar, BarChart};

BarChart::new([Bar::with_label("A", 10), Bar::with_label("B", 20)]);
```

**Generic Parameters:**
- 'a

**Methods:**

- `fn new<T>(bars: T) -> Self` - Creates a new vertical `BarChart` widget with the given bars.
- `fn vertical<impl Into<Vec<Bar<'a>>>>(bars: impl Trait) -> Self` - Creates a new `BarChart` widget with a vertical direction.
- `fn horizontal<impl Into<Vec<Bar<'a>>>>(bars: impl Trait) -> Self` - Creates a new `BarChart` widget with a horizontal direction.
- `fn grouped<T>(groups: T) -> Self` - Creates a new `BarChart` widget with a group of bars.
- `fn data<impl Into<BarGroup<'a>>>(self: Self, data: impl Trait) -> Self` - Add group of bars to the `BarChart`
- `fn block(self: Self, block: Block<'a>) -> Self` - Surround the [`BarChart`] with a [`Block`].
- `fn max(self: Self, max: u64) -> Self` - Set the value necessary for a [`Bar`] to reach the maximum height.
- `fn bar_style<S>(self: Self, style: S) -> Self` - Set the default style of the bar.
- `fn bar_width(self: Self, width: u16) -> Self` - Set the width of the displayed bars.
- `fn bar_gap(self: Self, gap: u16) -> Self` - Set the gap between each bar.
- `fn bar_set(self: Self, bar_set: symbols::bar::Set<'a>) -> Self` - The [`bar::Set`](ratatui_core::symbols::bar::Set) to use for displaying the bars.
- `fn value_style<S>(self: Self, style: S) -> Self` - Set the default value style of the bar.
- `fn label_style<S>(self: Self, style: S) -> Self` - Set the default label style of the groups and bars.
- `fn group_gap(self: Self, gap: u16) -> Self` - Set the gap between [`BarGroup`].
- `fn style<S>(self: Self, style: S) -> Self` - Set the style of the entire chart.
- `fn direction(self: Self, direction: Direction) -> Self` - Set the direction of the bars.

**Traits:** Eq

**Trait Implementations:**

- **AsRef**
  - `fn as_ref(self: &Self) -> &crate::barchart::BarChart<'a>`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Styled**
  - `fn style(self: &Self) -> Style`
  - `fn set_style<S>(self: Self, style: S) -> <Self as >::Item`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Widget**
  - `fn render(self: Self, area: Rect, buf: & mut Buffer)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



