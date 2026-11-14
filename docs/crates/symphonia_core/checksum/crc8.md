**symphonia_core > checksum > crc8**

# Module: checksum::crc8

## Contents

**Structs**

- [`Crc8Ccitt`](#crc8ccitt) - `Crc8Ccitt` implements the CRC-8 algorithm using the CCITT polynominal.

---

## symphonia_core::checksum::crc8::Crc8Ccitt

*Struct*

`Crc8Ccitt` implements the CRC-8 algorithm using the CCITT polynominal.

* Polynomial = 0x07
* RefIn = false
* RefOut = false
* XorOut = false

**Methods:**

- `fn new(state: u8) -> Self` - Instantiate a `Crc8Ccitt` instance with an initial state.
- `fn crc(self: &Self) -> u8` - Returns the computed CRC.

**Trait Implementations:**

- **Monitor**
  - `fn process_byte(self: & mut Self, byte: u8)`
  - `fn process_buf_bytes(self: & mut Self, buf: &[u8])`



