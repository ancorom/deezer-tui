**ratatui_core > style**

# Module: style

## Contents

**Modules**

- [`palette`](#palette) - A module for defining color palettes.

**Structs**

- [`Modifier`](#modifier) - Modifier changes the way a piece of text is displayed.
- [`Style`](#style) - Style lets you control the main characteristics of the displayed elements.

---

## ratatui_core::style::Modifier

*Struct*

Modifier changes the way a piece of text is displayed.

They are bitflags so they can easily be composed.

`From<Modifier> for Style` is implemented so you can use `Modifier` anywhere that accepts
`Into<Style>`.

## Examples

```rust
use ratatui_core::style::Modifier;

let m = Modifier::BOLD | Modifier::ITALIC;
```

**Tuple Struct**: `()`

**Methods:**

- `fn iter(self: &Self) -> $crate::iter::Iter<Modifier>` - Yield a set of contained flags values.
- `fn iter_names(self: &Self) -> $crate::iter::IterNames<Modifier>` - Yield a set of contained named flags values.
- `fn empty() -> Self` - Get a flags value with all bits unset.
- `fn all() -> Self` - Get a flags value with all known bits set.
- `fn bits(self: &Self) -> u16` - Get the underlying bits value.
- `fn from_bits(bits: u16) -> $crate::__private::core::option::Option<Self>` - Convert from a bits value.
- `fn from_bits_truncate(bits: u16) -> Self` - Convert from a bits value, unsetting any unknown bits.
- `fn from_bits_retain(bits: u16) -> Self` - Convert from a bits value exactly.
- `fn from_name(name: &str) -> $crate::__private::core::option::Option<Self>` - Get a flags value with the bits of a flag with the given name set.
- `fn is_empty(self: &Self) -> bool` - Whether all bits in `self` are unset.
- `fn is_all(self: &Self) -> bool` - Whether all known bits in this flags value are set.
- `fn intersects(self: &Self, other: Self) -> bool` - Whether any set bits in `other` are also set in `self`.
- `fn contains(self: &Self, other: Self) -> bool` - Whether all set bits in `other` are also set in `self`.
- `fn insert(self: & mut Self, other: Self)` - The bitwise or (`|`) of the bits in `self` and `other`.
- `fn remove(self: & mut Self, other: Self)` - The intersection of `self` with the complement of `other` (`&!`).
- `fn toggle(self: & mut Self, other: Self)` - The bitwise exclusive-or (`^`) of the bits in `self` and `other`.
- `fn set(self: & mut Self, other: Self, value: bool)` - Call `insert` when `value` is `true` or `remove` when `value` is `false`.
- `fn intersection(self: Self, other: Self) -> Self` - The bitwise and (`&`) of the bits in `self` and `other`.
- `fn union(self: Self, other: Self) -> Self` - The bitwise or (`|`) of the bits in `self` and `other`.
- `fn difference(self: Self, other: Self) -> Self` - The intersection of `self` with the complement of `other` (`&!`).
- `fn symmetric_difference(self: Self, other: Self) -> Self` - The bitwise exclusive-or (`^`) of the bits in `self` and `other`.
- `fn complement(self: Self) -> Self` - The bitwise negation (`!`) of the bits in `self`, truncating the result.

**Traits:** Copy, Eq, PublicFlags

**Trait Implementations:**

- **LowerHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **FromIterator**
  - `fn from_iter<T>(iterator: T) -> Self` - The bitwise or (`|`) of the bits in each flags value.
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: Self)` - The intersection of `self` with the complement of `other` (`&!`).
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitXor**
  - `fn bitxor(self: Self, other: Self) -> Self` - The bitwise exclusive-or (`^`) of the bits in `self` and `other`.
- **Not**
  - `fn not(self: Self) -> Self` - The bitwise negation (`!`) of the bits in `self`, truncating the result.
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: Self)` - The bitwise exclusive-or (`^`) of the bits in `self` and `other`.
- **Extend**
  - `fn extend<T>(self: & mut Self, iterator: T)` - The bitwise or (`|`) of the bits in each flags value.
- **Sub**
  - `fn sub(self: Self, other: Self) -> Self` - The intersection of `self` with the complement of `other` (`&!`).
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **BitOr**
  - `fn bitor(self: Self, other: Modifier) -> Self` - The bitwise or (`|`) of the bits in `self` and `other`.
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result` - Format the modifier as `NONE` if the modifier is empty or as a list of flags separated by
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: Self)` - The bitwise or (`|`) of the bits in `self` and `other`.
- **BitAnd**
  - `fn bitand(self: Self, other: Self) -> Self` - The bitwise and (`&`) of the bits in `self` and `other`.
- **Binary**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **Flags**
  - `fn bits(self: &Self) -> u16`
  - `fn from_bits_retain(bits: u16) -> Modifier`
  - `fn all_named() -> Modifier`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: Self)` - The bitwise and (`&`) of the bits in `self` and `other`.
- **Octal**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`



## ratatui_core::style::Style

*Struct*

Style lets you control the main characteristics of the displayed elements.

```rust
use ratatui_core::style::{Color, Modifier, Style};

Style::default()
    .fg(Color::Black)
    .bg(Color::Green)
    .add_modifier(Modifier::ITALIC | Modifier::BOLD);
```

Styles can also be created with a [shorthand notation](crate::style#using-style-shorthands).

```rust
use ratatui_core::style::{Style, Stylize};

Style::new().black().on_green().italic().bold();
```

For more information about the style shorthands, see the [`Stylize`] trait.

We implement conversions from [`Color`] and [`Modifier`] to [`Style`] so you can use them
anywhere that accepts `Into<Style>`.

```rust
use ratatui_core::style::{Color, Modifier, Style};
use ratatui_core::text::Line;

Line::styled("hello", Style::new().fg(Color::Red));
// simplifies to
Line::styled("hello", Color::Red);

Line::styled("hello", Style::new().add_modifier(Modifier::BOLD));
// simplifies to
Line::styled("hello", Modifier::BOLD);
```

Styles represents an incremental change. If you apply the styles S1, S2, S3 to a cell of the
terminal buffer, the style of this cell will be the result of the merge of S1, S2 and S3, not
just S3.

```rust
use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;
use ratatui_core::style::{Color, Modifier, Style};

let styles = [
    Style::default()
        .fg(Color::Blue)
        .add_modifier(Modifier::BOLD | Modifier::ITALIC),
    Style::default()
        .bg(Color::Red)
        .add_modifier(Modifier::UNDERLINED),
    #[cfg(feature = "underline-color")]
    Style::default().underline_color(Color::Green),
    Style::default()
        .fg(Color::Yellow)
        .remove_modifier(Modifier::ITALIC),
];
let mut buffer = Buffer::empty(Rect::new(0, 0, 1, 1));
for style in &styles {
    buffer[(0, 0)].set_style(*style);
}
assert_eq!(
    Style {
        fg: Some(Color::Yellow),
        bg: Some(Color::Red),
        #[cfg(feature = "underline-color")]
        underline_color: Some(Color::Green),
        add_modifier: Modifier::BOLD | Modifier::UNDERLINED,
        sub_modifier: Modifier::empty(),
    },
    buffer[(0, 0)].style(),
);
```

The default implementation returns a `Style` that does not modify anything. If you wish to
reset all properties until that point use [`Style::reset`].

```
use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;
use ratatui_core::style::{Color, Modifier, Style};

let styles = [
    Style::default()
        .fg(Color::Blue)
        .add_modifier(Modifier::BOLD | Modifier::ITALIC),
    Style::reset().fg(Color::Yellow),
];
let mut buffer = Buffer::empty(Rect::new(0, 0, 1, 1));
for style in &styles {
    buffer[(0, 0)].set_style(*style);
}
assert_eq!(
    Style {
        fg: Some(Color::Yellow),
        bg: Some(Color::Reset),
        #[cfg(feature = "underline-color")]
        underline_color: Some(Color::Reset),
        add_modifier: Modifier::empty(),
        sub_modifier: Modifier::empty(),
    },
    buffer[(0, 0)].style(),
);
```

**Fields:**
- `fg: Option<Color>` - The foreground color.
- `bg: Option<Color>` - The background color.
- `underline_color: Option<Color>` - The underline color.
- `add_modifier: Modifier` - The modifiers to add.
- `sub_modifier: Modifier` - The modifiers to remove.

**Methods:**

- `fn new() -> Self` - Returns a `Style` with default properties.
- `fn reset() -> Self` - Returns a `Style` resetting all properties.
- `fn fg(self: Self, color: Color) -> Self` - Changes the foreground color.
- `fn bg(self: Self, color: Color) -> Self` - Changes the background color.
- `fn underline_color(self: Self, color: Color) -> Self` - Changes the underline color. The text must be underlined with a modifier for this to work.
- `fn add_modifier(self: Self, modifier: Modifier) -> Self` - Changes the text emphasis.
- `fn remove_modifier(self: Self, modifier: Modifier) -> Self` - Changes the text emphasis.
- `fn has_modifier(self: Self, modifier: Modifier) -> bool` - Returns `true` if the style has the given modifier set.
- `fn patch<S>(self: Self, other: S) -> Self` - Results in a combined style that is equivalent to applying the two individual styles to
- `fn black(self: Self) -> Self` - Sets the foreground color to [`black`](Color::Black).
- `fn on_black(self: Self) -> Self` - Sets the background color to [`black`](Color::Black).
- `fn red(self: Self) -> Self` - Sets the foreground color to [`red`](Color::Red).
- `fn on_red(self: Self) -> Self` - Sets the background color to [`red`](Color::Red).
- `fn green(self: Self) -> Self` - Sets the foreground color to [`green`](Color::Green).
- `fn on_green(self: Self) -> Self` - Sets the background color to [`green`](Color::Green).
- `fn yellow(self: Self) -> Self` - Sets the foreground color to [`yellow`](Color::Yellow).
- `fn on_yellow(self: Self) -> Self` - Sets the background color to [`yellow`](Color::Yellow).
- `fn blue(self: Self) -> Self` - Sets the foreground color to [`blue`](Color::Blue).
- `fn on_blue(self: Self) -> Self` - Sets the background color to [`blue`](Color::Blue).
- `fn magenta(self: Self) -> Self` - Sets the foreground color to [`magenta`](Color::Magenta).
- `fn on_magenta(self: Self) -> Self` - Sets the background color to [`magenta`](Color::Magenta).
- `fn cyan(self: Self) -> Self` - Sets the foreground color to [`cyan`](Color::Cyan).
- `fn on_cyan(self: Self) -> Self` - Sets the background color to [`cyan`](Color::Cyan).
- `fn gray(self: Self) -> Self` - Sets the foreground color to [`gray`](Color::Gray).
- `fn on_gray(self: Self) -> Self` - Sets the background color to [`gray`](Color::Gray).
- `fn dark_gray(self: Self) -> Self` - Sets the foreground color to [`dark_gray`](Color::DarkGray).
- `fn on_dark_gray(self: Self) -> Self` - Sets the background color to [`dark_gray`](Color::DarkGray).
- `fn light_red(self: Self) -> Self` - Sets the foreground color to [`light_red`](Color::LightRed).
- `fn on_light_red(self: Self) -> Self` - Sets the background color to [`light_red`](Color::LightRed).
- `fn light_green(self: Self) -> Self` - Sets the foreground color to [`light_green`](Color::LightGreen).
- `fn on_light_green(self: Self) -> Self` - Sets the background color to [`light_green`](Color::LightGreen).
- `fn light_yellow(self: Self) -> Self` - Sets the foreground color to [`light_yellow`](Color::LightYellow).
- `fn on_light_yellow(self: Self) -> Self` - Sets the background color to [`light_yellow`](Color::LightYellow).
- `fn light_blue(self: Self) -> Self` - Sets the foreground color to [`light_blue`](Color::LightBlue).
- `fn on_light_blue(self: Self) -> Self` - Sets the background color to [`light_blue`](Color::LightBlue).
- `fn light_magenta(self: Self) -> Self` - Sets the foreground color to [`light_magenta`](Color::LightMagenta).
- `fn on_light_magenta(self: Self) -> Self` - Sets the background color to [`light_magenta`](Color::LightMagenta).
- `fn light_cyan(self: Self) -> Self` - Sets the foreground color to [`light_cyan`](Color::LightCyan).
- `fn on_light_cyan(self: Self) -> Self` - Sets the background color to [`light_cyan`](Color::LightCyan).
- `fn white(self: Self) -> Self` - Sets the foreground color to [`white`](Color::White).
- `fn on_white(self: Self) -> Self` - Sets the background color to [`white`](Color::White).
- `fn bold(self: Self) -> Self` - Adds the [`bold`](Modifier::BOLD) modifier.
- `fn not_bold(self: Self) -> Self` - Removes the [`bold`](Modifier::BOLD) modifier.
- `fn dim(self: Self) -> Self` - Adds the [`dim`](Modifier::DIM) modifier.
- `fn not_dim(self: Self) -> Self` - Removes the [`dim`](Modifier::DIM) modifier.
- `fn italic(self: Self) -> Self` - Adds the [`italic`](Modifier::ITALIC) modifier.
- `fn not_italic(self: Self) -> Self` - Removes the [`italic`](Modifier::ITALIC) modifier.
- `fn underlined(self: Self) -> Self` - Adds the [`underlined`](Modifier::UNDERLINED) modifier.
- `fn not_underlined(self: Self) -> Self` - Removes the [`underlined`](Modifier::UNDERLINED) modifier.
- `fn slow_blink(self: Self) -> Self` - Adds the [`slow_blink`](Modifier::SLOW_BLINK) modifier.
- `fn not_slow_blink(self: Self) -> Self` - Removes the [`slow_blink`](Modifier::SLOW_BLINK) modifier.
- `fn rapid_blink(self: Self) -> Self` - Adds the [`rapid_blink`](Modifier::RAPID_BLINK) modifier.
- `fn not_rapid_blink(self: Self) -> Self` - Removes the [`rapid_blink`](Modifier::RAPID_BLINK) modifier.
- `fn reversed(self: Self) -> Self` - Adds the [`reversed`](Modifier::REVERSED) modifier.
- `fn not_reversed(self: Self) -> Self` - Removes the [`reversed`](Modifier::REVERSED) modifier.
- `fn hidden(self: Self) -> Self` - Adds the [`hidden`](Modifier::HIDDEN) modifier.
- `fn not_hidden(self: Self) -> Self` - Removes the [`hidden`](Modifier::HIDDEN) modifier.
- `fn crossed_out(self: Self) -> Self` - Adds the [`crossed_out`](Modifier::CROSSED_OUT) modifier.
- `fn not_crossed_out(self: Self) -> Self` - Removes the [`crossed_out`](Modifier::CROSSED_OUT) modifier.

**Traits:** Copy, Eq

**Trait Implementations:**

- **From**
  - `fn from((fg, bg, modifier): (Color, Color, Modifier)) -> Self` - Creates a new `Style` with the given foreground and background colors and modifier added.
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **From**
  - `fn from((fg, modifier): (Color, Modifier)) -> Self` - Creates a new `Style` with the given foreground color and modifier added.
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **From**
  - `fn from(modifier: Modifier) -> Self` - Creates a new `Style` with the given modifier added.
- **From**
  - `fn from((add_modifier, sub_modifier): (Modifier, Modifier)) -> Self` - Creates a new `Style` with the given modifiers added and removed.
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **From**
  - `fn from((fg, bg, add_modifier, sub_modifier): (Color, Color, Modifier, Modifier)) -> Self` - Creates a new `Style` with the given foreground and background colors and modifiers added
- **From**
  - `fn from((fg, bg): (Color, Color)) -> Self` - Creates a new `Style` with the given foreground and background colors.
- **From**
  - `fn from(color: Color) -> Self` - Creates a new `Style` with the given foreground color.
- **Default**
  - `fn default() -> Self`



## Module: palette

A module for defining color palettes.



