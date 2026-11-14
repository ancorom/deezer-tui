**ratatui_widgets > sparkline**

# Module: sparkline

## Contents

**Structs**

- [`Sparkline`](#sparkline) - Widget to render a sparkline over one or more lines.
- [`SparklineBar`](#sparklinebar) - An bar in a `Sparkline`.

**Enums**

- [`RenderDirection`](#renderdirection) - Defines the direction in which sparkline will be rendered.

---

## ratatui_widgets::sparkline::RenderDirection

*Enum*

Defines the direction in which sparkline will be rendered.

See [`Sparkline::direction`].

**Variants:**
- `LeftToRight` - The first value is on the left, going to the right
- `RightToLeft` - The first value is on the right, going to the left

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::result::Result<(), ::core::fmt::Error>`
- **FromStr**
  - `fn from_str(s: &str) -> ::core::result::Result<RenderDirection, <Self as ::core::str::FromStr>::Err>`
- **TryFrom**
  - `fn try_from(s: &str) -> ::core::result::Result<RenderDirection, <Self as ::core::convert::TryFrom>::Error>`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## ratatui_widgets::sparkline::Sparkline

*Struct*

Widget to render a sparkline over one or more lines.

Each bar in a `Sparkline` represents a value from the provided dataset. The height of the bar
is determined by the value in the dataset.

You can create a `Sparkline` using [`Sparkline::default`].

The data is set using [`Sparkline::data`]. The data can be a slice of `u64`, `Option<u64>`, or a
[`SparklineBar`].  For the `Option<u64>` and [`SparklineBar`] cases, a data point with a value
of `None` is interpreted an as the _absence_ of a value.

`Sparkline` can be styled either using [`Sparkline::style`] or preferably using the methods
provided by the [`Stylize`](ratatui_core::style::Stylize) trait.  The style may be set for the
entire widget or for individual bars by setting individual [`SparklineBar::style`].

The bars are rendered using a set of symbols. The default set is [`symbols::bar::NINE_LEVELS`].
You can change the set using [`Sparkline::bar_set`].

If the data provided is a slice of `u64` or `Option<u64>`, the bars will be styled with the
style of the sparkline. If the data is a slice of [`SparklineBar`], the bars will be
styled with the style of the sparkline combined with the style provided in the [`SparklineBar`]
if it is set, otherwise the sparkline style will be used.

Absent values and will be rendered with the style set by [`Sparkline::absent_value_style`] and
the symbol set by [`Sparkline::absent_value_symbol`].

# Setter methods

- [`Sparkline::block`] wraps the sparkline in a [`Block`]
- [`Sparkline::data`] defines the dataset, you'll almost always want to use it
- [`Sparkline::max`] sets the maximum value of bars
- [`Sparkline::direction`] sets the render direction

# Examples

```
use ratatui::style::{Color, Style, Stylize};
use ratatui::symbols;
use ratatui::widgets::{Block, RenderDirection, Sparkline};

Sparkline::default()
    .block(Block::bordered().title("Sparkline"))
    .data(&[0, 2, 3, 4, 1, 4, 10])
    .max(5)
    .direction(RenderDirection::RightToLeft)
    .style(Style::default().red().on_white())
    .absent_value_style(Style::default().fg(Color::Red))
    .absent_value_symbol(symbols::shade::FULL);
```

**Generic Parameters:**
- 'a

**Methods:**

- `fn block(self: Self, block: Block<'a>) -> Self` - Wraps the sparkline with the given `block`.
- `fn style<S>(self: Self, style: S) -> Self` - Sets the style of the entire widget.
- `fn absent_value_style<S>(self: Self, style: S) -> Self` - Sets the style to use for absent values.
- `fn absent_value_symbol<impl Into<String>>(self: Self, symbol: impl Trait) -> Self` - Sets the symbol to use for absent values.
- `fn data<T>(self: Self, data: T) -> Self` - Sets the dataset for the sparkline.
- `fn max(self: Self, max: u64) -> Self` - Sets the maximum value of bars.
- `fn bar_set(self: Self, bar_set: symbols::bar::Set<'a>) -> Self` - Sets the characters used to display the bars.
- `fn direction(self: Self, direction: RenderDirection) -> Self` - Sets the direction of the sparkline.

**Traits:** Eq

**Trait Implementations:**

- **Styled**
  - `fn style(self: &Self) -> Style`
  - `fn set_style<S>(self: Self, style: S) -> <Self as >::Item`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **AsRef**
  - `fn as_ref(self: &Self) -> &crate::sparkline::Sparkline<'a>`
- **Default**
  - `fn default() -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Widget**
  - `fn render(self: Self, area: Rect, buf: & mut Buffer)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ratatui_widgets::sparkline::SparklineBar

*Struct*

An bar in a `Sparkline`.

The height of the bar is determined by the value and a value of `None` is interpreted as the
_absence_ of a value, as distinct from a value of `Some(0)`.

**Methods:**

- `fn style<S>(self: Self, style: S) -> Self` - Sets the style of the bar.

**Traits:** Eq, Copy

**Trait Implementations:**

- **From**
  - `fn from(value: &Option<u64>) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **From**
  - `fn from(value: &u64) -> Self`
- **Default**
  - `fn default() -> Self`
- **From**
  - `fn from(value: u64) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(value: Option<u64>) -> Self`



