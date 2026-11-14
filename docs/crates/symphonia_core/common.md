**symphonia_core > common**

# Module: common

## Contents

**Structs**

- [`FourCc`](#fourcc) - A four character code.

**Enums**

- [`Limit`](#limit) - `Limit` defines an upper-bound of how much of a resource should be allocated when the amount to
- [`Tier`](#tier) - Describes the relative preference of a registered decoder, format reader, or metadata reader if

---

## symphonia_core::common::FourCc

*Struct*

A four character code.

**Tuple Struct**: `()`

**Methods:**

- `fn new(val: [u8; 4]) -> Self` - Construct a new FourCC code from the given ASCII character byte array.
- `fn try_new(val: [u8; 4]) -> Option<Self>` - Try to construct a new FourCC code from the given byte array.
- `fn get(self: &Self) -> [u8; 4]` - Returns the contained byte array.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## symphonia_core::common::Limit

*Enum*

`Limit` defines an upper-bound of how much of a resource should be allocated when the amount to
be allocated is specified by the media stream, which is untrusted. A limit will place an
upper-bound on this allocation at the risk of breaking potentially valid streams. Limits are
used to prevent denial-of-service attacks.

All limits can be defaulted to a reasonable value specific to the situation. These defaults will
generally not break any normal streams.

**Variants:**
- `None` - Do not impose any limit.
- `Default` - Use the a reasonable default specified by the `FormatReader` or `Decoder` implementation.
- `Maximum(usize)` - Specify the upper limit of the resource. Units are case specific.

**Methods:**

- `fn limit_or_default(self: &Self, default: usize) -> Option<usize>` - Gets the numeric limit of the limit, or default value. If there is no limit, None is

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## symphonia_core::common::Tier

*Enum*

Describes the relative preference of a registered decoder, format reader, or metadata reader if
multiple registered implementations support the same codec or format.

**Variants:**
- `Preferred` - Prefer over others.
- `Standard` - Standard tier: neither preferred nor a fallback. Symphonia's first-party decoders and
- `Fallback` - Use as a fallback if nothing else is available.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`



