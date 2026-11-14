**rustfft > algorithm > radix4**

# Module: algorithm::radix4

## Contents

**Structs**

- [`Radix4`](#radix4) - FFT algorithm optimized for power-of-two sizes

---

## rustfft::algorithm::radix4::Radix4

*Struct*

FFT algorithm optimized for power-of-two sizes

~~~
// Computes a forward FFT of size 4096
use rustfft::algorithm::Radix4;
use rustfft::{Fft, FftDirection};
use rustfft::num_complex::Complex;

let mut buffer = vec![Complex{ re: 0.0f32, im: 0.0f32 }; 4096];

let fft = Radix4::new(4096, FftDirection::Forward);
fft.process(&mut buffer);
~~~

**Generic Parameters:**
- T

**Methods:**

- `fn new(len: usize, direction: FftDirection) -> Self` - Preallocates necessary arrays and precomputes necessary data to efficiently compute the power-of-two FFT
- `fn new_with_base(k: u32, base_fft: Arc<dyn Fft>) -> Self` - Constructs a Radix4 instance which computes FFTs of length `4^k * base_fft.len()`

**Trait Implementations:**

- **Length**
  - `fn len(self: &Self) -> usize`
- **Direction**
  - `fn fft_direction(self: &Self) -> FftDirection`
- **Fft**
  - `fn process_immutable_with_scratch(self: &Self, input: &[Complex<T>], output: & mut [Complex<T>], scratch: & mut [Complex<T>])`
  - `fn process_outofplace_with_scratch(self: &Self, input: & mut [Complex<T>], output: & mut [Complex<T>], scratch: & mut [Complex<T>])`
  - `fn process_with_scratch(self: &Self, buffer: & mut [Complex<T>], scratch: & mut [Complex<T>])`
  - `fn get_inplace_scratch_len(self: &Self) -> usize`
  - `fn get_outofplace_scratch_len(self: &Self) -> usize`
  - `fn get_immutable_scratch_len(self: &Self) -> usize`



