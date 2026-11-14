**rustfft > algorithm > mixed_radix**

# Module: algorithm::mixed_radix

## Contents

**Structs**

- [`MixedRadix`](#mixedradix) - Implementation of the Mixed-Radix FFT algorithm
- [`MixedRadixSmall`](#mixedradixsmall) - Implementation of the Mixed-Radix FFT algorithm, specialized for smaller input sizes

---

## rustfft::algorithm::mixed_radix::MixedRadix

*Struct*

Implementation of the Mixed-Radix FFT algorithm

This algorithm factors a size n FFT into n1 * n2, computes several inner FFTs of size n1 and n2, then combines the
results to get the final answer

~~~
// Computes a forward FFT of size 1200, using the Mixed-Radix Algorithm
use rustfft::algorithm::MixedRadix;
use rustfft::{Fft, FftPlanner};
use rustfft::num_complex::Complex;

let mut buffer = vec![Complex{ re: 0.0f32, im: 0.0f32 }; 1200];

// we need to find an n1 and n2 such that n1 * n2 == 1200
// n1 = 30 and n2 = 40 satisfies this
let mut planner = FftPlanner::new();
let inner_fft_n1 = planner.plan_fft_forward(30);
let inner_fft_n2 = planner.plan_fft_forward(40);

// the mixed radix FFT length will be inner_fft_n1.len() * inner_fft_n2.len() = 1200
let fft = MixedRadix::new(inner_fft_n1, inner_fft_n2);
fft.process(&mut buffer);
~~~

**Generic Parameters:**
- T

**Methods:**

- `fn new(width_fft: Arc<dyn Fft>, height_fft: Arc<dyn Fft>) -> Self` - Creates a FFT instance which will process inputs/outputs of size `width_fft.len() * height_fft.len()`

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



## rustfft::algorithm::mixed_radix::MixedRadixSmall

*Struct*

Implementation of the Mixed-Radix FFT algorithm, specialized for smaller input sizes

This algorithm factors a size n FFT into n1 * n2, computes several inner FFTs of size n1 and n2, then combines the
results to get the final answer

~~~
// Computes a forward FFT of size 40 using MixedRadixSmall
use std::sync::Arc;
use rustfft::algorithm::MixedRadixSmall;
use rustfft::algorithm::butterflies::{Butterfly5, Butterfly8};
use rustfft::{Fft, FftDirection};
use rustfft::num_complex::Complex;

let len = 40;

let mut buffer = vec![Complex{ re: 0.0f32, im: 0.0f32 }; len];

// we need to find an n1 and n2 such that n1 * n2 == 40
// n1 = 5 and n2 = 8 satisfies this
let inner_fft_n1 = Arc::new(Butterfly5::new(FftDirection::Forward));
let inner_fft_n2 = Arc::new(Butterfly8::new(FftDirection::Forward));

// the mixed radix FFT length will be inner_fft_n1.len() * inner_fft_n2.len() = 40
let fft = MixedRadixSmall::new(inner_fft_n1, inner_fft_n2);
fft.process(&mut buffer);
~~~

**Generic Parameters:**
- T

**Methods:**

- `fn new(width_fft: Arc<dyn Fft>, height_fft: Arc<dyn Fft>) -> Self` - Creates a FFT instance which will process inputs/outputs of size `width_fft.len() * height_fft.len()`

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



