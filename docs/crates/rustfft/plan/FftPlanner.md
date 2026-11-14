**rustfft > plan > FftPlanner**

# Module: plan::FftPlanner

## Contents

**Functions**

- [`plan_fft`](#plan_fft) - Returns a `Fft` instance which computes FFTs of size `len`.

---

## rustfft::plan::FftPlanner::plan_fft

*Function*

Returns a `Fft` instance which computes FFTs of size `len`.

If the provided `direction` is `FftDirection::Forward`, the returned instance will compute forward FFTs. If it's `FftDirection::Inverse`, it will compute inverse FFTs.

If this is called multiple times, the planner will attempt to re-use internal data between calls, reducing memory usage and FFT initialization time.

```rust
fn plan_fft(self: & mut Self, len: usize, direction: FftDirection) -> Arc<dyn Fft>
```



