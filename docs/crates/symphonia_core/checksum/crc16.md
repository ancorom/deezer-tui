**symphonia_core > checksum > crc16**

# Module: checksum::crc16

## Contents

**Structs**

- [`Crc16Ansi`](#crc16ansi) - `Crc16Ansi` implements the CRC-16 algorithm using the ANSI polynominal.
- [`Crc16AnsiLe`](#crc16ansile) - `Crc16AnsiLe` implements the CRC-16 algorithm using the ANSI polynominal in little-endian byte

---

## symphonia_core::checksum::crc16::Crc16Ansi

*Struct*

`Crc16Ansi` implements the CRC-16 algorithm using the ANSI polynominal.

* Polynomial = 0x8005
* RefIn = false
* RefOut = false
* XorOut = false

**Methods:**

- `fn new(state: u16) -> Self` - Instantiate a `Crc16Ansi` instance with an initial state.
- `fn crc(self: &Self) -> u16` - Returns the computed CRC.

**Trait Implementations:**

- **Monitor**
  - `fn process_byte(self: & mut Self, byte: u8)`
  - `fn process_buf_bytes(self: & mut Self, buf: &[u8])`



## symphonia_core::checksum::crc16::Crc16AnsiLe

*Struct*

`Crc16AnsiLe` implements the CRC-16 algorithm using the ANSI polynominal in little-endian byte
order.

* Polynomial = 0x8005
* RefIn = false
* RefOut = false
* XorOut = false

**Methods:**

- `fn new(state: u16) -> Self` - Instantiate a `Crc16AnsiLe` instance with an initial state.
- `fn crc(self: &Self) -> u16` - Returns the computed CRC.

**Trait Implementations:**

- **Monitor**
  - `fn process_byte(self: & mut Self, byte: u8)`
  - `fn process_buf_bytes(self: & mut Self, buf: &[u8])`



