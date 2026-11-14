**rustfft > algorithm > good_thomas_algorithm**

# Module: algorithm::good_thomas_algorithm

## Contents

**Structs**

- [`GoodThomasAlgorithm`](#goodthomasalgorithm) - Implementation of the [Good-Thomas Algorithm (AKA Prime Factor Algorithm)](https://en.wikipedia.org/wiki/Prime-factor_FFT_algorithm)
- [`GoodThomasAlgorithmSmall`](#goodthomasalgorithmsmall) - Implementation of the Good-Thomas Algorithm, specialized for smaller input sizes

---

## rustfft::algorithm::good_thomas_algorithm::GoodThomasAlgorithm

*Struct*

Implementation of the [Good-Thomas Algorithm (AKA Prime Factor Algorithm)](https://en.wikipedia.org/wiki/Prime-factor_FFT_algorithm)

This algorithm factors a size n FFT into n1 * n2, where GCD(n1, n2) == 1

Conceptually, this algorithm is very similar to the Mixed-Radix, except because GCD(n1, n2) == 1 we can do some
number theory trickery to reduce the number of floating-point multiplications and additions. Additionally, It can
be faster than Mixed-Radix at sizes below 10,000 or so.

~~~
// Computes a forward FFT of size 1200, using the Good-Thomas Algorithm
use rustfft::algorithm::GoodThomasAlgorithm;
use rustfft::{Fft, FftPlanner};
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;

let mut buffer = vec![Complex{ re: 0.0f32, im: 0.0f32 }; 1200];

// we need to find an n1 and n2 such that n1 * n2 == 1200 and GCD(n1, n2) == 1
// n1 = 48 and n2 = 25 satisfies this
let mut planner = FftPlanner::new();
let inner_fft_n1 = planner.plan_fft_forward(48);
let inner_fft_n2 = planner.plan_fft_forward(25);

// the good-thomas FFT length will be inner_fft_n1.len() * inner_fft_n2.len() = 1200
let fft = GoodThomasAlgorithm::new(inner_fft_n1, inner_fft_n2);
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



## rustfft::algorithm::good_thomas_algorithm::GoodThomasAlgorithmSmall

*Struct*

Implementation of the Good-Thomas Algorithm, specialized for smaller input sizes

This algorithm factors a size n FFT into n1 * n2, where GCD(n1, n2) == 1

Conceptually, this algorithm is very similar to MixedRadix, except because GCD(n1, n2) == 1 we can do some
number theory trickery to reduce the number of floating point operations. It typically performs
better than MixedRadixSmall, especially at the smallest sizes.

~~~
// Computes a forward FFT of size 56 using GoodThomasAlgorithmSmall
use std::sync::Arc;
use rustfft::algorithm::GoodThomasAlgorithmSmall;
use rustfft::algorithm::butterflies::{Butterfly7, Butterfly8};
use rustfft::{Fft, FftDirection};
use rustfft::num_complex::Complex;

let mut buffer = vec![Complex{ re: 0.0f32, im: 0.0f32 }; 56];

// we need to find an n1 and n2 such that n1 * n2 == 56 and GCD(n1, n2) == 1
// n1 = 7 and n2 = 8 satisfies this
let inner_fft_n1 = Arc::new(Butterfly7::new(FftDirection::Forward));
let inner_fft_n2 = Arc::new(Butterfly8::new(FftDirection::Forward));

// the good-thomas FFT length will be inner_fft_n1.len() * inner_fft_n2.len() = 56
let fft = GoodThomasAlgorithmSmall::new(inner_fft_n1, inner_fft_n2);
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



