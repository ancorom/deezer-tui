**ratatui_widgets > list > item**

# Module: list::item

## Contents

**Structs**

- [`ListItem`](#listitem) - A single item in a [`List`]

---

## ratatui_widgets::list::item::ListItem

*Struct*

A single item in a [`List`]

The item's height is defined by the number of lines it contains. This can be queried using
[`ListItem::height`]. Similarly, [`ListItem::width`] will return the maximum width of all
lines.

You can set the style of an item with [`ListItem::style`] or using the [`Stylize`] trait.
This [`Style`] will be combined with the [`Style`] of the inner [`Text`]. The [`Style`]
of the [`Text`] will be added to the [`Style`] of the [`ListItem`].

You can also align a `ListItem` by aligning its underlying [`Text`] and [`Line`]s. For that,
see [`Text::alignment`] and [`Line::alignment`]. On a multiline `Text`, one `Line` can override
the alignment by setting it explicitly.

# Examples

You can create [`ListItem`]s from simple `&str`

```rust
use ratatui::widgets::ListItem;
let item = ListItem::new("Item 1");
```

Anything that can be converted to [`Text`] can be a [`ListItem`].

```rust
use ratatui::text::Line;
use ratatui::widgets::ListItem;

let item1: ListItem = "Item 1".into();
let item2: ListItem = Line::raw("Item 2").into();
```

A [`ListItem`] styled with [`Stylize`]

```rust
use ratatui::style::Stylize;
use ratatui::widgets::ListItem;

let item = ListItem::new("Item 1").red().on_white();
```

If you need more control over the item's style, you can explicitly style the underlying
[`Text`]

```rust
use ratatui::style::Stylize;
use ratatui::text::{Span, Text};
use ratatui::widgets::ListItem;

let mut text = Text::default();
text.extend(["Item".blue(), Span::raw(" "), "1".bold().red()]);
let item = ListItem::new(text);
```

A right-aligned `ListItem`

```rust
use ratatui::text::Text;
use ratatui::widgets::ListItem;

ListItem::new(Text::from("foo").right_aligned());
```

[`List`]: crate::list::List
[`Stylize`]: ratatui_core::style::Stylize
[`Line`]: ratatui_core::text::Line
[`Line::alignment`]: ratatui_core::text::Line::alignment

**Generic Parameters:**
- 'a

**Methods:**

- `fn new<T>(content: T) -> Self` - Creates a new [`ListItem`]
- `fn style<S>(self: Self, style: S) -> Self` - Sets the item style
- `fn height(self: &Self) -> usize` - Returns the item height
- `fn width(self: &Self) -> usize` - Returns the max width of all the lines

**Traits:** Eq

**Trait Implementations:**

- **From**
  - `fn from(value: T) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Styled**
  - `fn style(self: &Self) -> Style`
  - `fn set_style<S>(self: Self, style: S) -> <Self as >::Item`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



