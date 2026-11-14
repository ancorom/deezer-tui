**ratatui_core > symbols > border**

# Module: symbols::border

## Contents

**Structs**

- [`Set`](#set)

**Constants**

- [`DOUBLE`](#double) - Border Set with a double line width
- [`EMPTY`](#empty) - Empty border set
- [`FULL`](#full) - Solid border set
- [`HEAVY_DOUBLE_DASHED`](#heavy_double_dashed) - Border Set with thick double-dashed border lines
- [`HEAVY_QUADRUPLE_DASHED`](#heavy_quadruple_dashed) - Border Set with thick quadruple-dashed border lines
- [`HEAVY_TRIPLE_DASHED`](#heavy_triple_dashed) - Border Set with thick triple-dashed border lines
- [`LIGHT_DOUBLE_DASHED`](#light_double_dashed) - Border Set with light double-dashed border lines
- [`LIGHT_QUADRUPLE_DASHED`](#light_quadruple_dashed) - Border Set with light quadruple-dashed border lines
- [`LIGHT_TRIPLE_DASHED`](#light_triple_dashed) - Border Set with light triple-dashed border lines
- [`ONE_EIGHTH_BOTTOM_EIGHT`](#one_eighth_bottom_eight)
- [`ONE_EIGHTH_LEFT_EIGHT`](#one_eighth_left_eight)
- [`ONE_EIGHTH_RIGHT_EIGHT`](#one_eighth_right_eight)
- [`ONE_EIGHTH_TALL`](#one_eighth_tall) - Tall border set based on McGugan box technique
- [`ONE_EIGHTH_TOP_EIGHT`](#one_eighth_top_eight)
- [`ONE_EIGHTH_WIDE`](#one_eighth_wide) - Wide border set based on McGugan box technique
- [`PLAIN`](#plain) - Border Set with a single line width
- [`PROPORTIONAL_TALL`](#proportional_tall) - Tall proportional (visually equal width and height) border with using set of quadrants.
- [`PROPORTIONAL_WIDE`](#proportional_wide) - Wide proportional (visually equal width and height) border with using set of quadrants.
- [`QUADRANT_BLOCK`](#quadrant_block)
- [`QUADRANT_BOTTOM_HALF`](#quadrant_bottom_half)
- [`QUADRANT_BOTTOM_LEFT`](#quadrant_bottom_left)
- [`QUADRANT_BOTTOM_RIGHT`](#quadrant_bottom_right)
- [`QUADRANT_INSIDE`](#quadrant_inside) - Quadrant used for setting a border inside a block by one half cell "pixel".
- [`QUADRANT_LEFT_HALF`](#quadrant_left_half)
- [`QUADRANT_OUTSIDE`](#quadrant_outside) - Quadrant used for setting a border outside a block by one half cell "pixel".
- [`QUADRANT_RIGHT_HALF`](#quadrant_right_half)
- [`QUADRANT_TOP_HALF`](#quadrant_top_half)
- [`QUADRANT_TOP_LEFT`](#quadrant_top_left)
- [`QUADRANT_TOP_LEFT_BOTTOM_LEFT_BOTTOM_RIGHT`](#quadrant_top_left_bottom_left_bottom_right)
- [`QUADRANT_TOP_LEFT_BOTTOM_RIGHT`](#quadrant_top_left_bottom_right)
- [`QUADRANT_TOP_LEFT_TOP_RIGHT_BOTTOM_LEFT`](#quadrant_top_left_top_right_bottom_left)
- [`QUADRANT_TOP_LEFT_TOP_RIGHT_BOTTOM_RIGHT`](#quadrant_top_left_top_right_bottom_right)
- [`QUADRANT_TOP_RIGHT`](#quadrant_top_right)
- [`QUADRANT_TOP_RIGHT_BOTTOM_LEFT`](#quadrant_top_right_bottom_left)
- [`QUADRANT_TOP_RIGHT_BOTTOM_LEFT_BOTTOM_RIGHT`](#quadrant_top_right_bottom_left_bottom_right)
- [`ROUNDED`](#rounded) - Border Set with a single line width and rounded corners
- [`THICK`](#thick) - Border Set with a thick line width

---

## ratatui_core::symbols::border::DOUBLE

*Constant*: `Set`

Border Set with a double line width

```text
╔═════╗
║xxxxx║
║xxxxx║
╚═════╝
```



## ratatui_core::symbols::border::EMPTY

*Constant*: `Set`

Empty border set

The border is created by using empty strings for all sides.

This is useful for ensuring that the border style is applied to a border on a block with a title
without actually drawing a border.

░ Example

`░` represents the content in the area not covered by the border to make it easier to see the
blank symbols.

```text
░░░░░░░░
░░    ░░
░░ ░░ ░░
░░ ░░ ░░
░░    ░░
░░░░░░░░
```



## ratatui_core::symbols::border::FULL

*Constant*: `Set`

Solid border set

The border is created by using full blocks for all sides.

```text
████
█xx█
█xx█
████
```



## ratatui_core::symbols::border::HEAVY_DOUBLE_DASHED

*Constant*: `Set`

Border Set with thick double-dashed border lines

```text
┏╍╍╍╍╍┓
╏xxxxx╏
╏xxxxx╏
┗╍╍╍╍╍┛
```



## ratatui_core::symbols::border::HEAVY_QUADRUPLE_DASHED

*Constant*: `Set`

Border Set with thick quadruple-dashed border lines

```text
┏┉┉┉┉┉┓
┋xxxxx┋
┋xxxxx┋
┗┉┉┉┉┉┛
```



## ratatui_core::symbols::border::HEAVY_TRIPLE_DASHED

*Constant*: `Set`

Border Set with thick triple-dashed border lines

```text
┏┅┅┅┅┅┓
┇xxxxx┇
┇xxxxx┇
┗┅┅┅┅┅┛
```



## ratatui_core::symbols::border::LIGHT_DOUBLE_DASHED

*Constant*: `Set`

Border Set with light double-dashed border lines

```text
┌╌╌╌╌╌┐
╎xxxxx╎
╎xxxxx╎
└╌╌╌╌╌┘
```



## ratatui_core::symbols::border::LIGHT_QUADRUPLE_DASHED

*Constant*: `Set`

Border Set with light quadruple-dashed border lines

```text
┌┈┈┈┈┈┐
┊xxxxx┊
┊xxxxx┊
└┈┈┈┈┈┘
```



## ratatui_core::symbols::border::LIGHT_TRIPLE_DASHED

*Constant*: `Set`

Border Set with light triple-dashed border lines

```text
┌┄┄┄┄┄┐
┆xxxxx┆
┆xxxxx┆
└┄┄┄┄┄┘
```



## ratatui_core::symbols::border::ONE_EIGHTH_BOTTOM_EIGHT

*Constant*: `&str`



## ratatui_core::symbols::border::ONE_EIGHTH_LEFT_EIGHT

*Constant*: `&str`



## ratatui_core::symbols::border::ONE_EIGHTH_RIGHT_EIGHT

*Constant*: `&str`



## ratatui_core::symbols::border::ONE_EIGHTH_TALL

*Constant*: `Set`

Tall border set based on McGugan box technique

```text
▕▔▔▏
▕xx▏
▕xx▏
▕▁▁▏
```



## ratatui_core::symbols::border::ONE_EIGHTH_TOP_EIGHT

*Constant*: `&str`



## ratatui_core::symbols::border::ONE_EIGHTH_WIDE

*Constant*: `Set`

Wide border set based on McGugan box technique

```text
▁▁▁▁▁▁▁
▏xxxxx▕
▏xxxxx▕
▔▔▔▔▔▔▔
```



## ratatui_core::symbols::border::PLAIN

*Constant*: `Set`

Border Set with a single line width

```text
┌─────┐
│xxxxx│
│xxxxx│
└─────┘
```



## ratatui_core::symbols::border::PROPORTIONAL_TALL

*Constant*: `Set`

Tall proportional (visually equal width and height) border with using set of quadrants.

The border is created by using full blocks for all sides, except for the top and bottom,
which use half blocks to make horizontal and vertical borders seem equal.

```text
▕█▀▀█
▕█xx█
▕█xx█
▕█▄▄█
```



## ratatui_core::symbols::border::PROPORTIONAL_WIDE

*Constant*: `Set`

Wide proportional (visually equal width and height) border with using set of quadrants.

The border is created by using half blocks for top and bottom, and full
blocks for right and left sides to make horizontal and vertical borders seem equal.

```text
▄▄▄▄
█xx█
█xx█
▀▀▀▀
```



## ratatui_core::symbols::border::QUADRANT_BLOCK

*Constant*: `&str`



## ratatui_core::symbols::border::QUADRANT_BOTTOM_HALF

*Constant*: `&str`



## ratatui_core::symbols::border::QUADRANT_BOTTOM_LEFT

*Constant*: `&str`



## ratatui_core::symbols::border::QUADRANT_BOTTOM_RIGHT

*Constant*: `&str`



## ratatui_core::symbols::border::QUADRANT_INSIDE

*Constant*: `Set`

Quadrant used for setting a border inside a block by one half cell "pixel".

```text
▗▄▄▄▄▄▖
▐xxxxx▌
▐xxxxx▌
▝▀▀▀▀▀▘
```



## ratatui_core::symbols::border::QUADRANT_LEFT_HALF

*Constant*: `&str`



## ratatui_core::symbols::border::QUADRANT_OUTSIDE

*Constant*: `Set`

Quadrant used for setting a border outside a block by one half cell "pixel".

```text
▛▀▀▀▀▀▜
▌xxxxx▐
▌xxxxx▐
▙▄▄▄▄▄▟
```



## ratatui_core::symbols::border::QUADRANT_RIGHT_HALF

*Constant*: `&str`



## ratatui_core::symbols::border::QUADRANT_TOP_HALF

*Constant*: `&str`



## ratatui_core::symbols::border::QUADRANT_TOP_LEFT

*Constant*: `&str`



## ratatui_core::symbols::border::QUADRANT_TOP_LEFT_BOTTOM_LEFT_BOTTOM_RIGHT

*Constant*: `&str`



## ratatui_core::symbols::border::QUADRANT_TOP_LEFT_BOTTOM_RIGHT

*Constant*: `&str`



## ratatui_core::symbols::border::QUADRANT_TOP_LEFT_TOP_RIGHT_BOTTOM_LEFT

*Constant*: `&str`



## ratatui_core::symbols::border::QUADRANT_TOP_LEFT_TOP_RIGHT_BOTTOM_RIGHT

*Constant*: `&str`



## ratatui_core::symbols::border::QUADRANT_TOP_RIGHT

*Constant*: `&str`



## ratatui_core::symbols::border::QUADRANT_TOP_RIGHT_BOTTOM_LEFT

*Constant*: `&str`



## ratatui_core::symbols::border::QUADRANT_TOP_RIGHT_BOTTOM_LEFT_BOTTOM_RIGHT

*Constant*: `&str`



## ratatui_core::symbols::border::ROUNDED

*Constant*: `Set`

Border Set with a single line width and rounded corners

```text
╭─────╮
│xxxxx│
│xxxxx│
╰─────╯
```



## ratatui_core::symbols::border::Set

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `top_left: &'a str`
- `top_right: &'a str`
- `bottom_left: &'a str`
- `bottom_right: &'a str`
- `vertical_left: &'a str`
- `vertical_right: &'a str`
- `horizontal_top: &'a str`
- `horizontal_bottom: &'a str`

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## ratatui_core::symbols::border::THICK

*Constant*: `Set`

Border Set with a thick line width

```text
┏━━━━━┓
┃xxxxx┃
┃xxxxx┃
┗━━━━━┛
```



