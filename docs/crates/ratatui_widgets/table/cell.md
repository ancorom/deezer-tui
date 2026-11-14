**ratatui_widgets > table > cell**

# Module: table::cell

## Contents

**Structs**

- [`Cell`](#cell) - A [`Cell`] contains the [`Text`] to be displayed in a [`Row`] of a [`Table`].

---

## ratatui_widgets::table::cell::Cell

*Struct*

A [`Cell`] contains the [`Text`] to be displayed in a [`Row`] of a [`Table`].

You can apply a [`Style`] to the [`Cell`] using [`Cell::style`]. This will set the style for the
entire area of the cell. Any [`Style`] set on the [`Text`] content will be combined with the
[`Style`] of the [`Cell`] by adding the [`Style`] of the [`Text`] content to the [`Style`] of
the [`Cell`]. Styles set on the text content will only affect the content.

You can use [`Text::alignment`] when creating a cell to align its content.

# Examples

You can create a `Cell` from anything that can be converted to a [`Text`].

```rust
use std::borrow::Cow;

use ratatui::style::Stylize;
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::Cell;

Cell::from("simple string");
Cell::from(Span::from("span"));
Cell::from(Line::from(vec![
    Span::from("a vec of "),
    Span::from("spans").bold(),
]));
Cell::from(Text::from("a text"));
Cell::from(Text::from(Cow::Borrowed("hello")));
```

`Cell` implements [`Styled`] which means you can use style shorthands from the [`Stylize`] trait
to set the style of the cell concisely.

```rust
use ratatui::style::Stylize;
use ratatui::widgets::Cell;

Cell::new("Cell 1").red().italic();
```

[`Row`]: super::Row
[`Table`]: super::Table
[`Stylize`]: ratatui_core::style::Stylize

**Generic Parameters:**
- 'a

**Methods:**

- `fn new<T>(content: T) -> Self` - Creates a new [`Cell`]
- `fn content<T>(self: Self, content: T) -> Self` - Set the content of the [`Cell`]
- `fn column_span(self: Self, column_span: u16) -> Self` - Set the `column_span` of this cell
- `fn style<S>(self: Self, style: S) -> Self` - Set the `Style` of this cell

**Traits:** Eq

**Trait Implementations:**

- **From**
  - `fn from(content: T) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Default**
  - `fn default() -> Self`
- **Styled**
  - `fn style(self: &Self) -> Style`
  - `fn set_style<S>(self: Self, style: S) -> <Self as >::Item`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



