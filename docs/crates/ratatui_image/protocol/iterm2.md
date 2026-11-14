**ratatui_image > protocol > iterm2**

# Module: protocol::iterm2

## Contents

**Structs**

- [`Iterm2`](#iterm2)

---

## ratatui_image::protocol::iterm2::Iterm2

*Struct*

**Fields:**
- `data: String`
- `size: ratatui::Size`
- `is_tmux: bool`

**Methods:**

- `fn new(image: DynamicImage, size: Size, is_tmux: bool) -> std::result::Result<Self, errors::Errors>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Default**
  - `fn default() -> Self`



