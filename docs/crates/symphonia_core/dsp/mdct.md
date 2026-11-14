**symphonia_core > dsp > mdct**

# Module: dsp::mdct

## Contents

**Structs**

- [`Imdct`](#imdct) - The Inverse Modified Discrete Transform (IMDCT).

---

## symphonia_core::dsp::mdct::Imdct

*Struct*

The Inverse Modified Discrete Transform (IMDCT).

**Methods:**

- `fn new(n: usize) -> Self` - Instantiate a N-point IMDCT with no scaling.
- `fn new_scaled(n: usize, scale: f64) -> Self` - Instantiate a N-point IMDCT with scaling.
- `fn imdct(self: & mut Self, spec: &[f32], out: & mut [f32])` - Performs the the N-point Inverse Modified Discrete Cosine Transform.



