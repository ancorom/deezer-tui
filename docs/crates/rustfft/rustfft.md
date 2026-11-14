**rustfft**

# Module: rustfft

## Contents

**Modules**

- [`algorithm`](#algorithm) - Individual FFT algorithms

**Enums**

- [`FftDirection`](#fftdirection) - Represents a FFT direction, IE a forward FFT or an inverse FFT

**Traits**

- [`Direction`](#direction) - A trait that allows FFT algorithms to report whether they compute forward FFTs or inverse FFTs
- [`Fft`](#fft) - Trait for algorithms that compute FFTs.
- [`Length`](#length) - A trait that allows FFT algorithms to report their expected input/output size

---

## rustfft::Direction

*Trait*

A trait that allows FFT algorithms to report whether they compute forward FFTs or inverse FFTs

**Methods:**

- `fft_direction`: Returns FftDirection::Forward if this instance computes forward FFTs, or FftDirection::Inverse for inverse FFTs



## rustfft::Fft

*Trait*

Trait for algorithms that compute FFTs.

This trait has a few methods for computing FFTs. Its most conveinent method is [`process(slice)`](crate::Fft::process).
It takes in a slice of `Complex<T>` and computes a FFT on that slice, in-place. It may copy the data over to internal scratch buffers
if that speeds up the computation, but the output will always end up in the same slice as the input.

**Methods:**

- `process`: Computes a FFT in-place.
- `process_with_scratch`: Divides `buffer` into chunks of size `self.len()`, and computes a FFT on each chunk.
- `process_outofplace_with_scratch`: Divides `input` and `output` into chunks of size `self.len()`, and computes a FFT on each chunk.
- `process_immutable_with_scratch`: Divides `input` and `output` into chunks of `self.len()`, and computes a FFT on each chunk while
- `get_inplace_scratch_len`: Returns the size of the scratch buffer required by `process_with_scratch`
- `get_outofplace_scratch_len`: Returns the size of the scratch buffer required by `process_outofplace_with_scratch`
- `get_immutable_scratch_len`: Returns the size of the scratch buffer required by `process_immutable_with_scratch`



## rustfft::FftDirection

*Enum*

Represents a FFT direction, IE a forward FFT or an inverse FFT

**Variants:**
- `Forward`
- `Inverse`

**Methods:**

- `fn opposite_direction(self: &Self) -> FftDirection` - Returns the opposite direction of `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustfft::Length

*Trait*

A trait that allows FFT algorithms to report their expected input/output size

**Methods:**

- `len`: The FFT size that this algorithm can process



## Module: algorithm

Individual FFT algorithms



