**cipher > stream_wrapper**

# Module: stream_wrapper

## Contents

**Structs**

- [`StreamCipherCoreWrapper`](#streamciphercorewrapper) - Wrapper around [`StreamCipherCore`] implementations.

---

## cipher::stream_wrapper::StreamCipherCoreWrapper

*Struct*

Wrapper around [`StreamCipherCore`] implementations.

It handles data buffering and implements the slice-based traits.

**Generic Parameters:**
- T

**Methods:**

- `fn get_core(self: &Self) -> &T` - Return reference to the core type.
- `fn from_core(core: T) -> Self` - Return reference to the core type.

**Traits:** IvSizeUser, KeySizeUser

**Trait Implementations:**

- **KeyIvInit**
  - `fn new(key: &Key<Self>, iv: &Iv<Self>) -> Self`
- **StreamCipherSeek**
  - `fn try_current_pos<SN>(self: &Self) -> Result<SN, OverflowError>`
  - `fn try_seek<SN>(self: & mut Self, new_pos: SN) -> Result<(), StreamCipherError>`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **KeyInit**
  - `fn new(key: &Key<Self>) -> Self`
- **StreamCipher**
  - `fn try_apply_keystream_inout(self: & mut Self, data: InOutBuf<u8>) -> Result<(), StreamCipherError>`



