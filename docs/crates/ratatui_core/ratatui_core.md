**ratatui_core**

# Module: ratatui_core

## Contents

**Modules**

- [`backend`](#backend) - This module provides the backend implementations for different terminal libraries.
- [`buffer`](#buffer) - A module for the [`Buffer`] and [`Cell`] types.
- [`layout`](#layout) - Layout and positioning in terminal user interfaces.
- [`style`](#style) - `style` contains the primitives used to control how your user interface will look.
- [`symbols`](#symbols) - Symbols and markers for drawing various widgets.
- [`terminal`](#terminal) - Provides the [`Terminal`], [`Frame`], [`CompletedFrame`], and [`Viewport`] types.
- [`text`](#text) - Primitives for styled text.
- [`widgets`](#widgets) - The `widgets` module contains the `Widget` and `StatefulWidget` traits, which are used to

**Macros**

- [`assert_buffer_eq`](#assert_buffer_eq) - Assert that two buffers are equal by comparing their areas and content.

---

## ratatui_core::assert_buffer_eq

*Declarative Macro*

Assert that two buffers are equal by comparing their areas and content.

# Panics
When the buffers differ this method panics and displays the differences similar to
`assert_eq!()`.

```rust
macro_rules! assert_buffer_eq {
    ($actual_expr:expr, $expected_expr:expr) => { ... };
}
```



## Module: backend

This module provides the backend implementations for different terminal libraries.

It defines the [`Backend`] trait which is used to abstract over the specific terminal library
being used.

Supported terminal backends:
- [Crossterm]: enable the `crossterm` feature (enabled by default) and use [`CrosstermBackend`]
- [Termion]: enable the `termion` feature and use [`TermionBackend`]
- [Termina]: enable the `termina` feature and use [`TerminaBackend`]
- [Termwiz]: enable the `termwiz` feature and use [`TermwizBackend`]

Additionally, a [`TestBackend`] is provided for testing purposes.

See the [Backend Comparison] section of the [Ratatui Website] for more details on the different
backends.

Each backend supports a number of features, such as [raw mode](#raw-mode), [alternate
screen](#alternate-screen), and [mouse capture](#mouse-capture). These features are generally
not enabled by default, and must be enabled by the application before they can be used. See the
documentation for each backend for more details.

Note: most applications should use the [`Terminal`] struct instead of directly calling methods
on the backend.

# Example

```rust,ignore
use std::io::stdout;

use ratatui::{backend::CrosstermBackend, Terminal};

let backend = CrosstermBackend::new(stdout());
let mut terminal = Terminal::new(backend)?;
terminal.clear()?;
terminal.draw(|frame| {
    // -- snip --
})?;
# std::io::Result::Ok(())
```

See the [Examples] directory for more examples.

# Raw Mode

Raw mode is a mode where the terminal does not perform any processing or handling of the input
and output. This means that features such as echoing input characters, line buffering, and
special character processing (e.g., CTRL-C for SIGINT) are disabled. This is useful for
applications that want to have complete control over the terminal input and output, processing
each keystroke themselves.

For example, in raw mode, the terminal will not perform line buffering on the input, so the
application will receive each key press as it is typed, instead of waiting for the user to
press enter. This makes it suitable for real-time applications like text editors,
terminal-based games, and more.

Each backend handles raw mode differently, so the behavior may vary depending on the backend
being used. Be sure to consult the backend's specific documentation for exact details on how it
implements raw mode.

# Alternate Screen

The alternate screen is a separate buffer that some terminals provide, distinct from the main
screen. When activated, the terminal will display the alternate screen, hiding the current
content of the main screen. Applications can write to this screen as if it were the regular
terminal display, but when the application exits, the terminal will switch back to the main
screen, and the contents of the alternate screen will be cleared. This is useful for
applications like text editors or terminal games that want to use the full terminal window
without disrupting the command line or other terminal content.

This creates a seamless transition between the application and the regular terminal session, as
the content displayed before launching the application will reappear after the application
exits.

Note that not all terminal emulators support the alternate screen, and even those that do may
handle it differently. As a result, the behavior may vary depending on the backend being used.
Always consult the specific backend's documentation to understand how it implements the
alternate screen.

# Mouse Capture

Mouse capture is a mode where the terminal captures mouse events such as clicks, scrolls, and
movement, and sends them to the application as special sequences or events. This enables the
application to handle and respond to mouse actions, providing a more interactive and graphical
user experience within the terminal. It's particularly useful for applications like
terminal-based games, text editors, or other programs that require more direct interaction from
the user.

Each backend handles mouse capture differently, with variations in the types of events that can
be captured and how they are represented. As such, the behavior may vary depending on the
backend being used, and developers should consult the specific backend's documentation to
understand how it implements mouse capture.

[`CrosstermBackend`]: https://docs.rs/ratatui/latest/ratatui/backend/struct.CrosstermBackend.html
[`TermionBackend`]: https://docs.rs/ratatui/latest/ratatui/backend/struct.TermionBackend.html
[`TerminaBackend`]: https://docs.rs/ratatui/latest/ratatui/backend/struct.TerminaBackend.html
[`TermwizBackend`]: https://docs.rs/ratatui/latest/ratatui/backend/struct.TermwizBackend.html
[`Terminal`]: https://docs.rs/ratatui/latest/ratatui/struct.Terminal.html
[Crossterm]: https://crates.io/crates/crossterm
[Termion]: https://crates.io/crates/termion
[Termina]: https://crates.io/crates/termina
[Termwiz]: https://crates.io/crates/termwiz
[Examples]: https://github.com/ratatui/ratatui/tree/main/ratatui/examples/README.md
[Backend Comparison]: https://ratatui.rs/concepts/backends/comparison/
[Ratatui Website]: https://ratatui.rs



## Module: buffer

A module for the [`Buffer`] and [`Cell`] types.



## Module: layout

Layout and positioning in terminal user interfaces.

This module provides a comprehensive set of types and traits for working with layout and
positioning in terminal applications. It implements a flexible layout system that allows you to
divide the terminal screen into different areas using constraints, manage positioning and
sizing, and handle complex UI arrangements.

The layout system in Ratatui is based on the Cassowary constraint solver algorithm, implemented
through the [`kasuari`] crate. This allows for sophisticated constraint-based layouts where
multiple requirements can be satisfied simultaneously, with priorities determining which
constraints take precedence when conflicts arise.

[`kasuari`]: https://crates.io/crates/kasuari

# Core Concepts

## Coordinate System

The coordinate system runs left to right, top to bottom, with the origin `(0, 0)` in the top
left corner of the terminal. The x and y coordinates are represented by `u16` values.

```text
     x (columns)
  ┌─────────────→
y │ (0,0)
  │
(rows)
  ↓
```

## Layout Fundamentals

Layouts form the structural foundation of your terminal UI. The [`Layout`] struct divides
available screen space into rectangular areas using a constraint-based approach. You define
multiple constraints for how space should be allocated, and the Cassowary solver determines
the optimal layout that satisfies as many constraints as possible. These areas can then be
used to render widgets or nested layouts.

Note that the [`Layout`] struct is not required to create layouts - you can also manually
calculate and create [`Rect`] areas using simple mathematics to divide up the terminal space
if you prefer direct control over positioning and sizing.

## Rectangular Areas

All layout operations work with rectangular areas represented by the [`Rect`] type. A [`Rect`]
defines a position and size in the terminal, specified by its top-left corner coordinates and
dimensions.

# Available Types

## Core Layout Types

- [`Layout`] - The primary layout engine that divides space using constraints and direction
- [`Rect`] - Represents a rectangular area with position and dimensions
- [`Constraint`] - Defines how space should be allocated (length, percentage, ratio, etc.)
- [`Direction`] - Specifies layout orientation (horizontal or vertical)
- [`Flex`] - Controls space distribution when constraints are satisfied

## Positioning and Sizing

- [`Position`] - Represents a point in the terminal coordinate system
- [`Size`] - Represents dimensions (width and height)
- [`Margin`] - Defines spacing around rectangular areas
- [`Offset`] - Represents relative movement in the coordinate system
- [`Spacing`] - Controls spacing or overlap between layout segments

## Alignment

- [`Alignment`] (alias for [`HorizontalAlignment`]) - Horizontal text/content alignment
- [`HorizontalAlignment`] - Horizontal alignment options (left, center, right)
- [`VerticalAlignment`] - Vertical alignment options (top, center, bottom)

## Iteration Support

- [`Rows`] - Iterator over horizontal rows within a rectangular area
- [`Columns`] - Iterator over vertical columns within a rectangular area
- [`Positions`] - Iterator over all positions within a rectangular area

# Quick Start

Here's a simple example of creating a basic layout using the [`Layout`] struct:

```rust
use ratatui_core::layout::{Constraint, Direction, Layout, Rect};

// Create a terminal area
let area = Rect::new(0, 0, 80, 24);

// Divide it vertically into two equal parts using Layout
let layout = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]);
let [top, bottom] = layout.areas(area);

// Now you have two areas: top and bottom
```

**Note**: When the number of layout areas is known at compile time, use destructuring
assignment with descriptive variable names for better readability:

```rust
use ratatui_core::layout::{Constraint, Layout, Rect};

let area = Rect::new(0, 0, 80, 24);
let [header, content, footer] = Layout::vertical([
    Constraint::Length(3),
    Constraint::Fill(1),
    Constraint::Length(1),
])
.areas(area);
```

Use [`Layout::split`] when the number of areas is only known at runtime.

Alternatively, you can create layouts manually using mathematics:

```rust
use ratatui_core::layout::Rect;

// Create a terminal area
let area = Rect::new(0, 0, 80, 24);

// Manually divide into two equal parts
let top_half = Rect::new(area.x, area.y, area.width, area.height / 2);
let bottom_half = Rect::new(
    area.x,
    area.y + area.height / 2,
    area.width,
    area.height / 2,
);
```

# Layout Examples

## Basic Vertical Split

```rust
use ratatui_core::layout::{Constraint, Layout, Rect};

let area = Rect::new(0, 0, 80, 24);
let [header, content, footer] = Layout::vertical([
    Constraint::Length(3), // Header: fixed height
    Constraint::Fill(1),   // Content: flexible
    Constraint::Length(1), // Footer: fixed height
])
.areas(area);
```

## Horizontal Sidebar Layout

```rust
use ratatui_core::layout::{Constraint, Layout, Rect};

let area = Rect::new(0, 0, 80, 24);
let [sidebar, main] = Layout::horizontal([
    Constraint::Length(20), // Sidebar: fixed width
    Constraint::Fill(1),    // Main content: flexible
])
.areas(area);
```

## Complex Nested Layout

```rust
use ratatui_core::layout::{Constraint, Layout, Rect};

fn create_complex_layout(area: Rect) -> [Rect; 4] {
    // First, split vertically
    let [header, body, footer] = Layout::vertical([
        Constraint::Length(3), // Header
        Constraint::Fill(1),   // Body
        Constraint::Length(1), // Footer
    ])
    .areas(area);

    // Then split the body horizontally
    let [sidebar, main] = Layout::horizontal([
        Constraint::Length(20), // Sidebar
        Constraint::Fill(1),    // Main
    ])
    .areas(body);

    [header, sidebar, main, footer]
}
```

# Working with Constraints

[`Constraint`]s define how space is allocated within a layout using the Cassowary constraint
solver algorithm. The constraint solver attempts to satisfy all constraints simultaneously,
with priorities determining which constraints take precedence when conflicts arise. Different
constraint types serve different purposes:

- [`Constraint::Min`] - Minimum size constraint
- [`Constraint::Max`] - Maximum size constraint
- [`Constraint::Length`] - Fixed size in character cells
- [`Constraint::Percentage`] - Relative size as a percentage of available space
- [`Constraint::Ratio`] - Proportional size using ratios
- [`Constraint::Fill`] - Proportional fill of remaining space

Constraints are resolved in priority order, with [`Constraint::Min`] having the highest
priority and [`Constraint::Fill`] having the lowest. The constraint solver will satisfy as
many constraints as possible while respecting these priorities.

# Flexible Space Distribution

The [`Flex`] enum controls how extra space is distributed when constraints are satisfied:

- [`Flex::Start`] - Align content to the start, leaving excess space at the end
- [`Flex::End`] - Align content to the end, leaving excess space at the start
- [`Flex::Center`] - Center content, distributing excess space equally on both sides
- [`Flex::SpaceBetween`] - Distribute excess space evenly *between* elements, none at the ends
- [`Flex::SpaceAround`] - Distribute space *around* elements: equal padding on both sides of
  each element; gaps between elements are twice the edge spacing
- [`Flex::SpaceEvenly`] - Distribute space *evenly*: equal spacing between all elements,
  including before the first and after the last.
- [`Flex::Legacy`] - Legacy behavior (puts excess space in the last element)

# Positioning and Alignment

Use [`Position`] to represent specific points in the terminal, [`Size`] for dimensions, and the
alignment types for controlling content positioning within areas:

```rust
use ratatui_core::layout::{Alignment, Position, Rect, Size};

let pos = Position::new(10, 5);
let size = Size::new(80, 24);
let rect = Rect::new(pos.x, pos.y, size.width, size.height);

// Alignment for content within areas
let center = Alignment::Center;
```

# Advanced Features

## Margins and Spacing

Add spacing around areas using uniform margins or between layout segments using [`Spacing`]:

```rust
use ratatui_core::layout::{Constraint, Layout, Margin, Rect, Spacing};

let layout = Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)])
    .margin(2) // 2-cell margin on all sides
    .spacing(Spacing::Space(1)); // 1-cell spacing between segments

// For asymmetric margins, use the Rect inner method directly
let area = Rect::new(0, 0, 80, 24).inner(Margin::new(2, 1));
```

## Area Iteration

Iterate over rows, columns, or all positions within a rectangular area. The `rows()` and
`columns()` iterators return full [`Rect`] regions that can be used to render widgets or
passed to other layout methods for more complex nested layouts. The `positions()` iterator
returns [`Position`] values representing individual cell coordinates:

```rust
use ratatui_core::buffer::Buffer;
use ratatui_core::layout::{Constraint, Layout, Rect};
use ratatui_core::widgets::Widget;

let area = Rect::new(0, 0, 20, 10);
let mut buffer = Buffer::empty(area);

// Renders "Row 0", "Row 1", etc. in each horizontal row
for (i, row) in area.rows().enumerate() {
    format!("Row {i}").render(row, &mut buffer);
}

// Renders column indices (0-9 repeating) in each vertical column
for (i, col) in area.columns().enumerate() {
    format!("{}", i % 10).render(col, &mut buffer);
}

// Renders position indices (0-9 repeating) at each cell position
for (i, pos) in area.positions().enumerate() {
    buffer[pos].set_symbol(&format!("{}", i % 10));
}
```

# Performance Considerations

The layout system includes optional caching to improve performance for repeated layout
calculations. Layout caching is enabled by default in the main `ratatui` crate, but requires
explicitly enabling the `layout-cache` feature when using `ratatui-core` directly. When
enabled, layout results are cached based on the area and layout configuration.

# Related Documentation

For more detailed information and practical examples:

- [Layout Concepts](https://ratatui.rs/concepts/layout/) - Comprehensive guide to layout
  concepts
- [Layout Recipes](https://ratatui.rs/recipes/layout/) - Practical layout examples and patterns
- [Grid Layout Recipe](https://ratatui.rs/recipes/layout/grid/) - Creating grid-based layouts
- [Center a Widget Recipe](https://ratatui.rs/recipes/layout/center-a-widget/) - Centering
  content
- [Dynamic Layouts Recipe](https://ratatui.rs/recipes/layout/dynamic/) - Creating responsive
  layouts

# Examples

See the Ratatui repository for complete examples:

- [`constraints`](https://github.com/ratatui/ratatui/blob/main/examples/apps/constraints/) -
  Demonstrates different constraint types
- [`flex`](https://github.com/ratatui/ratatui/blob/main/examples/apps/flex/) - Shows flex space
  distribution
- [`layout`](https://github.com/ratatui/ratatui/blob/main/examples/apps/layout/) - Basic layout
  examples



## Module: style

`style` contains the primitives used to control how your user interface will look.

There are two ways to set styles:
- Creating and using the [`Style`] struct. (e.g. `Style::new().fg(Color::Red)`).
- Using style shorthands. (e.g. `"hello".red()`).

# Using the `Style` struct

This is the original approach to styling and likely the most common. This is useful when
creating style variables to reuse, however the shorthands are often more convenient and
readable for most use cases.

## Example

```
use ratatui_core::style::{Color, Modifier, Style};
use ratatui_core::text::Span;

let heading_style = Style::new()
    .fg(Color::Black)
    .bg(Color::Green)
    .add_modifier(Modifier::ITALIC | Modifier::BOLD);
let span = Span::styled("hello", heading_style);
```

# Using style shorthands

Originally Ratatui only had the ability to set styles using the `Style` struct. This is still
supported, but there are now shorthands for all the styles that can be set. These save you from
having to create a `Style` struct every time you want to set a style.

The shorthands are implemented in the [`Stylize`] trait which is automatically implemented for
many types via the [`Styled`] trait. This means that you can use the shorthands on any type
that implements [`Styled`]. E.g.:
- Strings and string slices when styled return a [`Span`]
- [`Span`]s can be styled again, which will merge the styles.
- Many widget types can be styled directly rather than calling their `style()` method.

See the [`Stylize`] and [`Styled`] traits for more information.

## Example

```
use ratatui_core::style::{Color, Modifier, Style, Stylize};
use ratatui_core::text::{Span, Text};

assert_eq!(
    "hello".red().on_blue().bold(),
    Span::styled(
        "hello",
        Style::default()
            .fg(Color::Red)
            .bg(Color::Blue)
            .add_modifier(Modifier::BOLD)
    )
);

assert_eq!(
    Text::from("hello").red().on_blue().bold(),
    Text::from("hello").style(
        Style::default()
            .fg(Color::Red)
            .bg(Color::Blue)
            .add_modifier(Modifier::BOLD)
    )
);
```

[`Span`]: crate::text::Span



## Module: symbols

Symbols and markers for drawing various widgets.



## Module: terminal

Provides the [`Terminal`], [`Frame`], [`CompletedFrame`], and [`Viewport`] types.

This module contains Ratatui's rendering surface abstraction. [`Terminal`] ties together a
backend, a viewport, and a double-buffered renderer. In a typical application you create a
`Terminal`, render by calling [`Terminal::draw`] or [`Terminal::try_draw`] in a loop, and let
Ratatui diffs successive frames so only changed cells are sent to the backend.

[`Frame`] is the mutable view used during one render pass. Widgets write into the current
buffer through it, and cursor state for the end of the pass is requested through
[`Frame::set_cursor_position`]. After rendering completes, Ratatui applies the buffer diff,
updates the cursor, swaps buffers, and flushes any buffered backend output.

This module focuses on rendering contracts. Process-wide terminal setup such as raw mode,
alternate screen handling, and panic restoration lives in the higher-level `ratatui` crate.

# Example

```rust,no_run
# #![allow(unexpected_cfgs)]
# #[cfg(feature = "crossterm")]
# {
use std::io::stdout;

use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::widgets::Paragraph;

let backend = CrosstermBackend::new(stdout());
let mut terminal = Terminal::new(backend)?;
terminal.draw(|frame| {
    frame.render_widget(Paragraph::new("Hello world!"), frame.area());
})?;
# }
# Ok::<(), Box<dyn std::error::Error>>(())
```

[Crossterm]: https://crates.io/crates/crossterm
[Termion]: https://crates.io/crates/termion
[Termwiz]: https://crates.io/crates/termwiz
[`backend`]: crate::backend
[`Backend`]: crate::backend::Backend
[`Buffer`]: crate::buffer::Buffer



## Module: text

Primitives for styled text.

A terminal UI is at its root a lot of strings. In order to make it accessible and stylish, those
strings may be associated to a set of styles. `ratatui` has three ways to represent them:
- A single line string where all graphemes have the same style is represented by a [`Span`].
- A single line string where each grapheme may have its own style is represented by [`Line`].
- A multiple line string where each grapheme may have its own style is represented by a
  [`Text`].

These types form a hierarchy: [`Line`] is a collection of [`Span`] and each line of [`Text`] is
a [`Line`].

Keep it mind that a lot of widgets will use those types to advertise what kind of string is
supported for their properties. Moreover, `ratatui` provides convenient `From` implementations
so that you can start by using simple `String` or `&str` and then promote them to the previous
primitives when you need additional styling capabilities.

For example, for the `Block` widget, all the following calls are valid to set its `title`
property (which is a [`Line`] under the hood):

```rust,ignore
use ratatui_core::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::Block,
};

// A simple string with no styling.
// Converted to Line(vec![
//   Span { content: Cow::Borrowed("My title"), style: Style { .. } }
// ])
let block = Block::new().title("My title");

// A simple string with a unique style.
// Converted to Line(vec![
//   Span { content: Cow::Borrowed("My title"), style: Style { fg: Some(Color::Yellow), .. }
// ])
let block = Block::new().title(Span::styled("My title", Style::default().fg(Color::Yellow)));

// A string with multiple styles.
// Converted to Line(vec![
//   Span { content: Cow::Borrowed("My"), style: Style { fg: Some(Color::Yellow), .. } },
//   Span { content: Cow::Borrowed(" title"), .. }
// ])
let block = Block::new().title(vec![
    Span::styled("My", Style::default().fg(Color::Yellow)),
    Span::raw(" title"),
]);
```



## Module: widgets

The `widgets` module contains the `Widget` and `StatefulWidget` traits, which are used to
render UI elements on the screen.



