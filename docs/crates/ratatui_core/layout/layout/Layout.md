**ratatui_core > layout > layout > Layout**

# Module: layout::layout::Layout

## Contents

**Functions**

- [`areas`](#areas) - Split the rect into a number of sub-rects according to the given [`Layout`].
- [`constraints`](#constraints) - Sets the constraints of the layout.
- [`direction`](#direction) - Set the direction of the layout.
- [`flex`](#flex) - The `flex` method  allows you to specify the flex behavior of the layout.
- [`horizontal`](#horizontal) - Creates a new horizontal layout with default values.
- [`horizontal_margin`](#horizontal_margin) - Set the horizontal margin of the layout.
- [`init_cache`](#init_cache) - Initialize an empty cache with a custom size. The cache is keyed on the layout and area, so
- [`margin`](#margin) - Set the margin of the layout.
- [`new`](#new) - Creates a new layout with default values.
- [`spacers`](#spacers) - Split the rect into a number of sub-rects according to the given [`Layout`] and return just
- [`spacing`](#spacing) - Sets the spacing between items in the layout.
- [`split`](#split) - Wrapper function around the [`kasuari`] solver to be able to split a given area into
- [`split_with_spacers`](#split_with_spacers) - Wrapper function around the [`kasuari`] solver that splits the given area into smaller ones
- [`vertical`](#vertical) - Creates a new vertical layout with default values.
- [`vertical_margin`](#vertical_margin) - Set the vertical margin of the layout.

---

## ratatui_core::layout::layout::Layout::areas

*Function*

Split the rect into a number of sub-rects according to the given [`Layout`].

An ergonomic wrapper around [`Layout::split`] that returns an array of `Rect`s instead of
`Rc<[Rect]>`.

This method requires the number of constraints to be known at compile time. If you don't
know the number of constraints at compile time, use [`Layout::split`] instead.

# Panics

Panics if the number of constraints is not equal to the length of the returned array.

# Examples

```rust
use ratatui_core::layout::{Constraint, Layout, Rect};

let area = Rect::new(0, 0, 10, 10);
let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]);
let [top, main] = layout.areas(area);

// or explicitly specify the number of constraints:
let areas = layout.areas::<2>(area);
```

```rust
fn areas<const N>(self: &Self, area: Rect) -> [Rect; N]
```



## ratatui_core::layout::layout::Layout::constraints

*Function*

Sets the constraints of the layout.

The `constraints` parameter accepts any type that implements `IntoIterator<Item =
Into<Constraint>>`. This includes arrays, slices, vectors, iterators. `Into<Constraint>` is
implemented on u16, so you can pass an array or vec of u16 to this function to create a
layout with fixed size chunks.

Note that the constraints are applied to the whole area that is to be split, so using
percentages and ratios with the other constraints may not have the desired effect of
splitting the area up. (e.g. splitting 100 into [min 20, 50%, 50%], may not result in [20,
40, 40] but rather an indeterminate result between [20, 50, 30] and [20, 30, 50]).

# Examples

```rust
use ratatui_core::layout::{Constraint, Layout, Rect};

let layout = Layout::default()
    .constraints([
        Constraint::Percentage(20),
        Constraint::Ratio(1, 5),
        Constraint::Length(2),
        Constraint::Min(2),
        Constraint::Max(2),
    ])
    .split(Rect::new(0, 0, 10, 10));
assert_eq!(
    layout[..],
    [
        Rect::new(0, 0, 10, 2),
        Rect::new(0, 2, 10, 2),
        Rect::new(0, 4, 10, 2),
        Rect::new(0, 6, 10, 2),
        Rect::new(0, 8, 10, 2),
    ]
);

Layout::default().constraints([Constraint::Fill(1)]);
Layout::default().constraints(&[Constraint::Fill(1)]);
Layout::default().constraints(vec![Constraint::Fill(1)]);
Layout::default().constraints([Constraint::Fill(1)].iter().filter(|_| true));
Layout::default().constraints([1, 2, 3].iter().map(|&c| Constraint::Length(c)));
Layout::default().constraints([1, 2, 3]);
Layout::default().constraints(vec![1, 2, 3]);
```

```rust
fn constraints<I>(self: Self, constraints: I) -> Self
```



## ratatui_core::layout::layout::Layout::direction

*Function*

Set the direction of the layout.

# Examples

```rust
use ratatui_core::layout::{Constraint, Direction, Layout, Rect};

let layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Length(5), Constraint::Fill(1)])
    .split(Rect::new(0, 0, 10, 10));
assert_eq!(layout[..], [Rect::new(0, 0, 5, 10), Rect::new(5, 0, 5, 10)]);

let layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Length(5), Constraint::Fill(1)])
    .split(Rect::new(0, 0, 10, 10));
assert_eq!(layout[..], [Rect::new(0, 0, 10, 5), Rect::new(0, 5, 10, 5)]);
```

```rust
fn direction(self: Self, direction: Direction) -> Self
```



## ratatui_core::layout::layout::Layout::flex

*Function*

The `flex` method  allows you to specify the flex behavior of the layout.

# Arguments

* `flex`: A [`Flex`] enum value that represents the flex behavior of the layout. It can be
  one of the following:
  - [`Flex::Legacy`]: The last item is stretched to fill the excess space.
  - [`Flex::Start`]: The items are aligned to the start of the layout.
  - [`Flex::Center`]: The items are aligned to the center of the layout.
  - [`Flex::End`]: The items are aligned to the end of the layout.
  - [`Flex::SpaceBetween`]: The items are evenly distributed with equal space between them.
  - [`Flex::SpaceAround`]: The items are evenly distributed with equal space around them,
    except the first and last items, which have half the space on their sides.
  - [`Flex::SpaceEvenly`]: The items are evenly distributed with equal space around them.

# Examples

In this example, the items in the layout will be aligned to the start.

```rust
use ratatui_core::layout::Constraint::*;
use ratatui_core::layout::{Flex, Layout};

let layout = Layout::horizontal([Length(20), Length(20), Length(20)]).flex(Flex::Start);
```

In this example, the items in the layout will be stretched equally to fill the available
space.

```rust
use ratatui_core::layout::Constraint::*;
use ratatui_core::layout::{Flex, Layout};

let layout = Layout::horizontal([Length(20), Length(20), Length(20)]).flex(Flex::Legacy);
```

```rust
fn flex(self: Self, flex: Flex) -> Self
```



## ratatui_core::layout::layout::Layout::horizontal

*Function*

Creates a new horizontal layout with default values.

The `constraints` parameter accepts any type that implements `IntoIterator<Item =
Into<Constraint>>`. This includes arrays, slices, vectors, iterators, etc.

# Examples

```rust
use ratatui_core::layout::{Constraint, Layout};

let layout = Layout::horizontal([Constraint::Length(5), Constraint::Fill(1)]);
```

```rust
fn horizontal<I>(constraints: I) -> Self
```



## ratatui_core::layout::layout::Layout::horizontal_margin

*Function*

Set the horizontal margin of the layout.

# Examples

```rust
use ratatui_core::layout::{Constraint, Layout, Rect};

let layout = Layout::default()
    .constraints([Constraint::Fill(1)])
    .horizontal_margin(2)
    .split(Rect::new(0, 0, 10, 10));
assert_eq!(layout[..], [Rect::new(2, 0, 6, 10)]);
```

```rust
fn horizontal_margin(self: Self, horizontal: u16) -> Self
```



## ratatui_core::layout::layout::Layout::init_cache

*Function*

Initialize an empty cache with a custom size. The cache is keyed on the layout and area, so
that subsequent calls with the same parameters are faster. The cache is a `LruCache`, and
grows until `cache_size` is reached.

By default, the cache size is [`Self::DEFAULT_CACHE_SIZE`].

```rust
fn init_cache(cache_size: NonZeroUsize)
```



## ratatui_core::layout::layout::Layout::margin

*Function*

Set the margin of the layout.

# Examples

```rust
use ratatui_core::layout::{Constraint, Layout, Rect};

let layout = Layout::default()
    .constraints([Constraint::Fill(1)])
    .margin(2)
    .split(Rect::new(0, 0, 10, 10));
assert_eq!(layout[..], [Rect::new(2, 2, 6, 6)]);
```

```rust
fn margin(self: Self, margin: u16) -> Self
```



## ratatui_core::layout::layout::Layout::new

*Function*

Creates a new layout with default values.

The `constraints` parameter accepts any type that implements `IntoIterator<Item =
Into<Constraint>>`. This includes arrays, slices, vectors, iterators. `Into<Constraint>` is
implemented on `u16`, so you can pass an array, `Vec`, etc. of `u16` to this function to
create a layout with fixed size chunks.

Default values for the other fields are:

- `margin`: 0, 0
- `flex`: [`Flex::Start`]
- `spacing`: 0

# Examples

```rust
use ratatui_core::layout::{Constraint, Direction, Layout};

Layout::new(
    Direction::Horizontal,
    [Constraint::Length(5), Constraint::Fill(1)],
);

Layout::new(
    Direction::Vertical,
    [1, 2, 3].iter().map(|&c| Constraint::Length(c)),
);

Layout::new(Direction::Horizontal, vec![1, 2]);
```

```rust
fn new<I>(direction: Direction, constraints: I) -> Self
```



## ratatui_core::layout::layout::Layout::spacers

*Function*

Split the rect into a number of sub-rects according to the given [`Layout`] and return just
the spacers between the areas.

This method requires the number of constraints to be known at compile time. If you don't
know the number of constraints at compile time, use [`Layout::split_with_spacers`] instead.

This method is similar to [`Layout::areas`], and can be called with the same parameters, but
it returns just the spacers between the areas. The result of calling the `areas` method is
cached, so this will generally not re-run the solver, but will just return the cached
result.

# Panics

Panics if the number of constraints + 1 is not equal to the length of the returned array.

# Examples

```rust
use ratatui_core::layout::{Constraint, Layout, Rect};

let area = Rect::new(0, 0, 10, 10);
let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]);
let [top, main] = layout.areas(area);
let [before, inbetween, after] = layout.spacers(area);

// or explicitly specify the number of constraints:
let spacers = layout.spacers::<3>(area);
```

```rust
fn spacers<const N>(self: &Self, area: Rect) -> [Rect; N]
```



## ratatui_core::layout::layout::Layout::spacing

*Function*

Sets the spacing between items in the layout.

The `spacing` method sets the spacing between items in the layout. The spacing is applied
evenly between all segments. The spacing value represents the number of cells between each
item.

Spacing can be positive integers, representing gaps between segments; or negative integers
representing overlaps. Additionally, one of the variants of the [`Spacing`] enum can be
passed to this function. See the documentation of the [`Spacing`] enum for more information.

Note that if the layout has only one segment, the spacing will not be applied.
Also, spacing will not be applied for [`Flex::SpaceAround`], [`Flex::SpaceEvenly`] and
[`Flex::SpaceBetween`]

# Examples

In this example, the spacing between each item in the layout is set to 2 cells.

```rust
use ratatui_core::layout::Constraint::*;
use ratatui_core::layout::Layout;

let layout = Layout::horizontal([Length(20), Length(20), Length(20)]).spacing(2);
```

In this example, the spacing between each item in the layout is set to -1 cells, i.e. the
three segments will have an overlapping border.

```rust
use ratatui_core::layout::Constraint::*;
use ratatui_core::layout::Layout;
let layout = Layout::horizontal([Length(20), Length(20), Length(20)]).spacing(-1);
```

```rust
fn spacing<T>(self: Self, spacing: T) -> Self
```



## ratatui_core::layout::layout::Layout::split

*Function*

Wrapper function around the [`kasuari`] solver to be able to split a given area into
smaller ones based on the preferred widths or heights and the direction.

Note that the constraints are applied to the whole area that is to be split, so using
percentages and ratios with the other constraints may not have the desired effect of
splitting the area up. (e.g. splitting 100 into [min 20, 50%, 50%], may not result in [20,
40, 40] but rather an indeterminate result between [20, 50, 30] and [20, 30, 50]).

This method stores the result of the computation in a thread-local cache keyed on the layout
and area, so that subsequent calls with the same parameters are faster. The cache is a
`LruCache`, and grows until [`Self::DEFAULT_CACHE_SIZE`] is reached by default. If the cache
is initialized with [`Layout::init_cache()`], it grows until the initialized cache size.

There is a helper method that can be used to split the whole area into smaller ones based on
the layout: [`Layout::areas()`]. That method is a shortcut for calling this method. It
allows you to destructure the result directly into variables, which is useful when you know
at compile time the number of areas that will be created.

# Examples

```
use ratatui_core::layout::{Constraint, Direction, Layout, Rect};
let layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Length(5), Constraint::Fill(1)])
    .split(Rect::new(2, 2, 10, 10));
assert_eq!(layout[..], [Rect::new(2, 2, 10, 5), Rect::new(2, 7, 10, 5)]);

let layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)])
    .split(Rect::new(0, 0, 9, 2));
assert_eq!(layout[..], [Rect::new(0, 0, 3, 2), Rect::new(3, 0, 6, 2)]);
```

```rust
fn split(self: &Self, area: Rect) -> alloc::rc::Rc<[crate::layout::Rect]>
```



## ratatui_core::layout::layout::Layout::split_with_spacers

*Function*

Wrapper function around the [`kasuari`] solver that splits the given area into smaller ones
based on the preferred widths or heights and the direction, with the ability to include
spacers between the areas.

This method is similar to `split`, but it returns two sets of rectangles: one for the areas
and one for the spacers.

This method stores the result of the computation in a thread-local cache keyed on the layout
and area, so that subsequent calls with the same parameters are faster. The cache is a
`LruCache`, and grows until [`Self::DEFAULT_CACHE_SIZE`] is reached by default. If the cache
is initialized with [`Layout::init_cache()`], it grows until the initialized cache size.

# Examples

```
use ratatui_core::layout::{Constraint, Direction, Layout, Rect};

let (areas, spacers) = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Length(5), Constraint::Fill(1)])
    .split_with_spacers(Rect::new(2, 2, 10, 10));
assert_eq!(areas[..], [Rect::new(2, 2, 10, 5), Rect::new(2, 7, 10, 5)]);
assert_eq!(
    spacers[..],
    [
        Rect::new(2, 2, 10, 0),
        Rect::new(2, 7, 10, 0),
        Rect::new(2, 12, 10, 0)
    ]
);

let (areas, spacers) = Layout::default()
    .direction(Direction::Horizontal)
    .spacing(1)
    .constraints([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)])
    .split_with_spacers(Rect::new(0, 0, 10, 2));
assert_eq!(areas[..], [Rect::new(0, 0, 3, 2), Rect::new(4, 0, 6, 2)]);
assert_eq!(
    spacers[..],
    [
        Rect::new(0, 0, 0, 2),
        Rect::new(3, 0, 1, 2),
        Rect::new(10, 0, 0, 2)
    ]
);
```

```rust
fn split_with_spacers(self: &Self, area: Rect) -> (alloc::rc::Rc<[crate::layout::Rect]>, alloc::rc::Rc<[crate::layout::Rect]>)
```



## ratatui_core::layout::layout::Layout::vertical

*Function*

Creates a new vertical layout with default values.

The `constraints` parameter accepts any type that implements `IntoIterator<Item =
Into<Constraint>>`. This includes arrays, slices, vectors, iterators, etc.

# Examples

```rust
use ratatui_core::layout::{Constraint, Layout};

let layout = Layout::vertical([Constraint::Length(5), Constraint::Fill(1)]);
```

```rust
fn vertical<I>(constraints: I) -> Self
```



## ratatui_core::layout::layout::Layout::vertical_margin

*Function*

Set the vertical margin of the layout.

# Examples

```rust
use ratatui_core::layout::{Constraint, Layout, Rect};

let layout = Layout::default()
    .constraints([Constraint::Fill(1)])
    .vertical_margin(2)
    .split(Rect::new(0, 0, 10, 10));
assert_eq!(layout[..], [Rect::new(0, 2, 10, 6)]);
```

```rust
fn vertical_margin(self: Self, vertical: u16) -> Self
```



