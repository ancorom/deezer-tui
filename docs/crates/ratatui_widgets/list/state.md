**ratatui_widgets > list > state**

# Module: list::state

## Contents

**Structs**

- [`ListState`](#liststate) - State of the [`List`] widget

---

## ratatui_widgets::list::state::ListState

*Struct*

State of the [`List`] widget

This state can be used to scroll through items and select one. When the list is rendered as a
stateful widget, the selected item will be highlighted and the list will be shifted to ensure
that the selected item is visible. This will modify the [`ListState`] object passed to the
`Frame::render_stateful_widget` method.

The state consists of two fields:
- [`offset`]: the index of the first item to be displayed
- [`selected`]: the index of the selected item, which can be `None` if no item is selected

[`offset`]: ListState::offset()
[`selected`]: ListState::selected()

See the list in the [Examples] directory for a more in depth example of the various
configuration options and for how to handle state.

[Examples]: https://github.com/ratatui/ratatui/blob/main/examples/README.md

# Example

```rust
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::{List, ListState};

# fn ui(frame: &mut Frame) {
# let area = Rect::default();
let items = ["Item 1"];
let list = List::new(items);

// This should be stored outside of the function in your application state.
let mut state = ListState::default();

*state.offset_mut() = 1; // display the second item and onwards
state.select(Some(3)); // select the forth item (0-indexed)

frame.render_stateful_widget(list, area, &mut state);
# }
```

[`List`]: super::List

**Methods:**

- `fn with_offset(self: Self, offset: usize) -> Self` - Sets the index of the first item to be displayed
- `fn with_selected(self: Self, selected: Option<usize>) -> Self` - Sets the index of the selected item
- `fn offset(self: &Self) -> usize` - Index of the first item to be displayed
- `fn offset_mut(self: & mut Self) -> & mut usize` - Mutable reference to the index of the first item to be displayed
- `fn selected(self: &Self) -> Option<usize>` - Index of the selected item
- `fn selected_mut(self: & mut Self) -> & mut Option<usize>` - Mutable reference to the index of the selected item
- `fn select(self: & mut Self, index: Option<usize>)` - Sets the index of the selected item
- `fn select_next(self: & mut Self)` - Selects the next item or the first one if no item is selected
- `fn select_previous(self: & mut Self)` - Selects the previous item or the last one if no item is selected
- `fn select_first(self: & mut Self)` - Selects the first item
- `fn select_last(self: & mut Self)` - Selects the last item
- `fn scroll_down_by(self: & mut Self, amount: u16)` - Scrolls down by a specified `amount` in the list.
- `fn scroll_up_by(self: & mut Self, amount: u16)` - Scrolls up by a specified `amount` in the list.

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



