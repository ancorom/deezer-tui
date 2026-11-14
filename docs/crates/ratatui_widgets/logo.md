**ratatui_widgets > logo**

# Module: logo

## Contents

**Structs**

- [`RatatuiLogo`](#ratatuilogo) - A widget that renders the Ratatui logo

**Enums**

- [`Size`](#size) - The size of the logo

---

## ratatui_widgets::logo::RatatuiLogo

*Struct*

A widget that renders the Ratatui logo

The Ratatui logo takes up two lines of text and comes in two sizes: `Tiny` and `Small`. This may
be used in an application's help or about screen to show that it is powered by Ratatui.

# Examples

The [Ratatui-logo] example demonstrates how to use the `RatatuiLogo` widget. This can be run by
cloning the Ratatui repository and then running the following command with an optional size
argument:

```shell
cargo run --example logo [size]
```

[Ratatui-logo]: https://github.com/ratatui/ratatui/blob/main/ratatui-widgets/examples/logo.rs

## Tiny (default, 2x15 characters)

```
use ratatui::widgets::RatatuiLogo;

# fn draw(frame: &mut ratatui::Frame) {
frame.render_widget(RatatuiLogo::tiny(), frame.area());
# }
```

Renders:

```text
▛▚▗▀▖▜▘▞▚▝▛▐ ▌▌
▛▚▐▀▌▐ ▛▜ ▌▝▄▘▌
```

## Small (2x27 characters)

```
use ratatui::widgets::RatatuiLogo;

# fn draw(frame: &mut ratatui::Frame) {
frame.render_widget(RatatuiLogo::small(), frame.area());
# }
```

Renders:

```text
█▀▀▄ ▄▀▀▄▝▜▛▘▄▀▀▄▝▜▛▘█  █ █
█▀▀▄ █▀▀█ ▐▌ █▀▀█ ▐▌ ▀▄▄▀ █
```

**Methods:**

- `fn new(size: Size) -> Self` - Create a new Ratatui logo widget
- `fn size(self: Self, size: Size) -> Self` - Set the size of the logo
- `fn tiny() -> Self` - Create a new Ratatui logo widget with a tiny size
- `fn small() -> Self` - Create a new Ratatui logo widget with a small size

**Traits:** Eq, Copy

**Trait Implementations:**

- **Widget**
  - `fn render(self: Self, area: Rect, buf: & mut Buffer)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Default**
  - `fn default() -> Self`
- **AsRef**
  - `fn as_ref(self: &Self) -> &crate::logo::RatatuiLogo`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ratatui_widgets::logo::Size

*Enum*

The size of the logo

**Variants:**
- `Tiny` - A tiny logo
- `Small` - A small logo

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Default**
  - `fn default() -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



