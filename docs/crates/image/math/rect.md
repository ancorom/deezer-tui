**image > math > rect**

# Module: math::rect

## Contents

**Structs**

- [`Rect`](#rect) - A Rectangle defined by its top left corner, width and height.

---

## image::math::rect::Rect

*Struct*

A Rectangle defined by its top left corner, width and height.

**Fields:**
- `x: u32` - The x coordinate of the top left corner.
- `y: u32` - The y coordinate of the top left corner.
- `width: u32` - The rectangle's width.
- `height: u32` - The rectangle's height.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



