**digest > core_api > ct_variable**

# Module: core_api::ct_variable

## Contents

**Structs**

- [`CtVariableCoreWrapper`](#ctvariablecorewrapper) - Wrapper around [`VariableOutputCore`] which selects output size
- [`NoOid`](#nooid) - Dummy type used with [`CtVariableCoreWrapper`] in cases when

---

## digest::core_api::ct_variable::CtVariableCoreWrapper

*Struct*

Wrapper around [`VariableOutputCore`] which selects output size
at compile time.

**Generic Parameters:**
- T
- OutSize
- O

**Traits:** HashMarker, BufferKindUser, OutputSizeUser, BlockSizeUser

**Trait Implementations:**

- **AlgorithmName**
  - `fn write_alg_name(f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Reset**
  - `fn reset(self: & mut Self)`
- **Default**
  - `fn default() -> Self`
- **FixedOutputCore**
  - `fn finalize_fixed_core(self: & mut Self, buffer: & mut Buffer<Self>, out: & mut GenericArray<u8, <Self as >::OutputSize>)`
- **UpdateCore**
  - `fn update_blocks(self: & mut Self, blocks: &[Block<Self>])`



## digest::core_api::ct_variable::NoOid

*Struct*

Dummy type used with [`CtVariableCoreWrapper`] in cases when
resulting hash does not have a known OID.

**Unit Struct**



