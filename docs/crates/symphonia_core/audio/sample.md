**symphonia_core > audio > sample**

# Module: audio::sample

## Contents

**Structs**

- [`i24`](#i24) - A signed 24-bit integer sample with an internal signed 32-bit integer representation.
- [`u24`](#u24) - An unsigned 24-bit integer sample with an internal unsigned 32-bit integer representation.

**Enums**

- [`SampleFormat`](#sampleformat) - An enumeration of standard sample formats.

**Traits**

- [`Sample`](#sample) - The sample trait defines the mandatory operations and attributes an audio sample data type must
- [`SampleBytes`](#samplebytes) - `SampleBytes` provides interfaces to get the packed byte representation of a `Sample`.

---

## symphonia_core::audio::sample::Sample

*Trait*

The sample trait defines the mandatory operations and attributes an audio sample data type must
implement and provide.

**Methods:**

- `EFF_BITS`: The effective number of bits of the valid (clamped) sample range. Quantifies the dynamic
- `MID`: The mid-point value between the maximum and minimum sample value. If a sample is set to this
- `clamped`: If the sample format does not use the full range of the underlying data type, returns the



## symphonia_core::audio::sample::SampleBytes

*Trait*

`SampleBytes` provides interfaces to get the packed byte representation of a `Sample`.

The packed byte representation is the byte representation of a sample in its native stream
format. This may differ from its in-memory working representation.

**Methods:**

- `RawType`: The data type that stores the packed byte representation of the sample.
- `to_ne_sample_bytes`: Return the sample in its packed byte representation in the native byte order.
- `to_le_sample_bytes`: Return the sample in its packed byte representation in the little-endian byte order.
- `to_be_sample_bytes`: Return the sample in its packed byte representation in the big-endian byte order.



## symphonia_core::audio::sample::SampleFormat

*Enum*

An enumeration of standard sample formats.

**Variants:**
- `U8` - Unsigned 8-bit integer.
- `U16` - Unsigned 16-bit integer.
- `U24` - Unsigned 24-bit integer.
- `U32` - Unsigned 32-bit integer.
- `S8` - Signed 8-bit integer.
- `S16` - Signed 16-bit integer.
- `S24` - Signed 24-bit integer.
- `S32` - Signed 32-bit integer.
- `F32` - Single precision (32-bit) floating point.
- `F64` - Double precision (64-bit) floating point.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## symphonia_core::audio::sample::i24

*Struct*

A signed 24-bit integer sample with an internal signed 32-bit integer representation.

There are **no** guarantees the sample is within the valid range 24-bit range. Use the
[`Sample::clamped`] function to clamp the sample to the valid range.

**Tuple Struct**: `(i32)`

**Methods:**

- `fn inner(self: Self) -> i32` - Get the underlying `i32` backing this `i24`.
- `fn to_le_bytes(self: Self) -> [u8; 3]` - Return the memory representation of this `i24` as a byte array in little-endian byte order.
- `fn to_be_bytes(self: Self) -> [u8; 3]` - Return the memory representation of this `i24` as a byte array in big-endian byte order.
- `fn to_ne_bytes(self: Self) -> [u8; 3]` - Return the memory representation of this `i24` as a byte array in native byte order.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Shl**
  - `fn shl(self: Self, other: i32) -> i24`
- **Shl**
  - `fn shl(self: Self, other: Self) -> Self`
- **MaybeDither**
  - `fn maybe_dither<D>(self: Self, _: & mut D) -> Self`
- **Shr**
  - `fn shr(self: Self, other: i32) -> i24`
- **Shr**
  - `fn shr(self: Self, other: Self) -> Self`
- **FromSample**
  - `fn from_sample(s: u32) -> Self`
- **MaybeDither**
  - `fn maybe_dither<D>(self: Self, dither: & mut D) -> Self`
- **Shl**
  - `fn shl(self: Self, other: i64) -> i24`
- **Shl**
  - `fn shl(self: Self, other: u8) -> i24`
- **Default**
  - `fn default() -> Self`
- **Shr**
  - `fn shr(self: Self, other: i64) -> i24`
- **FromSample**
  - `fn from_sample(s: i8) -> Self`
- **Shr**
  - `fn shr(self: Self, other: u8) -> i24`
- **MaybeDither**
  - `fn maybe_dither<D>(self: Self, _: & mut D) -> Self`
- **Sample**
  - `fn clamped(self: Self) -> Self`
- **Shl**
  - `fn shl(self: Self, other: i128) -> i24`
- **Shl**
  - `fn shl(self: Self, other: u16) -> i24`
- **From**
  - `fn from(val: i16) -> Self`
- **FromSample**
  - `fn from_sample(s: i32) -> Self`
- **Shr**
  - `fn shr(self: Self, other: i128) -> i24`
- **Shr**
  - `fn shr(self: Self, other: u16) -> i24`
- **MaybeDither**
  - `fn maybe_dither<D>(self: Self, dither: & mut D) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Shl**
  - `fn shl(self: Self, other: isize) -> i24`
- **Shl**
  - `fn shl(self: Self, other: u32) -> i24`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **FromSample**
  - `fn from_sample(s: u24) -> Self`
- **Shr**
  - `fn shr(self: Self, other: isize) -> i24`
- **MaybeDither**
  - `fn maybe_dither<D>(self: Self, _: & mut D) -> Self`
- **Shr**
  - `fn shr(self: Self, other: u32) -> i24`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`
- **BitAnd**
  - `fn bitand(self: Self, other: Self) -> Self`
- **Shl**
  - `fn shl(self: Self, other: u64) -> i24`
- **FromSample**
  - `fn from_sample(s: f64) -> Self`
- **MaybeDither**
  - `fn maybe_dither<D>(self: Self, _: & mut D) -> Self`
- **BitOr**
  - `fn bitor(self: Self, other: Self) -> Self`
- **Shr**
  - `fn shr(self: Self, other: u64) -> i24`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **BitXor**
  - `fn bitxor(self: Self, other: Self) -> Self`
- **Shl**
  - `fn shl(self: Self, other: u128) -> i24`
- **FromSample**
  - `fn from_sample(s: i24) -> Self`
- **Shr**
  - `fn shr(self: Self, other: u128) -> i24`
- **From**
  - `fn from(val: i8) -> Self`
- **MaybeDither**
  - `fn maybe_dither<D>(self: Self, dither: & mut D) -> Self`
- **Shl**
  - `fn shl(self: Self, other: usize) -> i24`
- **FromSample**
  - `fn from_sample(s: u16) -> Self`
- **Add**
  - `fn add(self: Self, other: Self) -> Self`
- **Shr**
  - `fn shr(self: Self, other: usize) -> i24`
- **Sub**
  - `fn sub(self: Self, other: Self) -> Self`
- **MaybeDither**
  - `fn maybe_dither<D>(self: Self, _: & mut D) -> Self`
- **FromSample**
  - `fn from_sample(s: f32) -> Self`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> $crate::cmp::Ordering`
- **Shl**
  - `fn shl(self: Self, other: i8) -> i24`
- **Mul**
  - `fn mul(self: Self, other: Self) -> Self`
- **Shr**
  - `fn shr(self: Self, other: i8) -> i24`
- **MaybeDither**
  - `fn maybe_dither<D>(self: Self, dither: & mut D) -> Self`
- **Div**
  - `fn div(self: Self, other: Self) -> Self`
- **FromSample**
  - `fn from_sample(s: i16) -> Self`
- **From**
  - `fn from(val: i32) -> Self`
- **Shl**
  - `fn shl(self: Self, other: i16) -> i24`
- **Not**
  - `fn not(self: Self) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **MaybeDither**
  - `fn maybe_dither<D>(self: Self, _: & mut D) -> Self`
- **Shr**
  - `fn shr(self: Self, other: i16) -> i24`
- **Rem**
  - `fn rem(self: Self, other: Self) -> Self`
- **FromSample**
  - `fn from_sample(s: u8) -> Self`
- **SampleBytes**
  - `fn to_ne_sample_bytes(self: Self) -> <Self as >::RawType`
  - `fn to_le_sample_bytes(self: Self) -> <Self as >::RawType`
  - `fn to_be_sample_bytes(self: Self) -> <Self as >::RawType`



## symphonia_core::audio::sample::u24

*Struct*

An unsigned 24-bit integer sample with an internal unsigned 32-bit integer representation.

There are **no** guarantees the sample is within the valid range 24-bit range. Use the
[`Sample::clamped`] function to clamp the sample to the valid range.

**Tuple Struct**: `(u32)`

**Methods:**

- `fn inner(self: Self) -> u32` - Get the underlying `u32` backing this `u24`.
- `fn to_le_bytes(self: Self) -> [u8; 3]` - Return the memory representation of this `u24` as a byte array in little-endian byte order.
- `fn to_be_bytes(self: Self) -> [u8; 3]` - Return the memory representation of this `u24` as a byte array in big-endian byte order.
- `fn to_ne_bytes(self: Self) -> [u8; 3]` - Return the memory representation of this `u24` as a byte array in native byte order.

**Traits:** Eq, Copy

**Trait Implementations:**

- **FromSample**
  - `fn from_sample(s: f64) -> Self`
- **Shl**
  - `fn shl(self: Self, other: u32) -> u24`
- **Sub**
  - `fn sub(self: Self, other: Self) -> Self`
- **Shr**
  - `fn shr(self: Self, other: Self) -> Self`
- **Shr**
  - `fn shr(self: Self, other: isize) -> u24`
- **Shr**
  - `fn shr(self: Self, other: u32) -> u24`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> $crate::cmp::Ordering`
- **MaybeDither**
  - `fn maybe_dither<D>(self: Self, dither: & mut D) -> Self`
- **Mul**
  - `fn mul(self: Self, other: Self) -> Self`
- **FromSample**
  - `fn from_sample(s: i24) -> Self`
- **BitAnd**
  - `fn bitand(self: Self, other: Self) -> Self`
- **Shl**
  - `fn shl(self: Self, other: u64) -> u24`
- **Div**
  - `fn div(self: Self, other: Self) -> Self`
- **BitOr**
  - `fn bitor(self: Self, other: Self) -> Self`
- **Shr**
  - `fn shr(self: Self, other: u64) -> u24`
- **From**
  - `fn from(val: u32) -> Self`
- **MaybeDither**
  - `fn maybe_dither<D>(self: Self, _: & mut D) -> Self`
- **Not**
  - `fn not(self: Self) -> Self`
- **FromSample**
  - `fn from_sample(s: u16) -> Self`
- **BitXor**
  - `fn bitxor(self: Self, other: Self) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Shl**
  - `fn shl(self: Self, other: u128) -> u24`
- **Rem**
  - `fn rem(self: Self, other: Self) -> Self`
- **Shr**
  - `fn shr(self: Self, other: u128) -> u24`
- **MaybeDither**
  - `fn maybe_dither<D>(self: Self, _: & mut D) -> Self`
- **FromSample**
  - `fn from_sample(s: f32) -> Self`
- **Shl**
  - `fn shl(self: Self, other: Self) -> Self`
- **Shl**
  - `fn shl(self: Self, other: usize) -> u24`
- **Shr**
  - `fn shr(self: Self, other: usize) -> u24`
- **FromSample**
  - `fn from_sample(s: i16) -> Self`
- **MaybeDither**
  - `fn maybe_dither<D>(self: Self, dither: & mut D) -> Self`
- **Default**
  - `fn default() -> Self`
- **Shl**
  - `fn shl(self: Self, other: i8) -> u24`
- **Sample**
  - `fn clamped(self: Self) -> Self`
- **FromSample**
  - `fn from_sample(s: u8) -> Self`
- **Shr**
  - `fn shr(self: Self, other: i8) -> u24`
- **MaybeDither**
  - `fn maybe_dither<D>(self: Self, _: & mut D) -> Self`
- **From**
  - `fn from(val: u16) -> Self`
- **Shl**
  - `fn shl(self: Self, other: i16) -> u24`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FromSample**
  - `fn from_sample(s: u32) -> Self`
- **Shr**
  - `fn shr(self: Self, other: i16) -> u24`
- **MaybeDither**
  - `fn maybe_dither<D>(self: Self, dither: & mut D) -> Self`
- **SampleBytes**
  - `fn to_ne_sample_bytes(self: Self) -> <Self as >::RawType`
  - `fn to_le_sample_bytes(self: Self) -> <Self as >::RawType`
  - `fn to_be_sample_bytes(self: Self) -> <Self as >::RawType`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Shl**
  - `fn shl(self: Self, other: i32) -> u24`
- **FromSample**
  - `fn from_sample(s: i8) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Shr**
  - `fn shr(self: Self, other: i32) -> u24`
- **MaybeDither**
  - `fn maybe_dither<D>(self: Self, _: & mut D) -> Self`
- **Shl**
  - `fn shl(self: Self, other: i64) -> u24`
- **Shl**
  - `fn shl(self: Self, other: u8) -> u24`
- **FromSample**
  - `fn from_sample(s: i32) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Shr**
  - `fn shr(self: Self, other: i64) -> u24`
- **MaybeDither**
  - `fn maybe_dither<D>(self: Self, _: & mut D) -> Self`
- **Shr**
  - `fn shr(self: Self, other: u8) -> u24`
- **MaybeDither**
  - `fn maybe_dither<D>(self: Self, dither: & mut D) -> Self`
- **Shl**
  - `fn shl(self: Self, other: i128) -> u24`
- **Shl**
  - `fn shl(self: Self, other: u16) -> u24`
- **FromSample**
  - `fn from_sample(s: u24) -> Self`
- **From**
  - `fn from(val: u8) -> Self`
- **Shr**
  - `fn shr(self: Self, other: i128) -> u24`
- **Shr**
  - `fn shr(self: Self, other: u16) -> u24`
- **Add**
  - `fn add(self: Self, other: Self) -> Self`
- **MaybeDither**
  - `fn maybe_dither<D>(self: Self, _: & mut D) -> Self`
- **Shl**
  - `fn shl(self: Self, other: isize) -> u24`



