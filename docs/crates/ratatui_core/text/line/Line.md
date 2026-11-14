**ratatui_core > text > line > Line**

# Module: text::line::Line

## Contents

**Functions**

- [`alignment`](#alignment) - Sets the target alignment for this line of text.
- [`centered`](#centered) - Center-aligns this line of text.
- [`iter`](#iter) - Returns an iterator over the spans of this line.
- [`iter_mut`](#iter_mut) - Returns a mutable iterator over the spans of this line.
- [`left_aligned`](#left_aligned) - Left-aligns this line of text.
- [`patch_style`](#patch_style) - Patches the style of this Line, adding modifiers from the given style.
- [`push_span`](#push_span) - Adds a span to the line.
- [`raw`](#raw) - Create a line with the default style.
- [`reset_style`](#reset_style) - Resets the style of this Line.
- [`right_aligned`](#right_aligned) - Right-aligns this line of text.
- [`spans`](#spans) - Sets the spans of this line of text.
- [`style`](#style) - Sets the style of this line of text.
- [`styled`](#styled) - Create a line with the given style.
- [`styled_graphemes`](#styled_graphemes) - Returns an iterator over the graphemes held by this line.
- [`width`](#width) - Returns the width of the underlying string.

---

## ratatui_core::text::line::Line::alignment

*Function*

Sets the target alignment for this line of text.

Defaults to: [`None`], meaning the alignment is determined by the rendering widget.
Setting the alignment of a Line generally overrides the alignment of its
parent Text or Widget.

# Examples

```rust
use ratatui_core::layout::Alignment;
use ratatui_core::text::Line;

let mut line = Line::from("Hi, what's up?");
assert_eq!(None, line.alignment);
assert_eq!(
    Some(Alignment::Right),
    line.alignment(Alignment::Right).alignment
)
```

```rust
fn alignment(self: Self, alignment: Alignment) -> Self
```



## ratatui_core::text::line::Line::centered

*Function*

Center-aligns this line of text.

Convenience shortcut for `Line::alignment(Alignment::Center)`.
Setting the alignment of a Line generally overrides the alignment of its
parent Text or Widget, with the default alignment being inherited from the parent.

# Examples

```rust
use ratatui_core::text::Line;

let line = Line::from("Hi, what's up?").centered();
```

```rust
fn centered(self: Self) -> Self
```



## ratatui_core::text::line::Line::iter

*Function*

Returns an iterator over the spans of this line.

```rust
fn iter(self: &Self) -> core::slice::Iter<Span<'a>>
```



## ratatui_core::text::line::Line::iter_mut

*Function*

Returns a mutable iterator over the spans of this line.

```rust
fn iter_mut(self: & mut Self) -> core::slice::IterMut<Span<'a>>
```



## ratatui_core::text::line::Line::left_aligned

*Function*

Left-aligns this line of text.

Convenience shortcut for `Line::alignment(Alignment::Left)`.
Setting the alignment of a Line generally overrides the alignment of its
parent Text or Widget, with the default alignment being inherited from the parent.

# Examples

```rust
use ratatui_core::text::Line;

let line = Line::from("Hi, what's up?").left_aligned();
```

```rust
fn left_aligned(self: Self) -> Self
```



## ratatui_core::text::line::Line::patch_style

*Function*

Patches the style of this Line, adding modifiers from the given style.

This is useful for when you want to apply a style to a line that already has some styling.
In contrast to [`Line::style`], this method will not overwrite the existing style, but
instead will add the given style's modifiers to this Line's style.

`style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
your own type that implements [`Into<Style>`]).

This is a fluent setter method which must be chained or used as it consumes self

# Examples

```rust
use ratatui_core::style::{Color, Modifier};
use ratatui_core::text::Line;

let line = Line::styled("My text", Modifier::ITALIC);

let styled_line = Line::styled("My text", (Color::Yellow, Modifier::ITALIC));

assert_eq!(styled_line, line.patch_style(Color::Yellow));
```

[`Color`]: crate::style::Color

```rust
fn patch_style<S>(self: Self, style: S) -> Self
```



## ratatui_core::text::line::Line::push_span

*Function*

Adds a span to the line.

`span` can be any type that is convertible into a `Span`. For example, you can pass a
`&str`, a `String`, or a `Span`.

# Examples

```rust
use ratatui_core::text::{Line, Span};

let mut line = Line::from("Hello, ");
line.push_span(Span::raw("world!"));
line.push_span(" How are you?");
```

```rust
fn push_span<T>(self: & mut Self, span: T)
```



## ratatui_core::text::line::Line::raw

*Function*

Create a line with the default style.

`content` can be any type that is convertible to [`Cow<str>`] (e.g. [`&str`], [`String`],
[`Cow<str>`], or your own type that implements [`Into<Cow<str>>`]).

A [`Line`] can specify a [`Style`], which will be applied before the style of each [`Span`]
in the line.

Any newlines in the content are removed.

# Examples

```rust
use std::borrow::Cow;

use ratatui_core::text::Line;

Line::raw("test content");
Line::raw(String::from("test content"));
Line::raw(Cow::from("test content"));
```

```rust
fn raw<T>(content: T) -> Self
```



## ratatui_core::text::line::Line::reset_style

*Function*

Resets the style of this Line.

Equivalent to calling `patch_style(Style::reset())`.

This is a fluent setter method which must be chained or used as it consumes self

# Examples

```rust
# let style = Style::default().yellow();
use ratatui_core::style::{Style, Stylize};
use ratatui_core::text::Line;

let line = Line::styled("My text", style);

assert_eq!(Style::reset(), line.reset_style().style);
```

```rust
fn reset_style(self: Self) -> Self
```



## ratatui_core::text::line::Line::right_aligned

*Function*

Right-aligns this line of text.

Convenience shortcut for `Line::alignment(Alignment::Right)`.
Setting the alignment of a Line generally overrides the alignment of its
parent Text or Widget, with the default alignment being inherited from the parent.

# Examples

```rust
use ratatui_core::text::Line;

let line = Line::from("Hi, what's up?").right_aligned();
```

```rust
fn right_aligned(self: Self) -> Self
```



## ratatui_core::text::line::Line::spans

*Function*

Sets the spans of this line of text.

`spans` accepts any iterator that yields items that are convertible to [`Span`] (e.g.
[`&str`], [`String`], [`Span`], or your own type that implements [`Into<Span>`]).

# Examples

```rust
use ratatui_core::style::Stylize;
use ratatui_core::text::Line;

let line = Line::default().spans(vec!["Hello".blue(), " world!".green()]);
let line = Line::default().spans([1, 2, 3].iter().map(|i| format!("Item {}", i)));
```

```rust
fn spans<I>(self: Self, spans: I) -> Self
```



## ratatui_core::text::line::Line::style

*Function*

Sets the style of this line of text.

Defaults to [`Style::default()`].

Note: This field was added in v0.26.0. Prior to that, the style of a line was determined
only by the style of each [`Span`] contained in the line. For this reason, this field may
not be supported by all widgets (outside of the `ratatui` crate itself).

`style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
your own type that implements [`Into<Style>`]).

# Examples
```rust
use ratatui_core::style::{Style, Stylize};
use ratatui_core::text::Line;

let mut line = Line::from("foo").style(Style::new().red());
```

[`Color`]: crate::style::Color

```rust
fn style<S>(self: Self, style: S) -> Self
```



## ratatui_core::text::line::Line::styled

*Function*

Create a line with the given style.

`content` can be any type that is convertible to [`Cow<str>`] (e.g. [`&str`], [`String`],
[`Cow<str>`], or your own type that implements [`Into<Cow<str>>`]).

`style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
your own type that implements [`Into<Style>`]).

# Examples

Any newlines in the content are removed.

```rust
use std::borrow::Cow;

use ratatui_core::style::{Style, Stylize};
use ratatui_core::text::Line;

let style = Style::new().yellow().italic();
Line::styled("My text", style);
Line::styled(String::from("My text"), style);
Line::styled(Cow::from("test content"), style);
```

[`Color`]: crate::style::Color

```rust
fn styled<T, S>(content: T, style: S) -> Self
```



## ratatui_core::text::line::Line::styled_graphemes

*Function*

Returns an iterator over the graphemes held by this line.

`base_style` is the [`Style`] that will be patched with each grapheme [`Style`] to get
the resulting [`Style`].

`base_style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`],
or your own type that implements [`Into<Style>`]).

# Examples

```rust
use std::iter::Iterator;

use ratatui_core::style::{Color, Style};
use ratatui_core::text::{Line, StyledGrapheme};

let line = Line::styled("Text", Style::default().fg(Color::Yellow));
let style = Style::default().fg(Color::Green).bg(Color::Black);
assert_eq!(
    line.styled_graphemes(style)
        .collect::<Vec<StyledGrapheme>>(),
    vec![
        StyledGrapheme::new("T", Style::default().fg(Color::Yellow).bg(Color::Black)),
        StyledGrapheme::new("e", Style::default().fg(Color::Yellow).bg(Color::Black)),
        StyledGrapheme::new("x", Style::default().fg(Color::Yellow).bg(Color::Black)),
        StyledGrapheme::new("t", Style::default().fg(Color::Yellow).bg(Color::Black)),
    ]
);
```

[`Color`]: crate::style::Color

```rust
fn styled_graphemes<S>(self: &'a Self, base_style: S) -> impl Trait
```



## ratatui_core::text::line::Line::width

*Function*

Returns the width of the underlying string.

# Examples

```rust
use ratatui_core::style::Stylize;
use ratatui_core::text::Line;

let line = Line::from(vec!["Hello".blue(), " world!".green()]);
assert_eq!(12, line.width());
```

```rust
fn width(self: &Self) -> usize
```



