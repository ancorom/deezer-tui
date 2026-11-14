**ratatui_image > protocol > kitty**

# Module: protocol::kitty

## Contents

**Structs**

- [`Kitty`](#kitty)
- [`StatefulKitty`](#statefulkitty)

---

## ratatui_image::protocol::kitty::Kitty

*Struct*

**Methods:**

- `fn new(image: DynamicImage, size: Size, id: u32, is_tmux: bool, compress: bool) -> std::result::Result<Self, errors::Errors>`

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## ratatui_image::protocol::kitty::StatefulKitty

*Struct*

**Methods:**

- `fn new(id: u32, is_tmux: bool, compress: bool) -> StatefulKitty`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`



