**rustfft > algorithm > dft**

# Module: algorithm::dft

## Contents

**Structs**

- [`Dft`](#dft) - Naive O(n^2 ) Discrete Fourier Transform implementation

---

## rustfft::algorithm::dft::Dft

*Struct*

Naive O(n^2 ) Discrete Fourier Transform implementation

This implementation is primarily used to test other FFT algorithms.

~~~
// Computes a naive DFT of size 123
use rustfft::algorithm::Dft;
use rustfft::{Fft, FftDirection};
use rustfft::num_complex::Complex;

let mut buffer = vec![Complex{ re: 0.0f32, im: 0.0f32 }; 123];

let dft = Dft::new(123, FftDirection::Forward);
dft.process(&mut buffer);
~~~

**Generic Parameters:**
- T

**Methods:**

- `fn new(len: usize, direction: FftDirection) -> Self` - Preallocates necessary arrays and precomputes necessary data to efficiently compute Dft

**Trait Implementations:**

- **Direction**
  - `fn fft_direction(self: &Self) -> FftDirection`
- **Fft**
  - `fn process_immutable_with_scratch(self: &Self, input: &[Complex<T>], output: & mut [Complex<T>], scratch: & mut [Complex<T>])`
  - `fn process_outofplace_with_scratch(self: &Self, input: & mut [Complex<T>], output: & mut [Complex<T>], scratch: & mut [Complex<T>])`
  - `fn process_with_scratch(self: &Self, buffer: & mut [Complex<T>], scratch: & mut [Complex<T>])`
  - `fn get_inplace_scratch_len(self: &Self) -> usize`
  - `fn get_outofplace_scratch_len(self: &Self) -> usize`
  - `fn get_immutable_scratch_len(self: &Self) -> usize`
- **Length**
  - `fn len(self: &Self) -> usize`



