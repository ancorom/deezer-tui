**symphonia_core > audio > conv > dither**

# Module: audio::conv::dither

## Contents

**Structs**

- [`Identity`](#identity) - The `Identity` dithering algorithm performs no dithering and returns the original sample.
- [`RandomNoise`](#randomnoise) - `RandomNoise` represents a sample of noise of a specified length in bits.
- [`Rectangular`](#rectangular) - `Rectangular` implements a dither using uniformly distributed (white) noise without shaping.
- [`Triangular`](#triangular) - `Triangular` implements a dither using a triangular distribution of noise without shaping.

**Enums**

- [`DitherType`](#dithertype) - Enumeration of dither algorithms.

**Traits**

- [`AddNoise`](#addnoise) - `AddNoise` is a trait for converting random noise into a `Sample`.
- [`Dither`](#dither) - `Dither` is a trait for implementing dithering algorithms.
- [`MaybeDither`](#maybedither) - `MaybeDither` conditionally applies a dither to a sample depending on the source and

---

## symphonia_core::audio::conv::dither::AddNoise

*Trait*

`AddNoise` is a trait for converting random noise into a `Sample`.

**Methods:**

- `add_noise`



## symphonia_core::audio::conv::dither::Dither

*Trait*

`Dither` is a trait for implementing dithering algorithms.

**Methods:**

- `dither`: Dithers a `Sample` of source sample format `F` for an eventual conversion to the



## symphonia_core::audio::conv::dither::DitherType

*Enum*

Enumeration of dither algorithms.

**Variants:**
- `Identity` - No dithering.
- `Rectangular` - Apply rectangular dithering. See `Rectangular` for more details.
- `Triangular` - Apply triangular dithering. See `Triangular` for more details.



## symphonia_core::audio::conv::dither::Identity

*Struct*

The `Identity` dithering algorithm performs no dithering and returns the original sample.

**Generic Parameters:**
- F
- T

**Methods:**

- `fn new() -> Self`

**Trait Implementations:**

- **Dither**
  - `fn dither(self: & mut Self, sample: F) -> F`
- **Default**
  - `fn default() -> Self`



## symphonia_core::audio::conv::dither::MaybeDither

*Trait*

`MaybeDither` conditionally applies a dither to a sample depending on the source and
destination sample types.

**Methods:**

- `DITHERABLE`
- `maybe_dither`



## symphonia_core::audio::conv::dither::RandomNoise

*Struct*

`RandomNoise` represents a sample of noise of a specified length in bits.

TODO: `RandomNoise` should be parameterized by the number of bits once const generics land.

**Tuple Struct**: `(i32)`

**Methods:**

- `fn from(random: i32, n_bits: u32) -> Self` - Instantiate a noise sample from a random 32-bit source.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **AddNoise**
  - `fn add_noise(self: Self, s: i16) -> i16`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`
- **AddNoise**
  - `fn add_noise(self: Self, s: u24) -> u24`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> $crate::cmp::Ordering`
- **Default**
  - `fn default() -> Self`
- **AddNoise**
  - `fn add_noise(self: Self, s: f32) -> f32`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **AddNoise**
  - `fn add_noise(self: Self, s: u16) -> u16`
- **AddNoise**
  - `fn add_noise(self: Self, s: i32) -> i32`
- **AddNoise**
  - `fn add_noise(self: Self, s: i8) -> i8`
- **AddNoise**
  - `fn add_noise(self: Self, s: f64) -> f64`
- **AddNoise**
  - `fn add_noise(self: Self, s: u8) -> u8`
- **AddNoise**
  - `fn add_noise(self: Self, s: i24) -> i24`
- **AddNoise**
  - `fn add_noise(self: Self, s: u32) -> u32`



## symphonia_core::audio::conv::dither::Rectangular

*Struct*

`Rectangular` implements a dither using uniformly distributed (white) noise without shaping.

**Generic Parameters:**
- F
- T

**Methods:**

- `fn new() -> Self`

**Trait Implementations:**

- **Dither**
  - `fn dither(self: & mut Self, sample: F) -> F`
- **Default**
  - `fn default() -> Self`



## symphonia_core::audio::conv::dither::Triangular

*Struct*

`Triangular` implements a dither using a triangular distribution of noise without shaping.

**Generic Parameters:**
- F
- T

**Methods:**

- `fn new() -> Self`

**Trait Implementations:**

- **Dither**
  - `fn dither(self: & mut Self, sample: F) -> F`
- **Default**
  - `fn default() -> Self`



