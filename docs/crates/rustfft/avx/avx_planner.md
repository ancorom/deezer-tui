**rustfft > avx > avx_planner**

# Module: avx::avx_planner

## Contents

**Structs**

- [`FftPlannerAvx`](#fftplanneravx) - The AVX FFT planner creates new FFT algorithm instances which take advantage of the AVX instruction set.

---

## rustfft::avx::avx_planner::FftPlannerAvx

*Struct*

The AVX FFT planner creates new FFT algorithm instances which take advantage of the AVX instruction set.

Creating an instance of `FftPlannerAvx` requires the `avx` and `fma` instructions to be available on the current machine, and it requires RustFFT's
 `avx` feature flag to be set. A few algorithms will use `avx2` if it's available, but it isn't required.

For the time being, AVX acceleration is black box, and AVX accelerated algorithms are not available without a planner. This may change in the future.

~~~
// Perform a forward Fft of size 1234, accelerated by AVX
use std::sync::Arc;
use rustfft::{FftPlannerAvx, num_complex::Complex};

// If FftPlannerAvx::new() returns Ok(), we'll know AVX algorithms are available
// on this machine, and that RustFFT was compiled with the `avx` feature flag
if let Ok(mut planner) = FftPlannerAvx::new() {
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

- `fn new() -> Result<Self, ()>` - Constructs a new `FftPlannerAvx` instance.
- `fn plan_fft(self: & mut Self, len: usize, direction: FftDirection) -> Arc<dyn Fft>` - Returns a `Fft` instance which uses AVX instructions to compute FFTs of size `len`.
- `fn plan_fft_forward(self: & mut Self, len: usize) -> Arc<dyn Fft>` - Returns a `Fft` instance which uses AVX instructions to compute forward FFTs of size `len`.
- `fn plan_fft_inverse(self: & mut Self, len: usize) -> Arc<dyn Fft>` - Returns a `Fft` instance which uses AVX instructions to compute inverse FFTs of size `len.



