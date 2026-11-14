**ratatui_image > protocol > halfblocks**

# Module: protocol::halfblocks

## Contents

**Structs**

- [`HalfBlock`](#halfblock)
- [`Halfblocks`](#halfblocks)

---

## ratatui_image::protocol::halfblocks::HalfBlock

*Struct*

**Fields:**
- `upper: ratatui::style::Color`
- `lower: ratatui::style::Color`
- `char: char`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ratatui_image::protocol::halfblocks::Halfblocks

*Struct*

**Methods:**

- `fn new(image: DynamicImage, size: Size) -> std::result::Result<Self, errors::Errors>` - Create a FixedHalfblocks from an image.

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



