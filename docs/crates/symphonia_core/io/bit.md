**symphonia_core > io > bit**

# Module: io::bit

## Contents

**Modules**

- [`vlc`](#vlc) - The `vlc` module provides support for decoding variable-length codes (VLC).

**Structs**

- [`BitReaderLtr`](#bitreaderltr) - `BitReaderLtr` reads bits from most-significant to least-significant from any `&[u8]`.
- [`BitReaderRtl`](#bitreaderrtl) - `BitReaderRtl` reads bits from least-significant to most-significant from any `&[u8]`.
- [`BitStreamLtr`](#bitstreamltr) - `BitStreamLtr` reads bits from most-significant to least-significant from any source
- [`BitStreamRtl`](#bitstreamrtl) - `BitStreamRtl` reads bits from least-significant to most-significant from any source

**Traits**

- [`FiniteBitStream`](#finitebitstream) - A `FiniteBitStream` is a bit stream that has a known length in bits.
- [`ReadBitsLtr`](#readbitsltr) - `ReadBitsLtr` reads bits from most-significant to least-significant.
- [`ReadBitsRtl`](#readbitsrtl) - `ReadBitsRtl` reads bits from least-significant to most-significant.

---

## symphonia_core::io::bit::BitReaderLtr

*Struct*

`BitReaderLtr` reads bits from most-significant to least-significant from any `&[u8]`.

Stated another way, if N-bits are read from a `BitReaderLtr` then bit 0, the first bit read,
is the most-significant bit, and bit N-1, the last bit read, is the least-significant.

**Generic Parameters:**
- 'a

**Methods:**

- `fn new(buf: &'a [u8]) -> Self` - Instantiate a new `BitReaderLtr` with the given buffer.

**Traits:** ReadBitsLtr

**Trait Implementations:**

- **FiniteBitStream**
  - `fn bits_left(self: &Self) -> u64`



## symphonia_core::io::bit::BitReaderRtl

*Struct*

`BitReaderRtl` reads bits from least-significant to most-significant from any `&[u8]`.

Stated another way, if N-bits are read from a `BitReaderRtl` then bit 0, the first bit read,
is the least-significant bit, and bit N-1, the last bit read, is the most-significant.

**Generic Parameters:**
- 'a

**Methods:**

- `fn new(buf: &'a [u8]) -> Self` - Instantiate a new `BitReaderRtl` with the given buffer.

**Traits:** ReadBitsRtl

**Trait Implementations:**

- **FiniteBitStream**
  - `fn bits_left(self: &Self) -> u64`



## symphonia_core::io::bit::BitStreamLtr

*Struct*

`BitStreamLtr` reads bits from most-significant to least-significant from any source
that implements [`ReadBytes`].

Stated another way, if N-bits are read from a `BitReaderLtr` then bit 0, the first bit read,
is the most-significant bit, and bit N-1, the last bit read, is the least-significant.

**Generic Parameters:**
- 'a
- B

**Methods:**

- `fn new(reader: &'a  mut B) -> Self` - Instantiate a new `BitStreamLtr` with the given source.

**Traits:** ReadBitsLtr



## symphonia_core::io::bit::BitStreamRtl

*Struct*

`BitStreamRtl` reads bits from least-significant to most-significant from any source
that implements [`ReadBytes`].

Stated another way, if N-bits are read from a `BitReaderLtr` then bit 0, the first bit read,
is the least-significant bit, and bit N-1, the last bit read, is the most-significant.

**Generic Parameters:**
- 'a
- B

**Methods:**

- `fn new(reader: &'a  mut B) -> Self` - Instantiate a new `BitStreamRtl` with the given buffer.

**Traits:** ReadBitsRtl



## symphonia_core::io::bit::FiniteBitStream

*Trait*

A `FiniteBitStream` is a bit stream that has a known length in bits.

**Methods:**

- `bits_left`: Gets the number of bits left unread.



## symphonia_core::io::bit::ReadBitsLtr

*Trait*

`ReadBitsLtr` reads bits from most-significant to least-significant.

**Methods:**

- `realign`: Discards any saved bits and resets the `BitStream` to prepare it for a byte-aligned read.
- `ignore_bits`: Ignores the specified number of bits from the stream or returns an error.
- `ignore_bit`: Ignores one bit from the stream or returns an error.
- `read_bool`: Read a single bit as a boolean value or returns an error.
- `read_bit`: Reads and returns a single bit or returns an error.
- `read_bits_leq32`: Reads and returns up to 32-bits or returns an error.
- `read_bits_leq32_signed`: Reads up to 32-bits and interprets them as a signed two's complement integer or returns an
- `read_bits_leq64`: Reads and returns up to 64-bits or returns an error.
- `read_bits_leq64_signed`: Reads up to 64-bits and interprets them as a signed two's complement integer or returns an
- `read_unary_zeros`: Reads and returns a unary zeros encoded integer or an error.
- `read_unary_zeros_capped`: Reads and returns a unary zeros encoded integer that is capped to a maximum value.
- `read_unary_ones`: Reads and returns a unary ones encoded integer or an error.
- `read_unary_ones_capped`: Reads and returns a unary ones encoded integer that is capped to a maximum value.
- `read_codebook`: Reads a codebook value from the `BitStream` using the provided `Codebook` and returns the



## symphonia_core::io::bit::ReadBitsRtl

*Trait*

`ReadBitsRtl` reads bits from least-significant to most-significant.

**Methods:**

- `realign`: Discards any saved bits and resets the `BitStream` to prepare it for a byte-aligned read.
- `ignore_bits`: Ignores the specified number of bits from the stream or returns an error.
- `ignore_bit`: Ignores one bit from the stream or returns an error.
- `read_bool`: Read a single bit as a boolean value or returns an error.
- `read_bit`: Reads and returns a single bit or returns an error.
- `read_bits_leq32`: Reads and returns up to 32-bits or returns an error.
- `read_bits_leq32_signed`: Reads up to 32-bits and interprets them as a signed two's complement integer or returns an
- `read_bits_leq64`: Reads and returns up to 64-bits or returns an error.
- `read_bits_leq64_signed`: Reads up to 64-bits and interprets them as a signed two's complement integer or returns an
- `read_unary_zeros`: Reads and returns a unary zeros encoded integer or an error.
- `read_unary_zeros_capped`: Reads and returns a unary zeros encoded integer that is capped to a maximum value.
- `read_unary_ones`: Reads and returns a unary ones encoded integer or an error.
- `read_unary_ones_capped`: Reads and returns a unary ones encoded integer or an error.
- `read_codebook`



## Module: vlc

The `vlc` module provides support for decoding variable-length codes (VLC).



