**ratatui_widgets > paragraph**

# Module: paragraph

## Contents

**Structs**

- [`Paragraph`](#paragraph) - A widget to display some text.
- [`Wrap`](#wrap) - Describes how to wrap text across lines.

---

## ratatui_widgets::paragraph::Paragraph

*Struct*

A widget to display some text.

It is used to display a block of text. The text can be styled and aligned. It can also be
wrapped to the next line if it is too long to fit in the given area.

The text can be any type that can be converted into a [`Text`]. By default, the text is styled
with [`Style::default()`], not wrapped, and aligned to the left.

The text can be wrapped to the next line if it is too long to fit in the given area. The
wrapping can be configured with the [`wrap`] method. For more complex wrapping, consider using
the [Textwrap crate].

The text can be aligned to the left, right, or center. The alignment can be configured with the
[`alignment`] method or with the [`left_aligned`], [`right_aligned`], and [`centered`] methods.

The text can be scrolled to show a specific part of the text. The scroll offset can be set with
the [`scroll`] method.

The text can be surrounded by a [`Block`] with a title and borders. The block can be configured
with the [`block`] method.

The style of the text can be set with the [`style`] method. This style will be applied to the
entire widget, including the block if one is present. Any style set on the block or text will be
added to this style. See the [`Style`] type for more information on how styles are combined.

Note: If neither wrapping or a block is needed, consider rendering the [`Text`], [`Line`], or
[`Span`] widgets directly.

[Textwrap crate]: https://crates.io/crates/textwrap
[`wrap`]: Self::wrap
[`alignment`]: Self::alignment
[`left_aligned`]: Self::left_aligned
[`right_aligned`]: Self::right_aligned
[`centered`]: Self::centered
[`scroll`]: Self::scroll
[`block`]: Self::block
[`style`]: Self::style

# Example

```
use ratatui::layout::Alignment;
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph, Wrap};

let text = vec![
    Line::from(vec![
        Span::raw("First"),
        Span::styled("line", Style::new().green().italic()),
        ".".into(),
    ]),
    Line::from("Second line".red()),
    "Third line".into(),
];
Paragraph::new(text)
    .block(Block::bordered().title("Paragraph"))
    .style(Style::new().white().on_black())
    .alignment(Alignment::Center)
    .wrap(Wrap { trim: true });
```

[`Span`]: ratatui_core::text::Span

**Generic Parameters:**
- 'a

**Methods:**

- `fn new<T>(text: T) -> Self` - Creates a new [`Paragraph`] widget with the given text.
- `fn block(self: Self, block: Block<'a>) -> Self` - Surrounds the [`Paragraph`] widget with a [`Block`].
- `fn style<S>(self: Self, style: S) -> Self` - Sets the style of the entire widget.
- `fn wrap(self: Self, wrap: Wrap) -> Self` - Sets the wrapping configuration for the widget.
- `fn scroll(self: Self, offset: (u16, u16)) -> Self` - Set the scroll offset for the given paragraph
- `fn alignment(self: Self, alignment: Alignment) -> Self` - Set the text alignment for the given paragraph
- `fn left_aligned(self: Self) -> Self` - Left-aligns the text in the given paragraph.
- `fn centered(self: Self) -> Self` - Center-aligns the text in the given paragraph.
- `fn right_aligned(self: Self) -> Self` - Right-aligns the text in the given paragraph.
- `fn line_count(self: &Self, width: u16) -> usize` - Calculates the number of lines needed to fully render.
- `fn line_width(self: &Self) -> usize` - Calculates the shortest line width needed to avoid any word being wrapped or truncated.

**Traits:** Eq

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **AsRef**
  - `fn as_ref(self: &Self) -> &crate::paragraph::Paragraph<'a>`
- **Widget**
  - `fn render(self: Self, area: Rect, buf: & mut Buffer)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Styled**
  - `fn style(self: &Self) -> Style`
  - `fn set_style<S>(self: Self, style: S) -> <Self as >::Item`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## ratatui_widgets::paragraph::Wrap

*Struct*

Describes how to wrap text across lines.

## Examples

```
use ratatui::text::Text;
use ratatui::widgets::{Paragraph, Wrap};

let bullet_points = Text::from(
    r#"Some indented points:
    - First thing goes here and is long so that it wraps
    - Here is another point that is long enough to wrap"#,
);

// With leading spaces trimmed (window width of 30 chars):
Paragraph::new(bullet_points.clone()).wrap(Wrap { trim: true });
// Some indented points:
// - First thing goes here and is
// long so that it wraps
// - Here is another point that
// is long enough to wrap

// But without trimming, indentation is preserved:
Paragraph::new(bullet_points).wrap(Wrap { trim: false });
// Some indented points:
//     - First thing goes here
// and is long so that it wraps
//     - Here is another point
// that is long enough to wrap
```

**Fields:**
- `trim: bool` - Should leading whitespace be trimmed

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Default**
  - `fn default() -> Self`



