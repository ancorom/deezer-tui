**rustfft > algorithm > radix3**

# Module: algorithm::radix3

## Contents

**Structs**

- [`Radix3`](#radix3) - FFT algorithm optimized for power-of-three sizes

---

## rustfft::algorithm::radix3::Radix3

*Struct*

FFT algorithm optimized for power-of-three sizes

~~~
// Computes a forward FFT of size 2187
use rustfft::algorithm::Radix3;
use rustfft::{Fft, FftDirection};
use rustfft::num_complex::Complex;

let mut buffer = vec![Complex{ re: 0.0f32, im: 0.0f32 }; 2187];

let fft = Radix3::new(2187, FftDirection::Forward);
fft.process(&mut buffer);
~~~

**Generic Parameters:**
- T

**Methods:**

- `fn new(len: usize, direction: FftDirection) -> Self` - Preallocates necessary arrays and precomputes necessary data to efficiently compute the power-of-three FFT
- `fn new_with_base(k: u32, base_fft: Arc<dyn Fft>) -> Self` - Constructs a Radix3 instance which computes FFTs of length `3^k * base_fft.len()`

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



