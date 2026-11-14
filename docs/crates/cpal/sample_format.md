**cpal > sample_format**

# Module: sample_format

## Contents

**Enums**

- [`SampleFormat`](#sampleformat) - Format that each sample has. Usually, this corresponds to the sampling

**Traits**

- [`SizedSample`](#sizedsample) - A [`Sample`] type with a known corresponding [`SampleFormat`].

---

## cpal::sample_format::SampleFormat

*Enum*

Format that each sample has. Usually, this corresponds to the sampling
depth of the audio source. For example, 16 bit quantized samples can be
encoded in `i16` or `u16`. Note that the quantized sampling depth is not
directly visible for formats where [`is_float`] is true.

Also note that the backend must support the encoding of the quantized
samples in the given format, as there is no generic transformation from one
format into the other done inside the frontend-library code. You can query
the supported formats by using [`supported_input_configs`].

A good rule of thumb is to use [`SampleFormat::I16`] as this covers typical
music (WAV, MP3) as well as typical audio input devices on most platforms,

[`is_float`]: SampleFormat::is_float
[`supported_input_configs`]: crate::traits::DeviceTrait::supported_input_configs

**Variants:**
- `I8` - `i8` with a valid range of `i8::MIN..=i8::MAX` with `0` being the origin.
- `I16` - `i16` with a valid range of `i16::MIN..=i16::MAX` with `0` being the origin.
- `I24` - `I24` with a valid range of `-(1 << 23)..=((1 << 23) - 1)` with `0` being the origin.
- `I32` - `i32` with a valid range of `i32::MIN..=i32::MAX` with `0` being the origin.
- `I64` - `i64` with a valid range of `i64::MIN..=i64::MAX` with `0` being the origin.
- `U8` - `u8` with a valid range of `u8::MIN..=u8::MAX` with `1 << 7 == 128` being the origin.
- `U16` - `u16` with a valid range of `u16::MIN..=u16::MAX` with `1 << 15 == 32768` being the origin.
- `U24` - `U24` with a valid range of `0..=((1 << 24) - 1)` with `1 << 23 == 8388608` being the origin.
- `U32` - `u32` with a valid range of `u32::MIN..=u32::MAX` with `1 << 31` being the origin.
- `U64` - `U48` with a valid range of '0..(1 << 48)' with `1 << 47` being the origin
- `F32` - `f32` with a valid range of `-1.0..=1.0` with `0.0` being the origin.
- `F64` - `f64` with a valid range of `-1.0..=1.0` with `0.0` being the origin.
- `DsdU8` - DSD 1-bit stream in u8 container (8 bits = 8 DSD samples) with 0x69 being the silence byte pattern.
- `DsdU16` - DSD 1-bit stream in u16 container (16 bits = 16 DSD samples) with 0x69 being the silence byte pattern.
- `DsdU32` - DSD 1-bit stream in u32 container (32 bits = 32 DSD samples) with 0x69 being the silence byte pattern.

**Methods:**

- `fn sample_size(self: &Self) -> usize` - Returns the size in bytes of a sample of this format. This corresponds to
- `fn bits_per_sample(self: &Self) -> u32` - Returns the number of bits of a sample of this format. Note that this is
- `fn is_int(self: &Self) -> bool`
- `fn is_uint(self: &Self) -> bool`
- `fn is_float(self: &Self) -> bool`
- `fn is_dsd(self: &Self) -> bool`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> $crate::cmp::Ordering`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## cpal::sample_format::SizedSample

*Trait*

A [`Sample`] type with a known corresponding [`SampleFormat`].

This trait is automatically implemented for all primitive sample types and provides
a way to determine the [`SampleFormat`] at compile time.

# Example

```
use cpal::SizedSample;

assert_eq!(i16::FORMAT, cpal::SampleFormat::I16);
assert_eq!(f32::FORMAT, cpal::SampleFormat::F32);
```

**Methods:**

- `FORMAT`: The corresponding [`SampleFormat`] for this sample type.



