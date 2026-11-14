**ratatui_widgets > chart**

# Module: chart

## Contents

**Structs**

- [`Axis`](#axis) - An X or Y axis for the [`Chart`] widget
- [`Chart`](#chart) - A widget to plot one or more [`Dataset`] in a cartesian coordinate system
- [`Dataset`](#dataset) - A group of data points

**Enums**

- [`GraphType`](#graphtype) - Used to determine which style of graphing to use
- [`LegendPosition`](#legendposition) - Allow users to specify the position of a legend in a [`Chart`]

---

## ratatui_widgets::chart::Axis

*Struct*

An X or Y axis for the [`Chart`] widget

An axis can have a [title](Axis::title) which will be displayed at the end of the axis. For an
X axis this is the right, for a Y axis, this is the top.

You can also set the bounds and labels on this axis using respectively [`Axis::bounds`] and
[`Axis::labels`].

See [`Chart::x_axis`] and [`Chart::y_axis`] to set an axis on a chart.

# Example

```rust
use ratatui::style::{Style, Stylize};
use ratatui::widgets::Axis;

let axis = Axis::default()
    .title("X Axis")
    .style(Style::default().gray())
    .bounds([0.0, 50.0])
    .labels(["0".bold(), "25".into(), "50".bold()]);
```

**Generic Parameters:**
- 'a

**Methods:**

- `fn title<T>(self: Self, title: T) -> Self` - Sets the axis title
- `fn bounds(self: Self, bounds: [f64; 2]) -> Self` - Sets the bounds of this axis
- `fn labels<Labels>(self: Self, labels: Labels) -> Self` - Sets the axis labels
- `fn style<S>(self: Self, style: S) -> Self` - Sets the axis style
- `fn labels_alignment(self: Self, alignment: Alignment) -> Self` - Sets the labels alignment of the axis

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Styled**
  - `fn style(self: &Self) -> Style`
  - `fn set_style<S>(self: Self, style: S) -> <Self as >::Item`



## ratatui_widgets::chart::Chart

*Struct*

A widget to plot one or more [`Dataset`] in a cartesian coordinate system

To use this widget, start by creating one or more [`Dataset`]. With it, you can set the
[data points](Dataset::data), the [name](Dataset::name) or the
[chart type](Dataset::graph_type). See [`Dataset`] for a complete documentation of what is
possible.

Then, you'll usually want to configure the [`Axis`]. Axis [titles](Axis::title),
[bounds](Axis::bounds) and [labels](Axis::labels) can be configured on both axis. See [`Axis`]
for a complete documentation of what is possible.

Finally, you can pass all of that to the `Chart` via [`Chart::new`], [`Chart::x_axis`] and
[`Chart::y_axis`].

Additionally, `Chart` allows configuring the legend [position](Chart::legend_position) and
[hiding constraints](Chart::hidden_legend_constraints).

# Examples

```
use ratatui::style::{Style, Stylize};
use ratatui::symbols;
use ratatui::widgets::{Axis, Block, Chart, Dataset, GraphType};

// Create the datasets to fill the chart with
let datasets = vec![
    // Scatter chart
    Dataset::default()
        .name("data1")
        .marker(symbols::Marker::Dot)
        .graph_type(GraphType::Scatter)
        .style(Style::default().cyan())
        .data(&[(0.0, 5.0), (1.0, 6.0), (1.5, 6.434)]),
    // Line chart
    Dataset::default()
        .name("data2")
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().magenta())
        .data(&[(4.0, 5.0), (5.0, 8.0), (7.66, 13.5)]),
];

// Create the X axis and define its properties
let x_axis = Axis::default()
    .title("X Axis".red())
    .style(Style::default().white())
    .bounds([0.0, 10.0])
    .labels(["0.0", "5.0", "10.0"]);

// Create the Y axis and define its properties
let y_axis = Axis::default()
    .title("Y Axis".red())
    .style(Style::default().white())
    .bounds([0.0, 10.0])
    .labels(["0.0", "5.0", "10.0"]);

// Create the chart and link all the parts together
let chart = Chart::new(datasets)
    .block(Block::new().title("Chart"))
    .x_axis(x_axis)
    .y_axis(y_axis);
```

**Generic Parameters:**
- 'a

**Methods:**

- `fn new(datasets: Vec<Dataset<'a>>) -> Self` - Creates a chart with the given [datasets](Dataset)
- `fn block(self: Self, block: Block<'a>) -> Self` - Wraps the chart with the given [`Block`]
- `fn style<S>(self: Self, style: S) -> Self` - Sets the style of the entire chart
- `fn x_axis(self: Self, axis: Axis<'a>) -> Self` - Sets the X [`Axis`]
- `fn y_axis(self: Self, axis: Axis<'a>) -> Self` - Sets the Y [`Axis`]
- `fn hidden_legend_constraints(self: Self, constraints: (Constraint, Constraint)) -> Self` - Sets the constraints used to determine whether the legend should be shown or not.
- `fn legend_position(self: Self, position: Option<LegendPosition>) -> Self` - Sets the position of a legend or hide it

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **AsRef**
  - `fn as_ref(self: &Self) -> &crate::chart::Chart<'a>`
- **Widget**
  - `fn render(self: Self, area: Rect, buf: & mut Buffer)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Styled**
  - `fn style(self: &Self) -> Style`
  - `fn set_style<S>(self: Self, style: S) -> <Self as >::Item`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## ratatui_widgets::chart::Dataset

*Struct*

A group of data points

This is the main element composing a [`Chart`].

A dataset can be [named](Dataset::name). Only named datasets will be rendered in the legend.

After that, you can pass it data with [`Dataset::data`]. Data is an array of `f64` tuples
(`(f64, f64)`), the first element being X and the second Y. It's also worth noting that, unlike
the [`Rect`], here the Y axis is bottom to top, as in math.

You can also customize the rendering by using [`Dataset::marker`] and [`Dataset::graph_type`].

# Example

This example draws a red line between two points.

```rust
use ratatui::style::Stylize;
use ratatui::symbols::Marker;
use ratatui::widgets::{Dataset, GraphType};

let dataset = Dataset::default()
    .name("dataset 1")
    .data(&[(1., 1.), (5., 5.)])
    .marker(Marker::Braille)
    .graph_type(GraphType::Line)
    .red();
```

**Generic Parameters:**
- 'a

**Methods:**

- `fn name<S>(self: Self, name: S) -> Self` - Sets the name of the dataset
- `fn data(self: Self, data: &'a [(f64, f64)]) -> Self` - Sets the data points of this dataset
- `fn marker(self: Self, marker: symbols::Marker) -> Self` - Sets the kind of character to use to display this dataset
- `fn graph_type(self: Self, graph_type: GraphType) -> Self` - Sets how the dataset should be drawn
- `fn style<S>(self: Self, style: S) -> Self` - Sets the style of this dataset
- `fn fill_to_y(self: Self, fill_to_y: f64) -> Self` - Sets the y-coordinate to fill the area to when using [`GraphType::Area`]

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Styled**
  - `fn style(self: &Self) -> Style`
  - `fn set_style<S>(self: Self, style: S) -> <Self as >::Item`



## ratatui_widgets::chart::GraphType

*Enum*

Used to determine which style of graphing to use

**Variants:**
- `Scatter` - Draw each point. This is the default.
- `Line` - Draw a line between each following point.
- `Bar` - Draw a bar chart. This will draw a bar for each point in the dataset.
- `Area` - Draw a line chart with the area filled. Like [`Line`](GraphType::Line), this draws a line

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::result::Result<(), ::core::fmt::Error>`
- **FromStr**
  - `fn from_str(s: &str) -> ::core::result::Result<GraphType, <Self as ::core::str::FromStr>::Err>`
- **TryFrom**
  - `fn try_from(s: &str) -> ::core::result::Result<GraphType, <Self as ::core::convert::TryFrom>::Error>`
- **Default**
  - `fn default() -> Self`



## ratatui_widgets::chart::LegendPosition

*Enum*

Allow users to specify the position of a legend in a [`Chart`]

See [`Chart::legend_position`]

**Variants:**
- `Top` - Legend is centered on top
- `TopRight` - Legend is in the top-right corner. This is the **default**.
- `TopLeft` - Legend is in the top-left corner
- `Left` - Legend is centered on the left
- `Right` - Legend is centered on the right
- `Bottom` - Legend is centered on the bottom
- `BottomRight` - Legend is in the bottom-right corner
- `BottomLeft` - Legend is in the bottom-left corner

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



