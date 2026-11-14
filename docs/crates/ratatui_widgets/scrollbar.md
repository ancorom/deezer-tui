**ratatui_widgets > scrollbar**

# Module: scrollbar

## Contents

**Structs**

- [`Scrollbar`](#scrollbar) - A widget to display a scrollbar
- [`ScrollbarState`](#scrollbarstate) - A struct representing the state of a Scrollbar widget.

**Enums**

- [`ScrollDirection`](#scrolldirection) - An enum representing a scrolling direction.
- [`ScrollbarOrientation`](#scrollbarorientation) - This is the position of the scrollbar around a given area.

---

## ratatui_widgets::scrollbar::ScrollDirection

*Enum*

An enum representing a scrolling direction.

This is used with [`ScrollbarState::scroll`].

It is useful for example when you want to store in which direction to scroll.

**Variants:**
- `Forward` - Forward scroll direction, usually corresponds to scrolling downwards or rightwards.
- `Backward` - Backward scroll direction, usually corresponds to scrolling upwards or leftwards.

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::result::Result<(), ::core::fmt::Error>`
- **FromStr**
  - `fn from_str(s: &str) -> ::core::result::Result<ScrollDirection, <Self as ::core::str::FromStr>::Err>`
- **TryFrom**
  - `fn try_from(s: &str) -> ::core::result::Result<ScrollDirection, <Self as ::core::convert::TryFrom>::Error>`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## ratatui_widgets::scrollbar::Scrollbar

*Struct*

A widget to display a scrollbar

The following components of the scrollbar are customizable in symbol and style. Note the
scrollbar is represented horizontally but it can also be set vertically (which is actually the
default).

```text
<--▮------->
^  ^   ^   ^
│  │   │   └ end
│  │   └──── track
│  └──────── thumb
└─────────── begin
```

# Important

You must specify the [`ScrollbarState::content_length`] before rendering the `Scrollbar`, or
else the `Scrollbar` will render blank.

# Examples

```rust
use ratatui::Frame;
use ratatui::layout::{Margin, Rect};
use ratatui::text::Line;
use ratatui::widgets::{
    Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget,
};

# fn render_paragraph_with_scrollbar(frame: &mut Frame, area: Rect) {
let vertical_scroll = 0; // from app state

let items = vec![
    Line::from("Item 1"),
    Line::from("Item 2"),
    Line::from("Item 3"),
];
let paragraph = Paragraph::new(items.clone())
    .scroll((vertical_scroll as u16, 0))
    .block(Block::new().borders(Borders::RIGHT)); // to show a background for the scrollbar

let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
    .begin_symbol(Some("↑"))
    .end_symbol(Some("↓"));

let mut scrollbar_state = ScrollbarState::new(items.len()).position(vertical_scroll);

let area = frame.area();
// Note we render the paragraph
frame.render_widget(paragraph, area);
// and the scrollbar, those are separate widgets
frame.render_stateful_widget(
    scrollbar,
    area.inner(Margin {
        // using an inner vertical margin of 1 unit makes the scrollbar inside the block
        vertical: 1,
        horizontal: 0,
    }),
    &mut scrollbar_state,
);
# }
```

**Generic Parameters:**
- 'a

**Methods:**

- `fn new(orientation: ScrollbarOrientation) -> Self` - Creates a new scrollbar with the given orientation.
- `fn orientation(self: Self, orientation: ScrollbarOrientation) -> Self` - Sets the position of the scrollbar.
- `fn orientation_and_symbol(self: Self, orientation: ScrollbarOrientation, symbols: Set<'a>) -> Self` - Sets the orientation and symbols for the scrollbar from a [`Set`].
- `fn thumb_symbol(self: Self, thumb_symbol: &'a str) -> Self` - Sets the symbol that represents the thumb of the scrollbar.
- `fn thumb_style<S>(self: Self, thumb_style: S) -> Self` - Sets the style on the scrollbar thumb.
- `fn track_symbol(self: Self, track_symbol: Option<&'a str>) -> Self` - Sets the symbol that represents the track of the scrollbar.
- `fn track_style<S>(self: Self, track_style: S) -> Self` - Sets the style that is used for the track of the scrollbar.
- `fn begin_symbol(self: Self, begin_symbol: Option<&'a str>) -> Self` - Sets the symbol that represents the beginning of the scrollbar.
- `fn begin_style<S>(self: Self, begin_style: S) -> Self` - Sets the style that is used for the beginning of the scrollbar.
- `fn end_symbol(self: Self, end_symbol: Option<&'a str>) -> Self` - Sets the symbol that represents the end of the scrollbar.
- `fn end_style<S>(self: Self, end_style: S) -> Self` - Sets the style that is used for the end of the scrollbar.
- `fn symbols(self: Self, symbols: Set<'a>) -> Self` - Sets the symbols used for the various parts of the scrollbar from a [`Set`].
- `fn style<S>(self: Self, style: S) -> Self` - Sets the style used for the various parts of the scrollbar from a [`Style`].

**Traits:** Eq

**Trait Implementations:**

- **AsRef**
  - `fn as_ref(self: &Self) -> &crate::scrollbar::Scrollbar<'a>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **StatefulWidget**
  - `fn render(self: Self, area: Rect, buf: & mut Buffer, state: & mut <Self as >::State)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## ratatui_widgets::scrollbar::ScrollbarOrientation

*Enum*

This is the position of the scrollbar around a given area.

```plain
          HorizontalTop
            ┌───────┐
VerticalLeft│       │VerticalRight
            └───────┘
         HorizontalBottom
```

**Variants:**
- `VerticalRight` - Positions the scrollbar on the right, scrolling vertically
- `VerticalLeft` - Positions the scrollbar on the left, scrolling vertically
- `HorizontalBottom` - Positions the scrollbar on the bottom, scrolling horizontally
- `HorizontalTop` - Positions the scrollbar on the top, scrolling horizontally

**Methods:**

- `fn is_vertical(self: &Self) -> bool` - Returns `true` if the scrollbar is vertical.
- `fn is_horizontal(self: &Self) -> bool` - Returns `true` if the scrollbar is horizontal.

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::result::Result<(), ::core::fmt::Error>`
- **FromStr**
  - `fn from_str(s: &str) -> ::core::result::Result<ScrollbarOrientation, <Self as ::core::str::FromStr>::Err>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **TryFrom**
  - `fn try_from(s: &str) -> ::core::result::Result<ScrollbarOrientation, <Self as ::core::convert::TryFrom>::Error>`
- **Default**
  - `fn default() -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ratatui_widgets::scrollbar::ScrollbarState

*Struct*

A struct representing the state of a Scrollbar widget.

# Important

It's essential to set the `content_length` field when using this struct. This field
represents the total length of the scrollable content. The default value is zero
which will result in the Scrollbar not rendering.

For example, in the following list, assume there are 4 bullet points:

- the `content_length` is 4
- the `position` is 0
- the `viewport_content_length` is 2

```text
┌───────────────┐
│1. this is a   █
│   single item █
│2. this is a   ║
│   second item ║
└───────────────┘
```

If you don't have multi-line content, you can leave the `viewport_content_length` set to the
default and it'll use the track size as a `viewport_content_length`.

**Methods:**

- `fn new(content_length: usize) -> Self` - Constructs a new [`ScrollbarState`] with the specified content length.
- `fn position(self: Self, position: usize) -> Self` - Sets the scroll position of the scrollbar.
- `fn content_length(self: Self, content_length: usize) -> Self` - Sets the length of the scrollable content.
- `fn viewport_content_length(self: Self, viewport_content_length: usize) -> Self` - Sets the items' size.
- `fn prev(self: & mut Self)` - Decrements the scroll position by one, ensuring it doesn't go below zero.
- `fn next(self: & mut Self)` - Increments the scroll position by one, ensuring it doesn't exceed the length of the content.
- `fn first(self: & mut Self)` - Sets the scroll position to the start of the scrollable content.
- `fn last(self: & mut Self)` - Sets the scroll position to the end of the scrollable content.
- `fn scroll(self: & mut Self, direction: ScrollDirection)` - Changes the scroll position based on the provided [`ScrollDirection`].
- `fn get_position(self: &Self) -> usize` - Returns the current position within the scrollable content.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



