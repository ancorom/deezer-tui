**rustfft > wasm_simd > wasm_simd_planner**

# Module: wasm_simd::wasm_simd_planner

## Contents

**Structs**

- [`FftPlannerWasmSimd`](#fftplannerwasmsimd) - The WASM FFT planner creates new FFT algorithm instances using a mix of scalar and WASM SIMD accelerated algorithms.

---

## rustfft::wasm_simd::wasm_simd_planner::FftPlannerWasmSimd

*Struct*

The WASM FFT planner creates new FFT algorithm instances using a mix of scalar and WASM SIMD accelerated algorithms.
It is supported when using fairly recent browser versions as outlined in [the WebAssembly roadmap](https://webassembly.org/roadmap/).

RustFFT has several FFT algorithms available. For a given FFT size, `FftPlannerWasmSimd` decides which of the
available FFT algorithms to use and then initializes them.

~~~
// Perform a forward Fft of size 1234
use std::sync::Arc;
use rustfft::{FftPlannerWasmSimd, num_complex::Complex};

if let Ok(mut planner) = FftPlannerWasmSimd::new() {
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

- `fn new() -> Result<Self, ()>` - Creates a new `FftPlannerWasmSimd` instance.
- `fn plan_fft(self: & mut Self, _len: usize, _direction: FftDirection) -> Arc<dyn Fft>` - Returns a `Fft` instance which uses WebAssembly SIMD instructions to compute FFTs of size `len`.
- `fn plan_fft_forward(self: & mut Self, _len: usize) -> Arc<dyn Fft>` - Returns a `Fft` instance which uses WebAssembly SIMD instructions to compute forward FFTs of size `len`.
- `fn plan_fft_inverse(self: & mut Self, _len: usize) -> Arc<dyn Fft>` - Returns a `Fft` instance which uses WebAssembly SIMD instructions to compute inverse FFTs of size `len.



