**ratatui_core > text > text > Text**

# Module: text::text::Text

## Contents

**Functions**

- [`alignment`](#alignment) - Sets the alignment for this text.
- [`centered`](#centered) - Center-aligns the whole text.
- [`height`](#height) - Returns the height.
- [`iter`](#iter) - Returns an iterator over the lines of the text.
- [`iter_mut`](#iter_mut) - Returns an iterator that allows modifying each line.
- [`left_aligned`](#left_aligned) - Left-aligns the whole text.
- [`patch_style`](#patch_style) - Patches the style of this Text, adding modifiers from the given style.
- [`push_line`](#push_line) - Adds a line to the text.
- [`push_span`](#push_span) - Adds a span to the last line of the text.
- [`raw`](#raw) - Create some text (potentially multiple lines) with no style.
- [`reset_style`](#reset_style) - Resets the style of the Text.
- [`right_aligned`](#right_aligned) - Right-aligns the whole text.
- [`style`](#style) - Sets the style of this text.
- [`styled`](#styled) - Create some text (potentially multiple lines) with a style.
- [`width`](#width) - Returns the max width of all the lines.

---

## ratatui_core::text::text::Text::alignment

*Function*

Sets the alignment for this text.

Defaults to: [`None`], meaning the alignment is determined by the rendering widget.
Setting the alignment of a Text generally overrides the alignment of its
parent Widget.

Alignment can be set individually on each line to override this text's alignment.

# Examples

Set alignment to the whole text.

```rust
use ratatui_core::layout::Alignment;
use ratatui_core::text::Text;

let mut text = Text::from("Hi, what's up?");
assert_eq!(None, text.alignment);
assert_eq!(
    Some(Alignment::Right),
    text.alignment(Alignment::Right).alignment
)
```

Set a default alignment and override it on a per line basis.

```rust
use ratatui_core::layout::Alignment;
use ratatui_core::text::{Line, Text};

let text = Text::from(vec![
    Line::from("left").alignment(Alignment::Left),
    Line::from("default"),
    Line::from("default"),
    Line::from("right").alignment(Alignment::Right),
])
.alignment(Alignment::Center);
```

Will render the following

```plain
left
  default
  default
      right
```

```rust
fn alignment(self: Self, alignment: Alignment) -> Self
```



## ratatui_core::text::text::Text::centered

*Function*

Center-aligns the whole text.

Convenience shortcut for `Text::alignment(Alignment::Center)`.
Setting the alignment of a Text generally overrides the alignment of its
parent Widget, with the default alignment being inherited from the parent.

Alignment can be set individually on each line to override this text's alignment.

# Examples

```rust
use ratatui_core::text::Text;

let text = Text::from("Hi, what's up?").centered();
```

```rust
fn centered(self: Self) -> Self
```



## ratatui_core::text::text::Text::height

*Function*

Returns the height.

# Examples

```rust
use ratatui_core::text::Text;

let text = Text::from("The first line\nThe second line");
assert_eq!(2, text.height());
```

```rust
fn height(self: &Self) -> usize
```



## ratatui_core::text::text::Text::iter

*Function*

Returns an iterator over the lines of the text.

```rust
fn iter(self: &Self) -> core::slice::Iter<Line<'a>>
```



## ratatui_core::text::text::Text::iter_mut

*Function*

Returns an iterator that allows modifying each line.

```rust
fn iter_mut(self: & mut Self) -> core::slice::IterMut<Line<'a>>
```



## ratatui_core::text::text::Text::left_aligned

*Function*

Left-aligns the whole text.

Convenience shortcut for `Text::alignment(Alignment::Left)`.
Setting the alignment of a Text generally overrides the alignment of its
parent Widget, with the default alignment being inherited from the parent.

Alignment can be set individually on each line to override this text's alignment.

# Examples

```rust
use ratatui_core::text::Text;

let text = Text::from("Hi, what's up?").left_aligned();
```

```rust
fn left_aligned(self: Self) -> Self
```



## ratatui_core::text::text::Text::patch_style

*Function*

Patches the style of this Text, adding modifiers from the given style.

This is useful for when you want to apply a style to a text that already has some styling.
In contrast to [`Text::style`], this method will not overwrite the existing style, but
instead will add the given style's modifiers to this text's style.

`Text` also implements [`Styled`] which means you can use the methods of the [`Stylize`]
trait.

`style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
your own type that implements [`Into<Style>`]).

This is a fluent setter method which must be chained or used as it consumes self

# Examples

```rust
use ratatui_core::style::{Color, Modifier};
use ratatui_core::text::Text;

let raw_text = Text::styled("The first line\nThe second line", Modifier::ITALIC);
let styled_text = Text::styled(
    String::from("The first line\nThe second line"),
    (Color::Yellow, Modifier::ITALIC),
);
assert_ne!(raw_text, styled_text);

let raw_text = raw_text.patch_style(Color::Yellow);
assert_eq!(raw_text, styled_text);
```

[`Color`]: crate::style::Color
[`Stylize`]: crate::style::Stylize

```rust
fn patch_style<S>(self: Self, style: S) -> Self
```



## ratatui_core::text::text::Text::push_line

*Function*

Adds a line to the text.

`line` can be any type that can be converted into a `Line`. For example, you can pass a
`&str`, a `String`, a `Span`, or a `Line`.

# Examples

```rust
use ratatui_core::text::{Line, Span, Text};

let mut text = Text::from("Hello, world!");
text.push_line(Line::from("How are you?"));
text.push_line(Span::from("How are you?"));
text.push_line("How are you?");
```

```rust
fn push_line<T>(self: & mut Self, line: T)
```



## ratatui_core::text::text::Text::push_span

*Function*

Adds a span to the last line of the text.

`span` can be any type that is convertible into a `Span`. For example, you can pass a
`&str`, a `String`, or a `Span`.

# Examples

```rust
use ratatui_core::text::{Span, Text};

let mut text = Text::from("Hello, world!");
text.push_span(Span::from("How are you?"));
text.push_span("How are you?");
```

```rust
fn push_span<T>(self: & mut Self, span: T)
```



## ratatui_core::text::text::Text::raw

*Function*

Create some text (potentially multiple lines) with no style.

# Examples

```rust
use ratatui_core::text::Text;

Text::raw("The first line\nThe second line");
Text::raw(String::from("The first line\nThe second line"));
```

```rust
fn raw<T>(content: T) -> Self
```



## ratatui_core::text::text::Text::reset_style

*Function*

Resets the style of the Text.

Equivalent to calling [`patch_style(Style::reset())`](Text::patch_style).

This is a fluent setter method which must be chained or used as it consumes self

# Examples

```rust
use ratatui_core::style::{Color, Modifier, Style};
use ratatui_core::text::Text;

let text = Text::styled(
    "The first line\nThe second line",
    (Color::Yellow, Modifier::ITALIC),
);

let text = text.reset_style();
assert_eq!(Style::reset(), text.style);
```

```rust
fn reset_style(self: Self) -> Self
```



## ratatui_core::text::text::Text::right_aligned

*Function*

Right-aligns the whole text.

Convenience shortcut for `Text::alignment(Alignment::Right)`.
Setting the alignment of a Text generally overrides the alignment of its
parent Widget, with the default alignment being inherited from the parent.

Alignment can be set individually on each line to override this text's alignment.

# Examples

```rust
use ratatui_core::text::Text;

let text = Text::from("Hi, what's up?").right_aligned();
```

```rust
fn right_aligned(self: Self) -> Self
```



## ratatui_core::text::text::Text::style

*Function*

Sets the style of this text.

Defaults to [`Style::default()`].

Note: This field was added in v0.26.0. Prior to that, the style of a text was determined
only by the style of each [`Line`] contained in the line. For this reason, this field may
not be supported by all widgets (outside of the `ratatui` crate itself).

`style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
your own type that implements [`Into<Style>`]).

# Examples
```rust
use ratatui_core::style::{Style, Stylize};
use ratatui_core::text::Text;

let mut line = Text::from("foo").style(Style::new().red());
```

[`Color`]: crate::style::Color

```rust
fn style<S>(self: Self, style: S) -> Self
```



## ratatui_core::text::text::Text::styled

*Function*

Create some text (potentially multiple lines) with a style.

`style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
your own type that implements [`Into<Style>`]).

# Examples

```rust
use ratatui_core::style::{Color, Modifier, Style};
use ratatui_core::text::Text;

let style = Style::default()
    .fg(Color::Yellow)
    .add_modifier(Modifier::ITALIC);
Text::styled("The first line\nThe second line", style);
Text::styled(String::from("The first line\nThe second line"), style);
```

[`Color`]: crate::style::Color

```rust
fn styled<T, S>(content: T, style: S) -> Self
```



## ratatui_core::text::text::Text::width

*Function*

Returns the max width of all the lines.

# Examples

```rust
use ratatui_core::text::Text;

let text = Text::from("The first line\nThe second line");
assert_eq!(15, text.width());
```

```rust
fn width(self: &Self) -> usize
```



