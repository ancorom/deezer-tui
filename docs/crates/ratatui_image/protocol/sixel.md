**ratatui_image > protocol > sixel**

# Module: protocol::sixel

## Contents

**Structs**

- [`Sixel`](#sixel)

---

## ratatui_image::protocol::sixel::Sixel

*Struct*

**Fields:**
- `data: String`
- `size: ratatui::Size`
- `is_tmux: bool`

**Methods:**

- `fn new(image: DynamicImage, size: Size, is_tmux: bool) -> std::result::Result<Self, errors::Errors>`

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



