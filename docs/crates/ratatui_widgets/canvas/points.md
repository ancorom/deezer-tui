**ratatui_widgets > canvas > points**

# Module: canvas::points

## Contents

**Structs**

- [`Points`](#points) - A group of points with a given color

---

## ratatui_widgets::canvas::points::Points

*Struct*

A group of points with a given color

**Generic Parameters:**
- 'a

**Fields:**
- `coords: &'a [(f64, f64)]` - List of points to draw
- `color: ratatui_core::style::Color` - Color of the points

**Methods:**

- `fn new(coords: &'a [(f64, f64)], color: Color) -> Self` - Create a new Points shape with the given coordinates and color

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



