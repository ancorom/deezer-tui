**ratatui_core > text > line**

# Module: text::line

## Contents

**Structs**

- [`Line`](#line) - A line of text, consisting of one or more [`Span`]s.

**Traits**

- [`ToLine`](#toline) - A trait for converting a value to a [`Line`].

---

## ratatui_core::text::line::Line

*Struct*

A line of text, consisting of one or more [`Span`]s.

[`Line`]s are used wherever text is displayed in the terminal and represent a single line of
text. When a [`Line`] is rendered, it is rendered as a single line of text, with each [`Span`]
being rendered in order (left to right).

Any newlines in the content are removed when creating a [`Line`] using the constructor or
conversion methods.

# Constructor Methods

- [`Line::default`] creates a line with empty content and the default style.
- [`Line::raw`] creates a line with the given content and the default style.
- [`Line::styled`] creates a line with the given content and style.

# Conversion Methods

- [`Line::from`] creates a `Line` from a [`String`].
- [`Line::from`] creates a `Line` from a [`&str`].
- [`Line::from`] creates a `Line` from a [`Vec`] of [`Span`]s.
- [`Line::from`] creates a `Line` from a `&[Into<Span>]`.
- [`Line::from`] creates a `Line` from single [`Span`].
- [`String::from`] converts a line into a [`String`].
- [`Line::from_iter`] creates a line from an iterator of items that are convertible to [`Span`].

# Setter Methods

These methods are fluent setters. They return a `Line` with the property set.

- [`Line::spans`] sets the content of the line.
- [`Line::style`] sets the style of the line.
- [`Line::alignment`] sets the alignment of the line.
- [`Line::left_aligned`] sets the alignment of the line to [`Alignment::Left`].
- [`Line::centered`] sets the alignment of the line to [`Alignment::Center`].
- [`Line::right_aligned`] sets the alignment of the line to [`Alignment::Right`].

# Iteration Methods

- [`Line::iter`] returns an iterator over the spans of this line.
- [`Line::iter_mut`] returns a mutable iterator over the spans of this line.
- [`Line::into_iter`] returns an iterator over the spans of this line.

# Other Methods

- [`Line::patch_style`] patches the style of the line, adding modifiers from the given style.
- [`Line::reset_style`] resets the style of the line.
- [`Line::width`] returns the unicode width of the content held by this line.
- [`Line::styled_graphemes`] returns an iterator over the graphemes held by this line.
- [`Line::push_span`] adds a span to the line.

# Compatibility Notes

Before v0.26.0, [`Line`] did not have a `style` field and instead relied on only the styles that
were set on each [`Span`] contained in the `spans` field. The [`Line::patch_style`] method was
the only way to set the overall style for individual lines. For this reason, this field may not
be supported yet by all widgets (outside of the `ratatui` crate itself).

# Examples

## Creating Lines
[`Line`]s can be created from [`Span`]s, [`String`]s, and [`&str`]s. They can be styled with a
[`Style`].

```rust
use ratatui_core::style::{Color, Modifier, Style, Stylize};
use ratatui_core::text::{Line, Span};

let style = Style::new().yellow();
let line = Line::raw("Hello, world!").style(style);
let line = Line::styled("Hello, world!", style);
let line = Line::styled("Hello, world!", (Color::Yellow, Modifier::BOLD));

let line = Line::from("Hello, world!");
let line = Line::from(String::from("Hello, world!"));
let line = Line::from(vec![
    Span::styled("Hello", Style::new().blue()),
    Span::raw(" world!"),
]);
```

## Styling Lines

The line's [`Style`] is used by the rendering widget to determine how to style the line. Each
[`Span`] in the line will be styled with the [`Style`] of the line, and then with its own
[`Style`]. If the line is longer than the available space, the style is applied to the entire
line, and the line is truncated. `Line` also implements [`Styled`] which means you can use the
methods of the [`Stylize`] trait.

```rust
use ratatui_core::style::{Color, Modifier, Style, Stylize};
use ratatui_core::text::Line;

let line = Line::from("Hello world!").style(Style::new().yellow().italic());
let line = Line::from("Hello world!").style(Color::Yellow);
let line = Line::from("Hello world!").style((Color::Yellow, Color::Black));
let line = Line::from("Hello world!").style((Color::Yellow, Modifier::ITALIC));
let line = Line::from("Hello world!").yellow().italic();
```

## Aligning Lines

The line's [`Alignment`] is used by the rendering widget to determine how to align the line
within the available space. If the line is longer than the available space, the alignment is
ignored and the line is truncated.

```rust
use ratatui_core::layout::Alignment;
use ratatui_core::text::Line;

let line = Line::from("Hello world!").alignment(Alignment::Right);
let line = Line::from("Hello world!").centered();
let line = Line::from("Hello world!").left_aligned();
let line = Line::from("Hello world!").right_aligned();
```

## Rendering Lines

`Line` implements the [`Widget`] trait, which means it can be rendered to a [`Buffer`].

```rust
use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;
use ratatui_core::style::{Style, Stylize};
use ratatui_core::text::Line;
use ratatui_core::widgets::Widget;

# fn render(area: Rect, buf: &mut Buffer) {
// in another widget's render method
let line = Line::from("Hello world!").style(Style::new().yellow().italic());
line.render(area, buf);
# }
```

Or you can use the `render_widget` method on the `Frame` in a `Terminal::draw` closure.

```rust,ignore
# use ratatui::{Frame, layout::Rect, text::Line};
# fn draw(frame: &mut Frame, area: Rect) {
let line = Line::from("Hello world!");
frame.render_widget(line, area);
# }
```
## Rendering Lines with a Paragraph widget

Usually apps will use the `Paragraph` widget instead of rendering a [`Line`] directly as it
provides more functionality.

```rust,ignore
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    text::Line,
    widgets::{Paragraph, Widget, Wrap},
};

# fn render(area: Rect, buf: &mut Buffer) {
let line = Line::from("Hello world!").yellow().italic();
Paragraph::new(line)
    .wrap(Wrap { trim: true })
    .render(area, buf);
# }
```

[`Stylize`]: crate::style::Stylize

**Generic Parameters:**
- 'a

**Fields:**
- `style: crate::style::Style` - The style of this line of text.
- `alignment: Option<crate::layout::Alignment>` - The alignment of this line of text.
- `spans: alloc::vec::Vec<crate::text::Span<'a>>` - The spans that make up this line of text.

**Methods:**

- `fn raw<T>(content: T) -> Self` - Create a line with the default style.
- `fn styled<T, S>(content: T, style: S) -> Self` - Create a line with the given style.
- `fn spans<I>(self: Self, spans: I) -> Self` - Sets the spans of this line of text.
- `fn style<S>(self: Self, style: S) -> Self` - Sets the style of this line of text.
- `fn alignment(self: Self, alignment: Alignment) -> Self` - Sets the target alignment for this line of text.
- `fn left_aligned(self: Self) -> Self` - Left-aligns this line of text.
- `fn centered(self: Self) -> Self` - Center-aligns this line of text.
- `fn right_aligned(self: Self) -> Self` - Right-aligns this line of text.
- `fn width(self: &Self) -> usize` - Returns the width of the underlying string.
- `fn styled_graphemes<S>(self: &'a Self, base_style: S) -> impl Trait` - Returns an iterator over the graphemes held by this line.
- `fn patch_style<S>(self: Self, style: S) -> Self` - Patches the style of this Line, adding modifiers from the given style.
- `fn reset_style(self: Self) -> Self` - Resets the style of this Line.
- `fn iter(self: &Self) -> core::slice::Iter<Span<'a>>` - Returns an iterator over the spans of this line.
- `fn iter_mut(self: & mut Self) -> core::slice::IterMut<Span<'a>>` - Returns a mutable iterator over the spans of this line.
- `fn push_span<T>(self: & mut Self, span: T)` - Adds a span to the line.

**Traits:** Eq

**Trait Implementations:**

- **UnicodeWidthStr**
  - `fn width(self: &Self) -> usize`
  - `fn width_cjk(self: &Self) -> usize`
- **From**
  - `fn from(spans: Vec<Span<'a>>) -> Self`
- **Widget**
  - `fn render(self: Self, area: Rect, buf: & mut Buffer)`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Default**
  - `fn default() -> Self`
- **From**
  - `fn from(s: Cow<'a, str>) -> Self`
- **Extend**
  - `fn extend<T>(self: & mut Self, iter: T)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`
- **From**
  - `fn from(span: Span<'a>) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **From**
  - `fn from(s: &'a str) -> Self`
- **AddAssign**
  - `fn add_assign(self: & mut Self, rhs: Span<'a>)`
- **FromIterator**
  - `fn from_iter<I>(iter: I) -> Self`
- **Add**
  - `fn add(self: Self, rhs: Span<'a>) -> <Self as >::Output`
- **From**
  - `fn from(value: &'b [T]) -> Self`
- **From**
  - `fn from(s: String) -> Self`
- **Add**
  - `fn add(self: Self, rhs: Self) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Styled**
  - `fn style(self: &Self) -> Style`
  - `fn set_style<S>(self: Self, style: S) -> <Self as >::Item`



## ratatui_core::text::line::ToLine

*Trait*

A trait for converting a value to a [`Line`].

This trait is automatically implemented for any type that implements the [`Display`] trait. As
such, `ToLine` shouldn't be implemented directly: [`Display`] should be implemented instead, and
you get the `ToLine` implementation for free.

[`Display`]: std::fmt::Display

**Methods:**

- `to_line`: Converts the value to a [`Line`].



