**ratatui_widgets > paragraph > Paragraph**

# Module: paragraph::Paragraph

## Contents

**Functions**

- [`alignment`](#alignment) - Set the text alignment for the given paragraph
- [`block`](#block) - Surrounds the [`Paragraph`] widget with a [`Block`].
- [`centered`](#centered) - Center-aligns the text in the given paragraph.
- [`left_aligned`](#left_aligned) - Left-aligns the text in the given paragraph.
- [`right_aligned`](#right_aligned) - Right-aligns the text in the given paragraph.
- [`scroll`](#scroll) - Set the scroll offset for the given paragraph
- [`style`](#style) - Sets the style of the entire widget.
- [`wrap`](#wrap) - Sets the wrapping configuration for the widget.

---

## ratatui_widgets::paragraph::Paragraph::alignment

*Function*

Set the text alignment for the given paragraph

The alignment is a variant of the [`Alignment`] enum which can be one of Left, Right, or
Center. If no alignment is specified, the text in a paragraph will be left-aligned.

# Example

```rust
use ratatui::layout::Alignment;
use ratatui::widgets::Paragraph;

let paragraph = Paragraph::new("Hello World").alignment(Alignment::Center);
```

```rust
fn alignment(self: Self, alignment: Alignment) -> Self
```



## ratatui_widgets::paragraph::Paragraph::block

*Function*

Surrounds the [`Paragraph`] widget with a [`Block`].

# Example

```rust
use ratatui::widgets::{Block, Paragraph};

let paragraph = Paragraph::new("Hello, world!").block(Block::bordered().title("Paragraph"));
```

```rust
fn block(self: Self, block: Block<'a>) -> Self
```



## ratatui_widgets::paragraph::Paragraph::centered

*Function*

Center-aligns the text in the given paragraph.

Convenience shortcut for `Paragraph::alignment(Alignment::Center)`.

# Examples

```rust
use ratatui::widgets::Paragraph;

let paragraph = Paragraph::new("Hello World").centered();
```

```rust
fn centered(self: Self) -> Self
```



## ratatui_widgets::paragraph::Paragraph::left_aligned

*Function*

Left-aligns the text in the given paragraph.

Convenience shortcut for `Paragraph::alignment(Alignment::Left)`.

# Examples

```rust
use ratatui::widgets::Paragraph;

let paragraph = Paragraph::new("Hello World").left_aligned();
```

```rust
fn left_aligned(self: Self) -> Self
```



## ratatui_widgets::paragraph::Paragraph::right_aligned

*Function*

Right-aligns the text in the given paragraph.

Convenience shortcut for `Paragraph::alignment(Alignment::Right)`.

# Examples

```rust
use ratatui::widgets::Paragraph;

let paragraph = Paragraph::new("Hello World").right_aligned();
```

```rust
fn right_aligned(self: Self) -> Self
```



## ratatui_widgets::paragraph::Paragraph::scroll

*Function*

Set the scroll offset for the given paragraph

The scroll offset is a tuple of (y, x) offset. The y offset is the number of lines to
scroll, and the x offset is the number of characters to scroll. The scroll offset is applied
after the text is wrapped and aligned.

Note: the order of the tuple is (y, x) instead of (x, y), which is different from general
convention across the crate.

For more information about future scrolling design and concerns, see [RFC: Design of
Scrollable Widgets](https://github.com/ratatui/ratatui/discussions/1924) on GitHub.

```rust
fn scroll(self: Self, offset: (u16, u16)) -> Self
```



## ratatui_widgets::paragraph::Paragraph::style

*Function*

Sets the style of the entire widget.

`style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
your own type that implements [`Into<Style>`]).

This applies to the entire widget, including the block if one is present. Any style set on
the block or text will be added to this style.

# Example

```rust
use ratatui::style::{Style, Stylize};
use ratatui::widgets::Paragraph;

let paragraph = Paragraph::new("Hello, world!").style(Style::new().red().on_white());
```

[`Color`]: ratatui_core::style::Color

```rust
fn style<S>(self: Self, style: S) -> Self
```



## ratatui_widgets::paragraph::Paragraph::wrap

*Function*

Sets the wrapping configuration for the widget.

See [`Wrap`] for more information on the different options.

# Example

```rust
use ratatui::widgets::{Paragraph, Wrap};

let paragraph = Paragraph::new("Hello, world!").wrap(Wrap { trim: true });
```

```rust
fn wrap(self: Self, wrap: Wrap) -> Self
```



