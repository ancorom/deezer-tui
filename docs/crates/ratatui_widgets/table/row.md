**ratatui_widgets > table > row**

# Module: table::row

## Contents

**Structs**

- [`Row`](#row) - A single row of data to be displayed in a [`Table`] widget.

---

## ratatui_widgets::table::row::Row

*Struct*

A single row of data to be displayed in a [`Table`] widget.

A `Row` is a collection of [`Cell`]s.

By default, a row has a height of 1 but you can change this using [`Row::height`].

You can set the style of the entire row using [`Row::style`]. This [`Style`] will be combined
with the [`Style`] of each individual [`Cell`] by adding the [`Style`] of the [`Cell`] to the
[`Style`] of the [`Row`].

# Examples

You can create `Row`s from simple strings.

```rust
use ratatui::widgets::Row;

Row::new(vec!["Cell1", "Cell2", "Cell3"]);
```

If you need a bit more control over individual cells, you can explicitly create [`Cell`]s:

```rust
use ratatui::style::Stylize;
use ratatui::widgets::{Cell, Row};

Row::new(vec![
    Cell::from("Cell1"),
    Cell::from("Cell2").red().italic(),
]);
```

You can also construct a row from any type that can be converted into [`Text`]:

```rust
use std::borrow::Cow;

use ratatui::widgets::{Cell, Row};

Row::new(vec![
    Cow::Borrowed("hello"),
    Cow::Owned("world".to_uppercase()),
]);
```

An iterator whose item type is convertible into [`Text`] can be collected into a row.

```rust
use ratatui::widgets::Row;

(0..10).map(|i| format!("{i}")).collect::<Row>();
```

`Row` implements [`Styled`] which means you can use style shorthands from the [`Stylize`] trait
to set the style of the row concisely.

```rust
use ratatui::style::Stylize;
use ratatui::widgets::Row;

let cells = vec!["Cell1", "Cell2", "Cell3"];
Row::new(cells).red().italic();
```

[`Table`]: super::Table
[`Text`]: ratatui_core::text::Text
[`Stylize`]: ratatui_core::style::Stylize

**Generic Parameters:**
- 'a

**Methods:**

- `fn new<T>(cells: T) -> Self` - Creates a new [`Row`]
- `fn cells<T>(self: Self, cells: T) -> Self` - Set the cells of the [`Row`]
- `fn height(self: Self, height: u16) -> Self` - Set the fixed height of the [`Row`]
- `fn top_margin(self: Self, margin: u16) -> Self` - Set the top margin. By default, the top margin is `0`.
- `fn bottom_margin(self: Self, margin: u16) -> Self` - Set the bottom margin. By default, the bottom margin is `0`.
- `fn style<S>(self: Self, style: S) -> Self` - Set the [`Style`] of the entire row

**Traits:** Eq

**Trait Implementations:**

- **Styled**
  - `fn style(self: &Self) -> Style`
  - `fn set_style<S>(self: Self, style: S) -> <Self as >::Item`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Default**
  - `fn default() -> Self`
- **FromIterator**
  - `fn from_iter<IterCells>(cells: IterCells) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



