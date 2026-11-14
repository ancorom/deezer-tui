**crossterm > style > content_style**

# Module: style::content_style

## Contents

**Structs**

- [`ContentStyle`](#contentstyle) - The style that can be put on content.

---

## crossterm::style::content_style::ContentStyle

*Struct*

The style that can be put on content.

**Fields:**
- `foreground_color: Option<crate::style::Color>` - The foreground color.
- `background_color: Option<crate::style::Color>` - The background color.
- `underline_color: Option<crate::style::Color>` - The underline color.
- `attributes: crate::style::Attributes` - List of attributes.

**Methods:**

- `fn apply<D>(self: Self, val: D) -> StyledContent<D>` - Creates a `StyledContent` by applying the style to the given `val`.
- `fn new() -> ContentStyle` - Creates a new `ContentStyle`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **AsRef**
  - `fn as_ref(self: &Self) -> &Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **AsMut**
  - `fn as_mut(self: & mut Self) -> & mut Self`
- **Stylize**
  - `fn stylize(self: Self) -> <Self as >::Styled`



