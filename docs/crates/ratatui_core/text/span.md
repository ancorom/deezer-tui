**ratatui_core > text > span**

# Module: text::span

## Contents

**Structs**

- [`Span`](#span) - Represents a part of a line that is contiguous and where all characters share the same style.

**Traits**

- [`ToSpan`](#tospan) - A trait for converting a value to a [`Span`].

---

## ratatui_core::text::span::Span

*Struct*

Represents a part of a line that is contiguous and where all characters share the same style.

A `Span` is the smallest unit of text that can be styled. It is usually combined in the [`Line`]
type to represent a line of text where each `Span` may have a different style.

# Constructor Methods

- [`Span::default`] creates an span with empty content and the default style.
- [`Span::raw`] creates an span with the specified content and the default style.
- [`Span::styled`] creates an span with the specified content and style.

# Setter Methods

These methods are fluent setters. They return a new `Span` with the specified property set.

- [`Span::content`] sets the content of the span.
- [`Span::style`] sets the style of the span.

# Other Methods

- [`Span::patch_style`] patches the style of the span, adding modifiers from the given style.
- [`Span::reset_style`] resets the style of the span.
- [`Span::width`] returns the unicode width of the content held by this span.
- [`Span::styled_graphemes`] returns an iterator over the graphemes held by this span.

# Examples

A `Span` with `style` set to [`Style::default()`] can be created from a `&str`, a `String`, or
any type convertible to [`Cow<str>`].

```rust
use ratatui_core::text::Span;

let span = Span::raw("test content");
let span = Span::raw(String::from("test content"));
let span = Span::from("test content");
let span = Span::from(String::from("test content"));
let span: Span = "test content".into();
let span: Span = String::from("test content").into();
```

Styled spans can be created using [`Span::styled`] or by converting strings using methods from
the [`Stylize`] trait.

```rust
use ratatui_core::style::{Style, Stylize};
use ratatui_core::text::Span;

let span = Span::styled("test content", Style::new().green());
let span = Span::styled(String::from("test content"), Style::new().green());

// using Stylize trait shortcuts
let span = "test content".green();
let span = String::from("test content").green();
```

`Span` implements the [`Styled`] trait, which allows it to be styled using the shortcut methods
defined in the [`Stylize`] trait.

```rust
use ratatui_core::style::Stylize;
use ratatui_core::text::Span;

let span = Span::raw("test content").green().on_yellow().italic();
let span = Span::raw(String::from("test content"))
    .green()
    .on_yellow()
    .italic();
```

`Span` implements the [`Widget`] trait, which allows it to be rendered to a [`Buffer`]. Often
apps will use the `Paragraph` widget instead of rendering `Span` directly, as it handles text
wrapping and alignment for you.

```rust,ignore
use ratatui::{style::Stylize, Frame};

# fn render_frame(frame: &mut Frame) {
frame.render_widget("test content".green().on_yellow().italic(), frame.area());
# }
```
[`Line`]: crate::text::Line
[`Stylize`]: crate::style::Stylize
[`Cow<str>`]: std::borrow::Cow

**Generic Parameters:**
- 'a

**Fields:**
- `style: crate::style::Style` - The style of the span.
- `content: alloc::borrow::Cow<'a, str>` - The content of the span as a Clone-on-write string.

**Methods:**

- `fn raw<T>(content: T) -> Self` - Create a span with the default style.
- `fn styled<T, S>(content: T, style: S) -> Self` - Create a span with the specified style.
- `fn content<T>(self: Self, content: T) -> Self` - Sets the content of the span.
- `fn style<S>(self: Self, style: S) -> Self` - Sets the style of the span.
- `fn patch_style<S>(self: Self, style: S) -> Self` - Patches the style of the Span, adding modifiers from the given style.
- `fn reset_style(self: Self) -> Self` - Resets the style of the Span.
- `fn width(self: &Self) -> usize` - Returns the unicode width of the content held by this span.
- `fn styled_graphemes<S>(self: &'a Self, base_style: S) -> impl Trait` - Returns an iterator over the graphemes held by this span.
- `fn into_left_aligned_line(self: Self) -> Line<'a>` - Converts this Span into a left-aligned [`Line`]
- `fn to_left_aligned_line(self: Self) -> Line<'a>`
- `fn into_centered_line(self: Self) -> Line<'a>` - Converts this Span into a center-aligned [`Line`]
- `fn to_centered_line(self: Self) -> Line<'a>`
- `fn into_right_aligned_line(self: Self) -> Line<'a>` - Converts this Span into a right-aligned [`Line`]
- `fn to_right_aligned_line(self: Self) -> Line<'a>`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **UnicodeWidthStr**
  - `fn width(self: &Self) -> usize`
  - `fn width_cjk(self: &Self) -> usize`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Default**
  - `fn default() -> Self`
- **Styled**
  - `fn style(self: &Self) -> Style`
  - `fn set_style<S>(self: Self, style: S) -> <Self as >::Item`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Widget**
  - `fn render(self: Self, area: Rect, buf: & mut Buffer)`
- **From**
  - `fn from(s: T) -> Self`
- **Add**
  - `fn add(self: Self, rhs: Self) -> <Self as >::Output`



## ratatui_core::text::span::ToSpan

*Trait*

A trait for converting a value to a [`Span`].

This trait is automatically implemented for any type that implements the [`Display`] trait. As
such, `ToSpan` shouldn't be implemented directly: [`Display`] should be implemented instead, and
you get the `ToSpan` implementation for free.

[`Display`]: std::fmt::Display

**Methods:**

- `to_span`: Converts the value to a [`Span`].



