**digest > core_api > rt_variable**

# Module: core_api::rt_variable

## Contents

**Structs**

- [`RtVariableCoreWrapper`](#rtvariablecorewrapper) - Wrapper around [`VariableOutputCore`] which selects output size

---

## digest::core_api::rt_variable::RtVariableCoreWrapper

*Struct*

Wrapper around [`VariableOutputCore`] which selects output size
at run time.

**Generic Parameters:**
- T

**Traits:** HashMarker

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> Result<(), fmt::Error>`
- **VariableOutputReset**
  - `fn finalize_variable_reset(self: & mut Self, out: & mut [u8]) -> Result<(), InvalidBufferSize>`
- **VariableOutput**
  - `fn new(output_size: usize) -> Result<Self, InvalidOutputSize>`
  - `fn output_size(self: &Self) -> usize`
  - `fn finalize_variable(self: Self, out: & mut [u8]) -> Result<(), InvalidBufferSize>`
- **Update**
  - `fn update(self: & mut Self, input: &[u8])`
- **Reset**
  - `fn reset(self: & mut Self)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Write**
  - `fn write(self: & mut Self, buf: &[u8]) -> std::io::Result<usize>`
  - `fn flush(self: & mut Self) -> std::io::Result<()>`



