**digest > core_api**

# Module: core_api

## Contents

**Enums**

- [`TruncSide`](#truncside) - Type which used for defining truncation side in the [`VariableOutputCore`]

**Traits**

- [`BufferKindUser`](#bufferkinduser) - Types which use [`BlockBuffer`] functionality.
- [`ExtendableOutputCore`](#extendableoutputcore) - Core trait for hash functions with extendable (XOF) output size.
- [`FixedOutputCore`](#fixedoutputcore) - Core trait for hash functions with fixed output size.
- [`UpdateCore`](#updatecore) - Types which consume data in blocks.
- [`VariableOutputCore`](#variableoutputcore) - Core trait for hash functions with variable output size.
- [`XofReaderCore`](#xofreadercore) - Core reader trait for extendable-output function (XOF) result.

**Type Aliases**

- [`Buffer`](#buffer) - Buffer type used by type which implements [`BufferKindUser`].

---

## digest::core_api::Buffer

*Type Alias*: `block_buffer::BlockBuffer<<S as BlockSizeUser>::BlockSize, <S as BufferKindUser>::BufferKind>`

Buffer type used by type which implements [`BufferKindUser`].



## digest::core_api::BufferKindUser

*Trait*

Types which use [`BlockBuffer`] functionality.

**Methods:**

- `BufferKind`: Block buffer kind over which type operates.



## digest::core_api::ExtendableOutputCore

*Trait*

Core trait for hash functions with extendable (XOF) output size.

**Methods:**

- `ReaderCore`: XOF reader core state.
- `finalize_xof_core`: Retrieve XOF reader using remaining data stored in the block buffer



## digest::core_api::FixedOutputCore

*Trait*

Core trait for hash functions with fixed output size.

**Methods:**

- `finalize_fixed_core`: Finalize state using remaining data stored in the provided block buffer,



## digest::core_api::TruncSide

*Enum*

Type which used for defining truncation side in the [`VariableOutputCore`]
trait.

**Variants:**
- `Left` - Truncate left side, i.e. `&out[..n]`.
- `Right` - Truncate right side, i.e. `&out[m..]`.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## digest::core_api::UpdateCore

*Trait*

Types which consume data in blocks.

**Methods:**

- `update_blocks`: Update state using the provided data blocks.



## digest::core_api::VariableOutputCore

*Trait*

Core trait for hash functions with variable output size.

Maximum output size is equal to [`OutputSizeUser::OutputSize`].
Users are expected to truncate result returned by the
[`finalize_variable_core`] to `output_size` passed to the [`new`] method
during construction. Truncation side is defined by the [`TRUNC_SIDE`]
associated constant.

[`finalize_variable_core`]: VariableOutputCore::finalize_variable_core
[`new`]: VariableOutputCore::new
[`TRUNC_SIDE`]: VariableOutputCore::TRUNC_SIDE

**Methods:**

- `TRUNC_SIDE`: Side which should be used in a truncated result.
- `new`: Initialize hasher state for given output size.
- `finalize_variable_core`: Finalize hasher and write full hashing result into the `out` buffer.



## digest::core_api::XofReaderCore

*Trait*

Core reader trait for extendable-output function (XOF) result.

**Methods:**

- `read_block`: Read next XOF block.



