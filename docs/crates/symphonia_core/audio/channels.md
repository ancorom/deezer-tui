**symphonia_core > audio > channels**

# Module: audio::channels

## Contents

**Modules**

- [`layouts`](#layouts) - Constants for well known, common, or standardized channel layouts.

**Structs**

- [`Position`](#position) - A bitmask representing positional audio channels.

**Enums**

- [`AmbisonicBFormat`](#ambisonicbformat) - Ambisonic B-format channel components up to the 3rd order.
- [`ChannelLabel`](#channellabel) - A channel label.
- [`Channels`](#channels) - A set of channels.

---

## symphonia_core::audio::channels::AmbisonicBFormat

*Enum*

Ambisonic B-format channel components up to the 3rd order.

The traditional B-format only defines components up to the first order. Second and third-order
components are defined by Furse-Malham.

**Variants:**
- `W` - First-order Ambisonic B-format component W.
- `X` - First-order Ambisonic B-format component X.
- `Y` - First-order Ambisonic B-format component Y.
- `Z` - First-order Ambisonic B-format component Z.
- `R` - Second-order Ambisonic B-format component R.
- `S` - Second-order Ambisonic B-format component S.
- `T` - Second-order Ambisonic B-format component T.
- `U` - Second-order Ambisonic B-format component U.
- `V` - Second-order Ambisonic B-format component V.
- `K` - Third-order Ambisonic B-format component K.
- `L` - Third-order Ambisonic B-format component L.
- `M` - Third-order Ambisonic B-format component M.
- `N` - Third-order Ambisonic B-format component N.
- `O` - Third-order Ambisonic B-format component O.
- `P` - Third-order Ambisonic B-format component P.
- `Q` - Third-order Ambisonic B-format component Q.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Display**
  - `fn fmt(self: &Self, f: & mut std::fmt::Formatter) -> std::fmt::Result`



## symphonia_core::audio::channels::ChannelLabel

*Enum*

A channel label.

**Variants:**
- `Positioned(Position)` - The channel is a positioned channel. Only one bit may be set in the position mask.
- `Discrete(u16)` - The channel is a discrete and independent channel.
- `Ambisonic(u16)` - The channel is an Ambisonic component identified by its Ambisonic Channel Number (ACN).
- `AmbisonicBFormat(AmbisonicBFormat)` - The channel is an Ambisonic B-format channel component.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **From**
  - `fn from(value: AmbisonicBFormat) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **From**
  - `fn from(value: Position) -> Self`



## symphonia_core::audio::channels::Channels

*Enum*

A set of channels.

**Variants:**
- `Positioned(Position)` - Channels are assigned explict speaker positions. A bit set to 1 indicates that the
- `Discrete(u16)` - Channels 0 to the specified count (0..count) are discrete and independent channels.
- `Ambisonic(u8)` - A full Ambisonic channel set of the specified (highest) Ambisonic order.
- `Custom(Box<[ChannelLabel]>)` - Channels are identified by channel labels.
- `None` - No channels.

**Methods:**

- `fn count(self: &Self) -> usize` - Get the total number of channels.
- `fn get_canonical_index_for_positioned_channel(self: &Self, pos: Position) -> Option<usize>` - Gets the canonical buffer index of a positioned channel given a set of positioned channels.

**Traits:** Eq

**Trait Implementations:**

- **From**
  - `fn from(labels: Box<[ChannelLabel]>) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(value: Position) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut std::fmt::Formatter) -> std::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **From**
  - `fn from(labels: Vec<ChannelLabel>) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## symphonia_core::audio::channels::Position

*Struct*

A bitmask representing positional audio channels.

The first 18 channel positions are guaranteed to be identical to those specified by the
channel mask in Microsoft's `WAVEFORMATEXTENSIBLE` structure. Positions after the first 18
are defined by Symphonia and are not in any standardized order.

**Tuple Struct**: `()`

**Methods:**

- `fn iter(self: &Self) -> $crate::iter::Iter<Position>` - Yield a set of contained flags values.
- `fn iter_names(self: &Self) -> $crate::iter::IterNames<Position>` - Yield a set of contained named flags values.
- `fn empty() -> Self` - Get a flags value with all bits unset.
- `fn all() -> Self` - Get a flags value with all known bits set.
- `fn bits(self: &Self) -> u64` - Get the underlying bits value.
- `fn from_bits(bits: u64) -> $crate::__private::core::option::Option<Self>` - Convert from a bits value.
- `fn from_bits_truncate(bits: u64) -> Self` - Convert from a bits value, unsetting any unknown bits.
- `fn from_bits_retain(bits: u64) -> Self` - Convert from a bits value exactly.
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
- `fn from_count(count: u32) -> Option<Position>` - Try to create a position mask with the first `count` positions.
- `fn from_wave_channel_count(count: u32) -> Option<Position>` - Try to convert a WAVE channel count into a position mask.
- `fn from_wave_channel_mask(mask: u32) -> Option<Position>` - Try to convert a WAVE channel mask into a position mask.

**Traits:** Copy, PublicFlags, Eq

**Trait Implementations:**

- **Not**
  - `fn not(self: Self) -> Self` - The bitwise negation (`!`) of the bits in `self`, truncating the result.
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **Sub**
  - `fn sub(self: Self, other: Self) -> Self` - The intersection of `self` with the complement of `other` (`&!`).
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: Self)` - The intersection of `self` with the complement of `other` (`&!`).
- **BitAnd**
  - `fn bitand(self: Self, other: Self) -> Self` - The bitwise and (`&`) of the bits in `self` and `other`.
- **Flags**
  - `fn bits(self: &Self) -> u64`
  - `fn from_bits_retain(bits: u64) -> Position`
  - `fn all_named() -> Position`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: Self)` - The bitwise and (`&`) of the bits in `self` and `other`.
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Display**
  - `fn fmt(self: &Self, f: & mut std::fmt::Formatter) -> std::fmt::Result`
- **BitXor**
  - `fn bitxor(self: Self, other: Self) -> Self` - The bitwise exclusive-or (`^`) of the bits in `self` and `other`.
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: Self)` - The bitwise exclusive-or (`^`) of the bits in `self` and `other`.
- **Extend**
  - `fn extend<T>(self: & mut Self, iterator: T)` - The bitwise or (`|`) of the bits in each flags value.
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FromIterator**
  - `fn from_iter<T>(iterator: T) -> Self` - The bitwise or (`|`) of the bits in each flags value.
- **BitOr**
  - `fn bitor(self: Self, other: Position) -> Self` - The bitwise or (`|`) of the bits in `self` and `other`.
- **Default**
  - `fn default() -> Self`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: Self)` - The bitwise or (`|`) of the bits in `self` and `other`.
- **Binary**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **Octal**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`



## Module: layouts

Constants for well known, common, or standardized channel layouts.

The channel layout constants defined in this module are defined as
[`Channels::Positioned`](super::Channels::Positioned) variants.

## Naming Conventions

 * `FRONT` layouts include the front left, front left-of-center, front right-of-center, and
   front right channels.
 * `WIDE` layouts include all channels from the `FRONT` layout plus the front center
   channel.
 * `SIDE` layouts include the side left and side right channels.
 * `XPY` layouts have `X` standard (non-LFE) channels, and `Y` LFE channels.

## Channel Order

Many standardized channel layouts only differ in the order in which the samples for each
channel appear in an audio frame (interleaved audio), or the order of the audio planes
of each channel (planar audio). These different orderings cannot be represented uniquely by
the [`Position`](super::Position) bitmask. Therefore, all such channel layouts will map
to the same value.

The order of the channels listed in the detailed description of each channel layout are
the order in which the standard describes but is not the order in which Symphonia will store
those channels in a buffer.



