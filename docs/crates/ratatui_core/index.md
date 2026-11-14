# ratatui_core

**ratatui-core** is the core library of the [ratatui] project,
providing the essential building blocks for creating rich terminal user interfaces in Rust.

[ratatui]: https://github.com/ratatui/ratatui

# Why `ratatui-core`?

The `ratatui-core` crate is split from the main [`ratatui`](https://crates.io/crates/ratatui)
crate to offer better stability for widget library authors and advanced integrations. Widget
libraries should generally depend on `ratatui-core`, benefiting from a stable API and reducing
the need for frequent updates.

Most applications, on the other hand, should depend on the main `ratatui` crate, which
includes built-in widgets, backend re-exports, and higher-level setup helpers.

In practice:

- Use [`ratatui`] to build applications.
- Use `ratatui-core` to implement widgets, backend integrations, or other code that needs the
  core rendering and layout contracts directly.

# Installation

Add `ratatui-core` to your `Cargo.toml`:

```shell
cargo add ratatui-core
```

# Crate Organization

`ratatui-core` is part of the Ratatui workspace that was modularized in version 0.30.0 to
improve compilation times, API stability, and dependency management. This crate provides the
foundational types and traits that other crates in the workspace depend on.

**When to use `ratatui-core`:**

- Building widget libraries that implement [`Widget`] or [`StatefulWidget`]
- Building custom integrations on top of Ratatui's core rendering contracts
- You want minimal dependencies and faster compilation times
- You need maximum API stability (core types change less frequently)

**When to use the main [`ratatui`] crate:**

- Building applications
- You want built-in widgets, backend re-exports, and setup helpers such as `ratatui::run`

For detailed information about the workspace organization, see [ARCHITECTURE.md].

[`ratatui`]: https://crates.io/crates/ratatui
[`Widget`]: widgets::Widget
[`StatefulWidget`]: widgets::StatefulWidget
[ARCHITECTURE.md]: https://github.com/ratatui/ratatui/blob/main/ARCHITECTURE.md

# Contributing

We welcome contributions from the community! Please see our [CONTRIBUTING](../CONTRIBUTING.md)
guide for more details on how to get involved.

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Modules

### [`ratatui_core`](ratatui_core.md)

*1 macro, 8 modules*

### [`backend`](backend.md)

*1 enum, 1 struct, 1 trait*

### [`backend::test`](backend/test.md)

*1 struct*

### [`buffer::buffer`](buffer/buffer.md)

*1 struct*

### [`buffer::buffer::Buffer`](buffer/buffer/Buffer.md)

*4 functions*

### [`buffer::cell`](buffer/cell.md)

*1 enum, 1 struct*

### [`buffer::cell::Cell`](buffer/cell/Cell.md)

*2 functions*

### [`buffer::cell_width`](buffer/cell_width.md)

*1 trait*

### [`buffer::diff`](buffer/diff.md)

*1 struct*

### [`layout::alignment`](layout/alignment.md)

*1 type alias, 2 enums*

### [`layout::constraint`](layout/constraint.md)

*1 enum*

### [`layout::constraint::Constraint`](layout/constraint/Constraint.md)

*7 functions*

### [`layout::direction`](layout/direction.md)

*1 enum*

### [`layout::flex`](layout/flex.md)

*1 enum*

### [`layout::layout`](layout/layout.md)

*1 enum, 1 struct*

### [`layout::layout::Layout`](layout/layout/Layout.md)

*15 functions*

### [`layout::margin`](layout/margin.md)

*1 struct*

### [`layout::margin::Margin`](layout/margin/Margin.md)

*1 function*

### [`layout::offset`](layout/offset.md)

*1 struct*

### [`layout::position`](layout/position.md)

*1 struct*

### [`layout::position::Position`](layout/position/Position.md)

*2 functions*

### [`layout::rect`](layout/rect.md)

*1 struct*

### [`layout::rect::Rect`](layout/rect/Rect.md)

*24 functions*

### [`layout::rect::iter`](layout/rect/iter.md)

*3 structs*

### [`layout::size`](layout/size.md)

*1 struct*

### [`layout::size::Size`](layout/size/Size.md)

*2 functions*

### [`style`](style.md)

*1 module, 2 structs*

### [`style::Style`](style/Style.md)

*1 function*

### [`style::color`](style/color.md)

*1 enum, 1 struct*

### [`style::palette`](style/palette.md)

*2 modules*

### [`style::palette::material`](style/palette/material.md)

*2 structs, 21 constants*

### [`style::palette::tailwind`](style/palette/tailwind.md)

*1 struct, 24 constants*

### [`style::stylize`](style/stylize.md)

*2 traits*

### [`symbols`](symbols.md)

*11 modules*

### [`symbols::bar`](symbols/bar.md)

*1 struct, 10 constants*

### [`symbols::block`](symbols/block.md)

*1 struct, 10 constants*

### [`symbols::border`](symbols/border.md)

*1 struct, 37 constants*

### [`symbols::braille`](symbols/braille.md)

*1 constant*

### [`symbols::half_block`](symbols/half_block.md)

*3 constants*

### [`symbols::line`](symbols/line.md)

*1 struct, 59 constants*

### [`symbols::marker`](symbols/marker.md)

*1 constant, 1 enum*

### [`symbols::merge`](symbols/merge.md)

*1 enum*

### [`symbols::pixel`](symbols/pixel.md)

*3 constants*

### [`symbols::scrollbar`](symbols/scrollbar.md)

*1 struct, 4 constants*

### [`symbols::shade`](symbols/shade.md)

*5 constants*

### [`terminal`](terminal.md)

*2 structs*

### [`terminal::Terminal`](terminal/Terminal.md)

*15 functions*

### [`terminal::frame`](terminal/frame.md)

*2 structs*

### [`terminal::frame::Frame`](terminal/frame/Frame.md)

*4 functions*

### [`terminal::viewport`](terminal/viewport.md)

*1 enum*

### [`text::grapheme`](text/grapheme.md)

*1 struct*

### [`text::line`](text/line.md)

*1 struct, 1 trait*

### [`text::line::Line`](text/line/Line.md)

*15 functions*

### [`text::masked`](text/masked.md)

*1 struct*

### [`text::span`](text/span.md)

*1 struct, 1 trait*

### [`text::span::Span`](text/span/Span.md)

*8 functions*

### [`text::text`](text/text.md)

*1 struct, 1 trait*

### [`text::text::Text`](text/text/Text.md)

*15 functions*

### [`widgets::stateful_widget`](widgets/stateful_widget.md)

*1 trait*

### [`widgets::widget`](widgets/widget.md)

*1 trait*

