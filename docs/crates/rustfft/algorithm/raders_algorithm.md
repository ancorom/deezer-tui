**rustfft > algorithm > raders_algorithm**

# Module: algorithm::raders_algorithm

## Contents

**Structs**

- [`RadersAlgorithm`](#radersalgorithm) - Implementation of Rader's Algorithm

---

## rustfft::algorithm::raders_algorithm::RadersAlgorithm

*Struct*

Implementation of Rader's Algorithm

This algorithm computes a prime-sized FFT in O(nlogn) time. It does this by converting this size-N FFT into a
size-(N - 1) FFT, which is guaranteed to be composite.

The worst case for this algorithm is when (N - 1) is 2 * prime, resulting in a
[Cunningham Chain](https://en.wikipedia.org/wiki/Cunningham_chain)

~~~
// Computes a forward FFT of size 1201 (prime number), using Rader's Algorithm
use rustfft::algorithm::RadersAlgorithm;
use rustfft::{Fft, FftPlanner};
use rustfft::num_complex::Complex;

let mut buffer = vec![Complex{ re: 0.0f32, im: 0.0f32 }; 1201];

// plan a FFT of size n - 1 = 1200
let mut planner = FftPlanner::new();
let inner_fft = planner.plan_fft_forward(1200);

let fft = RadersAlgorithm::new(inner_fft);
fft.process(&mut buffer);
~~~

Rader's Algorithm is relatively expensive compared to other FFT algorithms. Benchmarking shows that it is up to
an order of magnitude slower than similar composite sizes. In the example size above of 1201, benchmarking shows
that it takes 2.5x more time to compute than a FFT of size 1200.

**Generic Parameters:**
- T

**Methods:**

- `fn new(inner_fft: Arc<dyn Fft>) -> Self` - Creates a FFT instance which will process inputs/outputs of size `inner_fft.len() + 1`.

**Trait Implementations:**

- **Fft**
  - `fn process_immutable_with_scratch(self: &Self, input: &[Complex<T>], output: & mut [Complex<T>], scratch: & mut [Complex<T>])`
  - `fn process_outofplace_with_scratch(self: &Self, input: & mut [Complex<T>], output: & mut [Complex<T>], scratch: & mut [Complex<T>])`
  - `fn process_with_scratch(self: &Self, buffer: & mut [Complex<T>], scratch: & mut [Complex<T>])`
  - `fn get_inplace_scratch_len(self: &Self) -> usize`
  - `fn get_outofplace_scratch_len(self: &Self) -> usize`
  - `fn get_immutable_scratch_len(self: &Self) -> usize`
- **Length**
  - `fn len(self: &Self) -> usize`
- **Direction**
  - `fn fft_direction(self: &Self) -> FftDirection`



