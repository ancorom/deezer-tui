**ratatui_core > style > palette > material**

# Module: style::palette::material

## Contents

**Structs**

- [`AccentedPalette`](#accentedpalette) - A palette of colors for use in Material design with accent colors
- [`NonAccentedPalette`](#nonaccentedpalette) - A palette of colors for use in Material design without accent colors

**Constants**

- [`AMBER`](#amber)
- [`BLACK`](#black)
- [`BLUE`](#blue)
- [`BLUE_GRAY`](#blue_gray)
- [`BROWN`](#brown)
- [`CYAN`](#cyan)
- [`DEEP_ORANGE`](#deep_orange)
- [`DEEP_PURPLE`](#deep_purple)
- [`GRAY`](#gray)
- [`GREEN`](#green)
- [`INDIGO`](#indigo)
- [`LIGHT_BLUE`](#light_blue)
- [`LIGHT_GREEN`](#light_green)
- [`LIME`](#lime)
- [`ORANGE`](#orange)
- [`PINK`](#pink)
- [`PURPLE`](#purple)
- [`RED`](#red)
- [`TEAL`](#teal)
- [`WHITE`](#white)
- [`YELLOW`](#yellow)

---

## ratatui_core::style::palette::material::AMBER

*Constant*: `AccentedPalette`



## ratatui_core::style::palette::material::AccentedPalette

*Struct*

A palette of colors for use in Material design with accent colors

This is a collection of colors that are used in Material design. They consist of a set of
colors from 50 to 900, and a set of accent colors from 100 to 700.

**Fields:**
- `c50: crate::style::Color`
- `c100: crate::style::Color`
- `c200: crate::style::Color`
- `c300: crate::style::Color`
- `c400: crate::style::Color`
- `c500: crate::style::Color`
- `c600: crate::style::Color`
- `c700: crate::style::Color`
- `c800: crate::style::Color`
- `c900: crate::style::Color`
- `a100: crate::style::Color`
- `a200: crate::style::Color`
- `a400: crate::style::Color`
- `a700: crate::style::Color`

**Methods:**

- `fn from_variants(variants: [u32; 14]) -> Self` - Create a new `AccentedPalette` from the given variants

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## ratatui_core::style::palette::material::BLACK

*Constant*: `crate::style::Color`



## ratatui_core::style::palette::material::BLUE

*Constant*: `AccentedPalette`



## ratatui_core::style::palette::material::BLUE_GRAY

*Constant*: `NonAccentedPalette`



## ratatui_core::style::palette::material::BROWN

*Constant*: `NonAccentedPalette`



## ratatui_core::style::palette::material::CYAN

*Constant*: `AccentedPalette`



## ratatui_core::style::palette::material::DEEP_ORANGE

*Constant*: `AccentedPalette`



## ratatui_core::style::palette::material::DEEP_PURPLE

*Constant*: `AccentedPalette`



## ratatui_core::style::palette::material::GRAY

*Constant*: `NonAccentedPalette`



## ratatui_core::style::palette::material::GREEN

*Constant*: `AccentedPalette`



## ratatui_core::style::palette::material::INDIGO

*Constant*: `AccentedPalette`



## ratatui_core::style::palette::material::LIGHT_BLUE

*Constant*: `AccentedPalette`



## ratatui_core::style::palette::material::LIGHT_GREEN

*Constant*: `AccentedPalette`



## ratatui_core::style::palette::material::LIME

*Constant*: `AccentedPalette`



## ratatui_core::style::palette::material::NonAccentedPalette

*Struct*

A palette of colors for use in Material design without accent colors

This is a collection of colors that are used in Material design. They consist of a set of
colors from 50 to 900.

**Fields:**
- `c50: crate::style::Color`
- `c100: crate::style::Color`
- `c200: crate::style::Color`
- `c300: crate::style::Color`
- `c400: crate::style::Color`
- `c500: crate::style::Color`
- `c600: crate::style::Color`
- `c700: crate::style::Color`
- `c800: crate::style::Color`
- `c900: crate::style::Color`

**Methods:**

- `fn from_variants(variants: [u32; 10]) -> Self` - Create a new `NonAccented` from the given variants

**Traits:** Eq, Copy

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ratatui_core::style::palette::material::ORANGE

*Constant*: `AccentedPalette`



## ratatui_core::style::palette::material::PINK

*Constant*: `AccentedPalette`



## ratatui_core::style::palette::material::PURPLE

*Constant*: `AccentedPalette`



## ratatui_core::style::palette::material::RED

*Constant*: `AccentedPalette`



## ratatui_core::style::palette::material::TEAL

*Constant*: `AccentedPalette`



## ratatui_core::style::palette::material::WHITE

*Constant*: `crate::style::Color`



## ratatui_core::style::palette::material::YELLOW

*Constant*: `AccentedPalette`



