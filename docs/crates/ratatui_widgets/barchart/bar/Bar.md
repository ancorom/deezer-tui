**ratatui_widgets > barchart > bar > Bar**

# Module: barchart::bar::Bar

## Contents

**Functions**

- [`label`](#label) - Set the label of the bar.
- [`style`](#style) - Set the style of the bar.
- [`text_value`](#text_value) - Set the text value printed in the bar.
- [`value`](#value) - Set the value of this bar.
- [`value_style`](#value_style) - Set the style of the value.

---

## ratatui_widgets::barchart::bar::Bar::label

*Function*

Set the label of the bar.

`label` can be a [`&str`], [`String`] or anything that can be converted into [`Line`].

# Examples

From [`&str`] and [`String`]:

```rust
use ratatui::widgets::Bar;

Bar::default().label("label");
Bar::default().label(String::from("label"));
```

From a [`Line`] with red foreground color:

```rust
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets::Bar;

Bar::default().label(Line::from("Line").red());
```

For [`Vertical`](ratatui_core::layout::Direction::Vertical) bars,
display the label **under** the bar.
For [`Horizontal`](ratatui_core::layout::Direction::Horizontal) bars,
display the label **in** the bar.
See [`BarChart::direction`](crate::barchart::BarChart::direction) to set the direction.

```rust
fn label<T>(self: Self, label: T) -> Self
```



## ratatui_widgets::barchart::bar::Bar::style

*Function*

Set the style of the bar.

`style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
your own type that implements [`Into<Style>`]).

This will apply to every non-styled element. It can be seen and used as a default value.

[`Color`]: ratatui_core::style::Color

```rust
fn style<S>(self: Self, style: S) -> Self
```



## ratatui_widgets::barchart::bar::Bar::text_value

*Function*

Set the text value printed in the bar.

`text_value` can be a [`&str`], `Number` or anything that can be converted into [`String`].

If `text_value` is not set, then the [`ToString`] representation of `value` will be shown on
the bar.

# Examples

From [`&str`] and [`String`]:

```
use ratatui::widgets::Bar;

Bar::default().text_value("label");
Bar::default().text_value(String::from("label"));
```

# See also

[`Bar::value`] to set the value.

```rust
fn text_value<T>(self: Self, text_value: T) -> Self
```



## ratatui_widgets::barchart::bar::Bar::value

*Function*

Set the value of this bar.

The value will be displayed inside the bar.

# See also

- [`Bar::value_style`] to style the value.
- [`Bar::text_value`] to set the displayed value.

```rust
fn value(self: Self, value: u64) -> Self
```



## ratatui_widgets::barchart::bar::Bar::value_style

*Function*

Set the style of the value.

`style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
your own type that implements [`Into<Style>`]).

# See also

[`Bar::value`] to set the value.

[`Color`]: ratatui_core::style::Color

```rust
fn value_style<S>(self: Self, style: S) -> Self
```



