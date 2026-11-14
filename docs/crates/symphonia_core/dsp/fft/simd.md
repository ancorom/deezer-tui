**symphonia_core > dsp > fft > simd**

# Module: dsp::fft::simd

## Contents

**Structs**

- [`Fft`](#fft) - The complex Fast Fourier Transform (FFT).
- [`Ifft`](#ifft) - The complex inverse Fast Fourier Transform (IFFT).

---

## symphonia_core::dsp::fft::simd::Fft

*Struct*

The complex Fast Fourier Transform (FFT).

**Methods:**

- `fn new(n: usize) -> Self`
- `fn size(self: &Self) -> usize` - Get the size of the FFT.
- `fn fft(self: & mut Self, x: &[Complex<f32>], y: & mut [Complex<f32>])` - Calculate the FFT.
- `fn fft_inplace(self: & mut Self, x: & mut [Complex<f32>])` - Calculate the FFT in-place.



## symphonia_core::dsp::fft::simd::Ifft

*Struct*

The complex inverse Fast Fourier Transform (IFFT).

**Methods:**

- `fn new(n: usize) -> Self`
- `fn size(self: &Self) -> usize` - Get the size of the FFT.
- `fn ifft(self: & mut Self, x: &[Complex<f32>], y: & mut [Complex<f32>])` - Calculate the inverse FFT.
- `fn ifft_inplace(self: & mut Self, x: & mut [Complex<f32>])` - Calculate the inverse FFT in-place.



