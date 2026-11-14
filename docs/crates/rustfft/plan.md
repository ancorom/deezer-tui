**rustfft > plan**

# Module: plan

## Contents

**Structs**

- [`FftPlanner`](#fftplanner) - The FFT planner creates new FFT algorithm instances.
- [`FftPlannerScalar`](#fftplannerscalar) - The Scalar FFT planner creates new FFT algorithm instances using non-SIMD algorithms.

---

## rustfft::plan::FftPlanner

*Struct*

The FFT planner creates new FFT algorithm instances.

RustFFT has several FFT algorithms available. For a given FFT size, the `FftPlanner` decides which of the
available FFT algorithms to use and then initializes them.

~~~
// Perform a forward Fft of size 1234
use std::sync::Arc;
use rustfft::{FftPlanner, num_complex::Complex};

let mut planner = FftPlanner::new();
let fft = planner.plan_fft_forward(1234);

let mut buffer = vec![Complex{ re: 0.0f32, im: 0.0f32 }; 1234];
fft.process(&mut buffer);

// The FFT instance returned by the planner has the type `Arc<dyn Fft<T>>`,
// where T is the numeric type, ie f32 or f64, so it's cheap to clone
let fft_clone = Arc::clone(&fft);
~~~

If you plan on creating multiple FFT instances, it is recommended to reuse the same planner for all of them. This
is because the planner re-uses internal data across FFT instances wherever possible, saving memory and reducing
setup time. (FFT instances created with one planner will never re-use data and buffers with FFT instances created
by a different planner)

Each FFT instance owns [`Arc`s](std::sync::Arc) to its internal data, rather than borrowing it from the planner, so it's perfectly
safe to drop the planner after creating Fft instances.

In the constructor, the FftPlanner will detect available CPU features. If AVX, SSE, Neon, or WASM SIMD are available, it will set itself up to plan FFTs with the fastest available instruction set.
If no SIMD instruction sets are available, the planner will seamlessly fall back to planning non-SIMD FFTs.

If you'd prefer not to compute a FFT at all if a certain SIMD instruction set isn't available, or otherwise specify your own custom fallback, RustFFT exposes dedicated planners for each instruction set:
 - [`FftPlannerAvx`](crate::FftPlannerAvx)
 - [`FftPlannerSse`](crate::FftPlannerSse)
 - [`FftPlannerNeon`](crate::FftPlannerNeon)
 - [`FftPlannerWasmSimd`](crate::FftPlannerWasmSimd)

If you'd prefer to opt out of SIMD algorithms, consider creating a [`FftPlannerScalar`](crate::FftPlannerScalar) instead.

**Generic Parameters:**
- T

**Methods:**

- `fn new() -> Self` - Creates a new `FftPlanner` instance.
- `fn plan_fft(self: & mut Self, len: usize, direction: FftDirection) -> Arc<dyn Fft>` - Returns a `Fft` instance which computes FFTs of size `len`.
- `fn plan_fft_forward(self: & mut Self, len: usize) -> Arc<dyn Fft>` - Returns a `Fft` instance which computes forward FFTs of size `len`
- `fn plan_fft_inverse(self: & mut Self, len: usize) -> Arc<dyn Fft>` - Returns a `Fft` instance which computes inverse FFTs of size `len`



## rustfft::plan::FftPlannerScalar

*Struct*

The Scalar FFT planner creates new FFT algorithm instances using non-SIMD algorithms.

RustFFT has several FFT algorithms available. For a given FFT size, the `FftPlannerScalar` decides which of the
available FFT algorithms to use and then initializes them.

Use `FftPlannerScalar` instead of [`FftPlanner`](crate::FftPlanner) or [`FftPlannerAvx`](crate::FftPlannerAvx) when you want to explicitly opt out of using any SIMD-accelerated algorithms.

~~~
// Perform a forward Fft of size 1234
use std::sync::Arc;
use rustfft::{FftPlannerScalar, num_complex::Complex};

let mut planner = FftPlannerScalar::new();
let fft = planner.plan_fft_forward(1234);

let mut buffer = vec![Complex{ re: 0.0f32, im: 0.0f32 }; 1234];
fft.process(&mut buffer);

// The FFT instance returned by the planner has the type `Arc<dyn Fft<T>>`,
// where T is the numeric type, ie f32 or f64, so it's cheap to clone
let fft_clone = Arc::clone(&fft);
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

- `fn new() -> Self` - Creates a new `FftPlannerScalar` instance.
- `fn plan_fft(self: & mut Self, len: usize, direction: FftDirection) -> Arc<dyn Fft>` - Returns a `Fft` instance which computes FFTs of size `len`.
- `fn plan_fft_forward(self: & mut Self, len: usize) -> Arc<dyn Fft>` - Returns a `Fft` instance which computes forward FFTs of size `len`
- `fn plan_fft_inverse(self: & mut Self, len: usize) -> Arc<dyn Fft>` - Returns a `Fft` instance which computes inverse FFTs of size `len`



