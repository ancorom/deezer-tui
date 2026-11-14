**cbc > encrypt**

# Module: encrypt

## Contents

**Structs**

- [`Encryptor`](#encryptor) - CBC mode encryptor.

---

## cbc::encrypt::Encryptor

*Struct*

CBC mode encryptor.

**Generic Parameters:**
- C

**Traits:** BlockSizeUser, IvSizeUser, InnerUser

**Trait Implementations:**

- **BlockEncryptMut**
  - `fn encrypt_with_backend_mut<impl BlockClosure<BlockSize = Self::BlockSize>>(self: & mut Self, f: impl Trait)`
- **InnerIvInit**
  - `fn inner_iv_init(cipher: C, iv: &Iv<Self>) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **AlgorithmName**
  - `fn write_alg_name(f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **IvState**
  - `fn iv_state(self: &Self) -> Iv<Self>`



