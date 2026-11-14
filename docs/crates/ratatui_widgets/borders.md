**ratatui_widgets > borders**

# Module: borders

## Contents

**Structs**

- [`Borders`](#borders) - Bitflags that can be composed to set the visible borders essentially on the block widget.

**Enums**

- [`BorderType`](#bordertype) - The type of border of a [`Block`](crate::block::Block).

---

## ratatui_widgets::borders::BorderType

*Enum*

The type of border of a [`Block`](crate::block::Block).

See the [`borders`](crate::block::Block::borders) method of `Block` to configure its borders.

**Variants:**
- `Plain` - A plain, simple border.
- `Rounded` - A plain border with rounded corners.
- `Double` - A doubled border.
- `Thick` - A thick border.
- `LightDoubleDashed` - A light double-dashed border.
- `HeavyDoubleDashed` - A heavy double-dashed border.
- `LightTripleDashed` - A light triple-dashed border.
- `HeavyTripleDashed` - A heavy triple-dashed border.
- `LightQuadrupleDashed` - A light quadruple-dashed border.
- `HeavyQuadrupleDashed` - A heavy quadruple-dashed border.
- `QuadrantInside` - A border with a single line on the inside of a half block.
- `QuadrantOutside` - A border with a single line on the outside of a half block.

**Methods:**

- `fn border_symbols<'a>(border_type: Self) -> border::Set<'a>` - Convert this `BorderType` into the corresponding [`Set`](border::Set) of border symbols.
- `fn to_border_set<'a>(self: Self) -> border::Set<'a>` - Convert this `BorderType` into the corresponding [`Set`](border::Set) of border symbols.

**Traits:** Copy, Eq

**Trait Implementations:**

- **FromStr**
  - `fn from_str(s: &str) -> ::core::result::Result<BorderType, <Self as ::core::str::FromStr>::Err>`
- **TryFrom**
  - `fn try_from(s: &str) -> ::core::result::Result<BorderType, <Self as ::core::convert::TryFrom>::Error>`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::result::Result<(), ::core::fmt::Error>`



## ratatui_widgets::borders::Borders

*Struct*

Bitflags that can be composed to set the visible borders essentially on the block widget.

**Tuple Struct**: `()`

**Methods:**

- `fn iter(self: &Self) -> $crate::iter::Iter<Borders>` - Yield a set of contained flags values.
- `fn iter_names(self: &Self) -> $crate::iter::IterNames<Borders>` - Yield a set of contained named flags values.
- `fn empty() -> Self` - Get a flags value with all bits unset.
- `fn all() -> Self` - Get a flags value with all known bits set.
- `fn bits(self: &Self) -> u8` - Get the underlying bits value.
- `fn from_bits(bits: u8) -> $crate::__private::core::option::Option<Self>` - Convert from a bits value.
- `fn from_bits_truncate(bits: u8) -> Self` - Convert from a bits value, unsetting any unknown bits.
- `fn from_bits_retain(bits: u8) -> Self` - Convert from a bits value exactly.
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

**Traits:** Eq, PublicFlags, Copy

**Trait Implementations:**

- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: Self)` - The bitwise exclusive-or (`^`) of the bits in `self` and `other`.
- **Sub**
  - `fn sub(self: Self, other: Self) -> Self` - The intersection of `self` with the complement of `other` (`&!`).
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`
- **Extend**
  - `fn extend<T>(self: & mut Self, iterator: T)` - The bitwise or (`|`) of the bits in each flags value.
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: Self)` - The intersection of `self` with the complement of `other` (`&!`).
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **FromIterator**
  - `fn from_iter<T>(iterator: T) -> Self` - The bitwise or (`|`) of the bits in each flags value.
- **BitOr**
  - `fn bitor(self: Self, other: Borders) -> Self` - The bitwise or (`|`) of the bits in `self` and `other`.
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: Self)` - The bitwise or (`|`) of the bits in `self` and `other`.
- **BitAnd**
  - `fn bitand(self: Self, other: Self) -> Self` - The bitwise and (`&`) of the bits in `self` and `other`.
- **Binary**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **Flags**
  - `fn bits(self: &Self) -> u8`
  - `fn from_bits_retain(bits: u8) -> Borders`
  - `fn all_named() -> Borders`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: Self)` - The bitwise and (`&`) of the bits in `self` and `other`.
- **Octal**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **Not**
  - `fn not(self: Self) -> Self` - The bitwise negation (`!`) of the bits in `self`, truncating the result.
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result` - Display the Borders bitflags as a list of names.
- **BitXor**
  - `fn bitxor(self: Self, other: Self) -> Self` - The bitwise exclusive-or (`^`) of the bits in `self` and `other`.
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



