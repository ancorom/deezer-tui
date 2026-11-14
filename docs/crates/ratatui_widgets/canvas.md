**ratatui_widgets > canvas**

# Module: canvas

## Contents

**Structs**

- [`Canvas`](#canvas) - The Canvas widget provides a means to draw shapes (Lines, Rectangles, Circles, etc.) on a grid.
- [`Context`](#context) - Holds the state of the [`Canvas`] when painting to it.
- [`Label`](#label) - Label to draw some text on the canvas
- [`Painter`](#painter) - Painter is an abstraction over the [`Context`] that allows to draw shapes on the grid.

**Traits**

- [`Shape`](#shape) - Something that can be drawn on a [`Canvas`].

---

## ratatui_widgets::canvas::Canvas

*Struct*

The Canvas widget provides a means to draw shapes (Lines, Rectangles, Circles, etc.) on a grid.

By default the grid is made of Braille patterns but you may change the marker to use a different
set of symbols. If your terminal or font does not support this unicode block, you will see
unicode replacement characters (�) instead of braille dots. The Braille patterns (as well the
octant character patterns) provide a more fine grained result with a 2x4 resolution per
character, but you might want to use a simple dot, block, or bar instead by calling the
[`marker`] method if your target environment does not support those symbols.

See [Unicode Braille Patterns](https://en.wikipedia.org/wiki/Braille_Patterns) for more info.

The `Octant` marker is similar to the `Braille` marker but, instead of sparse dots, displays
densely packed and regularly spaced pseudo-pixels, without visible bands between rows and
columns. However, it uses characters that are not yet as widely supported as the Braille
unicode block.

The `Quadrant` and `Sextant` markers are in turn akin to the `Octant` marker, but with a 2x2
and 2x3 resolution, respectively.

The `HalfBlock` marker is useful when you want to draw shapes with a higher resolution than a
`CharGrid` but lower than a `PatternGrid`. This grid type supports a foreground and background
color for each terminal cell. This allows for more flexibility than the `PatternGrid` which
only supports a single foreground color for each 2x4 dots cell.

The Canvas widget is used by calling the [`Canvas::paint`] method and passing a closure that
will be used to draw on the canvas. The closure will be passed a [`Context`] object that can be
used to draw shapes on the canvas.

The [`Context`] object provides a [`Context::draw`] method that can be used to draw shapes on
the canvas. The [`Context::layer`] method can be used to save the current state of the canvas
and start a new layer. This is useful if you want to draw multiple shapes on the canvas in
specific order. The [`Context`] object also provides a [`Context::print`] method that can be
used to print text on the canvas. Note that the text is always printed on top of the canvas and
is not affected by the layers.

# Examples

```
use ratatui::style::Color;
use ratatui::widgets::Block;
use ratatui::widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle};

Canvas::default()
    .block(Block::bordered().title("Canvas"))
    .x_bounds([-180.0, 180.0])
    .y_bounds([-90.0, 90.0])
    .paint(|ctx| {
        ctx.draw(&Map {
            resolution: MapResolution::High,
            color: Color::White,
        });
        ctx.layer();
        ctx.draw(&Line {
            x1: 0.0,
            y1: 10.0,
            x2: 10.0,
            y2: 10.0,
            color: Color::White,
        });
        ctx.draw(&Rectangle {
            x: 10.0,
            y: 20.0,
            width: 10.0,
            height: 10.0,
            color: Color::Red,
        });
    });
```

[`marker`]: #method.marker

**Generic Parameters:**
- 'a
- F

**Methods:**

- `fn block(self: Self, block: Block<'a>) -> Self` - Wraps the canvas with a custom [`Block`] widget.
- `fn x_bounds(self: Self, bounds: [f64; 2]) -> Self` - Define the viewport of the canvas.
- `fn y_bounds(self: Self, bounds: [f64; 2]) -> Self` - Define the viewport of the canvas.
- `fn paint(self: Self, f: F) -> Self` - Store the closure that will be used to draw to the [`Canvas`]
- `fn background_color(self: Self, color: Color) -> Self` - Change the background [`Color`] of the entire canvas
- `fn marker(self: Self, marker: Marker) -> Self` - Change the type of points used to draw the shapes.

**Trait Implementations:**

- **Widget**
  - `fn render(self: Self, area: Rect, buf: & mut Buffer)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **AsRef**
  - `fn as_ref(self: &Self) -> &crate::canvas::Canvas<'a, F>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ratatui_widgets::canvas::Context

*Struct*

Holds the state of the [`Canvas`] when painting to it.

This is used by the [`Canvas`] widget to draw shapes on the grid. It can be useful to think of
this as similar to the `Frame` struct that is used to draw widgets on the terminal.

**Generic Parameters:**
- 'a

**Methods:**

- `fn new(width: u16, height: u16, x_bounds: [f64; 2], y_bounds: [f64; 2], marker: Marker) -> Self` - Create a new Context with the given width and height measured in terminal columns and rows
- `fn marker(self: & mut Self, marker: Marker)` - Change the marker being used in this context.
- `fn draw<S>(self: & mut Self, shape: &S)` - Draw the given [`Shape`] in this context
- `fn layer(self: & mut Self)` - Save the existing state of the grid as a layer.
- `fn print<T>(self: & mut Self, x: f64, y: f64, line: T)` - Print a [`Text`] on the [`Canvas`] at the given position.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ratatui_widgets::canvas::Label

*Struct*

Label to draw some text on the canvas

**Generic Parameters:**
- 'a

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Default**
  - `fn default() -> Self`



## ratatui_widgets::canvas::Painter

*Struct*

Painter is an abstraction over the [`Context`] that allows to draw shapes on the grid.

It is used by the [`Shape`] trait to draw shapes on the grid. It can be useful to think of this
as similar to the [`Buffer`] struct that is used to draw widgets on the terminal.

**Generic Parameters:**
- 'a
- 'b

**Methods:**

- `fn get_point(self: &Self, x: f64, y: f64) -> Option<(usize, usize)>` - Convert the `(x, y)` coordinates to location of a point on the grid
- `fn paint(self: & mut Self, x: usize, y: usize, color: Color)` - Paint a point of the grid
- `fn bounds(self: &Self) -> (&[f64; 2], &[f64; 2])` - Canvas context bounds by axis.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(context: &'a  mut Context<'b>) -> Self`



## ratatui_widgets::canvas::Shape

*Trait*

Something that can be drawn on a [`Canvas`].

You may implement your own canvas custom widgets by implementing this trait.

**Methods:**

- `draw`: Draws this [`Shape`] using the given [`Painter`].



