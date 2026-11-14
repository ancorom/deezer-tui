**ratatui_widgets > table > row > Row**

# Module: table::row::Row

## Contents

**Functions**

- [`height`](#height) - Set the fixed height of the [`Row`]
- [`style`](#style) - Set the [`Style`] of the entire row

---

## ratatui_widgets::table::row::Row::height

*Function*

Set the fixed height of the [`Row`]

Any [`Cell`] whose content has more lines than this height will see its content truncated.

By default, the height is `1`.

This is a fluent setter method which must be chained or used as it consumes self

# Examples

```rust
use ratatui::widgets::Row;

let cells = vec!["Cell 1\nline 2", "Cell 2", "Cell 3"];
let row = Row::new(cells).height(2);
```

```rust
fn height(self: Self, height: u16) -> Self
```



## ratatui_widgets::table::row::Row::style

*Function*

Set the [`Style`] of the entire row

`style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
your own type that implements [`Into<Style>`]).

This [`Style`] can be overridden by the [`Style`] of a any individual [`Cell`] or by their
[`Text`] content.

This is a fluent setter method which must be chained or used as it consumes self

# Examples

```rust
use ratatui::style::{Style, Stylize};
use ratatui::widgets::Row;
let cells = vec!["Cell 1", "Cell 2", "Cell 3"];
let row = Row::new(cells).style(Style::new().red().italic());
```

`Row` also implements the [`Styled`] trait, which means you can use style shorthands from
the [`Stylize`] trait to set the style of the widget more concisely.

```rust
use ratatui::style::Stylize;
use ratatui::widgets::Row;

let cells = vec!["Cell 1", "Cell 2", "Cell 3"];
let row = Row::new(cells).red().italic();
```

[`Color`]: ratatui_core::style::Color
[`Stylize`]: ratatui_core::style::Stylize
[`Text`]: ratatui_core::text::Text

```rust
fn style<S>(self: Self, style: S) -> Self
```



