**symphonia_core > checksum > md5**

# Module: checksum::md5

## Contents

**Structs**

- [`Md5`](#md5) - `Md5` implements the MD5 hashing algorithm.

---

## symphonia_core::checksum::md5::Md5

*Struct*

`Md5` implements the MD5 hashing algorithm.

**Methods:**

- `fn md5(self: &Self) -> [u8; 16]` - Finalizes and returns the computed MD5 hash.

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Monitor**
  - `fn process_byte(self: & mut Self, byte: u8)`
  - `fn process_buf_bytes(self: & mut Self, buf: &[u8])`



