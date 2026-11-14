**rustfft > algorithm > bluesteins_algorithm**

# Module: algorithm::bluesteins_algorithm

## Contents

**Structs**

- [`BluesteinsAlgorithm`](#bluesteinsalgorithm) - Implementation of Bluestein's Algorithm

---

## rustfft::algorithm::bluesteins_algorithm::BluesteinsAlgorithm

*Struct*

Implementation of Bluestein's Algorithm

This algorithm computes an arbitrary-sized FFT in O(nlogn) time. It does this by converting this size-N FFT into a
size-M FFT where M >= 2N - 1.

The choice of M is very important for the performance of Bluestein's Algorithm. The most obvious choice is the next-largest
power of two -- but if there's a smaller/faster FFT size that satisfies the `>= 2N - 1` requirement, that will significantly
improve this algorithm's overall performance.

~~~
// Computes a forward FFT of size 1201, using Bluestein's Algorithm
use rustfft::algorithm::BluesteinsAlgorithm;
use rustfft::{Fft, FftPlanner};
use rustfft::num_complex::Complex;

let mut buffer = vec![Complex{ re: 0.0f32, im: 0.0f32 }; 1201];

// We need to find an inner FFT whose size is greater than 1201*2 - 1.
// The size 2401 (7^4) satisfies this requirement, while also being relatively fast.
let mut planner = FftPlanner::new();
let inner_fft = planner.plan_fft_forward(2401);

let fft = BluesteinsAlgorithm::new(1201, inner_fft);
fft.process(&mut buffer);
~~~

Bluesteins's Algorithm is relatively expensive compared to other FFT algorithms. Benchmarking shows that it is up to
an order of magnitude slower than similar composite sizes. In the example size above of 1201, benchmarking shows
that it takes 5x more time to compute than computing a FFT of size 1200 via a step of MixedRadix.

**Generic Parameters:**
- T

**Methods:**

- `fn new(len: usize, inner_fft: Arc<dyn Fft>) -> Self` - Creates a FFT instance which will process inputs/outputs of size `len`. `inner_fft.len()` must be >= `len * 2 - 1`

**Trait Implementations:**

- **Fft**
  - `fn process_immutable_with_scratch(self: &Self, input: &[Complex<T>], output: & mut [Complex<T>], scratch: & mut [Complex<T>])`
  - `fn process_outofplace_with_scratch(self: &Self, input: & mut [Complex<T>], output: & mut [Complex<T>], scratch: & mut [Complex<T>])`
  - `fn process_with_scratch(self: &Self, buffer: & mut [Complex<T>], scratch: & mut [Complex<T>])`
  - `fn get_inplace_scratch_len(self: &Self) -> usize`
  - `fn get_outofplace_scratch_len(self: &Self) -> usize`
  - `fn get_immutable_scratch_len(self: &Self) -> usize`
- **Direction**
  - `fn fft_direction(self: &Self) -> FftDirection`
- **Length**
  - `fn len(self: &Self) -> usize`



