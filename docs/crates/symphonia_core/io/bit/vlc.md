**symphonia_core > io > bit > vlc**

# Module: io::bit::vlc

## Contents

**Structs**

- [`Codebook`](#codebook) - `Codebook` is a variable-length code decoding table that may be used to efficiently read
- [`CodebookBuilder`](#codebookbuilder) - `CodebookBuilder` generates a `Codebook` using a provided codebook specification and
- [`Entry16x16`](#entry16x16) - `Entry16x16` is a codebook entry for 16-bit values with codes up-to 16-bits.
- [`Entry16x32`](#entry16x32) - `Entry16x32` is a codebook entry for 16-bit values with codes up-to 32-bits.
- [`Entry16x8`](#entry16x8) - `Entry16x8` is a codebook entry for 16-bit values with codes up-to 8-bits.
- [`Entry32x16`](#entry32x16) - `Entry32x16` is a codebook entry for 32-bit values with codes up-to 16-bits.
- [`Entry32x32`](#entry32x32) - `Entry32x32` is a codebook entry for 32-bit values with codes up-to 32-bits.
- [`Entry32x8`](#entry32x8) - `Entry32x8` is a codebook entry for 32-bit values with codes up-to 8-bits.
- [`Entry8x16`](#entry8x16) - `Entry8x16` is a codebook entry for 8-bit values with codes up-to 16-bits.
- [`Entry8x32`](#entry8x32) - `Entry8x32` is a codebook entry for 8-bit values with codes up-to 32-bits.
- [`Entry8x8`](#entry8x8) - `Entry8x8` is a codebook entry for 8-bit values with codes up-to 8-bits.

**Enums**

- [`BitOrder`](#bitorder) - `BitOrder` describes the relationship between the order of bits in the provided codewords

**Traits**

- [`CodebookEntry`](#codebookentry) - The `CodebookEntry` trait describes an entry in a codebook.

---

## symphonia_core::io::bit::vlc::BitOrder

*Enum*

`BitOrder` describes the relationship between the order of bits in the provided codewords
and the order in which bits are read.

**Variants:**
- `Verbatim` - The provided codewords have bits in the same order as the order in which they're being
- `Reverse` - The provided codeword have bits in the reverse order as the order in which they're

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`



## symphonia_core::io::bit::vlc::Codebook

*Struct*

`Codebook` is a variable-length code decoding table that may be used to efficiently read
symbols from a source of bits.

**Generic Parameters:**
- E

**Methods:**

- `fn is_empty(self: &Self) -> bool` - Returns `true` if the `Codebook` is empty.

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## symphonia_core::io::bit::vlc::CodebookBuilder

*Struct*

`CodebookBuilder` generates a `Codebook` using a provided codebook specification and
description.

**Methods:**

- `fn new(bit_order: BitOrder) -> Self` - Instantiates a new `CodebookBuilder`.
- `fn bits_per_read(self: & mut Self, bits_per_read: u8) -> & mut Self` - Specify the number of bits that should be consumed from the source at a time. This value
- `fn make<E>(self: & mut Self, codes: &[u32], code_lens: &[u8], values: &[<E as >::ValueType]) -> io::Result<Codebook<E>>` - Construct a `Codebook` using the given codewords, their respective lengths in number of



## symphonia_core::io::bit::vlc::CodebookEntry

*Trait*

The `CodebookEntry` trait describes an entry in a codebook.

This trait should be implemented by a zero-sized marker struct. When instantiating a
`Codebook`, the marker struct implementing `CodebookEntry` is supplied as the type argument.

**Methods:**

- `IndexType`: The type of an index into the codebook table.
- `ValueType`: The type of a value in the codebook.



## symphonia_core::io::bit::vlc::Entry16x16

*Struct*

`Entry16x16` is a codebook entry for 16-bit values with codes up-to 16-bits.

**Unit Struct**

**Traits:** Copy, CodebookEntry

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Default**
  - `fn default() -> Self`



## symphonia_core::io::bit::vlc::Entry16x32

*Struct*

`Entry16x32` is a codebook entry for 16-bit values with codes up-to 32-bits.

**Unit Struct**

**Traits:** Copy, CodebookEntry

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## symphonia_core::io::bit::vlc::Entry16x8

*Struct*

`Entry16x8` is a codebook entry for 16-bit values with codes up-to 8-bits.

**Unit Struct**

**Traits:** CodebookEntry, Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## symphonia_core::io::bit::vlc::Entry32x16

*Struct*

`Entry32x16` is a codebook entry for 32-bit values with codes up-to 16-bits.

**Unit Struct**

**Traits:** CodebookEntry, Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## symphonia_core::io::bit::vlc::Entry32x32

*Struct*

`Entry32x32` is a codebook entry for 32-bit values with codes up-to 32-bits.

**Unit Struct**

**Traits:** Copy, CodebookEntry

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Default**
  - `fn default() -> Self`



## symphonia_core::io::bit::vlc::Entry32x8

*Struct*

`Entry32x8` is a codebook entry for 32-bit values with codes up-to 8-bits.

**Unit Struct**

**Traits:** CodebookEntry, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Default**
  - `fn default() -> Self`



## symphonia_core::io::bit::vlc::Entry8x16

*Struct*

`Entry8x16` is a codebook entry for 8-bit values with codes up-to 16-bits.

**Unit Struct**

**Traits:** CodebookEntry, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Default**
  - `fn default() -> Self`



## symphonia_core::io::bit::vlc::Entry8x32

*Struct*

`Entry8x32` is a codebook entry for 8-bit values with codes up-to 32-bits.

**Unit Struct**

**Traits:** Copy, CodebookEntry

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Default**
  - `fn default() -> Self`



## symphonia_core::io::bit::vlc::Entry8x8

*Struct*

`Entry8x8` is a codebook entry for 8-bit values with codes up-to 8-bits.

**Unit Struct**

**Traits:** Copy, CodebookEntry

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



