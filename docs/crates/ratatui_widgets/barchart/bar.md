**ratatui_widgets > barchart > bar**

# Module: barchart::bar

## Contents

**Structs**

- [`Bar`](#bar) - A bar to be shown by the [`BarChart`](super::BarChart) widget.

---

## ratatui_widgets::barchart::bar::Bar

*Struct*

A bar to be shown by the [`BarChart`](super::BarChart) widget.

Here is an explanation of a `Bar`'s components.
```plain
███                          ┐
█2█  <- text_value or value  │ bar
foo  <- label                ┘
```
Note that every element can be styled individually.

# Example

The following example creates a bar with the label "Bar 1", a value "10",
red background and a white value foreground.
```
use ratatui::style::{Style, Stylize};
use ratatui::widgets::Bar;

Bar::with_label("Bar 1", 10)
    .red()
    .value_style(Style::new().red().on_white())
    .text_value("10°C");
```

**Generic Parameters:**
- 'a

**Methods:**

- `fn new(value: u64) -> Self` - Creates a new `Bar` with the given value.
- `fn with_label<T>(label: T, value: u64) -> Self` - Creates a new `Bar` with the given `label` and value.
- `fn value(self: Self, value: u64) -> Self` - Set the value of this bar.
- `fn label<T>(self: Self, label: T) -> Self` - Set the label of the bar.
- `fn style<S>(self: Self, style: S) -> Self` - Set the style of the bar.
- `fn value_style<S>(self: Self, style: S) -> Self` - Set the style of the value.
- `fn text_value<T>(self: Self, text_value: T) -> Self` - Set the text value printed in the bar.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Styled**
  - `fn style(self: &Self) -> Style`
  - `fn set_style<S>(self: Self, style: S) -> <Self as >::Item`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



