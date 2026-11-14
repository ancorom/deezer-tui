**ratatui_widgets > canvas > circle**

# Module: canvas::circle

## Contents

**Structs**

- [`Circle`](#circle) - A circle with a given center and radius and with a given color

---

## ratatui_widgets::canvas::circle::Circle

*Struct*

A circle with a given center and radius and with a given color

**Fields:**
- `x: f64` - `x` coordinate of the circle's center
- `y: f64` - `y` coordinate of the circle's center
- `radius: f64` - Radius of the circle
- `color: ratatui_core::style::Color` - Color of the circle

**Methods:**

- `fn new(x: f64, y: f64, radius: f64, color: Color) -> Self` - Create a new circle with the given center, radius, and color

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Shape**
  - `fn draw(self: &Self, painter: & mut Painter)`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



