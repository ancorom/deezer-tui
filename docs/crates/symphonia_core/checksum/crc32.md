**symphonia_core > checksum > crc32**

# Module: checksum::crc32

## Contents

**Structs**

- [`Crc32`](#crc32) - `Crc32` implements the CRC-32 checksum algorithm using the standard polynomial.

---

## symphonia_core::checksum::crc32::Crc32

*Struct*

`Crc32` implements the CRC-32 checksum algorithm using the standard polynomial.

* Polynomial = 0x04c11db7
* RefIn = false
* RefOut = false
* XorOut = false

**Methods:**

- `fn new(state: u32) -> Self` - Instantiates a `Crc32` instance with an initial state.
- `fn crc(self: &Self) -> u32` - Returns the computed CRC.

**Trait Implementations:**

- **Monitor**
  - `fn process_byte(self: & mut Self, byte: u8)`
  - `fn process_buf_bytes(self: & mut Self, buf: &[u8])`



