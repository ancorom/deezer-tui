**ratatui_widgets > list > List**

# Module: list::List

## Contents

**Functions**

- [`direction`](#direction) - Defines the list direction (up or down)
- [`highlight_style`](#highlight_style) - Set the style of the selected item
- [`highlight_symbol`](#highlight_symbol) - Set the symbol to be displayed in front of the selected item
- [`items`](#items) - Set the items
- [`new`](#new) - Creates a new list from [`ListItem`]s
- [`repeat_highlight_symbol`](#repeat_highlight_symbol) - Set whether to repeat the highlight symbol and style over selected multi-line items

---

## ratatui_widgets::list::List::direction

*Function*

Defines the list direction (up or down)

Defines if the `List` is displayed *top to bottom* (default) or *bottom to top*.
If there is too few items to fill the screen, the list will stick to the starting edge.

This is a fluent setter method which must be chained or used as it consumes self

# Example

Bottom to top

```rust
use ratatui::widgets::{List, ListDirection};

let items = ["Item 1"];
let list = List::new(items).direction(ListDirection::BottomToTop);
```

```rust
fn direction(self: Self, direction: ListDirection) -> Self
```



## ratatui_widgets::list::List::highlight_style

*Function*

Set the style of the selected item

`style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
your own type that implements [`Into<Style>`]).

This style will be applied to the entire item, including the
[highlight symbol](List::highlight_symbol) if it is displayed, and will override any style
set on the item or on the individual cells.

This is a fluent setter method which must be chained or used as it consumes self

# Examples

```rust
use ratatui::style::{Style, Stylize};
use ratatui::widgets::List;

let items = ["Item 1", "Item 2"];
let list = List::new(items).highlight_style(Style::new().red().italic());
```

[`Color`]: ratatui_core::style::Color

```rust
fn highlight_style<S>(self: Self, style: S) -> Self
```



## ratatui_widgets::list::List::highlight_symbol

*Function*

Set the symbol to be displayed in front of the selected item

By default there are no highlight symbol.

This is a fluent setter method which must be chained or used as it consumes self

# Examples

```rust
use ratatui::widgets::List;

let items = ["Item 1", "Item 2"];
let list = List::new(items).highlight_symbol(">>");
```

```rust
fn highlight_symbol<L>(self: Self, highlight_symbol: L) -> Self
```



## ratatui_widgets::list::List::items

*Function*

Set the items

The `items` parameter accepts any value that can be converted into an iterator of
[`Into<ListItem>`]. This includes arrays of [`&str`] or [`Vec`]s of [`Text`].

This is a fluent setter method which must be chained or used as it consumes self.

# Example

```rust
use ratatui::widgets::List;

let list = List::default().items(["Item 1", "Item 2"]);
```

[`Text`]: ratatui_core::text::Text

```rust
fn items<T>(self: Self, items: T) -> Self
```



## ratatui_widgets::list::List::new

*Function*

Creates a new list from [`ListItem`]s

The `items` parameter accepts any value that can be converted into an iterator of
[`Into<ListItem>`]. This includes arrays of [`&str`] or [`Vec`]s of [`Text`].

# Example

From a slice of [`&str`]

```
use ratatui::widgets::List;

let list = List::new(["Item 1", "Item 2"]);
```

From [`Text`]

```
use ratatui::style::{Style, Stylize};
use ratatui::text::Text;
use ratatui::widgets::List;

let list = List::new([
    Text::styled("Item 1", Style::new().red()),
    Text::styled("Item 2", Style::new().red()),
]);
```

You can also create an empty list using the [`Default`] implementation and use the
[`List::items`] fluent setter.

```rust
use ratatui::widgets::List;

let empty_list = List::default();
let filled_list = empty_list.items(["Item 1"]);
```

[`Text`]: ratatui_core::text::Text

```rust
fn new<T>(items: T) -> Self
```



## ratatui_widgets::list::List::repeat_highlight_symbol

*Function*

Set whether to repeat the highlight symbol and style over selected multi-line items

This is `false` by default.

This is a fluent setter method which must be chained or used as it consumes self

```rust
fn repeat_highlight_symbol(self: Self, repeat: bool) -> Self
```



