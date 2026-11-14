# ratatui_widgets

**ratatui-widgets** contains all the widgets that were previously part of the [Ratatui] crate.
It is meant to be used in conjunction with `ratatui`, which provides the core functionality for
building terminal user interfaces.

[Ratatui]: https://crates.io/crates/ratatui

Most applications shouldn't need to depend directly on `ratatui-widgets`, `ratatui` crate
re-exports all the widgets from this crate. However, if you are building a widget library that
internally uses these widgets, or if you prefer finer grained dependencies, you may want to
depend on this crate rather than transitively through the `ratatui` crate.

Previously, a crate named `ratatui-widgets` was published with some formative ideas about an
eventual Ratatui framework. That crate has now moved to [tui-framework-experiment], pending a
new name.

[tui-framework-experiment]: https://crates.io/crates/tui-framework-experiment

# Installation

Run the following command to add this crate to your project:

```sh
cargo add ratatui-widgets
```

# Available Widgets

- [`BarChart`]: displays multiple datasets as bars with optional grouping.
- [`Block`]: a basic widget that draws a block with optional borders, titles, and styles.
- [`calendar::Monthly`]: displays a single month.
- [`Canvas`]: draws arbitrary shapes using drawing characters.
- [`Chart`]: displays multiple datasets as lines or scatter graphs.
- [`Clear`]: clears the area it occupies. Useful to render over previously drawn widgets.
- [`Fill`]: paints every cell in its area with a single repeated symbol and style.
- [`Gauge`]: displays progress percentage using block characters.
- [`LineGauge`]: displays progress as a line.
- [`List`]: displays a list of items and allows selection.
- [`RatatuiLogo`]: displays the Ratatui logo.
- [`RatatuiMascot`]: displays the Ratatui mascot.
- [`Paragraph`]: displays a paragraph of optionally styled and wrapped text.
- [`Scrollbar`]: displays a scrollbar.
- [`Sparkline`]: displays a single dataset as a sparkline.
- [`Table`]: displays multiple rows and columns in a grid and allows selection.
- [`Tabs`]: displays a tab bar and allows selection.

[`BarChart`]: crate::barchart::BarChart
[`Block`]: crate::block::Block
[`calendar::Monthly`]: crate::calendar::Monthly
[`Canvas`]: crate::canvas::Canvas
[`Chart`]: crate::chart::Chart
[`Clear`]: crate::clear::Clear
[`Fill`]: crate::fill::Fill
[`Gauge`]: crate::gauge::Gauge
[`LineGauge`]: crate::gauge::LineGauge
[`List`]: crate::list::List
[`RatatuiLogo`]: crate::logo::RatatuiLogo
[`RatatuiMascot`]: crate::mascot::RatatuiMascot
[`Paragraph`]: crate::paragraph::Paragraph
[`Scrollbar`]: crate::scrollbar::Scrollbar
[`Sparkline`]: crate::sparkline::Sparkline
[`Table`]: crate::table::Table
[`Tabs`]: crate::tabs::Tabs

All these widgets are re-exported directly under `ratatui::widgets` in the `ratatui` crate.

# Crate Organization

`ratatui-widgets` is part of the Ratatui workspace that was modularized in version 0.30.0.
This crate contains all the built-in widget implementations that were previously part of the
main `ratatui` crate.

**When to use `ratatui-widgets`:**

- Building widget libraries that need to compose with built-in widgets
- You want finer-grained dependencies and only need specific widgets
- Creating custom widgets that extend or wrap the built-in ones

**When to use the main [`ratatui`] crate:**

- Building applications (recommended - includes everything you need)
- You want the convenience of having all widgets available

For detailed information about the workspace organization, see [ARCHITECTURE.md].

[`ratatui`]: https://crates.io/crates/ratatui
[ARCHITECTURE.md]: https://github.com/ratatui/ratatui/blob/main/ARCHITECTURE.md

# Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub. For more
details on contributing, please see the [CONTRIBUTING](CONTRIBUTING.md) document.

# License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Modules

### [`ratatui_widgets`](ratatui_widgets.md)

*1 macro, 17 modules*

### [`barchart`](barchart.md)

*1 struct*

### [`barchart::BarChart`](barchart/BarChart.md)

*3 functions*

### [`barchart::bar`](barchart/bar.md)

*1 struct*

### [`barchart::bar::Bar`](barchart/bar/Bar.md)

*5 functions*

### [`barchart::bar_group`](barchart/bar_group.md)

*1 struct*

### [`block`](block.md)

*1 enum, 1 struct, 1 trait*

### [`block::Block`](block/Block.md)

*17 functions*

### [`block::padding`](block/padding.md)

*1 struct*

### [`block::padding::Padding`](block/padding/Padding.md)

*1 function*

### [`block::shadow`](block/shadow.md)

*1 function, 1 trait, 2 structs*

### [`block::shadow::Shadow`](block/shadow/Shadow.md)

*6 functions*

### [`borders`](borders.md)

*1 enum, 1 struct*

### [`calendar`](calendar.md)

*1 trait, 2 structs*

### [`canvas`](canvas.md)

*1 trait, 4 structs*

### [`canvas::Canvas`](canvas/Canvas.md)

*1 function*

### [`canvas::Context`](canvas/Context.md)

*3 functions*

### [`canvas::circle`](canvas/circle.md)

*1 struct*

### [`canvas::line`](canvas/line.md)

*2 structs*

### [`canvas::map`](canvas/map.md)

*1 enum, 1 struct*

### [`canvas::points`](canvas/points.md)

*1 struct*

### [`canvas::rectangle`](canvas/rectangle.md)

*1 struct*

### [`chart`](chart.md)

*2 enums, 3 structs*

### [`chart::Axis`](chart/Axis.md)

*3 functions*

### [`chart::Chart`](chart/Chart.md)

*5 functions*

### [`chart::Dataset`](chart/Dataset.md)

*5 functions*

### [`clear`](clear.md)

*1 struct*

### [`fill`](fill.md)

*1 struct*

### [`fill::Fill`](fill/Fill.md)

*1 function*

### [`gauge`](gauge.md)

*2 structs*

### [`gauge::Gauge`](gauge/Gauge.md)

*5 functions*

### [`gauge::LineGauge`](gauge/LineGauge.md)

*7 functions*

### [`list`](list.md)

*1 enum, 1 struct*

### [`list::List`](list/List.md)

*6 functions*

### [`list::item`](list/item.md)

*1 struct*

### [`list::item::ListItem`](list/item/ListItem.md)

*3 functions*

### [`list::state`](list/state.md)

*1 struct*

### [`list::state::ListState`](list/state/ListState.md)

*3 functions*

### [`logo`](logo.md)

*1 enum, 1 struct*

### [`mascot`](mascot.md)

*1 enum, 1 struct*

### [`paragraph`](paragraph.md)

*2 structs*

### [`paragraph::Paragraph`](paragraph/Paragraph.md)

*8 functions*

### [`scrollbar`](scrollbar.md)

*2 enums, 2 structs*

### [`scrollbar::Scrollbar`](scrollbar/Scrollbar.md)

*2 functions*

### [`scrollbar::ScrollbarState`](scrollbar/ScrollbarState.md)

*2 functions*

### [`sparkline`](sparkline.md)

*1 enum, 2 structs*

### [`sparkline::Sparkline`](sparkline/Sparkline.md)

*8 functions*

### [`sparkline::SparklineBar`](sparkline/SparklineBar.md)

*1 function*

### [`table`](table.md)

*1 struct*

### [`table::Table`](table/Table.md)

*13 functions*

### [`table::cell`](table/cell.md)

*1 struct*

### [`table::cell::Cell`](table/cell/Cell.md)

*1 function*

### [`table::highlight_spacing`](table/highlight_spacing.md)

*1 enum*

### [`table::row`](table/row.md)

*1 struct*

### [`table::row::Row`](table/row/Row.md)

*2 functions*

### [`table::state`](table/state.md)

*1 struct*

### [`table::state::TableState`](table/state/TableState.md)

*3 functions*

### [`tabs`](tabs.md)

*1 struct*

### [`tabs::Tabs`](tabs/Tabs.md)

*10 functions*

