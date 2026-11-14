**rustfft > sse > sse_planner**

# Module: sse::sse_planner

## Contents

**Structs**

- [`FftPlannerSse`](#fftplannersse) - The SSE FFT planner creates new FFT algorithm instances using a mix of scalar and SSE accelerated algorithms.

---

## rustfft::sse::sse_planner::FftPlannerSse

*Struct*

The SSE FFT planner creates new FFT algorithm instances using a mix of scalar and SSE accelerated algorithms.
It requires at least SSE4.1, which is available on all reasonably recent x86_64 cpus.

RustFFT has several FFT algorithms available. For a given FFT size, the `FftPlannerSse` decides which of the
available FFT algorithms to use and then initializes them.

~~~
// Perform a forward Fft of size 1234
use std::sync::Arc;
use rustfft::{FftPlannerSse, num_complex::Complex};

if let Ok(mut planner) = FftPlannerSse::new() {
  let fft = planner.plan_fft_forward(1234);

  let mut buffer = vec![Complex{ re: 0.0f32, im: 0.0f32 }; 1234];
  fft.process(&mut buffer);

  // The FFT instance returned by the planner has the type `Arc<dyn Fft<T>>`,
  // where T is the numeric type, ie f32 or f64, so it's cheap to clone
  let fft_clone = Arc::clone(&fft);
}
~~~

If you plan on creating multiple FFT instances, it is recommended to re-use the same planner for all of them. This
is because the planner re-uses internal data across FFT instances wherever possible, saving memory and reducing
setup time. (FFT instances created with one planner will never re-use data and buffers with FFT instances created
by a different planner)

Each FFT instance owns [`Arc`s](std::sync::Arc) to its internal data, rather than borrowing it from the planner, so it's perfectly
safe to drop the planner after creating Fft instances.

**Generic Parameters:**
- T

**Methods:**

- `fn new() -> Result<Self, ()>` - Creates a new `FftPlannerSse` instance.
- `fn plan_fft(self: & mut Self, len: usize, direction: FftDirection) -> Arc<dyn Fft>` - Returns a `Fft` instance which uses SSE4.1 instructions to compute FFTs of size `len`.
- `fn plan_fft_forward(self: & mut Self, len: usize) -> Arc<dyn Fft>` - Returns a `Fft` instance which uses SSE4.1 instructions to compute forward FFTs of size `len`
- `fn plan_fft_inverse(self: & mut Self, len: usize) -> Arc<dyn Fft>` - Returns a `Fft` instance which uses SSE4.1 instructions to compute inverse FFTs of size `len.



