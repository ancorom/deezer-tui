**cbc > decrypt**

# Module: decrypt

## Contents

**Structs**

- [`Decryptor`](#decryptor) - CBC mode decryptor.

---

## cbc::decrypt::Decryptor

*Struct*

CBC mode decryptor.

**Generic Parameters:**
- C

**Traits:** IvSizeUser, BlockSizeUser, InnerUser

**Trait Implementations:**

- **InnerIvInit**
  - `fn inner_iv_init(cipher: C, iv: &Iv<Self>) -> Self`
- **BlockDecryptMut**
  - `fn decrypt_with_backend_mut<impl BlockClosure<BlockSize = Self::BlockSize>>(self: & mut Self, f: impl Trait)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **IvState**
  - `fn iv_state(self: &Self) -> Iv<Self>`
- **AlgorithmName**
  - `fn write_alg_name(f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



