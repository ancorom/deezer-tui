**ratatui_widgets > block**

# Module: block

## Contents

**Structs**

- [`Block`](#block) - A widget that renders borders, titles, and padding around other widgets.

**Enums**

- [`TitlePosition`](#titleposition) - Defines the position of the title.

**Traits**

- [`BlockExt`](#blockext) - An extension trait for [`Block`] that provides some convenience methods.

---

## ratatui_widgets::block::Block

*Struct*

A widget that renders borders, titles, and padding around other widgets.

A `Block` is a foundational widget that creates visual containers by drawing borders around an
area. It serves as a wrapper or frame for other widgets, providing structure and visual
separation in terminal UIs. Most built-in widgets in Ratatui use a pattern where they accept an
optional `Block` parameter that wraps the widget's content.

When a widget renders with a block, the widget's style is applied first, then the block's style,
and finally the widget's content is rendered within the inner area calculated by the block. This
layered approach allows for flexible styling where the block can provide background colors,
borders, and padding while the inner widget handles its own content styling.

Multiple blocks can be nested within each other. The [`Block::inner`] method calculates the area
available for content after accounting for borders, titles, and padding, making it easy to nest
blocks or position widgets within a block's boundaries.

# Constructor Methods

- [`Block::new`] - Creates a block with no borders or padding
- [`Block::bordered`] - Creates a block with all borders enabled

# Border Configuration

- [`Block::borders`] - Specifies which borders to display
- [`Block::border_style`] - Sets the style of the borders
- [`Block::border_type`] - Sets border symbols (single, double, thick, rounded, etc.)
- [`Block::border_set`] - Sets custom border symbols as a [`border::Set`]
- [`Block::merge_borders`] - Controls how borders merge with adjacent blocks

# Title Configuration

- [`Block::title`] - Adds a title to the block
- [`Block::title_top`] - Adds a title to the top of the block
- [`Block::title_bottom`] - Adds a title to the bottom of the block
- [`Block::title_alignment`] - Sets default alignment for all titles
- [`Block::title_style`] - Sets the style for all titles
- [`Block::title_position`] - Sets default position for titles

# Styling and Layout

- [`Block::style`] - Sets the base style of the block
- [`Block::shadow`] - Adds a shadow rendered behind the block
- [`Block::padding`] - Adds internal padding within the borders
- [`Block::inner`] - Calculates the inner area available for content

# Title Behavior

You can add multiple titles to a block, and they will be rendered with spaces separating titles
that share the same position or alignment. When both centered and non-centered titles exist, the
centered space is calculated based on the full width of the block.

Titles are set using the `.title`, `.title_top`, and `.title_bottom` methods. These methods
accept a string or any type that can be converted into a [`Line`], such as a string slice,
`String`, or a vector of [`Span`]s. To control the alignment of a title (left, center, right),
pass a `Line` with the desired alignment, e.g. `Line::from("Title").centered()`.

By default, `.title` places the title at the top of the block, but you can use `.title_top` or
`.title_bottom` to explicitly set the position. The default alignment for all titles can be set
with [`Block::title_alignment`], and the default position for all titles can be set with
[`Block::title_position`].

Note that prior to `v0.30.0`, the `block::Title` struct was used to create titles. This struct
has been removed. The new recommended approach is to use [`Line`] with a specific alignment for
the title's content and the [`Block::title_top`] and [`Block::title_bottom`] methods for
positioning.

Titles avoid being rendered in corners when borders are present, but will align to edges when no
border exists on that side:

```plain
┌With at least a left border───

Without left border───
```

# Nesting Widgets with `inner`

The [`Block::inner`] method computes the area inside the block after accounting for borders,
titles, and padding. This allows you to nest widgets inside a block by rendering the block
first, then rendering other widgets in the returned inner area.

For example, you can nest a block inside another block:

```
use ratatui::Frame;
use ratatui::widgets::Block;

# fn render_nested_block(frame: &mut Frame) {
let outer_block = Block::bordered().title("Outer");
let inner_block = Block::bordered().title("Inner");

let outer_area = frame.area();
let inner_area = outer_block.inner(outer_area);

frame.render_widget(outer_block, outer_area);
frame.render_widget(inner_block, inner_area);
# }
```

You can also use the standard [`Layout`] functionality to further subdivide the inner area and
lay out multiple widgets inside a block.

# Integration with Other Widgets

Most widgets in Ratatui accept a block parameter. For example, [`Paragraph`], [`List`],
[`Table`], and other widgets can be wrapped with a block:

```
use ratatui::widgets::{Block, Paragraph};

let paragraph = Paragraph::new("Hello, world!").block(Block::bordered().title("My Paragraph"));
```

This pattern allows widgets to focus on their content while blocks handle the visual framing.

# Styling

Styles are applied in a specific order: first the block's base style, then border styles, then
title styles, and finally any content widget styles. This layered approach allows for flexible
styling where outer styles provide defaults that inner styles can override.

`Block` implements [`Stylize`](ratatui_core::style::Stylize), allowing you to use style
shorthand methods:

```
use ratatui::style::Stylize;
use ratatui::widgets::Block;

let block = Block::bordered().red().on_white().bold();
```

# Examples

Create a simple bordered block:

```
use ratatui::widgets::Block;

let block = Block::bordered().title("My Block");
```

Create a block with custom border styling:

```
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, BorderType};

let block = Block::bordered()
    .title("Styled Block")
    .border_type(BorderType::Rounded)
    .border_style(Style::new().cyan())
    .style(Style::new().on_black());
```

Use a block to wrap another widget:

```
use ratatui::widgets::{Block, Paragraph};

let paragraph = Paragraph::new("Hello, world!").block(Block::bordered().title("Greeting"));
```

Add multiple titles with different alignments:

```
use ratatui::text::Line;
use ratatui::widgets::Block;

let block = Block::bordered()
    .title_top(Line::from("Left").left_aligned())
    .title_top(Line::from("Center").centered())
    .title_top(Line::from("Right").right_aligned())
    .title_bottom("Status: OK");
```

# See Also

- [Block recipe] - Visual examples and common patterns (on the ratatui website)
- [Collapse borders recipe] - Techniques for creating seamless layouts (on the ratatui website)
- [`MergeStrategy`] - Controls how borders merge with adjacent elements

[Block recipe]: https://ratatui.rs/recipes/widgets/block/
[Collapse borders recipe]: https://ratatui.rs/recipes/layout/collapse-borders/
[`Paragraph`]: crate::paragraph::Paragraph
[`Span`]: ratatui_core::text::Span
[`Table`]: crate::table::Table
[`Stylize`]: ratatui_core::style::Stylize
[`List`]: crate::list::List
[`Layout`]: ratatui_core::layout::Layout

**Generic Parameters:**
- 'a

**Methods:**

- `fn new() -> Self` - Creates a new block with no [`Borders`] or [`Padding`].
- `fn bordered() -> Self` - Create a new block with [all borders](Borders::ALL) shown
- `fn title<T>(self: Self, title: T) -> Self` - Adds a title to the block using the default position.
- `fn title_top<T>(self: Self, title: T) -> Self` - Adds a title to the top of the block.
- `fn title_bottom<T>(self: Self, title: T) -> Self` - Adds a title to the bottom of the block.
- `fn title_style<S>(self: Self, style: S) -> Self` - Applies the style to all titles.
- `fn title_alignment(self: Self, alignment: Alignment) -> Self` - Sets the default [`Alignment`] for all block titles.
- `fn title_position(self: Self, position: TitlePosition) -> Self` - Sets the default [`TitlePosition`] for all block titles.
- `fn border_style<S>(self: Self, style: S) -> Self` - Defines the style of the borders.
- `fn style<S>(self: Self, style: S) -> Self` - Defines the style of the entire block.
- `fn borders(self: Self, flag: Borders) -> Self` - Defines which borders to display.
- `fn border_type(self: Self, border_type: BorderType) -> Self` - Sets the symbols used to display the border (e.g. single line, double line, thick or
- `fn border_set(self: Self, border_set: border::Set<'a>) -> Self` - Sets the symbols used to display the border as a [`ratatui_core::symbols::border::Set`].
- `fn padding(self: Self, padding: Padding) -> Self` - Defines the padding inside a `Block`.
- `fn merge_borders(self: Self, strategy: MergeStrategy) -> Self` - Sets the block's [`MergeStrategy`] for overlapping characters, defaulting to [`Replace`].
- `fn shadow(self: Self, shadow: Shadow) -> Self` - Adds a shadow behind the block.
- `fn inner(self: &Self, area: Rect) -> Rect` - Computes the inner area of a block after subtracting space for borders, titles, and padding.

**Traits:** Eq

**Trait Implementations:**

- **AsRef**
  - `fn as_ref(self: &Self) -> &crate::block::Block<'a>`
- **Widget**
  - `fn render(self: Self, area: Rect, buf: & mut Buffer)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Styled**
  - `fn style(self: &Self) -> Style`
  - `fn set_style<S>(self: Self, style: S) -> <Self as >::Item`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Default**
  - `fn default() -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ratatui_widgets::block::BlockExt

*Trait*

An extension trait for [`Block`] that provides some convenience methods.

This is implemented for [`Option<Block>`](Option) to simplify the common case of having a
widget with an optional block.

**Methods:**

- `inner_if_some`: Return the inner area of the block if it is `Some`. Otherwise, returns `area`.



## ratatui_widgets::block::TitlePosition

*Enum*

Defines the position of the title.

The title can be positioned on top or at the bottom of the block.

# Example

```
use ratatui::widgets::{Block, TitlePosition};

Block::bordered()
    .title_position(TitlePosition::Top)
    .title("Top Title");
Block::bordered()
    .title_position(TitlePosition::Bottom)
    .title("Bottom Title");
```

**Variants:**
- `Top` - Position the title at the top of the block.
- `Bottom` - Position the title at the bottom of the block.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **FromStr**
  - `fn from_str(s: &str) -> ::core::result::Result<TitlePosition, <Self as ::core::str::FromStr>::Err>`
- **Default**
  - `fn default() -> Self`
- **TryFrom**
  - `fn try_from(s: &str) -> ::core::result::Result<TitlePosition, <Self as ::core::convert::TryFrom>::Error>`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::result::Result<(), ::core::fmt::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



