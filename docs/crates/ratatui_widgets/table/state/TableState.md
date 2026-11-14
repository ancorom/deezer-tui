**ratatui_widgets > table > state > TableState**

# Module: table::state::TableState

## Contents

**Functions**

- [`offset`](#offset) - Index of the first row to be displayed
- [`selected`](#selected) - Index of the selected row
- [`selected_column`](#selected_column) - Index of the selected column

---

## ratatui_widgets::table::state::TableState::offset

*Function*

Index of the first row to be displayed

# Examples

```rust
use ratatui::widgets::TableState;

let state = TableState::new();
assert_eq!(state.offset(), 0);
```

```rust
fn offset(self: &Self) -> usize
```



## ratatui_widgets::table::state::TableState::selected

*Function*

Index of the selected row

Returns `None` if no row is selected

# Examples

```rust
use ratatui::widgets::TableState;

let state = TableState::new();
assert_eq!(state.selected(), None);
```

```rust
fn selected(self: &Self) -> Option<usize>
```



## ratatui_widgets::table::state::TableState::selected_column

*Function*

Index of the selected column

Returns `None` if no column is selected

# Examples

```rust
# use ratatui::widgets::{TableState};
let state = TableState::new();
assert_eq!(state.selected_column(), None);
```

```rust
fn selected_column(self: &Self) -> Option<usize>
```



