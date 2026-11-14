**ratatui_widgets > fill**

# Module: fill

## Contents

**Structs**

- [`Fill`](#fill) - A widget that fills its render area with a single repeated symbol and style.

---

## ratatui_widgets::fill::Fill

*Struct*

A widget that fills its render area with a single repeated symbol and style.

[`Fill`] is a small building block for painting solid blocks of one symbol — backgrounds,
separators, scrollbar tracks, custom borders, etc. — without writing the nested loop
yourself. It composes naturally with the [`Stylize`] trait so the typical call site is
a one-liner.

# Examples

```
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::widgets::{Fill, Widget};

# let mut buf = ratatui::buffer::Buffer::empty(Rect::new(0, 0, 10, 5));
let fill = Fill::new("X").blue().bold();
fill.render(Rect::new(0, 0, 10, 3), &mut buf);
```

This renders as:

```plain
XXXXXXXXXX
XXXXXXXXXX
XXXXXXXXXX
```

[`Fill`] accepts anything that converts into a [`Cow<str>`], so both string literals and
owned [`String`](alloc::string::String)s work:

```
use ratatui::widgets::Fill;

let _ = Fill::new("•");
let _ = Fill::new(String::from("•"));
```

Cells outside the buffer are silently clipped, mirroring the behavior of other widgets
such as [`Clear`](crate::clear::Clear).

[`Stylize`]: ratatui_core::style::Stylize

**Generic Parameters:**
- 'a

**Methods:**

- `fn new<S>(symbol: S) -> Self` - Create a new [`Fill`] widget that paints `symbol` into every cell of its render area.
- `fn style<S>(self: Self, style: S) -> Self` - Set the style used to paint each cell.
- `fn symbol<S>(self: Self, symbol: S) -> Self` - Set the symbol painted into each cell.

**Traits:** Eq

**Trait Implementations:**

- **Widget**
  - `fn render(self: Self, area: Rect, buf: & mut Buffer)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Styled**
  - `fn style(self: &Self) -> Style`
  - `fn set_style<S>(self: Self, style: S) -> <Self as >::Item`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Default**
  - `fn default() -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



