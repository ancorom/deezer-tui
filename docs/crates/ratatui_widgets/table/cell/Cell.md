**ratatui_widgets > table > cell > Cell**

# Module: table::cell::Cell

## Contents

**Functions**

- [`style`](#style) - Set the `Style` of this cell

---

## ratatui_widgets::table::cell::Cell::style

*Function*

Set the `Style` of this cell

`style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
your own type that implements [`Into<Style>`]).

This `Style` will override the `Style` of the [`Row`] and can be overridden by the `Style`
of the [`Text`] content.

This is a fluent setter method which must be chained or used as it consumes self

# Examples

```rust
use ratatui::style::{Style, Stylize};
use ratatui::widgets::Cell;

Cell::new("Cell 1").style(Style::new().red().italic());
```

`Cell` also implements the [`Styled`] trait, which means you can use style shorthands from
the [`Stylize`] trait to set the style of the widget more concisely.

```rust
use ratatui::style::Stylize;
use ratatui::widgets::Cell;

Cell::new("Cell 1").red().italic();
```

[`Row`]: super::Row
[`Color`]: ratatui_core::style::Color
[`Stylize`]: ratatui_core::style::Stylize

```rust
fn style<S>(self: Self, style: S) -> Self
```



