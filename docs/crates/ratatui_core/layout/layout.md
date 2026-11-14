**ratatui_core > layout > layout**

# Module: layout::layout

## Contents

**Structs**

- [`Layout`](#layout) - The primary layout engine for dividing terminal space using constraints and direction.

**Enums**

- [`Spacing`](#spacing) - Represents the spacing between segments in a layout.

---

## ratatui_core::layout::layout::Layout

*Struct*

The primary layout engine for dividing terminal space using constraints and direction.

A layout is a set of constraints that can be applied to a given area to split it into smaller
rectangular areas. This is the core building block for creating structured user interfaces in
terminal applications.

A layout is composed of:
- a direction (horizontal or vertical)
- a set of constraints (length, ratio, percentage, fill, min, max)
- a margin (horizontal and vertical), the space between the edge of the main area and the split
  areas
- a flex option that controls space distribution
- a spacing option that controls gaps between segments

The algorithm used to compute the layout is based on the [`kasuari`] solver, a linear constraint
solver that computes positions and sizes to satisfy as many constraints as possible in order of
their priorities.

When the layout is computed, the result is cached in a thread-local cache, so that subsequent
calls with the same parameters are faster. The cache is a `LruCache`, and the size of the cache
can be configured using [`Layout::init_cache()`] when the `layout-cache` feature is enabled.

# Construction

- [`default`](Default::default) - Create a layout with default values (vertical direction, no
  constraints, no margin)
- [`new`](Self::new) - Create a new layout with a given direction and constraints
- [`vertical`](Self::vertical) - Create a new vertical layout with the given constraints
- [`horizontal`](Self::horizontal) - Create a new horizontal layout with the given constraints

# Configuration

- [`direction`](Self::direction) - Set the direction of the layout
- [`constraints`](Self::constraints) - Set the constraints of the layout
- [`margin`](Self::margin) - Set uniform margin on all sides
- [`horizontal_margin`](Self::horizontal_margin) - Set the horizontal margin of the layout
- [`vertical_margin`](Self::vertical_margin) - Set the vertical margin of the layout
- [`flex`](Self::flex) - Set the way space is distributed when constraints are satisfied
- [`spacing`](Self::spacing) - Set the gap between the constraints of the layout

# Layout Operations

- [`areas`](Self::areas) - Split area into fixed number of rectangles (compile-time known)
- [`spacers`](Self::spacers) - Get spacer rectangles between layout areas
- [`split`](Self::split) - Split area into rectangles (runtime determined count)
- [`split_with_spacers`](Self::split_with_spacers) - Split area and return both areas and
  spacers

# Cache Management

- [`init_cache`](Self::init_cache) - Initialize layout cache with custom size

# Example

```rust
use ratatui_core::buffer::Buffer;
use ratatui_core::layout::{Constraint, Direction, Layout, Rect};
use ratatui_core::text::Text;
use ratatui_core::widgets::Widget;

fn render(area: Rect, buf: &mut Buffer) {
    let layout = Layout::vertical([Constraint::Length(5), Constraint::Fill(1)]);
    let [top, bottom] = layout.areas(area);
    Text::from("foo").render(top, buf);
    Text::from("bar").render(bottom, buf);
}
```

See the `layout`, `flex`, and `constraints` examples in the [Examples] folder for more details
about how to use layouts.

For comprehensive layout documentation and examples, see the [`layout`](crate::layout) module.

![layout
example](https://camo.githubusercontent.com/77d22f3313b782a81e5e033ef82814bb48d786d2598699c27f8e757ccee62021/68747470733a2f2f7668732e636861726d2e73682f7668732d315a4e6f4e4c4e6c4c746b4a58706767396e435635652e676966)

[`kasuari`]: https://crates.io/crates/kasuari
[Examples]: https://github.com/ratatui/ratatui/blob/main/examples/README.md

**Methods:**

- `fn new<I>(direction: Direction, constraints: I) -> Self` - Creates a new layout with default values.
- `fn vertical<I>(constraints: I) -> Self` - Creates a new vertical layout with default values.
- `fn horizontal<I>(constraints: I) -> Self` - Creates a new horizontal layout with default values.
- `fn init_cache(cache_size: NonZeroUsize)` - Initialize an empty cache with a custom size. The cache is keyed on the layout and area, so
- `fn direction(self: Self, direction: Direction) -> Self` - Set the direction of the layout.
- `fn constraints<I>(self: Self, constraints: I) -> Self` - Sets the constraints of the layout.
- `fn margin(self: Self, margin: u16) -> Self` - Set the margin of the layout.
- `fn horizontal_margin(self: Self, horizontal: u16) -> Self` - Set the horizontal margin of the layout.
- `fn vertical_margin(self: Self, vertical: u16) -> Self` - Set the vertical margin of the layout.
- `fn flex(self: Self, flex: Flex) -> Self` - The `flex` method  allows you to specify the flex behavior of the layout.
- `fn spacing<T>(self: Self, spacing: T) -> Self` - Sets the spacing between items in the layout.
- `fn areas<const N>(self: &Self, area: Rect) -> [Rect; N]` - Split the rect into a number of sub-rects according to the given [`Layout`].
- `fn try_areas<const N>(self: &Self, area: Rect) -> Result<[Rect; N], TryFromSliceError>` - Split the rect into a number of sub-rects according to the given [`Layout`].
- `fn spacers<const N>(self: &Self, area: Rect) -> [Rect; N]` - Split the rect into a number of sub-rects according to the given [`Layout`] and return just
- `fn split(self: &Self, area: Rect) -> alloc::rc::Rc<[crate::layout::Rect]>` - Wrapper function around the [`kasuari`] solver to be able to split a given area into
- `fn split_with_spacers(self: &Self, area: Rect) -> (alloc::rc::Rc<[crate::layout::Rect]>, alloc::rc::Rc<[crate::layout::Rect]>)` - Wrapper function around the [`kasuari`] solver that splits the given area into smaller ones

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Default**
  - `fn default() -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ratatui_core::layout::layout::Spacing

*Enum*

Represents the spacing between segments in a layout.

The `Spacing` enum is used to define the spacing between segments in a layout. It can represent
either positive spacing (space between segments) or negative spacing (overlap between segments).

# Variants

- `Space(u16)`: Represents positive spacing between segments. The value indicates the number of
  cells.
- `Overlap(u16)`: Represents negative spacing, causing overlap between segments. The value
  indicates the number of overlapping cells.

# Default

The default value for `Spacing` is `Space(0)`, which means no spacing or no overlap between
segments.

# Conversions

The `Spacing` enum can be created from different integer types:

- From `u16`: Directly converts the value to `Spacing::Space`.
- From `i16`: Converts negative values to `Spacing::Overlap` and non-negative values to
  `Spacing::Space`.
- From `i32`: Clamps the value to the range of `i16` and converts negative values to
  `Spacing::Overlap` and non-negative values to `Spacing::Space`.

See the [`Layout::spacing`] method for details on how to use this enum.

**Variants:**
- `Space(u16)`
- `Overlap(u16)`

**Traits:** Eq

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **From**
  - `fn from(value: i16) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(value: u16) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **From**
  - `fn from(value: i32) -> Self`



