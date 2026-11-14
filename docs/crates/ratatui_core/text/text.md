**ratatui_core > text > text**

# Module: text::text

## Contents

**Structs**

- [`Text`](#text) - A string split over one or more lines.

**Traits**

- [`ToText`](#totext) - A trait for converting a value to a [`Text`].

---

## ratatui_core::text::text::Text

*Struct*

A string split over one or more lines.

[`Text`] is used wherever text is displayed in the terminal and represents one or more [`Line`]s
of text. When a [`Text`] is rendered, each line is rendered as a single line of text from top to
bottom of the area. The text can be styled and aligned.

# Constructor Methods

- [`Text::raw`] creates a `Text` (potentially multiple lines) with no style.
- [`Text::styled`] creates a `Text` (potentially multiple lines) with a style.
- [`Text::default`] creates a `Text` with empty content and the default style.

# Conversion Methods

- [`Text::from`] creates a `Text` from a `String`.
- [`Text::from`] creates a `Text` from a `&str`.
- [`Text::from`] creates a `Text` from a `Cow<str>`.
- [`Text::from`] creates a `Text` from a [`Span`].
- [`Text::from`] creates a `Text` from a [`Line`].
- [`Text::from`] creates a `Text` from a `Vec<Line>`.
- [`Text::from`] creates a `Text` from a `&[Into<Line>]`.
- [`Text::from_iter`] creates a `Text` from an iterator of items that can be converted into
  `Line`.

# Setter Methods

These methods are fluent setters. They return a `Text` with the property set.

- [`Text::style`] sets the style of this `Text`.
- [`Text::alignment`] sets the alignment for this `Text`.
- [`Text::left_aligned`] sets the alignment to [`Alignment::Left`].
- [`Text::centered`] sets the alignment to [`Alignment::Center`].
- [`Text::right_aligned`] sets the alignment to [`Alignment::Right`].

# Iteration Methods

- [`Text::iter`] returns an iterator over the lines of the text.
- [`Text::iter_mut`] returns an iterator that allows modifying each line.
- [`Text::into_iter`] returns an iterator over the lines of the text.

# Other Methods

- [`Text::width`] returns the max width of all the lines.
- [`Text::height`] returns the height.
- [`Text::patch_style`] patches the style of this `Text`, adding modifiers from the given style.
- [`Text::reset_style`] resets the style of the `Text`.
- [`Text::push_line`] adds a line to the text.
- [`Text::push_span`] adds a span to the last line of the text.

# Examples

## Creating Text

A [`Text`], like a [`Line`], can be constructed using one of the many `From` implementations or
via the [`Text::raw`] and [`Text::styled`] methods. Helpfully, [`Text`] also implements
[`core::iter::Extend`] which enables the concatenation of several [`Text`] blocks.

```rust
use std::borrow::Cow;
use std::iter;

use ratatui_core::style::{Color, Modifier, Style, Stylize};
use ratatui_core::text::{Line, Span, Text};

let style = Style::new().yellow().italic();
let text = Text::raw("The first line\nThe second line").style(style);
let text = Text::styled("The first line\nThe second line", style);
let text = Text::styled(
    "The first line\nThe second line",
    (Color::Yellow, Modifier::ITALIC),
);

let text = Text::from("The first line\nThe second line");
let text = Text::from(String::from("The first line\nThe second line"));
let text = Text::from(Cow::Borrowed("The first line\nThe second line"));
let text = Text::from(Span::styled("The first line\nThe second line", style));
let text = Text::from(Line::from("The first line"));
let text = Text::from(vec![
    Line::from("The first line"),
    Line::from("The second line"),
]);
let text = Text::from_iter(iter::once("The first line").chain(iter::once("The second line")));

let mut text = Text::default();
text.extend(vec![
    Line::from("The first line"),
    Line::from("The second line"),
]);
text.extend(Text::from("The third line\nThe fourth line"));
```

## Styling Text

The text's [`Style`] is used by the rendering widget to determine how to style the text. Each
[`Line`] in the text will be styled with the [`Style`] of the text, and then with its own
[`Style`]. `Text` also implements [`Styled`] which means you can use the methods of the
[`Stylize`] trait.

```rust
use ratatui_core::style::{Color, Modifier, Style, Stylize};
use ratatui_core::text::{Line, Text};

let text = Text::from("The first line\nThe second line").style(Style::new().yellow().italic());
let text = Text::from("The first line\nThe second line")
    .yellow()
    .italic();
let text = Text::from(vec![
    Line::from("The first line").yellow(),
    Line::from("The second line").yellow(),
])
.italic();
```

## Aligning Text
The text's [`Alignment`] can be set using [`Text::alignment`] or the related helper methods.
Lines composing the text can also be individually aligned with [`Line::alignment`].

```rust
use ratatui_core::layout::Alignment;
use ratatui_core::text::{Line, Text};

let text = Text::from("The first line\nThe second line").alignment(Alignment::Right);
let text = Text::from("The first line\nThe second line").right_aligned();
let text = Text::from(vec![
    Line::from("The first line").left_aligned(),
    Line::from("The second line").right_aligned(),
    Line::from("The third line"),
])
.centered();
```

## Rendering Text
`Text` implements the [`Widget`] trait, which means it can be rendered to a [`Buffer`] or to a
`Frame`.

```rust
# use ratatui_core::{buffer::Buffer, layout::Rect};
use ratatui_core::text::Text;
use ratatui_core::widgets::Widget;

// within another widget's `render` method:
# fn render(area: Rect, buf: &mut Buffer) {
let text = Text::from("The first line\nThe second line");
text.render(area, buf);
# }
```

Or you can use the `render_widget` method on a `Frame` within a `Terminal::draw` closure.

```rust,ignore
# use ratatui::{Frame, layout::Rect, text::Text};
# fn draw(frame: &mut Frame, area: Rect) {
let text = Text::from("The first line\nThe second line");
frame.render_widget(text, area);
# }
```

## Rendering Text with a Paragraph Widget

Usually apps will use the `Paragraph` widget instead of rendering a `Text` directly as it
provides more functionality.

```rust,ignore
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Text,
    widgets::{Paragraph, Widget, Wrap},
};

# fn render(area: Rect, buf: &mut Buffer) {
let text = Text::from("The first line\nThe second line");
let paragraph = Paragraph::new(text)
    .wrap(Wrap { trim: true })
    .scroll((1, 1))
    .render(area, buf);
# }
```

[`Stylize`]: crate::style::Stylize

**Generic Parameters:**
- 'a

**Fields:**
- `alignment: Option<crate::layout::Alignment>` - The alignment of this text.
- `style: crate::style::Style` - The style of this text.
- `lines: alloc::vec::Vec<crate::text::Line<'a>>` - The lines that make up this piece of text.

**Methods:**

- `fn raw<T>(content: T) -> Self` - Create some text (potentially multiple lines) with no style.
- `fn styled<T, S>(content: T, style: S) -> Self` - Create some text (potentially multiple lines) with a style.
- `fn width(self: &Self) -> usize` - Returns the max width of all the lines.
- `fn height(self: &Self) -> usize` - Returns the height.
- `fn style<S>(self: Self, style: S) -> Self` - Sets the style of this text.
- `fn patch_style<S>(self: Self, style: S) -> Self` - Patches the style of this Text, adding modifiers from the given style.
- `fn reset_style(self: Self) -> Self` - Resets the style of the Text.
- `fn alignment(self: Self, alignment: Alignment) -> Self` - Sets the alignment for this text.
- `fn left_aligned(self: Self) -> Self` - Left-aligns the whole text.
- `fn centered(self: Self) -> Self` - Center-aligns the whole text.
- `fn right_aligned(self: Self) -> Self` - Right-aligns the whole text.
- `fn iter(self: &Self) -> core::slice::Iter<Line<'a>>` - Returns an iterator over the lines of the text.
- `fn iter_mut(self: & mut Self) -> core::slice::IterMut<Line<'a>>` - Returns an iterator that allows modifying each line.
- `fn push_line<T>(self: & mut Self, line: T)` - Adds a line to the text.
- `fn push_span<T>(self: & mut Self, span: T)` - Adds a span to the last line of the text.

**Traits:** Eq

**Trait Implementations:**

- **From**
  - `fn from(lines: Vec<Line<'a>>) -> Self`
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`
- **FromIterator**
  - `fn from_iter<I>(iter: I) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Default**
  - `fn default() -> Self`
- **Widget**
  - `fn render(self: Self, area: Rect, buf: & mut Buffer)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, line: Line<'a>)`
- **Add**
  - `fn add(self: Self, text: Self) -> <Self as >::Output`
- **From**
  - `fn from(s: Cow<'a, str>) -> Self`
- **From**
  - `fn from(masked: Masked<'a>) -> Self`
- **From**
  - `fn from(line: Line<'a>) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Add**
  - `fn add(self: Self, line: Line<'a>) -> <Self as >::Output`
- **From**
  - `fn from(s: &'a str) -> Self`
- **From**
  - `fn from(masked: &'a Masked) -> Self`
- **Extend**
  - `fn extend<I>(self: & mut Self, iter: I)`
- **From**
  - `fn from(value: &'b [T]) -> Self`
- **From**
  - `fn from(s: String) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **From**
  - `fn from(span: Span<'a>) -> Self`
- **Styled**
  - `fn style(self: &Self) -> Style`
  - `fn set_style<S>(self: Self, style: S) -> <Self as >::Item`
- **AddAssign**
  - `fn add_assign(self: & mut Self, rhs: Self)`
- **UnicodeWidthStr**
  - `fn width(self: &Self) -> usize` - Returns the max width of all the lines.
  - `fn width_cjk(self: &Self) -> usize`



## ratatui_core::text::text::ToText

*Trait*

A trait for converting a value to a [`Text`].

This trait is automatically implemented for any type that implements the [`Display`] trait. As
such, `ToText` shouldn't be implemented directly: [`Display`] should be implemented instead, and
you get the `ToText` implementation for free.

[`Display`]: std::fmt::Display

**Methods:**

- `to_text`: Converts the value to a [`Text`].



