**ratatui_widgets > canvas > rectangle**

# Module: canvas::rectangle

## Contents

**Structs**

- [`Rectangle`](#rectangle) - A rectangle to draw on a [`Canvas`](crate::canvas::Canvas)

---

## ratatui_widgets::canvas::rectangle::Rectangle

*Struct*

A rectangle to draw on a [`Canvas`](crate::canvas::Canvas)

Sizes used here are **not** in terminal cell. This is much more similar to the
mathematic coordinate system.

**Fields:**
- `x: f64` - The `x` position of the rectangle.
- `y: f64` - The `y` position of the rectangle.
- `width: f64` - The width of the rectangle.
- `height: f64` - The height of the rectangle.
- `color: ratatui_core::style::Color` - The color of the rectangle.

**Methods:**

- `fn new(x: f64, y: f64, width: f64, height: f64, color: Color) -> Self` - Create a new rectangle with the given position, size, and color

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



