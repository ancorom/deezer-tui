**crossterm > style > types > colors**

# Module: style::types::colors

## Contents

**Structs**

- [`Colors`](#colors) - Represents, optionally, a foreground and/or a background color.

---

## crossterm::style::types::colors::Colors

*Struct*

Represents, optionally, a foreground and/or a background color.

It can be applied using the `SetColors` command.

It can also be created from a [Colored](enum.Colored.html) value or a tuple of
`(Color, Color)` in the order `(foreground, background)`.

The [then](#method.then) method can be used to combine `Colors` values.

For example:
```no_run
use crossterm::style::{Color, Colors, Colored};

// An example color, loaded from a config, file in ANSI format.
let config_color = "38;2;23;147;209";

// Default to green text on a black background.
let default_colors = Colors::new(Color::Green, Color::Black);
// Load a colored value from a config and override the default colors
let colors = match Colored::parse_ansi(config_color) {
    Some(colored) => default_colors.then(&colored.into()),
    None => default_colors,
};
```

See [Color](enum.Color.html).

**Fields:**
- `foreground: Option<crate::style::Color>`
- `background: Option<crate::style::Color>`

**Methods:**

- `fn new(foreground: Color, background: Color) -> Colors`
- `fn then(self: &Self, other: &Colors) -> Colors` - Returns a new `Color` which, when applied, has the same effect as applying `self` and *then*

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(colored: Colored) -> Colors`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`



