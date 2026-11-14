**digest**

# Module: digest

## Contents

**Modules**

- [`core_api`](#core_api) - Low-level traits operating on blocks and wrappers around them.

**Macros**

- [`impl_oid_carrier`](#impl_oid_carrier) - Implement dummy type with hidden docs which is used to "carry" hasher

**Structs**

- [`InvalidBufferSize`](#invalidbuffersize) - Buffer length is not equal to hash output size.
- [`InvalidOutputSize`](#invalidoutputsize) - The error type used in variable hash traits.

**Traits**

- [`ExtendableOutput`](#extendableoutput) - Trait for hash functions with extendable-output (XOF).
- [`ExtendableOutputReset`](#extendableoutputreset) - Trait for hash functions with extendable-output (XOF) able to reset themselves.
- [`FixedOutput`](#fixedoutput) - Trait for hash functions with fixed-size output.
- [`FixedOutputReset`](#fixedoutputreset) - Trait for hash functions with fixed-size output able to reset themselves.
- [`Update`](#update) - Types which consume data with byte granularity.
- [`VariableOutput`](#variableoutput) - Trait for hash functions with variable-size output.
- [`VariableOutputReset`](#variableoutputreset) - Trait for hash functions with variable-size output able to reset themselves.
- [`XofReader`](#xofreader) - Trait for reader types which are used to extract extendable output

---

## digest::ExtendableOutput

*Trait*

Trait for hash functions with extendable-output (XOF).

**Methods:**

- `Reader`: Reader
- `finalize_xof`: Retrieve XOF reader and consume hasher instance.
- `finalize_xof_into`: Finalize XOF and write result into `out`.
- `digest_xof`: Compute hash of `data` and write it into `output`.
- `finalize_boxed`: Retrieve result into a boxed slice of the specified size and consume



## digest::ExtendableOutputReset

*Trait*

Trait for hash functions with extendable-output (XOF) able to reset themselves.

**Methods:**

- `finalize_xof_reset`: Retrieve XOF reader and reset hasher instance state.
- `finalize_xof_reset_into`: Finalize XOF, write result into `out`, and reset the hasher state.
- `finalize_boxed_reset`: Retrieve result into a boxed slice of the specified size and reset



## digest::FixedOutput

*Trait*

Trait for hash functions with fixed-size output.

**Methods:**

- `finalize_into`: Consume value and write result into provided array.
- `finalize_fixed`: Retrieve result and consume the hasher instance.



## digest::FixedOutputReset

*Trait*

Trait for hash functions with fixed-size output able to reset themselves.

**Methods:**

- `finalize_into_reset`: Write result into provided array and reset the hasher state.
- `finalize_fixed_reset`: Retrieve result and reset the hasher state.



## digest::InvalidBufferSize

*Struct*

Buffer length is not equal to hash output size.

**Unit Struct**

**Traits:** Error, Eq, Copy

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`



## digest::InvalidOutputSize

*Struct*

The error type used in variable hash traits.

**Unit Struct**

**Traits:** Error, Copy

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## digest::Update

*Trait*

Types which consume data with byte granularity.

**Methods:**

- `update`: Update state using the provided data.
- `chain`: Digest input data in a chained manner.



## digest::VariableOutput

*Trait*

Trait for hash functions with variable-size output.

**Methods:**

- `MAX_OUTPUT_SIZE`: Maximum size of output hash.
- `new`: Create new hasher instance with the given output size.
- `output_size`: Get output size of the hasher instance provided to the `new` method
- `finalize_variable`: Write result into the output buffer.
- `digest_variable`: Compute hash of `data` and write it to `output`.
- `finalize_boxed`: Retrieve result into a boxed slice and consume hasher.



## digest::VariableOutputReset

*Trait*

Trait for hash functions with variable-size output able to reset themselves.

**Methods:**

- `finalize_variable_reset`: Write result into the output buffer and reset the hasher state.
- `finalize_boxed_reset`: Retrieve result into a boxed slice and reset the hasher state.



## digest::XofReader

*Trait*

Trait for reader types which are used to extract extendable output
from a XOF (extendable-output function) result.

**Methods:**

- `read`: Read output into the `buffer`. Can be called an unlimited number of times.
- `read_boxed`: Read output into a boxed slice of the specified size.



## Module: core_api

Low-level traits operating on blocks and wrappers around them.

Usage of traits in this module in user code is discouraged. Instead use
core algorithm wrapped by the wrapper types, which implement the
higher-level traits.



## digest::impl_oid_carrier

*Declarative Macro*

Implement dummy type with hidden docs which is used to "carry" hasher
OID for [`CtVariableCoreWrapper`].

```rust
macro_rules! impl_oid_carrier {
    ($name:ident, $oid:literal) => { ... };
}
```



