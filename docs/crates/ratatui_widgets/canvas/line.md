**ratatui_widgets > canvas > line**

# Module: canvas::line

## Contents

**Structs**

- [`FilledLine`](#filledline) - A filled line from `(x1, y1)` to `(x2, y2)` that fills the area under/over the line
- [`Line`](#line) - A line from `(x1, y1)` to `(x2, y2)` with the given color

---

## ratatui_widgets::canvas::line::FilledLine

*Struct*

A filled line from `(x1, y1)` to `(x2, y2)` that fills the area under/over the line
to `fill_to_y` with the given color.

This is useful for creating area charts or filling the space under a line graph.

**Fields:**
- `x1: f64` - `x` of the starting point
- `y1: f64` - `y` of the starting point
- `x2: f64` - `x` of the ending point
- `y2: f64` - `y` of the ending point
- `fill_to_y: f64` - Y-coordinate to fill to (fills area between the line and this Y value)
- `color: ratatui_core::style::Color` - Color of the line and filled area

**Methods:**

- `fn new(x1: f64, y1: f64, x2: f64, y2: f64, fill_to_y: f64, color: Color) -> Self` - Create a new filled line from `(x1, y1)` to `(x2, y2)` that fills to `fill_to_y`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Shape**
  - `fn draw(self: &Self, painter: & mut Painter)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## ratatui_widgets::canvas::line::Line

*Struct*

A line from `(x1, y1)` to `(x2, y2)` with the given color

# Examples

```rust
# use ratatui_core::style::Color;
# use ratatui_widgets::canvas::{Canvas, Line};
Canvas::default().paint(|ctx| {
    ctx.draw(&Line::new(0.0, 0.0, 1.0, 0.0, Color::Red));
    ctx.draw(&Line::new(1.0, 0.0, 0.5, 1.0, Color::Red));
    ctx.draw(&Line::new(0.5, 1.0, 0.0, 0.0, Color::Red));
});
```

**Fields:**
- `x1: f64` - `x` of the starting point
- `y1: f64` - `y` of the starting point
- `x2: f64` - `x` of the ending point
- `y2: f64` - `y` of the ending point
- `color: ratatui_core::style::Color` - Color of the line

**Methods:**

- `fn new(x1: f64, y1: f64, x2: f64, y2: f64, color: Color) -> Self` - Create a new line from `(x1, y1)` to `(x2, y2)` with the given color

**Trait Implementations:**

- **Shape**
  - `fn draw(self: &Self, painter: & mut Painter)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



