**digest > core_api > wrapper**

# Module: core_api::wrapper

## Contents

**Structs**

- [`CoreWrapper`](#corewrapper) - Wrapper around [`BufferKindUser`].

**Traits**

- [`CoreProxy`](#coreproxy) - A proxy trait to a core type implemented by [`CoreWrapper`]

---

## digest::core_api::wrapper::CoreProxy

*Trait*

A proxy trait to a core type implemented by [`CoreWrapper`]

**Methods:**

- `Core`: Type wrapped by [`CoreWrapper`].



## digest::core_api::wrapper::CoreWrapper

*Struct*

Wrapper around [`BufferKindUser`].

It handles data buffering and implements the slice-based traits.

**Generic Parameters:**
- T

**Methods:**

- `fn from_core(core: T) -> Self` - Create new wrapper from `core`.
- `fn decompose(self: Self) -> (T, Buffer<T>)` - Decompose wrapper into inner parts.

**Traits:** KeySizeUser, BlockSizeUser, HashMarker, OutputSizeUser, CoreProxy

**Trait Implementations:**

- **Write**
  - `fn write(self: & mut Self, buf: &[u8]) -> std::io::Result<usize>`
  - `fn flush(self: & mut Self) -> std::io::Result<()>`
- **KeyInit**
  - `fn new(key: &Key<Self>) -> Self`
  - `fn new_from_slice(key: &[u8]) -> Result<Self, InvalidLength>`
- **ExtendableOutputReset**
  - `fn finalize_xof_reset(self: & mut Self) -> <Self as >::Reader`
- **ExtendableOutput**
  - `fn finalize_xof(self: Self) -> <Self as >::Reader`
- **Default**
  - `fn default() -> Self`
- **FixedOutputReset**
  - `fn finalize_into_reset(self: & mut Self, out: & mut Output<Self>)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **FixedOutput**
  - `fn finalize_into(self: Self, out: & mut Output<Self>)`
- **Update**
  - `fn update(self: & mut Self, input: &[u8])`
- **Reset**
  - `fn reset(self: & mut Self)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> Result<(), fmt::Error>`



