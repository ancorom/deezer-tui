**digest > core_api > xof_reader**

# Module: core_api::xof_reader

## Contents

**Structs**

- [`XofReaderCoreWrapper`](#xofreadercorewrapper) - Wrapper around [`XofReaderCore`] implementations.

---

## digest::core_api::xof_reader::XofReaderCoreWrapper

*Struct*

Wrapper around [`XofReaderCore`] implementations.

It handles data buffering and implements the mid-level traits.

**Generic Parameters:**
- T

**Trait Implementations:**

- **Read**
  - `fn read(self: & mut Self, buf: & mut [u8]) -> std::io::Result<usize>`
- **XofReader**
  - `fn read(self: & mut Self, buffer: & mut [u8])`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> Result<(), fmt::Error>`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



