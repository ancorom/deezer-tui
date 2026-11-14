**ratatui_core > text > span > Span**

# Module: text::span::Span

## Contents

**Functions**

- [`content`](#content) - Sets the content of the span.
- [`patch_style`](#patch_style) - Patches the style of the Span, adding modifiers from the given style.
- [`raw`](#raw) - Create a span with the default style.
- [`reset_style`](#reset_style) - Resets the style of the Span.
- [`style`](#style) - Sets the style of the span.
- [`styled`](#styled) - Create a span with the specified style.
- [`styled_graphemes`](#styled_graphemes) - Returns an iterator over the graphemes held by this span.
- [`width`](#width) - Returns the unicode width of the content held by this span.

---

## ratatui_core::text::span::Span::content

*Function*

Sets the content of the span.

This is a fluent setter method which must be chained or used as it consumes self

Accepts any type that can be converted to [`Cow<str>`] (e.g. `&str`, `String`, `&String`,
etc.).

# Examples

```rust
use ratatui_core::text::Span;

let mut span = Span::default().content("content");
```

```rust
fn content<T>(self: Self, content: T) -> Self
```



## ratatui_core::text::span::Span::patch_style

*Function*

Patches the style of the Span, adding modifiers from the given style.

`style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
your own type that implements [`Into<Style>`]).

This is a fluent setter method which must be chained or used as it consumes self

# Example

```rust
use ratatui_core::style::{Style, Stylize};
use ratatui_core::text::Span;

let span = Span::styled("test content", Style::new().green().italic())
    .patch_style(Style::new().red().on_yellow().bold());
assert_eq!(span.style, Style::new().red().on_yellow().italic().bold());
```

[`Color`]: crate::style::Color

```rust
fn patch_style<S>(self: Self, style: S) -> Self
```



## ratatui_core::text::span::Span::raw

*Function*

Create a span with the default style.

# Examples

```rust
use ratatui_core::text::Span;

Span::raw("test content");
Span::raw(String::from("test content"));
```

```rust
fn raw<T>(content: T) -> Self
```



## ratatui_core::text::span::Span::reset_style

*Function*

Resets the style of the Span.

This is Equivalent to calling `patch_style(Style::reset())`.

This is a fluent setter method which must be chained or used as it consumes self

# Example

```rust
use ratatui_core::style::{Style, Stylize};
use ratatui_core::text::Span;

let span = Span::styled(
    "Test Content",
    Style::new().dark_gray().on_yellow().italic(),
)
.reset_style();
assert_eq!(span.style, Style::reset());
```

```rust
fn reset_style(self: Self) -> Self
```



## ratatui_core::text::span::Span::style

*Function*

Sets the style of the span.

This is a fluent setter method which must be chained or used as it consumes self

In contrast to [`Span::patch_style`], this method replaces the style of the span instead of
patching it.

`style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
your own type that implements [`Into<Style>`]).

# Examples

```rust
use ratatui_core::style::{Style, Stylize};
use ratatui_core::text::Span;

let mut span = Span::default().style(Style::new().green());
```

[`Color`]: crate::style::Color

```rust
fn style<S>(self: Self, style: S) -> Self
```



## ratatui_core::text::span::Span::styled

*Function*

Create a span with the specified style.

`content` accepts any type that is convertible to [`Cow<str>`] (e.g. `&str`, `String`,
`&String`, etc.).

`style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
your own type that implements [`Into<Style>`]).

# Examples

```rust
use ratatui_core::style::{Style, Stylize};
use ratatui_core::text::Span;

let style = Style::new().yellow().on_green().italic();
Span::styled("test content", style);
Span::styled(String::from("test content"), style);
```

[`Color`]: crate::style::Color

```rust
fn styled<T, S>(content: T, style: S) -> Self
```



## ratatui_core::text::span::Span::styled_graphemes

*Function*

Returns an iterator over the graphemes held by this span.

`base_style` is the [`Style`] that will be patched with the `Span`'s `style` to get the
resulting [`Style`].

`base_style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`],
or your own type that implements [`Into<Style>`]).

# Example

```rust
use std::iter::Iterator;

use ratatui_core::style::{Style, Stylize};
use ratatui_core::text::{Span, StyledGrapheme};

let span = Span::styled("Test", Style::new().green().italic());
let style = Style::new().red().on_yellow();
assert_eq!(
    span.styled_graphemes(style)
        .collect::<Vec<StyledGrapheme>>(),
    vec![
        StyledGrapheme::new("T", Style::new().green().on_yellow().italic()),
        StyledGrapheme::new("e", Style::new().green().on_yellow().italic()),
        StyledGrapheme::new("s", Style::new().green().on_yellow().italic()),
        StyledGrapheme::new("t", Style::new().green().on_yellow().italic()),
    ],
);
```

[`Color`]: crate::style::Color

```rust
fn styled_graphemes<S>(self: &'a Self, base_style: S) -> impl Trait
```



## ratatui_core::text::span::Span::width

*Function*

Returns the unicode width of the content held by this span.

```rust
fn width(self: &Self) -> usize
```



