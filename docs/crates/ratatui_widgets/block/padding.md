**ratatui_widgets > block > padding**

# Module: block::padding

## Contents

**Structs**

- [`Padding`](#padding) - Defines the padding for a [`Block`].

---

## ratatui_widgets::block::padding::Padding

*Struct*

Defines the padding for a [`Block`].

See the [`padding`] method of [`Block`] to configure its padding.

This concept is similar to [CSS padding].

**NOTE**: Terminal cells are often taller than they are wide, so to make horizontal and vertical
padding seem equal, doubling the horizontal padding is usually pretty good.

# Example

```
use ratatui::widgets::Padding;

Padding::uniform(1);
Padding::horizontal(2);
Padding::left(3);
Padding::proportional(4);
Padding::symmetric(5, 6);
```

[`Block`]: crate::block::Block
[`padding`]: crate::block::Block::padding
[CSS padding]: https://developer.mozilla.org/en-US/docs/Web/CSS/padding

**Fields:**
- `left: u16` - Left padding
- `right: u16` - Right padding
- `top: u16` - Top padding
- `bottom: u16` - Bottom padding

**Methods:**

- `fn new(left: u16, right: u16, top: u16, bottom: u16) -> Self` - Creates a new `Padding` by specifying every field individually.
- `fn zero() -> Self` - Creates a `Padding` with all fields set to `0`.
- `fn horizontal(value: u16) -> Self` - Creates a `Padding` with the same value for `left` and `right`.
- `fn vertical(value: u16) -> Self` - Creates a `Padding` with the same value for `top` and `bottom`.
- `fn uniform(value: u16) -> Self` - Creates a `Padding` with the same value for all fields.
- `fn proportional(value: u16) -> Self` - Creates a `Padding` that is visually proportional to the terminal.
- `fn symmetric(x: u16, y: u16) -> Self` - Creates a `Padding` that is symmetric.
- `fn left(value: u16) -> Self` - Creates a `Padding` that only sets the `left` padding.
- `fn right(value: u16) -> Self` - Creates a `Padding` that only sets the `right` padding.
- `fn top(value: u16) -> Self` - Creates a `Padding` that only sets the `top` padding.
- `fn bottom(value: u16) -> Self` - Creates a `Padding` that only sets the `bottom` padding.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(value: u16) -> Self`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Default**
  - `fn default() -> Self`



