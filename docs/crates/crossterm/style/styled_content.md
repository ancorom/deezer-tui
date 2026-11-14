**crossterm > style > styled_content**

# Module: style::styled_content

## Contents

**Structs**

- [`StyledContent`](#styledcontent) - The style with the content to be styled.

---

## crossterm::style::styled_content::StyledContent

*Struct*

The style with the content to be styled.

# Examples

```rust
use crossterm::style::{style, Color, Attribute, Stylize};

let styled = "Hello there"
    .with(Color::Yellow)
    .on(Color::Blue)
    .attribute(Attribute::Bold);

println!("{}", styled);
```

**Generic Parameters:**
- D

**Methods:**

- `fn new(style: ContentStyle, content: D) -> StyledContent<D>` - Creates a new `StyledContent`.
- `fn content(self: &Self) -> &D` - Returns the content.
- `fn style(self: &Self) -> &ContentStyle` - Returns the style.
- `fn style_mut(self: & mut Self) -> & mut ContentStyle` - Returns a mutable reference to the style, so that it can be further

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **AsMut**
  - `fn as_mut(self: & mut Self) -> & mut ContentStyle`
- **AsRef**
  - `fn as_ref(self: &Self) -> &ContentStyle`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Stylize**
  - `fn stylize(self: Self) -> <Self as >::Styled`



