**symphonia_core > audio > conv**

# Module: audio::conv

## Contents

**Modules**

- [`dither`](#dither) - The `dither` module provides methods to apply a dither to a sample.

**Traits**

- [`ConvertibleSample`](#convertiblesample)
- [`FromSample`](#fromsample) - `FromSample` implements a conversion from `Sample` type `F` to `Self`.
- [`IntoSample`](#intosample) - `IntoSample` implements a conversion from `Self` to `Sample` type `T`.
- [`ReversibleSample`](#reversiblesample) - `ReversibleSample` is a trait that when implemented for `Self`, that `Sample` type implements

---

## symphonia_core::audio::conv::ConvertibleSample

*Trait*



## symphonia_core::audio::conv::FromSample

*Trait*

`FromSample` implements a conversion from `Sample` type `F` to `Self`.

This may be a lossy conversion if converting from a sample type of higher precision to one of
lower precision. No dithering is applied.

**Methods:**

- `from_sample`



## symphonia_core::audio::conv::IntoSample

*Trait*

`IntoSample` implements a conversion from `Self` to `Sample` type `T`.

This may be a lossy conversion if converting from a sample type of higher precision to one of
lower precision. No dithering is applied.

**Methods:**

- `into_sample`



## symphonia_core::audio::conv::ReversibleSample

*Trait*

`ReversibleSample` is a trait that when implemented for `Self`, that `Sample` type implements
reversible conversions between `Self` and the parameterized `Sample` type `S`.



## Module: dither

The `dither` module provides methods to apply a dither to a sample.

Dithering is the process of adding noise to the least significant digits of a sample before
down-converting (quantizing) it to a smaller sample type. The purpose of dithering is to
decorrelate the quantization error of the down-conversion from the source signal.

Dithering is only applied on lossy conversions. Therefore the `dither` module will only
apply a dither to the following down-conversions:

* { `i32`, `u32` } to { `i24`, `u24`, `i16`, `u16`, `i8`, `u8` }
* { `i24`, `u24` } to { `i16`, `u16`, `i8`, `u8` }
* { `i16`, `u16` } to { `i8`, `u8` }

Multiple dithering algorithms are provided, each drawing noise from a different probability
distribution. In addition to different distributions, a dithering algorithm may also shape
the noise such that the bulk of the noise is placed in an inaudible frequency range.



