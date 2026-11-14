**ratatui_widgets > chart > Dataset**

# Module: chart::Dataset

## Contents

**Functions**

- [`data`](#data) - Sets the data points of this dataset
- [`fill_to_y`](#fill_to_y) - Sets the y-coordinate to fill the area to when using [`GraphType::Area`]
- [`graph_type`](#graph_type) - Sets how the dataset should be drawn
- [`marker`](#marker) - Sets the kind of character to use to display this dataset
- [`name`](#name) - Sets the name of the dataset

---

## ratatui_widgets::chart::Dataset::data

*Function*

Sets the data points of this dataset

Points will then either be rendered as scattered points or with lines between them
depending on [`Dataset::graph_type`].

Data consist in an array of `f64` tuples (`(f64, f64)`), the first element being X and the
second Y. It's also worth noting that, unlike the [`Rect`], here the Y axis is bottom to
top, as in math.

This is a fluent setter method which must be chained or used as it consumes self

```rust
fn data(self: Self, data: &'a [(f64, f64)]) -> Self
```



## ratatui_widgets::chart::Dataset::fill_to_y

*Function*

Sets the y-coordinate to fill the area to when using [`GraphType::Area`]

When the graph type is set to [`GraphType::Area`], the area between the data points and the
specified y-coordinate will be filled with the dataset's style. The default is `0.0`.

This is a fluent setter method which must be chained or used as it consumes self

```rust
fn fill_to_y(self: Self, fill_to_y: f64) -> Self
```



## ratatui_widgets::chart::Dataset::graph_type

*Function*

Sets how the dataset should be drawn

[`Chart`] can draw [scatter](GraphType::Scatter), [line](GraphType::Line) or
[bar](GraphType::Bar) charts. A scatter chart draws only the points in the dataset, a line
char draws a line between each point, and a bar chart draws a line from the x axis to the
point.  See [`GraphType`] for more details

This is a fluent setter method which must be chained or used as it consumes self

```rust
fn graph_type(self: Self, graph_type: GraphType) -> Self
```



## ratatui_widgets::chart::Dataset::marker

*Function*

Sets the kind of character to use to display this dataset

You can use dots (`•`), blocks (`█`), bars (`▄`), braille (`⠓`, `⣇`, `⣿`), half-blocks
(`█`, `▄`, and `▀`) or if you need custom chars use
[`Marker::Custom`](symbols::Marker::Custom). See [`symbols::Marker`] for more details.

Note [`Marker::Braille`](symbols::Marker::Braille) requires a font that supports Unicode
Braille Patterns.

This is a fluent setter method which must be chained or used as it consumes self

```rust
fn marker(self: Self, marker: symbols::Marker) -> Self
```



## ratatui_widgets::chart::Dataset::name

*Function*

Sets the name of the dataset

The dataset's name is used when displaying the chart legend. Datasets don't require a name
and can be created without specifying one. Once assigned, a name can't be removed, only
changed

The name can be styled (see [`Line`] for that), but the dataset's style will always have
precedence.

This is a fluent setter method which must be chained or used as it consumes self

```rust
fn name<S>(self: Self, name: S) -> Self
```



