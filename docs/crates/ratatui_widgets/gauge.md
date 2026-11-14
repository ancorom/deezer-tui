**ratatui_widgets > gauge**

# Module: gauge

## Contents

**Structs**

- [`Gauge`](#gauge) - A widget to display a progress bar.
- [`LineGauge`](#linegauge) - A compact widget to display a progress bar over a single thin line.

---

## ratatui_widgets::gauge::Gauge

*Struct*

A widget to display a progress bar.

A `Gauge` renders a bar filled according to the value given to [`Gauge::percent`] or
[`Gauge::ratio`]. The bar width and height are defined by the [`Rect`] it is
[rendered](Widget::render) in.

The associated label is always centered horizontally and vertically. If not set with
[`Gauge::label`], the label is the percentage of the bar filled.

You might want to have a higher precision bar using [`Gauge::use_unicode`].

This can be useful to indicate the progression of a task, like a download.

# Example

```
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Block, Gauge};

Gauge::default()
    .block(Block::bordered().title("Progress"))
    .gauge_style(Style::new().white().on_black().italic())
    .percent(20);
```

# See also

- [`LineGauge`] for a thin progress bar

**Generic Parameters:**
- 'a

**Methods:**

- `fn block(self: Self, block: Block<'a>) -> Self` - Surrounds the `Gauge` with a [`Block`].
- `fn percent(self: Self, percent: u16) -> Self` - Sets the bar progression from a percentage.
- `fn ratio(self: Self, ratio: f64) -> Self` - Sets the bar progression from a ratio (float).
- `fn label<T>(self: Self, label: T) -> Self` - Sets the label to display in the center of the bar.
- `fn style<S>(self: Self, style: S) -> Self` - Sets the widget style.
- `fn gauge_style<S>(self: Self, style: S) -> Self` - Sets the style of the bar.
- `fn use_unicode(self: Self, unicode: bool) -> Self` - Sets whether to use unicode characters to display the progress bar.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **AsRef**
  - `fn as_ref(self: &Self) -> &crate::gauge::Gauge<'a>`
- **Widget**
  - `fn render(self: Self, area: Rect, buf: & mut Buffer)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Styled**
  - `fn style(self: &Self) -> Style`
  - `fn set_style<S>(self: Self, style: S) -> <Self as >::Item`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Default**
  - `fn default() -> Self`



## ratatui_widgets::gauge::LineGauge

*Struct*

A compact widget to display a progress bar over a single thin line.

This can be useful to indicate the progression of a task, like a download.

A `LineGauge` renders a line filled with symbols defined by [`LineGauge::filled_symbol`] and
[`LineGauge::unfilled_symbol`] according to the value given to [`LineGauge::ratio`].
Unlike [`Gauge`], only the width can be defined by the [rendering](Widget::render) [`Rect`]. The
height is always 1.

The associated label is always left-aligned. If not set with [`LineGauge::label`], the label is
the percentage of the bar filled.

You can also set the symbols used to draw the bar with [`LineGauge::line_set`].

To style the gauge line use [`LineGauge::filled_style`] and [`LineGauge::unfilled_style`] which
let you pick a color for foreground (i.e. line) and background of the filled and unfilled part
of gauge respectively.

# Examples:

```
use ratatui::style::{Style, Stylize};
use ratatui::symbols;
use ratatui::widgets::{Block, LineGauge};

LineGauge::default()
    .block(Block::bordered().title("Progress"))
    .filled_style(Style::new().white().on_black().bold())
    .filled_symbol(symbols::line::THICK_HORIZONTAL)
    .ratio(0.4);
```

# See also

- [`Gauge`] for bigger, higher precision and more configurable progress bar

**Generic Parameters:**
- 'a

**Methods:**

- `fn block(self: Self, block: Block<'a>) -> Self` - Surrounds the `LineGauge` with a [`Block`].
- `fn ratio(self: Self, ratio: f64) -> Self` - Sets the bar progression from a ratio (float).
- `fn line_set(self: Self, set: symbols::line::Set<'a>) -> Self` - Sets the characters to use for the line.
- `fn filled_symbol(self: Self, symbol: &'a str) -> Self` - Sets the symbol for the filled part of the gauge.
- `fn unfilled_symbol(self: Self, symbol: &'a str) -> Self` - Sets the symbol for the unfilled part of the gauge.
- `fn label<T>(self: Self, label: T) -> Self` - Sets the label to display.
- `fn style<S>(self: Self, style: S) -> Self` - Sets the widget style.
- `fn gauge_style<S>(self: Self, style: S) -> Self` - Sets the style of the bar.
- `fn filled_style<S>(self: Self, style: S) -> Self` - Sets the style of filled part of the bar.
- `fn unfilled_style<S>(self: Self, style: S) -> Self` - Sets the style of the unfilled part of the bar.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **AsRef**
  - `fn as_ref(self: &Self) -> &crate::gauge::LineGauge<'a>`
- **Widget**
  - `fn render(self: Self, area: Rect, buf: & mut Buffer)`
- **Styled**
  - `fn style(self: &Self) -> Style`
  - `fn set_style<S>(self: Self, style: S) -> <Self as >::Item`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



