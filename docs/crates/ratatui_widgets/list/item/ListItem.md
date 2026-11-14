**ratatui_widgets > list > item > ListItem**

# Module: list::item::ListItem

## Contents

**Functions**

- [`height`](#height) - Returns the item height
- [`style`](#style) - Sets the item style
- [`width`](#width) - Returns the max width of all the lines

---

## ratatui_widgets::list::item::ListItem::height

*Function*

Returns the item height

# Examples

One line item

```rust
use ratatui::widgets::ListItem;

let item = ListItem::new("Item 1");
assert_eq!(item.height(), 1);
```

Two lines item (note the `\n`)

```rust
use ratatui::widgets::ListItem;

let item = ListItem::new("Multi-line\nitem");
assert_eq!(item.height(), 2);
```

```rust
fn height(self: &Self) -> usize
```



## ratatui_widgets::list::item::ListItem::style

*Function*

Sets the item style

`style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
your own type that implements [`Into<Style>`]).

This [`Style`] can be overridden by the [`Style`] of the [`Text`] content.

This is a fluent setter method which must be chained or used as it consumes self

# Example

```rust
use ratatui::style::{Style, Stylize};
use ratatui::widgets::ListItem;

let item = ListItem::new("Item 1").style(Style::new().red().italic());
```

`ListItem` also implements the [`Styled`] trait, which means you can use style shorthands
from the [`Stylize`](ratatui_core::style::Stylize) trait to set the style of the widget more
concisely.

```rust
use ratatui::style::Stylize;
use ratatui::widgets::ListItem;

let item = ListItem::new("Item 1").red().italic();
```

[`Styled`]: ratatui_core::style::Styled
[`ListState`]: crate::list::ListState
[`Color`]: ratatui_core::style::Color

```rust
fn style<S>(self: Self, style: S) -> Self
```



## ratatui_widgets::list::item::ListItem::width

*Function*

Returns the max width of all the lines

# Examples

```rust
use ratatui::widgets::ListItem;

let item = ListItem::new("12345");
assert_eq!(item.width(), 5);
```

```rust
use ratatui::widgets::ListItem;

let item = ListItem::new("12345\n1234567");
assert_eq!(item.width(), 7);
```

```rust
fn width(self: &Self) -> usize
```



