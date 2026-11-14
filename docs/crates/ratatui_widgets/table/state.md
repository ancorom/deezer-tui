**ratatui_widgets > table > state**

# Module: table::state

## Contents

**Structs**

- [`TableState`](#tablestate) - State of a [`Table`] widget

---

## ratatui_widgets::table::state::TableState

*Struct*

State of a [`Table`] widget

This state can be used to scroll through the rows and select one of them. When the table is
rendered as a stateful widget, the selected row, column and cell will be highlighted and the
table will be shifted to ensure that the selected row is visible. This will modify the
[`TableState`] object passed to the `Frame::render_stateful_widget` method.

The state consists of two fields:
- [`offset`]: the index of the first row to be displayed
- [`selected`]: the index of the selected row, which can be `None` if no row is selected
- [`selected_column`]: the index of the selected column, which can be `None` if no column is
  selected

[`offset`]: TableState::offset()
[`selected`]: TableState::selected()
[`selected_column`]: TableState::selected_column()

See the `table` example and the `recipe` and `traceroute` tabs in the demo2 example in the
[Examples] directory for a more in depth example of the various configuration options and for
how to handle state.

[Examples]: https://github.com/ratatui/ratatui/blob/master/examples/README.md

# Example

```rust
use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::widgets::{Row, Table, TableState};

# fn ui(frame: &mut Frame) {
# let area = Rect::default();
let rows = [Row::new(vec!["Cell1", "Cell2"])];
let widths = [Constraint::Length(5), Constraint::Length(5)];
let table = Table::new(rows, widths).widths(widths);

// Note: TableState should be stored in your application state (not constructed in your render
// method) so that the selected row is preserved across renders
let mut table_state = TableState::default();
*table_state.offset_mut() = 1; // display the second row and onwards
table_state.select(Some(3)); // select the forth row (0-indexed)
table_state.select_column(Some(2)); // select the third column (0-indexed)

frame.render_stateful_widget(table, area, &mut table_state);
# }
```

Note that if [`Table::widths`] is not called before rendering, the rendered columns will have
equal width.

[`Table`]: super::Table
[`Table::widths`]: crate::table::Table::widths

**Methods:**

- `fn new() -> Self` - Creates a new [`TableState`]
- `fn with_offset(self: Self, offset: usize) -> Self` - Sets the index of the first row to be displayed
- `fn with_selected<T>(self: Self, selected: T) -> Self` - Sets the index of the selected row
- `fn with_selected_column<T>(self: Self, selected: T) -> Self` - Sets the index of the selected column
- `fn with_selected_cell<T>(self: Self, selected: T) -> Self` - Sets the indexes of the selected cell
- `fn offset(self: &Self) -> usize` - Index of the first row to be displayed
- `fn offset_mut(self: & mut Self) -> & mut usize` - Mutable reference to the index of the first row to be displayed
- `fn selected(self: &Self) -> Option<usize>` - Index of the selected row
- `fn selected_column(self: &Self) -> Option<usize>` - Index of the selected column
- `fn selected_cell(self: &Self) -> Option<(usize, usize)>` - Indexes of the selected cell
- `fn selected_mut(self: & mut Self) -> & mut Option<usize>` - Mutable reference to the index of the selected row
- `fn selected_column_mut(self: & mut Self) -> & mut Option<usize>` - Mutable reference to the index of the selected column
- `fn select(self: & mut Self, index: Option<usize>)` - Sets the index of the selected row
- `fn select_column(self: & mut Self, index: Option<usize>)` - Sets the index of the selected column
- `fn select_cell(self: & mut Self, indexes: Option<(usize, usize)>)` - Sets the indexes of the selected cell
- `fn select_next(self: & mut Self)` - Selects the next row or the first one if no row is selected
- `fn select_next_column(self: & mut Self)` - Selects the next column or the first one if no column is selected
- `fn select_previous(self: & mut Self)` - Selects the previous row or the last one if no item is selected
- `fn select_previous_column(self: & mut Self)` - Selects the previous column or the last one if no column is selected
- `fn select_first(self: & mut Self)` - Selects the first row
- `fn select_first_column(self: & mut Self)` - Selects the first column
- `fn select_last(self: & mut Self)` - Selects the last row
- `fn select_last_column(self: & mut Self)` - Selects the last column
- `fn scroll_down_by(self: & mut Self, amount: u16)` - Scrolls down by a specified `amount` in the table.
- `fn scroll_up_by(self: & mut Self, amount: u16)` - Scrolls up by a specified `amount` in the table.
- `fn scroll_right_by(self: & mut Self, amount: u16)` - Scrolls right by a specified `amount` in the table.
- `fn scroll_left_by(self: & mut Self, amount: u16)` - Scrolls left by a specified `amount` in the table.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



