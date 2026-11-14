**ratatui_widgets > list**

# Module: list

## Contents

**Structs**

- [`List`](#list) - A widget to display several items among which one can be selected (optional)

**Enums**

- [`ListDirection`](#listdirection) - Defines the direction in which the list will be rendered.

---

## ratatui_widgets::list::List

*Struct*

A widget to display several items among which one can be selected (optional)

A list is a collection of [`ListItem`]s.

This is different from a [`Table`] because it does not handle columns, headers or footers and
the item's height is automatically determined. A `List` can also be put in reverse order (i.e.
*bottom to top*) whereas a [`Table`] cannot.

[`Table`]: crate::table::Table

List items can be aligned using [`Text::alignment`], for more details see [`ListItem`].

[`List`] is also a [`StatefulWidget`], which means you can use it with [`ListState`] to allow
the user to [scroll] through items and [select] one of them.

See the list in the [Examples] directory for a more in depth example of the various
configuration options and for how to handle state.

[Examples]: https://github.com/ratatui/ratatui/blob/main/examples/README.md

# Fluent setters

- [`List::highlight_style`] sets the style of the selected item.
- [`List::highlight_symbol`] sets the symbol to be displayed in front of the selected item.
- [`List::repeat_highlight_symbol`] sets whether to repeat the symbol and style over selected
  multi-line items
- [`List::direction`] sets the list direction

# Examples

```
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Block, List, ListDirection, ListItem};

# fn ui(frame: &mut Frame) {
# let area = Rect::default();
let items = ["Item 1", "Item 2", "Item 3"];
let list = List::new(items)
    .block(Block::bordered().title("List"))
    .style(Style::new().white())
    .highlight_style(Style::new().italic())
    .highlight_symbol(">>")
    .repeat_highlight_symbol(true)
    .direction(ListDirection::BottomToTop);

frame.render_widget(list, area);
# }
```

# Stateful example

```rust
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Block, List, ListState};

# fn ui(frame: &mut Frame) {
# let area = Rect::default();
// This should be stored outside of the function in your application state.
let mut state = ListState::default();
let items = ["Item 1", "Item 2", "Item 3"];
let list = List::new(items)
    .block(Block::bordered().title("List"))
    .highlight_style(Style::new().reversed())
    .highlight_symbol(">>")
    .repeat_highlight_symbol(true);

frame.render_stateful_widget(list, area, &mut state);
# }
```

In addition to `List::new`, any iterator whose element is convertible to `ListItem` can be
collected into `List`.

```
use ratatui::widgets::List;

(0..5).map(|i| format!("Item{i}")).collect::<List>();
```

[`ListState`]: crate::list::ListState
[scroll]: crate::list::ListState::offset
[select]: crate::list::ListState::select
[`Text::alignment`]: ratatui_core::text::Text::alignment
[`StatefulWidget`]: ratatui_core::widgets::StatefulWidget
[`Widget`]: ratatui_core::widgets::Widget

**Generic Parameters:**
- 'a

**Methods:**

- `fn new<T>(items: T) -> Self` - Creates a new list from [`ListItem`]s
- `fn items<T>(self: Self, items: T) -> Self` - Set the items
- `fn block(self: Self, block: Block<'a>) -> Self` - Wraps the list with a custom [`Block`] widget.
- `fn style<S>(self: Self, style: S) -> Self` - Sets the base style of the widget
- `fn highlight_symbol<L>(self: Self, highlight_symbol: L) -> Self` - Set the symbol to be displayed in front of the selected item
- `fn highlight_style<S>(self: Self, style: S) -> Self` - Set the style of the selected item
- `fn repeat_highlight_symbol(self: Self, repeat: bool) -> Self` - Set whether to repeat the highlight symbol and style over selected multi-line items
- `fn highlight_spacing(self: Self, value: HighlightSpacing) -> Self` - Set when to show the highlight spacing
- `fn direction(self: Self, direction: ListDirection) -> Self` - Defines the list direction (up or down)
- `fn scroll_padding(self: Self, padding: usize) -> Self` - Sets the number of items around the currently selected item that should be kept visible
- `fn len(self: &Self) -> usize` - Returns the number of [`ListItem`]s in the list
- `fn is_empty(self: &Self) -> bool` - Returns true if the list contains no elements.

**Traits:** Eq

**Trait Implementations:**

- **FromIterator**
  - `fn from_iter<Iter>(iter: Iter) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **StatefulWidget**
  - `fn render(self: Self, area: Rect, buf: & mut Buffer, state: & mut <Self as >::State)`
- **Styled**
  - `fn style(self: &Self) -> Style`
  - `fn set_style<S>(self: Self, style: S) -> <Self as >::Item`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Widget**
  - `fn render(self: Self, area: Rect, buf: & mut Buffer)`
- **AsRef**
  - `fn as_ref(self: &Self) -> &crate::list::List<'a>`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## ratatui_widgets::list::ListDirection

*Enum*

Defines the direction in which the list will be rendered.

If there are too few items to fill the screen, the list will stick to the starting edge.

See [`List::direction`].

**Variants:**
- `TopToBottom` - The first value is on the top, going to the bottom
- `BottomToTop` - The first value is on the bottom, going to the top.

**Traits:** Copy, Eq

**Trait Implementations:**

- **FromStr**
  - `fn from_str(s: &str) -> ::core::result::Result<ListDirection, <Self as ::core::str::FromStr>::Err>`
- **TryFrom**
  - `fn try_from(s: &str) -> ::core::result::Result<ListDirection, <Self as ::core::convert::TryFrom>::Error>`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::result::Result<(), ::core::fmt::Error>`



