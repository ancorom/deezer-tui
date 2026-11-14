**ratatui_widgets > sparkline > Sparkline**

# Module: sparkline::Sparkline

## Contents

**Functions**

- [`absent_value_style`](#absent_value_style) - Sets the style to use for absent values.
- [`absent_value_symbol`](#absent_value_symbol) - Sets the symbol to use for absent values.
- [`bar_set`](#bar_set) - Sets the characters used to display the bars.
- [`block`](#block) - Wraps the sparkline with the given `block`.
- [`data`](#data) - Sets the dataset for the sparkline.
- [`direction`](#direction) - Sets the direction of the sparkline.
- [`max`](#max) - Sets the maximum value of bars.
- [`style`](#style) - Sets the style of the entire widget.

---

## ratatui_widgets::sparkline::Sparkline::absent_value_style

*Function*

Sets the style to use for absent values.

Absent values are values in the dataset that are `None`.

`style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
your own type that implements [`Into<Style>`]).

The foreground corresponds to the bars while the background is everything else.

[`Color`]: ratatui_core::style::Color

```rust
fn absent_value_style<S>(self: Self, style: S) -> Self
```



## ratatui_widgets::sparkline::Sparkline::absent_value_symbol

*Function*

Sets the symbol to use for absent values.

Absent values are values in the dataset that are `None`.

The default is [`symbols::shade::EMPTY`].

```rust
fn absent_value_symbol<impl Into<String>>(self: Self, symbol: impl Trait) -> Self
```



## ratatui_widgets::sparkline::Sparkline::bar_set

*Function*

Sets the characters used to display the bars.

Can be [`symbols::bar::THREE_LEVELS`], [`symbols::bar::NINE_LEVELS`] (default) or a custom
[`Set`](symbols::bar::Set).

```rust
fn bar_set(self: Self, bar_set: symbols::bar::Set<'a>) -> Self
```



## ratatui_widgets::sparkline::Sparkline::block

*Function*

Wraps the sparkline with the given `block`.

```rust
fn block(self: Self, block: Block<'a>) -> Self
```



## ratatui_widgets::sparkline::Sparkline::data

*Function*

Sets the dataset for the sparkline.

Each item in the dataset is a bar in the sparkline. The height of the bar is determined by
the value in the dataset.

The data can be a slice of `u64`, `Option<u64>`, or a [`SparklineBar`].  For the
`Option<u64>` and [`SparklineBar`] cases, a data point with a value of `None` is
interpreted an as the _absence_ of a value.

If the data provided is a slice of `u64` or `Option<u64>`, the bars will be styled with the
style of the sparkline. If the data is a slice of [`SparklineBar`], the bars will be
styled with the style of the sparkline combined with the style provided in the
[`SparklineBar`] if it is set, otherwise the sparkline style will be used.

Absent values and will be rendered with the style set by [`Sparkline::absent_value_style`]
and the symbol set by [`Sparkline::absent_value_symbol`].

# Examples

Create a `Sparkline` from a slice of `u64`:

```
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::Sparkline;

# fn ui(frame: &mut Frame) {
# let area = Rect::default();
let sparkline = Sparkline::default().data(&[1, 2, 3]);
frame.render_widget(sparkline, area);
# }
```

Create a `Sparkline` from a slice of `Option<u64>` such that some bars are absent:

```
# use ratatui::{prelude::*, widgets::*};
# fn ui(frame: &mut Frame) {
# let area = Rect::default();
let data = vec![Some(1), None, Some(3)];
let sparkline = Sparkline::default().data(data);
frame.render_widget(sparkline, area);
# }
```

Create a [`Sparkline`] from a Vec of [`SparklineBar`] such that some bars are styled:

```
# use ratatui::{prelude::*, widgets::*};
# fn ui(frame: &mut Frame) {
# let area = Rect::default();
let data = vec![
    SparklineBar::from(1).style(Some(Style::default().fg(Color::Red))),
    SparklineBar::from(2),
    SparklineBar::from(3).style(Some(Style::default().fg(Color::Blue))),
];
let sparkline = Sparkline::default().data(data);
frame.render_widget(sparkline, area);
# }
```

```rust
fn data<T>(self: Self, data: T) -> Self
```



## ratatui_widgets::sparkline::Sparkline::direction

*Function*

Sets the direction of the sparkline.

[`RenderDirection::LeftToRight`] by default.

```rust
fn direction(self: Self, direction: RenderDirection) -> Self
```



## ratatui_widgets::sparkline::Sparkline::max

*Function*

Sets the maximum value of bars.

Every bar will be scaled accordingly. If no max is given, this will be the max in the
dataset.

```rust
fn max(self: Self, max: u64) -> Self
```



## ratatui_widgets::sparkline::Sparkline::style

*Function*

Sets the style of the entire widget.

`style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
your own type that implements [`Into<Style>`]).

The foreground corresponds to the bars while the background is everything else.

[`Color`]: ratatui_core::style::Color

```rust
fn style<S>(self: Self, style: S) -> Self
```



