**ratatui_widgets > block > Block**

# Module: block::Block

## Contents

**Functions**

- [`border_set`](#border_set) - Sets the symbols used to display the border as a [`ratatui_core::symbols::border::Set`].
- [`border_style`](#border_style) - Defines the style of the borders.
- [`border_type`](#border_type) - Sets the symbols used to display the border (e.g. single line, double line, thick or
- [`bordered`](#bordered) - Create a new block with [all borders](Borders::ALL) shown
- [`borders`](#borders) - Defines which borders to display.
- [`inner`](#inner) - Computes the inner area of a block after subtracting space for borders, titles, and padding.
- [`merge_borders`](#merge_borders) - Sets the block's [`MergeStrategy`] for overlapping characters, defaulting to [`Replace`].
- [`new`](#new) - Creates a new block with no [`Borders`] or [`Padding`].
- [`padding`](#padding) - Defines the padding inside a `Block`.
- [`shadow`](#shadow) - Adds a shadow behind the block.
- [`style`](#style) - Defines the style of the entire block.
- [`title`](#title) - Adds a title to the block using the default position.
- [`title_alignment`](#title_alignment) - Sets the default [`Alignment`] for all block titles.
- [`title_bottom`](#title_bottom) - Adds a title to the bottom of the block.
- [`title_position`](#title_position) - Sets the default [`TitlePosition`] for all block titles.
- [`title_style`](#title_style) - Applies the style to all titles.
- [`title_top`](#title_top) - Adds a title to the top of the block.

---

## ratatui_widgets::block::Block::border_set

*Function*

Sets the symbols used to display the border as a [`ratatui_core::symbols::border::Set`].

Setting this overwrites any [`border_type`](Block::border_type) that was set.

# Examples

```
use ratatui::{widgets::Block, symbols};

Block::bordered().border_set(symbols::border::DOUBLE).title("Block");
// Renders
// ╔Block╗
// ║     ║
// ╚═════╝

```rust
fn border_set(self: Self, border_set: border::Set<'a>) -> Self
```



## ratatui_widgets::block::Block::border_style

*Function*

Defines the style of the borders.

This style is applied only to the areas covered by borders, and is applied to the block
after any [`Block::style`] is applied.

See [`Style`] for more information on how merging styles works.

`style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
your own type that implements [`Into<Style>`]).

# Example

This example shows a `Block` with blue borders.
```
use ratatui::style::{Style, Stylize};
use ratatui::widgets::Block;
Block::bordered().border_style(Style::new().blue());
```

[`Color`]: ratatui_core::style::Color

```rust
fn border_style<S>(self: Self, style: S) -> Self
```



## ratatui_widgets::block::Block::border_type

*Function*

Sets the symbols used to display the border (e.g. single line, double line, thick or
rounded borders).

Setting this overwrites any custom [`border_set`](Block::border_set) that was set.

See [`BorderType`] for the full list of available symbols.

# Examples

```
use ratatui::widgets::{Block, BorderType};
Block::bordered()
    .border_type(BorderType::Rounded)
    .title("Block");
// Renders
// ╭Block╮
// │     │
// ╰─────╯
```

```rust
fn border_type(self: Self, border_type: BorderType) -> Self
```



## ratatui_widgets::block::Block::bordered

*Function*

Create a new block with [all borders](Borders::ALL) shown

```
use ratatui::widgets::{Block, Borders};

assert_eq!(Block::bordered(), Block::new().borders(Borders::ALL));
```

```rust
fn bordered() -> Self
```



## ratatui_widgets::block::Block::borders

*Function*

Defines which borders to display.

[`Borders`] can also be styled with [`Block::border_style`] and [`Block::border_type`].

# Examples

Display left and right borders.
```
use ratatui::widgets::{Block, Borders};
Block::new().borders(Borders::LEFT | Borders::RIGHT);
```

To show all borders you can abbreviate this with [`Block::bordered`]

```rust
fn borders(self: Self, flag: Borders) -> Self
```



## ratatui_widgets::block::Block::inner

*Function*

Computes the inner area of a block after subtracting space for borders, titles, and padding.

# Examples

Draw a block nested within another block
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
// Renders
// ┌Outer────────┐
// │┌Inner──────┐│
// ││           ││
// │└───────────┘│
// └─────────────┘
```

```rust
fn inner(self: &Self, area: Rect) -> Rect
```



## ratatui_widgets::block::Block::merge_borders

*Function*

Sets the block's [`MergeStrategy`] for overlapping characters, defaulting to [`Replace`].

Changing the strategy to [`Exact`] or [`Fuzzy`] collapses border characters that intersect
with any previously rendered borders.

For more information and examples, see the [collapse borders recipe] and [`MergeStrategy`]
docs.

# Example

```
use ratatui::symbols::merge::MergeStrategy;
# use ratatui::widgets::{Block, BorderType};

// Given several blocks with plain borders (1)
Block::bordered();
// and other blocks with thick borders (2) which are rendered on top of the first
Block::bordered()
    .border_type(BorderType::Thick)
    .merge_borders(MergeStrategy::Exact);
```

Rendering these blocks with `MergeStrategy::Exact` or `MergeStrategy::Fuzzy` will collapse
the borders, resulting in a clean layout without connected borders.

```plain
┌───┐    ┌───┐  ┌───┲━━━┓┌───┐
│   │    │ 1 │  │   ┃   ┃│   │
│ 1 │    │ ┏━┿━┓│ 1 ┃ 2 ┃│ 1 │
│   │    │ ┃ │ ┃│   ┃   ┃│   │
└───╆━━━┓└─╂─┘ ┃└───┺━━━┛┢━━━┪
    ┃   ┃  ┃ 2 ┃         ┃   ┃
    ┃ 2 ┃  ┗━━━┛         ┃ 2 ┃
    ┃   ┃                ┃   ┃
    ┗━━━┛                ┗━━━┛
```

[collapse borders recipe]: https://ratatui.rs/recipes/layout/collapse-borders/
[`Replace`]: MergeStrategy::Replace
[`Exact`]: MergeStrategy::Exact
[`Fuzzy`]: MergeStrategy::Fuzzy

```rust
fn merge_borders(self: Self, strategy: MergeStrategy) -> Self
```



## ratatui_widgets::block::Block::new

*Function*

Creates a new block with no [`Borders`] or [`Padding`].

```rust
fn new() -> Self
```



## ratatui_widgets::block::Block::padding

*Function*

Defines the padding inside a `Block`.

See [`Padding`] for more information.

# Examples

This renders a `Block` with no padding (the default).
```
use ratatui::widgets::{Block, Padding};

Block::bordered().padding(Padding::ZERO);
// Renders
// ┌───────┐
// │content│
// └───────┘
```

This example shows a `Block` with padding left and right ([`Padding::horizontal`]).
Notice the two spaces before and after the content.
```
use ratatui::widgets::{Block, Padding};

Block::bordered().padding(Padding::horizontal(2));
// Renders
// ┌───────────┐
// │  content  │
// └───────────┘
```

```rust
fn padding(self: Self, padding: Padding) -> Self
```



## ratatui_widgets::block::Block::shadow

*Function*

Adds a shadow behind the block.

The shadow is rendered using the block area plus the shadow's configured offset.

# Example

```
use ratatui::layout::Offset;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Shadow};

let block = Block::bordered().title("Popup").shadow(
    Shadow::dark_shade()
        .black()
        .on_white()
        .offset(Offset::new(2, 1)),
);
```

```rust
fn shadow(self: Self, shadow: Shadow) -> Self
```



## ratatui_widgets::block::Block::style

*Function*

Defines the style of the entire block.

This is the most generic [`Style`] a block can receive, it will be merged with any other
more specific styles. Elements can be styled further with [`Block::title_style`] and
[`Block::border_style`], which will be applied on top of this style. If the block is used as
a container for another widget (e.g. a [`Paragraph`]), then the style of the widget is
generally applied before this style.

See [`Style`] for more information on how merging styles works.

`style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
your own type that implements [`Into<Style>`]).

# Example

```
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, Paragraph};

let block = Block::new().style(Style::new().red().on_black());

// For border and title you can additionally apply styles on top of the block level style.
let block = Block::new()
    .style(Style::new().red().bold().italic())
    .border_style(Style::new().not_italic()) // will be red and bold
    .title_style(Style::new().not_bold()) // will be red and italic
    .title("Title");

// To style the inner widget, you can style the widget itself.
let paragraph = Paragraph::new("Content")
    .block(block)
    .style(Style::new().white().not_bold()); // will be white, and italic
```

[`Paragraph`]: crate::paragraph::Paragraph
[`Color`]: ratatui_core::style::Color

```rust
fn style<S>(self: Self, style: S) -> Self
```



## ratatui_widgets::block::Block::title

*Function*

Adds a title to the block using the default position.

The position of the title is determined by the `title_position` field of the block, which
defaults to `Top`. This can be changed using the [`Block::title_position`] method. For
explicit positioning, use [`Block::title_top`] or [`Block::title_bottom`].

The `title` function allows you to add a title to the block. You can call this function
multiple times to add multiple titles.

Each title will be rendered with a single space separating titles that are in the same
position or alignment. When both centered and non-centered titles are rendered, the centered
space is calculated based on the full width of the block, rather than the leftover width.

You can provide any type that can be converted into [`Line`] including: strings, string
slices (`&str`), borrowed strings (`Cow<str>`), [spans](ratatui_core::text::Span), or
vectors of [spans](ratatui_core::text::Span) (`Vec<Span>`).

By default, the titles will avoid being rendered in the corners of the block but will align
against the left or right edge of the block if there is no border on that edge. The
following demonstrates this behavior, notice the second title is one character off to the
left.

```plain
┌With at least a left border───

Without left border───
```

Note: If the block is too small and multiple titles overlap, the border might get cut off at
a corner.

# Examples

See the [Block example] for a visual representation of how the various borders and styles
look when rendered.

The following example demonstrates:
- Default title alignment
- Multiple titles (notice "Center" is centered according to the full with of the block, not
  the leftover space)
- Two titles with the same alignment (notice the left titles are separated)
```
use ratatui::text::Line;
use ratatui::widgets::Block;

Block::bordered()
    .title("Title")
    .title(Line::from("Left").left_aligned())
    .title(Line::from("Right").right_aligned())
    .title(Line::from("Center").centered());
```

# See also

Titles attached to a block can have default behaviors. See
- [`Block::title_style`]
- [`Block::title_alignment`]

# History

In previous releases of Ratatui this method accepted `Into<Title>` instead of
[`Into<Line>`]. We found that storing the position in the block and the alignment in the
line better reflects the intended use of the block and its titles. See
<https://github.com/ratatui/ratatui/issues/738> for more information.

[Block example]: https://github.com/ratatui/ratatui/blob/main/examples/README.md#block

```rust
fn title<T>(self: Self, title: T) -> Self
```



## ratatui_widgets::block::Block::title_alignment

*Function*

Sets the default [`Alignment`] for all block titles.

Titles that explicitly set an [`Alignment`] will ignore this.

# Example

This example aligns all titles in the center except the "right" title which explicitly sets
[`Alignment::Right`].
```
use ratatui::layout::Alignment;
use ratatui::text::Line;
use ratatui::widgets::Block;

Block::bordered()
    .title_alignment(Alignment::Center)
    // This title won't be aligned in the center
    .title(Line::from("right").right_aligned())
    .title("foo")
    .title("bar");
```

```rust
fn title_alignment(self: Self, alignment: Alignment) -> Self
```



## ratatui_widgets::block::Block::title_bottom

*Function*

Adds a title to the bottom of the block.

You can provide any type that can be converted into [`Line`] including: strings, string
slices (`&str`), borrowed strings (`Cow<str>`), [spans](ratatui_core::text::Span), or
vectors of [spans](ratatui_core::text::Span) (`Vec<Span>`).

# Example

```
use ratatui::{ widgets::Block, text::Line };

Block::bordered()
    .title_bottom("Left1") // By default in the top left corner
    .title_bottom(Line::from("Left2").left_aligned())
    .title_bottom(Line::from("Right").right_aligned())
    .title_bottom(Line::from("Center").centered());

// Renders
// ┌──────────────────────────────────┐
// │                                  │
// └Left1─Left2───Center─────────Right┘
```

```rust
fn title_bottom<T>(self: Self, title: T) -> Self
```



## ratatui_widgets::block::Block::title_position

*Function*

Sets the default [`TitlePosition`] for all block titles.

# Example

This example positions all titles on the bottom by default. The "top" title explicitly sets
its position to `Top`, so it is not affected. The "foo" and "bar" titles will be positioned
at the bottom.

```
use ratatui::widgets::{Block, TitlePosition};

Block::bordered()
    .title_position(TitlePosition::Bottom)
    .title("foo") // will be at the bottom
    .title_top("top") // will be at the top
    .title("bar"); // will be at the bottom
```

```rust
fn title_position(self: Self, position: TitlePosition) -> Self
```



## ratatui_widgets::block::Block::title_style

*Function*

Applies the style to all titles.

This style will be applied to all titles of the block. If a title has a style set, it will
be applied after this style. This style will be applied after any [`Block::style`] or
[`Block::border_style`] is applied.

See [`Style`] for more information on how merging styles works.

`style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
your own type that implements [`Into<Style>`]).

[`Color`]: ratatui_core::style::Color

```rust
fn title_style<S>(self: Self, style: S) -> Self
```



## ratatui_widgets::block::Block::title_top

*Function*

Adds a title to the top of the block.

You can provide any type that can be converted into [`Line`] including: strings, string
slices (`&str`), borrowed strings (`Cow<str>`), [spans](ratatui_core::text::Span), or
vectors of [spans](ratatui_core::text::Span) (`Vec<Span>`).

# Example

```
use ratatui::{ widgets::Block, text::Line };

Block::bordered()
    .title_top("Left1") // By default in the top left corner
    .title_top(Line::from("Left2").left_aligned())
    .title_top(Line::from("Right").right_aligned())
    .title_top(Line::from("Center").centered());

// Renders
// ┌Left1─Left2───Center─────────Right┐
// │                                  │
// └──────────────────────────────────┘
```

```rust
fn title_top<T>(self: Self, title: T) -> Self
```



