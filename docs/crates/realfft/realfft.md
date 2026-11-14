**realfft**

# Module: realfft

## Contents

**Structs**

- [`ComplexToRealEven`](#complextorealeven)
- [`ComplexToRealOdd`](#complextorealodd)
- [`RealFftPlanner`](#realfftplanner) - A planner is used to create FFTs.
- [`RealToComplexEven`](#realtocomplexeven)
- [`RealToComplexOdd`](#realtocomplexodd)

**Enums**

- [`FftError`](#ffterror) - Custom error returned by FFTs

**Traits**

- [`ComplexToReal`](#complextoreal) - An inverse FFT that takes a complex spectrum of length N/2+1
- [`RealToComplex`](#realtocomplex) - A forward FFT that takes a real-valued input signal of length N

---

## realfft::ComplexToReal

*Trait*

An inverse FFT that takes a complex spectrum of length N/2+1
and transforms it to a real-valued signal of length N.

**Methods:**

- `process`: Inverse transform a complex spectrum corresponding to a real-valued signal of length N.
- `process_with_scratch`: Inverse transform a complex spectrum,
- `get_scratch_len`: Get the minimum length of the scratch space needed for `process_with_scratch`.
- `len`: The FFT length.
- `complex_len`: Get the length of the slice slice of complex values that this FFT accepts as input.
- `make_input_vec`: Convenience method to make an input vector of the right type and length.
- `make_output_vec`: Convenience method to make an output vector of the right type and length.
- `make_scratch_vec`: Convenience method to make a scratch vector of the right type and length.



## realfft::ComplexToRealEven

*Struct*

**Generic Parameters:**
- T

**Methods:**

- `fn new(length: usize, fft_planner: & mut FftPlanner<T>) -> Self` - Create a new ComplexToRealEven inverse FFT for complex input spectra.

**Trait Implementations:**

- **ComplexToReal**
  - `fn process(self: &Self, input: & mut [Complex<T>], output: & mut [T]) -> Result<(), FftError>`
  - `fn process_with_scratch(self: &Self, input: & mut [Complex<T>], output: & mut [T], scratch: & mut [Complex<T>]) -> Result<(), FftError>`
  - `fn get_scratch_len(self: &Self) -> usize`
  - `fn len(self: &Self) -> usize`
  - `fn make_input_vec(self: &Self) -> Vec<Complex<T>>`
  - `fn make_output_vec(self: &Self) -> Vec<T>`
  - `fn make_scratch_vec(self: &Self) -> Vec<Complex<T>>`



## realfft::ComplexToRealOdd

*Struct*

**Generic Parameters:**
- T

**Methods:**

- `fn new(length: usize, fft_planner: & mut FftPlanner<T>) -> Self` - Create a new ComplexToRealOdd inverse FFT for complex input spectra.

**Trait Implementations:**

- **ComplexToReal**
  - `fn process(self: &Self, input: & mut [Complex<T>], output: & mut [T]) -> Result<(), FftError>`
  - `fn process_with_scratch(self: &Self, input: & mut [Complex<T>], output: & mut [T], scratch: & mut [Complex<T>]) -> Result<(), FftError>`
  - `fn get_scratch_len(self: &Self) -> usize`
  - `fn len(self: &Self) -> usize`
  - `fn make_input_vec(self: &Self) -> Vec<Complex<T>>`
  - `fn make_output_vec(self: &Self) -> Vec<T>`
  - `fn make_scratch_vec(self: &Self) -> Vec<Complex<T>>`



## realfft::FftError

*Enum*

Custom error returned by FFTs

**Variants:**
- `InputBuffer(usize, usize)` - The input buffer has the wrong size. The transform was not performed.
- `OutputBuffer(usize, usize)` - The output buffer has the wrong size. The transform was not performed.
- `ScratchBuffer(usize, usize)` - The scratch buffer has the wrong size. The transform was not performed.
- `InputValues(bool, bool)` - The input data contained a non-zero imaginary part where there should have been a zero.

**Traits:** Error

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## realfft::RealFftPlanner

*Struct*

A planner is used to create FFTs.
It caches results internally,
so when making more than one FFT it is advisable to reuse the same planner.

**Generic Parameters:**
- T

**Methods:**

- `fn new() -> Self` - Create a new planner.
- `fn plan_fft_forward(self: & mut Self, len: usize) -> Arc<dyn RealToComplex>` - Plan a real-to-complex forward FFT. Returns the FFT in a shared reference.
- `fn plan_fft_inverse(self: & mut Self, len: usize) -> Arc<dyn ComplexToReal>` - Plan a complex-to-real inverse FFT. Returns the FFT in a shared reference.

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`



## realfft::RealToComplex

*Trait*

A forward FFT that takes a real-valued input signal of length N
and transforms it to a complex spectrum of length N/2+1.

**Methods:**

- `process`: Transform a signal of N real-valued samples,
- `process_with_scratch`: Transform a signal of N real-valued samples,
- `get_scratch_len`: Get the minimum length of the scratch buffer needed for `process_with_scratch`.
- `len`: The FFT length.
- `complex_len`: Get the number of complex data points that this FFT returns.
- `make_input_vec`: Convenience method to make an input vector of the right type and length.
- `make_output_vec`: Convenience method to make an output vector of the right type and length.
- `make_scratch_vec`: Convenience method to make a scratch vector of the right type and length.



## realfft::RealToComplexEven

*Struct*

**Generic Parameters:**
- T

**Methods:**

- `fn new(length: usize, fft_planner: & mut FftPlanner<T>) -> Self` - Create a new RealToComplex forward FFT for real-valued input data of a given length,

**Trait Implementations:**

- **RealToComplex**
  - `fn process(self: &Self, input: & mut [T], output: & mut [Complex<T>]) -> Result<(), FftError>`
  - `fn process_with_scratch(self: &Self, input: & mut [T], output: & mut [Complex<T>], scratch: & mut [Complex<T>]) -> Result<(), FftError>`
  - `fn get_scratch_len(self: &Self) -> usize`
  - `fn len(self: &Self) -> usize`
  - `fn make_input_vec(self: &Self) -> Vec<T>`
  - `fn make_output_vec(self: &Self) -> Vec<Complex<T>>`
  - `fn make_scratch_vec(self: &Self) -> Vec<Complex<T>>`



## realfft::RealToComplexOdd

*Struct*

**Generic Parameters:**
- T

**Methods:**

- `fn new(length: usize, fft_planner: & mut FftPlanner<T>) -> Self` - Create a new RealToComplex forward FFT for real-valued input data of a given length,

**Trait Implementations:**

- **RealToComplex**
  - `fn process(self: &Self, input: & mut [T], output: & mut [Complex<T>]) -> Result<(), FftError>`
  - `fn process_with_scratch(self: &Self, input: & mut [T], output: & mut [Complex<T>], scratch: & mut [Complex<T>]) -> Result<(), FftError>`
  - `fn get_scratch_len(self: &Self) -> usize`
  - `fn len(self: &Self) -> usize`
  - `fn make_input_vec(self: &Self) -> Vec<T>`
  - `fn make_output_vec(self: &Self) -> Vec<Complex<T>>`
  - `fn make_scratch_vec(self: &Self) -> Vec<Complex<T>>`



