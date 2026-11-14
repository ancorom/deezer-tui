**ratatui_widgets > chart > Axis**

# Module: chart::Axis

## Contents

**Functions**

- [`bounds`](#bounds) - Sets the bounds of this axis
- [`labels`](#labels) - Sets the axis labels
- [`title`](#title) - Sets the axis title

---

## ratatui_widgets::chart::Axis::bounds

*Function*

Sets the bounds of this axis

In other words, sets the min and max value on this axis.

This is a fluent setter method which must be chained or used as it consumes self

```rust
fn bounds(self: Self, bounds: [f64; 2]) -> Self
```



## ratatui_widgets::chart::Axis::labels

*Function*

Sets the axis labels

- For the X axis, the labels are displayed left to right.
- For the Y axis, the labels are displayed bottom to top.

Currently, you need to give at least two labels for them to be rendered. Also, giving more
than 3 labels is currently broken and the middle labels won't be in the correct position,
see [issue 334].

[issue 334]: https://github.com/ratatui/ratatui/issues/334

`labels` is a vector of any type that can be converted into a [`Line`] (e.g. `&str`,
`String`, `&Line`, `Span`, ...). This allows you to style the labels using the methods
provided by [`Line`]. Any alignment set on the labels will be ignored as the alignment is
determined by the axis.

This is a fluent setter method which must be chained or used as it consumes self

# Examples

```rust
use ratatui::style::Stylize;
use ratatui::widgets::Axis;

let axis = Axis::default()
    .bounds([0.0, 50.0])
    .labels(["0".bold(), "25".into(), "50".bold()]);
```

```rust
fn labels<Labels>(self: Self, labels: Labels) -> Self
```



## ratatui_widgets::chart::Axis::title

*Function*

Sets the axis title

It will be displayed at the end of the axis. For an X axis this is the right, for a Y axis,
this is the top.

This is a fluent setter method which must be chained or used as it consumes self

```rust
fn title<T>(self: Self, title: T) -> Self
```



