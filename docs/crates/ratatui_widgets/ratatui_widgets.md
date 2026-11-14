**ratatui_widgets**

# Module: ratatui_widgets

## Contents

**Modules**

- [`barchart`](#barchart) - The [`BarChart`] widget and its related types (e.g. [`Bar`], [`BarGroup`]).
- [`block`](#block) - Elements related to the `Block` base widget.
- [`borders`](#borders) - Border related types ([`Borders`], [`BorderType`]) and a macro to create borders ([`border`]).
- [`calendar`](#calendar) - A simple calendar widget. `(feature: widget-calendar)`
- [`canvas`](#canvas) - A [`Canvas`] and a collection of [`Shape`]s.
- [`chart`](#chart) - The [`Chart`] widget is used to plot one or more [`Dataset`] in a cartesian coordinate system.
- [`clear`](#clear) - The [`Clear`] widget allows you to clear a certain area to allow overdrawing (e.g. for popups).
- [`fill`](#fill) - The [`Fill`] widget paints every cell in its area with a single symbol and style.
- [`gauge`](#gauge) - The [`Gauge`] widget is used to display a horizontal progress bar.
- [`list`](#list) - The [`List`] widget is used to display a list of items and allows selecting one or multiple
- [`logo`](#logo) - The [`RatatuiLogo`] widget renders the Ratatui logo.
- [`mascot`](#mascot) - A Ratatui mascot widget
- [`paragraph`](#paragraph) - The [`Paragraph`] widget and related types allows displaying a block of text with optional
- [`scrollbar`](#scrollbar) - The [`Scrollbar`] widget is used to display a scrollbar alongside other widgets.
- [`sparkline`](#sparkline) - The [`Sparkline`] widget is used to display a sparkline over one or more lines.
- [`table`](#table) - The [`Table`] widget is used to display multiple rows and columns in a grid and allows selecting
- [`tabs`](#tabs) - The [`Tabs`] widget displays a horizontal set of tabs with a single tab selected.

**Macros**

- [`border`](#border) - Macro that constructs and returns a combination of the [`Borders`] object from TOP, BOTTOM, LEFT

---

## Module: barchart

The [`BarChart`] widget and its related types (e.g. [`Bar`], [`BarGroup`]).



## Module: block

Elements related to the `Block` base widget.

This holds everything needed to display and configure a [`Block`].

In its simplest form, a `Block` is a [border](Borders) around another widget. It can have a
[title](Block::title) and [padding](Block::padding).



## ratatui_widgets::border

*Declarative Macro*

Macro that constructs and returns a combination of the [`Borders`] object from TOP, BOTTOM, LEFT
and RIGHT.

When used with NONE you should consider omitting this completely. For ALL you should consider
[`Block::bordered()`](crate::block::Block::bordered) instead.

## Examples

```
use ratatui::border;
use ratatui::widgets::Block;

Block::new()
    .title("Construct Borders and use them in place")
    .borders(border!(TOP, BOTTOM));
```

`border!` can be called with any number of individual sides:

```
use ratatui::border;
use ratatui::widgets::Borders;
let right_open = border!(TOP, LEFT, BOTTOM);
assert_eq!(right_open, Borders::TOP | Borders::LEFT | Borders::BOTTOM);
```

Single borders work but using `Borders::` directly would be simpler.

```
use ratatui::border;
use ratatui::widgets::Borders;

assert_eq!(border!(TOP), Borders::TOP);
assert_eq!(border!(ALL), Borders::ALL);
assert_eq!(border!(), Borders::NONE);
```

```rust
macro_rules! border {
    () => { ... };
    ($b:ident) => { ... };
    ($first:ident,$($other:ident),*) => { ... };
}
```



## Module: borders

Border related types ([`Borders`], [`BorderType`]) and a macro to create borders ([`border`]).



## Module: calendar

A simple calendar widget. `(feature: widget-calendar)`



The [`Monthly`] widget will display a calendar for the month provided in `display_date`. Days
are styled using the default style unless:
* `show_surrounding` is set, then days not in the `display_date` month will use that style.
* a style is returned by the [`DateStyler`] for the day

[`Monthly`] has several controls for what should be displayed



## Module: canvas

A [`Canvas`] and a collection of [`Shape`]s.

The [`Canvas`] is a blank space on which you can draw anything manually or use one of the
predefined [`Shape`]s.

The available shapes are:

- [`Circle`]: A basic circle
- [`Line`]: A line between two points
- [`Map`]: A world map
- [`Points`]: A scatter of points
- [`Rectangle`]: A basic rectangle

You can also implement your own custom [`Shape`]s.



## Module: chart

The [`Chart`] widget is used to plot one or more [`Dataset`] in a cartesian coordinate system.



## Module: clear

The [`Clear`] widget allows you to clear a certain area to allow overdrawing (e.g. for popups).



## Module: fill

The [`Fill`] widget paints every cell in its area with a single symbol and style.



## Module: gauge

The [`Gauge`] widget is used to display a horizontal progress bar.



## Module: list

The [`List`] widget is used to display a list of items and allows selecting one or multiple
items.



## Module: logo

The [`RatatuiLogo`] widget renders the Ratatui logo.



## Module: mascot

A Ratatui mascot widget

The mascot takes 32x16 cells and is rendered using half block characters.



## Module: paragraph

The [`Paragraph`] widget and related types allows displaying a block of text with optional
wrapping, alignment, and block styling.



## Module: scrollbar

The [`Scrollbar`] widget is used to display a scrollbar alongside other widgets.



## Module: sparkline

The [`Sparkline`] widget is used to display a sparkline over one or more lines.



## Module: table

The [`Table`] widget is used to display multiple rows and columns in a grid and allows selecting
one or multiple cells.



## Module: tabs

The [`Tabs`] widget displays a horizontal set of tabs with a single tab selected.



