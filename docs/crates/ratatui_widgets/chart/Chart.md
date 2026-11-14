**ratatui_widgets > chart > Chart**

# Module: chart::Chart

## Contents

**Functions**

- [`hidden_legend_constraints`](#hidden_legend_constraints) - Sets the constraints used to determine whether the legend should be shown or not.
- [`legend_position`](#legend_position) - Sets the position of a legend or hide it
- [`new`](#new) - Creates a chart with the given [datasets](Dataset)
- [`x_axis`](#x_axis) - Sets the X [`Axis`]
- [`y_axis`](#y_axis) - Sets the Y [`Axis`]

---

## ratatui_widgets::chart::Chart::hidden_legend_constraints

*Function*

Sets the constraints used to determine whether the legend should be shown or not.

The tuple's first constraint is used for the width and the second for the height. If the
legend takes more space than what is allowed by any constraint, the legend is hidden.
[`Constraint::Min`] is an exception and will always show the legend.

If this is not set, the default behavior is to hide the legend if it is greater than 25% of
the chart, either horizontally or vertically.

This is a fluent setter method which must be chained or used as it consumes self

# Examples

Hide the legend when either its width is greater than 33% of the total widget width or if
its height is greater than 25% of the total widget height.

```
use ratatui::layout::Constraint;
use ratatui::widgets::Chart;

let constraints = (Constraint::Ratio(1, 3), Constraint::Ratio(1, 4));
let chart = Chart::new(vec![]).hidden_legend_constraints(constraints);
```

Always show the legend, note the second constraint doesn't matter in this case since the
first one is always true.

```
use ratatui::layout::Constraint;
use ratatui::widgets::Chart;

let constraints = (Constraint::Min(0), Constraint::Ratio(1, 4));
let chart = Chart::new(vec![]).hidden_legend_constraints(constraints);
```

Always hide the legend. Note this can be accomplished more explicitly by passing `None` to
[`Chart::legend_position`].

```
use ratatui::layout::Constraint;
use ratatui::widgets::Chart;

let constraints = (Constraint::Length(0), Constraint::Ratio(1, 4));
let chart = Chart::new(vec![]).hidden_legend_constraints(constraints);
```

```rust
fn hidden_legend_constraints(self: Self, constraints: (Constraint, Constraint)) -> Self
```



## ratatui_widgets::chart::Chart::legend_position

*Function*

Sets the position of a legend or hide it

The default is [`LegendPosition::TopRight`].

If [`None`] is given, hide the legend even if [`hidden_legend_constraints`] determines it
should be shown. In contrast, if `Some(...)` is given, [`hidden_legend_constraints`] might
still decide whether to show the legend or not.

See [`LegendPosition`] for all available positions.

[`hidden_legend_constraints`]: Self::hidden_legend_constraints

This is a fluent setter method which must be chained or used as it consumes self

# Examples

Show the legend on the top left corner.

```
use ratatui::widgets::{Chart, LegendPosition};

let chart: Chart = Chart::new(vec![]).legend_position(Some(LegendPosition::TopLeft));
```

Hide the legend altogether

```
use ratatui::widgets::{Chart, LegendPosition};

let chart = Chart::new(vec![]).legend_position(None);
```

```rust
fn legend_position(self: Self, position: Option<LegendPosition>) -> Self
```



## ratatui_widgets::chart::Chart::new

*Function*

Creates a chart with the given [datasets](Dataset)

A chart can render multiple datasets.

# Example

This creates a simple chart with one [`Dataset`]

```rust
use ratatui::widgets::{Chart, Dataset};

let data_points = vec![];
let chart = Chart::new(vec![Dataset::default().data(&data_points)]);
```

This creates a chart with multiple [`Dataset`]s

```rust
use ratatui::widgets::{Chart, Dataset};

let data_points = vec![];
let data_points2 = vec![];
let chart = Chart::new(vec![
    Dataset::default().data(&data_points),
    Dataset::default().data(&data_points2),
]);
```

```rust
fn new(datasets: Vec<Dataset<'a>>) -> Self
```



## ratatui_widgets::chart::Chart::x_axis

*Function*

Sets the X [`Axis`]

The default is an empty [`Axis`], i.e. only a line.

This is a fluent setter method which must be chained or used as it consumes self

# Example

```rust
use ratatui::widgets::{Axis, Chart};

let chart = Chart::new(vec![]).x_axis(
    Axis::default()
        .title("X Axis")
        .bounds([0.0, 20.0])
        .labels(["0", "20"]),
);
```

```rust
fn x_axis(self: Self, axis: Axis<'a>) -> Self
```



## ratatui_widgets::chart::Chart::y_axis

*Function*

Sets the Y [`Axis`]

The default is an empty [`Axis`], i.e. only a line.

This is a fluent setter method which must be chained or used as it consumes self

# Example

```rust
use ratatui::widgets::{Axis, Chart};

let chart = Chart::new(vec![]).y_axis(
    Axis::default()
        .title("Y Axis")
        .bounds([0.0, 20.0])
        .labels(["0", "20"]),
);
```

```rust
fn y_axis(self: Self, axis: Axis<'a>) -> Self
```



