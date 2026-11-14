**ratatui_widgets > list > state > ListState**

# Module: list::state::ListState

## Contents

**Functions**

- [`offset`](#offset) - Index of the first item to be displayed
- [`select`](#select) - Sets the index of the selected item
- [`selected`](#selected) - Index of the selected item

---

## ratatui_widgets::list::state::ListState::offset

*Function*

Index of the first item to be displayed

# Examples

```rust
use ratatui::widgets::ListState;

let state = ListState::default();
assert_eq!(state.offset(), 0);
```

```rust
fn offset(self: &Self) -> usize
```



## ratatui_widgets::list::state::ListState::select

*Function*

Sets the index of the selected item

Set to `None` if no item is selected. This will also reset the offset to `0`.

# Examples

```rust
use ratatui::widgets::ListState;

let mut state = ListState::default();
state.select(Some(1));
```

```rust
fn select(self: & mut Self, index: Option<usize>)
```



## ratatui_widgets::list::state::ListState::selected

*Function*

Index of the selected item

Returns `None` if no item is selected

# Examples

```rust
use ratatui::widgets::ListState;

let state = ListState::default();
assert_eq!(state.selected(), None);
```

```rust
fn selected(self: &Self) -> Option<usize>
```



