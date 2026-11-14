**ratatui_core > text > masked**

# Module: text::masked

## Contents

**Structs**

- [`Masked`](#masked) - A wrapper around a string that is masked when displayed.

---

## ratatui_core::text::masked::Masked

*Struct*

A wrapper around a string that is masked when displayed.

The masked string is displayed as a series of the same character. This might be used to display
a password field or similar secure data.

# Examples

```rust
use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;
use ratatui_core::text::{Masked, Text};
use ratatui_core::widgets::Widget;

let mut buffer = Buffer::empty(Rect::new(0, 0, 5, 1));
let password = Masked::new("12345", 'x');

Text::from(password).render(buffer.area, &mut buffer);
assert_eq!(buffer, Buffer::with_lines(["xxxxx"]));
```

**Generic Parameters:**
- 'a

**Methods:**

- `fn new<impl Into<Cow<'a, str>>>(s: impl Trait, mask_char: char) -> Self`
- `fn mask_char(self: &Self) -> char` - The character to use for masking.
- `fn value(self: &Self) -> Cow<'a, str>` - The underlying string, with all characters masked.

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result` - Display representation of a masked string is the masked string
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result` - Debug representation of a masked string is the underlying string
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Default**
  - `fn default() -> Self`



