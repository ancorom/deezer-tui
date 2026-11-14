**rustfft > neon > neon_planner**

# Module: neon::neon_planner

## Contents

**Structs**

- [`FftPlannerNeon`](#fftplannerneon) - The Neon FFT planner creates new FFT algorithm instances using a mix of scalar and Neon accelerated algorithms.

---

## rustfft::neon::neon_planner::FftPlannerNeon

*Struct*

The Neon FFT planner creates new FFT algorithm instances using a mix of scalar and Neon accelerated algorithms.
It is supported when using the 64-bit AArch64 instruction set.

RustFFT has several FFT algorithms available. For a given FFT size, the `FftPlannerNeon` decides which of the
available FFT algorithms to use and then initializes them.

~~~
// Perform a forward Fft of size 1234
use std::sync::Arc;
use rustfft::{FftPlannerNeon, num_complex::Complex};

if let Ok(mut planner) = FftPlannerNeon::new() {
  let fft = planner.plan_fft_forward(1234);

  let mut buffer = vec![Complex{ re: 0.0f32, im: 0.0f32 }; 1234];
  fft.process(&mut buffer);

  // The FFT instance returned by the planner has the type `Arc<dyn Fft<T>>`,
  // where T is the numeric type, ie f32 or f64, so it's cheap to clone
  let fft_clone = Arc::clone(&fft);
}
~~~

If you plan on creating multiple FFT instances, it is recommended to reuse the same planner for all of them. This
is because the planner re-uses internal data across FFT instances wherever possible, saving memory and reducing
setup time. (FFT instances created with one planner will never re-use data and buffers with FFT instances created
by a different planner)

Each FFT instance owns [`Arc`s](std::sync::Arc) to its internal data, rather than borrowing it from the planner, so it's perfectly
safe to drop the planner after creating Fft instances.

**Generic Parameters:**
- T

**Methods:**

- `fn new() -> Result<Self, ()>` - Creates a new `FftPlannerNeon` instance.
- `fn plan_fft(self: & mut Self, _len: usize, _direction: FftDirection) -> Arc<dyn Fft>` - Returns a `Fft` instance which uses Neon instructions to compute FFTs of size `len`.
- `fn plan_fft_forward(self: & mut Self, _len: usize) -> Arc<dyn Fft>` - Returns a `Fft` instance which uses Neon instructions to compute forward FFTs of size `len`.
- `fn plan_fft_inverse(self: & mut Self, _len: usize) -> Arc<dyn Fft>` - Returns a `Fft` instance which uses Neon instructions to compute inverse FFTs of size `len.



