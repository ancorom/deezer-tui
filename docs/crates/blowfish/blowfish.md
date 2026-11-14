**blowfish**

# Module: blowfish

## Contents

**Structs**

- [`Blowfish`](#blowfish) - Blowfish block cipher instance.

**Type Aliases**

- [`BlowfishLE`](#blowfishle) - Blowfish variant which uses Little Endian byte order read/writes.s.

---

## blowfish::Blowfish

*Struct*

Blowfish block cipher instance.

**Generic Parameters:**
- T

**Traits:** KeySizeUser, BlockCipher, BlockSizeUser

**Trait Implementations:**

- **BlockDecrypt**
  - `fn decrypt_with_backend<impl ::cipher::BlockClosure<BlockSize = U8>>(self: &Self, f: impl Trait)`
- **AlgorithmName**
  - `fn write_alg_name(f: & mut fmt::Formatter) -> fmt::Result`
- **KeyInit**
  - `fn new(key: &Key<Self>) -> Self`
  - `fn new_from_slice(key: &[u8]) -> Result<Self, InvalidLength>`
- **BlockEncrypt**
  - `fn encrypt_with_backend<impl ::cipher::BlockClosure<BlockSize = U8>>(self: &Self, f: impl Trait)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **AlgorithmName**
  - `fn write_alg_name(f: & mut fmt::Formatter) -> fmt::Result`



## blowfish::BlowfishLE

*Type Alias*: `Blowfish<byteorder::LE>`

Blowfish variant which uses Little Endian byte order read/writes.s.



