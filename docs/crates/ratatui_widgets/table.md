**ratatui_widgets > table**

# Module: table

## Contents

**Structs**

- [`Table`](#table) - A widget to display data in formatted columns.

---

## ratatui_widgets::table::Table

*Struct*

A widget to display data in formatted columns.

A `Table` is a collection of [`Row`]s, each composed of [`Cell`]s:

You can construct a [`Table`] using either [`Table::new`] or [`Table::default`] and then chain
builder style methods to set the desired properties.

Table cells can be aligned, for more details see [`Cell`].

Make sure to call the [`Table::widths`] method, otherwise the columns will all have a width of 0
and thus not be visible.

[`Table`] implements [`Widget`] and so it can be drawn using `Frame::render_widget`.

[`Table`] is also a [`StatefulWidget`], which means you can use it with [`TableState`] to allow
the user to scroll through the rows and select one of them. When rendering a [`Table`] with a
[`TableState`], the selected row, column and cell will be highlighted. If the selected row is
not visible (based on the offset), the table will be scrolled to make the selected row visible.

Note: if the `widths` field is empty, the table will be rendered with equal widths.
Note: Highlight styles are applied in the following order: Row, Column, Cell.

See the table example and the recipe and traceroute tabs in the demo2 example in the [Examples]
directory for a more in depth example of the various configuration options and for how to handle
state.

[Examples]: https://github.com/ratatui/ratatui/blob/master/examples/README.md

# Constructor methods

- [`Table::new`] creates a new [`Table`] with the given rows.
- [`Table::default`] creates an empty [`Table`]. You can then add rows using [`Table::rows`].

# Setter methods

These methods are fluent setters. They return a new `Table` with the specified property set.

- [`Table::rows`] sets the rows of the [`Table`].
- [`Table::header`] sets the header row of the [`Table`].
- [`Table::footer`] sets the footer row of the [`Table`].
- [`Table::widths`] sets the width constraints of each column.
- [`Table::column_spacing`] sets the spacing between each column.
- [`Table::block`] wraps the table in a [`Block`] widget.
- [`Table::style`] sets the base style of the widget.
- [`Table::row_highlight_style`] sets the style of the selected row.
- [`Table::column_highlight_style`] sets the style of the selected column.
- [`Table::cell_highlight_style`] sets the style of the selected cell.
- [`Table::highlight_symbol`] sets the symbol to be displayed in front of the selected row.
- [`Table::highlight_spacing`] sets when to show the highlight spacing.

# Example

```rust
use ratatui::layout::Constraint;
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Block, Row, Table};

let rows = [Row::new(vec!["Cell1", "Cell2", "Cell3"])];
// Columns widths are constrained in the same way as Layout...
let widths = [
    Constraint::Length(5),
    Constraint::Length(5),
    Constraint::Length(10),
];
let table = Table::new(rows, widths)
    // ...and they can be separated by a fixed spacing.
    .column_spacing(1)
    // You can set the style of the entire Table.
    .style(Style::new().blue())
    // It has an optional header, which is simply a Row always visible at the top.
    .header(
        Row::new(vec!["Col1", "Col2", "Col3"])
            .style(Style::new().bold())
            // To add space between the header and the rest of the rows, specify the margin
            .bottom_margin(1),
    )
    // It has an optional footer, which is simply a Row always visible at the bottom.
    .footer(Row::new(vec!["Updated on Dec 28"]))
    // As any other widget, a Table can be wrapped in a Block.
    .block(Block::new().title("Table"))
    // The selected row, column, cell and its content can also be styled.
    .row_highlight_style(Style::new().reversed())
    .column_highlight_style(Style::new().red())
    .cell_highlight_style(Style::new().blue())
    // ...and potentially show a symbol in front of the selection.
    .highlight_symbol(">>");
```

Rows can be created from an iterator of [`Cell`]s. Each row can have an associated height,
bottom margin, and style. See [`Row`] for more details.

```rust
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Cell, Row, Table};

// a Row can be created from simple strings.
let row = Row::new(vec!["Row11", "Row12", "Row13"]);

// You can style the entire row.
let row = Row::new(vec!["Row21", "Row22", "Row23"]).style(Style::new().red());

// If you need more control over the styling, create Cells directly
let row = Row::new(vec![
    Cell::from("Row31"),
    Cell::from("Row32").style(Style::new().yellow()),
    Cell::from(Line::from(vec![Span::raw("Row"), Span::from("33").green()])),
]);

// If a Row need to display some content over multiple lines, specify the height.
let row = Row::new(vec![
    Cell::from("Row\n41"),
    Cell::from("Row\n42"),
    Cell::from("Row\n43"),
])
.height(2);
```

Cells can be created from anything that can be converted to [`Text`]. See [`Cell`] for more
details.

```rust
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::Cell;

Cell::from("simple string");
Cell::from("simple styled span".red());
Cell::from(Span::raw("raw span"));
Cell::from(Span::styled("styled span", Style::new().red()));
Cell::from(Line::from(vec![
    Span::raw("a vec of "),
    Span::from("spans").bold(),
]));
Cell::from(Text::from("text"));
```

Just as rows can be collected from iterators of `Cell`s, tables can be collected from iterators
of `Row`s.  This will create a table with column widths evenly dividing the space available.
These default columns widths can be overridden using the `Table::widths` method.

```rust
use ratatui::layout::Constraint;
use ratatui::widgets::{Row, Table};

let text = "Mary had a\nlittle lamb.";

let table = text
    .split("\n")
    .map(|line: &str| -> Row { line.split_ascii_whitespace().collect() })
    .collect::<Table>()
    .widths([Constraint::Length(10); 3]);
```

`Table` also implements the [`Styled`] trait, which means you can use style shorthands from
the [`Stylize`] trait to set the style of the widget more concisely.

```rust
use ratatui::layout::Constraint;
use ratatui::style::Stylize;
use ratatui::widgets::{Row, Table};

let rows = [Row::new(vec!["Cell1", "Cell2", "Cell3"])];
let widths = [
    Constraint::Length(5),
    Constraint::Length(5),
    Constraint::Length(10),
];
let table = Table::new(rows, widths).red().italic();
```

# Stateful example

`Table` is a [`StatefulWidget`], which means you can use it with [`TableState`] to allow the
user to scroll through the rows and select one of them.

```rust
use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Block, Row, Table, TableState};

# fn ui(frame: &mut Frame) {
# let area = Rect::default();
// Note: TableState should be stored in your application state (not constructed in your render
// method) so that the selected row is preserved across renders
let mut table_state = TableState::default();
let rows = [
    Row::new(vec!["Row11", "Row12", "Row13"]),
    Row::new(vec!["Row21", "Row22", "Row23"]),
    Row::new(vec!["Row31", "Row32", "Row33"]),
];
let widths = [
    Constraint::Length(5),
    Constraint::Length(5),
    Constraint::Length(10),
];
let table = Table::new(rows, widths)
    .block(Block::new().title("Table"))
    .row_highlight_style(Style::new().reversed())
    .highlight_symbol(">>");

frame.render_stateful_widget(table, area, &mut table_state);
# }
```

[`Stylize`]: ratatui_core::style::Stylize

**Generic Parameters:**
- 'a

**Methods:**

- `fn new<R, C>(rows: R, widths: C) -> Self` - Creates a new [`Table`] widget with the given rows.
- `fn rows<T>(self: Self, rows: T) -> Self` - Set the rows
- `fn header(self: Self, header: Row<'a>) -> Self` - Sets the header row
- `fn footer(self: Self, footer: Row<'a>) -> Self` - Sets the footer row
- `fn widths<I>(self: Self, widths: I) -> Self` - Set the widths of the columns.
- `fn column_spacing(self: Self, spacing: u16) -> Self` - Set the spacing between columns
- `fn block(self: Self, block: Block<'a>) -> Self` - Wraps the table with a custom [`Block`] widget.
- `fn style<S>(self: Self, style: S) -> Self` - Sets the base style of the widget
- `fn highlight_style<S>(self: Self, highlight_style: S) -> Self` - Set the style of the selected row
- `fn row_highlight_style<S>(self: Self, highlight_style: S) -> Self` - Set the style of the selected row
- `fn column_highlight_style<S>(self: Self, highlight_style: S) -> Self` - Set the style of the selected column
- `fn cell_highlight_style<S>(self: Self, highlight_style: S) -> Self` - Set the style of the selected cell
- `fn highlight_symbol<T>(self: Self, highlight_symbol: T) -> Self` - Set the symbol to be displayed in front of the selected row
- `fn highlight_spacing(self: Self, value: HighlightSpacing) -> Self` - Set when to show the highlight spacing
- `fn flex(self: Self, flex: Flex) -> Self` - Set how extra space is distributed amongst columns.

**Traits:** Eq

**Trait Implementations:**

- **StatefulWidget**
  - `fn render(self: Self, area: Rect, buf: & mut Buffer, state: & mut <Self as >::State)`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **AsRef**
  - `fn as_ref(self: &Self) -> &crate::table::Table<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Widget**
  - `fn render(self: Self, area: Rect, buf: & mut Buffer)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **FromIterator**
  - `fn from_iter<Iter>(rows: Iter) -> Self` - Collects an iterator of rows into a table.
- **Default**
  - `fn default() -> Self`
- **Styled**
  - `fn style(self: &Self) -> Style`
  - `fn set_style<S>(self: Self, style: S) -> <Self as >::Item`
- **Clone**
  - `fn clone(self: &Self) -> Self`



