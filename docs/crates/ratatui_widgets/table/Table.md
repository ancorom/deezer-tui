**ratatui_widgets > table > Table**

# Module: table::Table

## Contents

**Functions**

- [`block`](#block) - Wraps the table with a custom [`Block`] widget.
- [`cell_highlight_style`](#cell_highlight_style) - Set the style of the selected cell
- [`column_highlight_style`](#column_highlight_style) - Set the style of the selected column
- [`column_spacing`](#column_spacing) - Set the spacing between columns
- [`footer`](#footer) - Sets the footer row
- [`header`](#header) - Sets the header row
- [`highlight_spacing`](#highlight_spacing) - Set when to show the highlight spacing
- [`highlight_symbol`](#highlight_symbol) - Set the symbol to be displayed in front of the selected row
- [`new`](#new) - Creates a new [`Table`] widget with the given rows.
- [`row_highlight_style`](#row_highlight_style) - Set the style of the selected row
- [`rows`](#rows) - Set the rows
- [`style`](#style) - Sets the base style of the widget
- [`widths`](#widths) - Set the widths of the columns.

---

## ratatui_widgets::table::Table::block

*Function*

Wraps the table with a custom [`Block`] widget.

The `block` parameter is of type [`Block`]. This holds the specified block to be
created around the [`Table`]

This is a fluent setter method which must be chained or used as it consumes self

# Examples

```rust
use ratatui::layout::Constraint;
use ratatui::widgets::{Block, Cell, Row, Table};

let rows = [Row::new(vec!["Cell1", "Cell2"])];
let widths = [Constraint::Length(5), Constraint::Length(5)];
let block = Block::bordered().title("Table");
let table = Table::new(rows, widths).block(block);
```

```rust
fn block(self: Self, block: Block<'a>) -> Self
```



## ratatui_widgets::table::Table::cell_highlight_style

*Function*

Set the style of the selected cell

`style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
your own type that implements [`Into<Style>`]).

This style will be applied to the selected cell, and will override any style set on the
row or on the individual cells.

This is a fluent setter method which must be chained or used as it consumes self

# Examples

```rust
# use ratatui::{layout::Constraint, style::{Style, Stylize}, widgets::{Row, Table}};
# let rows = [Row::new(vec!["Cell1", "Cell2"])];
# let widths = [Constraint::Length(5), Constraint::Length(5)];
let table = Table::new(rows, widths).cell_highlight_style(Style::new().red().italic());
```
[`Color`]: ratatui_core::style::Color

```rust
fn cell_highlight_style<S>(self: Self, highlight_style: S) -> Self
```



## ratatui_widgets::table::Table::column_highlight_style

*Function*

Set the style of the selected column

`style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
your own type that implements [`Into<Style>`]).

This style will be applied to the entire column, and will override any style set on the
row or on the individual cells.

This is a fluent setter method which must be chained or used as it consumes self

# Examples

```rust
# use ratatui::{layout::Constraint, style::{Style, Stylize}, widgets::{Row, Table}};
# let rows = [Row::new(vec!["Cell1", "Cell2"])];
# let widths = [Constraint::Length(5), Constraint::Length(5)];
let table = Table::new(rows, widths).column_highlight_style(Style::new().red().italic());
```
[`Color`]: ratatui_core::style::Color

```rust
fn column_highlight_style<S>(self: Self, highlight_style: S) -> Self
```



## ratatui_widgets::table::Table::column_spacing

*Function*

Set the spacing between columns

This is a fluent setter method which must be chained or used as it consumes self

# Examples

```rust
use ratatui::layout::Constraint;
use ratatui::widgets::{Row, Table};

let rows = [Row::new(vec!["Cell1", "Cell2"])];
let widths = [Constraint::Length(5), Constraint::Length(5)];
let table = Table::new(rows, widths).column_spacing(1);
```

```rust
fn column_spacing(self: Self, spacing: u16) -> Self
```



## ratatui_widgets::table::Table::footer

*Function*

Sets the footer row

The `footer` parameter is a [`Row`] which will be displayed at the bottom of the [`Table`]

This is a fluent setter method which must be chained or used as it consumes self

# Examples

```rust
use ratatui::widgets::{Cell, Row, Table};

let footer = Row::new(vec![
    Cell::from("Footer Cell 1"),
    Cell::from("Footer Cell 2"),
]);
let table = Table::default().footer(footer);
```

```rust
fn footer(self: Self, footer: Row<'a>) -> Self
```



## ratatui_widgets::table::Table::header

*Function*

Sets the header row

The `header` parameter is a [`Row`] which will be displayed at the top of the [`Table`]

This is a fluent setter method which must be chained or used as it consumes self

# Examples

```rust
use ratatui::widgets::{Cell, Row, Table};

let header = Row::new(vec![
    Cell::from("Header Cell 1"),
    Cell::from("Header Cell 2"),
]);
let table = Table::default().header(header);
```

```rust
fn header(self: Self, header: Row<'a>) -> Self
```



## ratatui_widgets::table::Table::highlight_spacing

*Function*

Set when to show the highlight spacing

The highlight spacing is the spacing that is allocated for the selection symbol column (if
enabled) and is used to shift the table when a row is selected. This method allows you to
configure when this spacing is allocated.

- [`HighlightSpacing::Always`] will always allocate the spacing, regardless of whether a row
  is selected or not. This means that the table will never change size, regardless of if a
  row is selected or not.
- [`HighlightSpacing::WhenSelected`] will only allocate the spacing if a row is selected.
  This means that the table will shift when a row is selected. This is the default setting
  for backwards compatibility, but it is recommended to use `HighlightSpacing::Always` for a
  better user experience.
- [`HighlightSpacing::Never`] will never allocate the spacing, regardless of whether a row
  is selected or not. This means that the highlight symbol will never be drawn.

This is a fluent setter method which must be chained or used as it consumes self

# Examples

```rust
use ratatui::layout::Constraint;
use ratatui::widgets::{HighlightSpacing, Row, Table};

let rows = [Row::new(vec!["Cell1", "Cell2"])];
let widths = [Constraint::Length(5), Constraint::Length(5)];
let table = Table::new(rows, widths).highlight_spacing(HighlightSpacing::Always);
```

```rust
fn highlight_spacing(self: Self, value: HighlightSpacing) -> Self
```



## ratatui_widgets::table::Table::highlight_symbol

*Function*

Set the symbol to be displayed in front of the selected row

This is a fluent setter method which must be chained or used as it consumes self

# Examples

```rust
use ratatui::layout::Constraint;
use ratatui::widgets::{Cell, Row, Table};

# let rows = [Row::new(vec!["Cell1", "Cell2"])];
# let widths = [Constraint::Length(5), Constraint::Length(5)];
let table = Table::new(rows, widths).highlight_symbol(">>");
```

```rust
fn highlight_symbol<T>(self: Self, highlight_symbol: T) -> Self
```



## ratatui_widgets::table::Table::new

*Function*

Creates a new [`Table`] widget with the given rows.

The `rows` parameter accepts any value that can be converted into an iterator of [`Row`]s.
This includes arrays, slices, and [`Vec`]s.

The `widths` parameter accepts any type that implements `IntoIterator<Item =
Into<Constraint>>`. This includes arrays, slices, vectors, iterators. `Into<Constraint>` is
implemented on u16, so you can pass an array, vec, etc. of u16 to this function to create a
table with fixed width columns.

# Examples

```rust
use ratatui::layout::Constraint;
use ratatui::widgets::{Row, Table};

let rows = [
    Row::new(vec!["Cell1", "Cell2"]),
    Row::new(vec!["Cell3", "Cell4"]),
];
let widths = [Constraint::Length(5), Constraint::Length(5)];
let table = Table::new(rows, widths);
```

```rust
fn new<R, C>(rows: R, widths: C) -> Self
```



## ratatui_widgets::table::Table::row_highlight_style

*Function*

Set the style of the selected row

`style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
your own type that implements [`Into<Style>`]).

This style will be applied to the entire row, including the selection symbol if it is
displayed, and will override any style set on the row or on the individual cells.

This is a fluent setter method which must be chained or used as it consumes self

# Examples

```rust
# use ratatui::{layout::Constraint, style::{Style, Stylize}, widgets::{Row, Table}};
# let rows = [Row::new(vec!["Cell1", "Cell2"])];
# let widths = [Constraint::Length(5), Constraint::Length(5)];
let table = Table::new(rows, widths).row_highlight_style(Style::new().red().italic());
```
[`Color`]: ratatui_core::style::Color

```rust
fn row_highlight_style<S>(self: Self, highlight_style: S) -> Self
```



## ratatui_widgets::table::Table::rows

*Function*

Set the rows

The `rows` parameter accepts any value that can be converted into an iterator of [`Row`]s.
This includes arrays, slices, and [`Vec`]s.

# Warning

This method does not currently set the column widths. You will need to set them manually by
calling [`Table::widths`].

This is a fluent setter method which must be chained or used as it consumes self

# Examples

```rust
use ratatui::widgets::{Row, Table};

let rows = [
    Row::new(vec!["Cell1", "Cell2"]),
    Row::new(vec!["Cell3", "Cell4"]),
];
let table = Table::default().rows(rows);
```

```rust
fn rows<T>(self: Self, rows: T) -> Self
```



## ratatui_widgets::table::Table::style

*Function*

Sets the base style of the widget

`style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
your own type that implements [`Into<Style>`]).

All text rendered by the widget will use this style, unless overridden by [`Block::style`],
[`Row::style`], [`Cell::style`], or the styles of cell's content.

This is a fluent setter method which must be chained or used as it consumes self

# Examples

```rust
use ratatui::layout::Constraint;
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Row, Table};

# let rows = [Row::new(vec!["Cell1", "Cell2"])];
# let widths = [Constraint::Length(5), Constraint::Length(5)];
let table = Table::new(rows, widths).style(Style::new().red().italic());
```

`Table` also implements the [`Styled`] trait, which means you can use style shorthands from
the [`Stylize`] trait to set the style of the widget more concisely.

```rust
use ratatui::layout::Constraint;
use ratatui::style::Stylize;
use ratatui::widgets::{Cell, Row, Table};

# let rows = [Row::new(vec!["Cell1", "Cell2"])];
# let widths = vec![Constraint::Length(5), Constraint::Length(5)];
let table = Table::new(rows, widths).red().italic();
```

[`Color`]: ratatui_core::style::Color
[`Stylize`]: ratatui_core::style::Stylize

```rust
fn style<S>(self: Self, style: S) -> Self
```



## ratatui_widgets::table::Table::widths

*Function*

Set the widths of the columns.

The `widths` parameter accepts any type that implements `IntoIterator<Item =
Into<Constraint>>`. This includes arrays, slices, vectors, iterators. `Into<Constraint>` is
implemented on u16, so you can pass an array, vec, etc. of u16 to this function to create a
table with fixed width columns.

If the widths are empty, the table will be rendered with equal widths.

This is a fluent setter method which must be chained or used as it consumes self

# Examples

```rust
use ratatui::layout::Constraint;
use ratatui::widgets::{Cell, Row, Table};

let table = Table::default().widths([Constraint::Length(5), Constraint::Length(5)]);
let table = Table::default().widths(vec![Constraint::Length(5); 2]);

// widths could also be computed at runtime
let widths = [10, 10, 20].into_iter().map(|c| Constraint::Length(c));
let table = Table::default().widths(widths);
```

```rust
fn widths<I>(self: Self, widths: I) -> Self
```



