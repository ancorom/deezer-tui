**ratatui_widgets > block > shadow**

# Module: block::shadow

## Contents

**Structs**

- [`Dimmed`](#dimmed) - A [`CellEffect`] that dims the shadow cells by setting the [`DIM`](Modifier::DIM) modifier.
- [`Shadow`](#shadow) - A configurable shadow that can be rendered behind a [`Block`](crate::block::Block).

**Functions**

- [`dimmed`](#dimmed) - Creates a [`Dimmed`] shadow effect.

**Traits**

- [`CellEffect`](#celleffect) - A cell effect that modifies the cells covered by a [`Shadow`].

---

## ratatui_widgets::block::shadow::CellEffect

*Trait*

A cell effect that modifies the cells covered by a [`Shadow`].

Custom effects must keep the same auto-trait guarantees as the widgets that store them:
[`Send`], [`Sync`], [`UnwindSafe`], and [`RefUnwindSafe`]. This preserves the thread-safety
and panic-safety behavior of widgets that contain a [`Block`](crate::block::Block).

See [`Shadow::custom`] for how to create a shadow from a custom effect.

**Methods:**

- `apply`: Applies the effect to the cells in `shadow_area`.



## ratatui_widgets::block::shadow::Dimmed

*Struct*

A [`CellEffect`] that dims the shadow cells by setting the [`DIM`](Modifier::DIM) modifier.

If the cell background is RGB, each channel is halved. Otherwise the background is replaced
with [`Color::Black`].

**Unit Struct**

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **CellEffect**
  - `fn apply(self: &Self, shadow_area: Rect, base_area: Rect, buf: & mut Buffer)`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## ratatui_widgets::block::shadow::Shadow

*Struct*

A configurable shadow that can be rendered behind a [`Block`](crate::block::Block).

A [`Shadow`] is rendered in an offset area relative to the block. Its [`Style`] is applied
first, then an optional cell effect can modify the affected cells, for example by filling them
with a shading symbol or dimming the existing background.

Built-in presets:

- [`Shadow::overlay`] applies only style
- [`Shadow::block`] fills with full block symbols
- [`Shadow::light_shade`], [`Shadow::medium_shade`], and [`Shadow::dark_shade`] fill with shade
  symbols

```plain
┌Popup─────┐
│content   │▒
└──────────┘▒
  ▒▒▒▒▒▒▒▒▒▒▒
```

# Custom effects

```
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::widgets::{Block, CellEffect, Shadow};

#[derive(Debug)]
struct Checker;

impl CellEffect for Checker {
    fn apply(&self, shadow_area: Rect, base_area: Rect, buf: &mut Buffer) {
        for y in shadow_area.top()..shadow_area.bottom() {
            for x in shadow_area.left()..shadow_area.right() {
                if base_area.contains(Position { x, y }) {
                    continue;
                }
                if (x + y) % 2 == 0 {
                    buf[(x, y)].set_symbol("░");
                }
            }
        }
    }
}

let shadow = Shadow::custom(Checker);
let block = Block::bordered().shadow(shadow);
```

**Methods:**

- `fn overlay() -> Self` - Creates a shadow that only applies style to the offset area.
- `fn block() -> Self` - Creates a shadow filled with full block symbols.
- `fn light_shade() -> Self` - Creates a shadow filled with light shade symbols.
- `fn medium_shade() -> Self` - Creates a shadow filled with medium shade symbols.
- `fn dark_shade() -> Self` - Creates a shadow filled with dark shade symbols.
- `fn symbol(symbol: &'static str) -> Self` - Creates a shadow filled with the given symbol.
- `fn custom<F>(effect: F) -> Self` - Creates a new shadow from a custom cell effect.
- `fn new<F>(effect: F) -> Self` - Creates a new shadow from a custom cell effect.
- `fn style<S>(self: Self, style: S) -> Self` - Sets the style applied to the shadow area.
- `fn offset(self: Self, offset: Offset) -> Self` - Sets the shadow offset relative to the original area.

**Traits:** Eq

**Trait Implementations:**

- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Styled**
  - `fn style(self: &Self) -> Style`
  - `fn set_style<S>(self: Self, style: S) -> <Self as >::Item`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`



## ratatui_widgets::block::shadow::dimmed

*Function*

Creates a [`Dimmed`] shadow effect.

```rust
fn dimmed() -> Dimmed
```



