**ratatui_widgets > mascot**

# Module: mascot

## Contents

**Structs**

- [`RatatuiMascot`](#ratatuimascot) - A widget that renders the Ratatui mascot

**Enums**

- [`MascotEyeColor`](#mascoteyecolor) - State for the mascot's eye

---

## ratatui_widgets::mascot::MascotEyeColor

*Enum*

State for the mascot's eye

**Variants:**
- `Default` - The default eye color
- `Red` - The red eye color

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Default**
  - `fn default() -> Self`



## ratatui_widgets::mascot::RatatuiMascot

*Struct*

A widget that renders the Ratatui mascot

**Methods:**

- `fn new() -> Self` - Create a new Ratatui mascot widget
- `fn set_eye(self: Self, rat_eye: MascotEyeColor) -> Self` - Set the eye state (open / blinking)

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **AsRef**
  - `fn as_ref(self: &Self) -> &crate::mascot::RatatuiMascot`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Widget**
  - `fn render(self: Self, area: Rect, buf: & mut Buffer)` - Use half block characters to render a logo based on the `RATATUI_LOGO` const.
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Default**
  - `fn default() -> Self`



