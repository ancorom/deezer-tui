**symphonia_core > codecs > subtitle**

# Module: codecs::subtitle

## Contents

**Modules**

- [`well_known`](#well_known) - IDs for well-known subtitle codecs.

**Structs**

- [`SubtitleCodecId`](#subtitlecodecid) - An `SubtitleCodecId` is a unique identifier used to identify a specific video codec.
- [`SubtitleCodecParameters`](#subtitlecodecparameters) - Codec parameters for subtitle codecs.

**Constants**

- [`CODEC_ID_NULL_SUBTITLE`](#codec_id_null_subtitle) - Null subtitle codec ID

---

## symphonia_core::codecs::subtitle::CODEC_ID_NULL_SUBTITLE

*Constant*: `SubtitleCodecId`

Null subtitle codec ID



## symphonia_core::codecs::subtitle::SubtitleCodecId

*Struct*

An `SubtitleCodecId` is a unique identifier used to identify a specific video codec.

# Creating a Codec ID

Using a [well-known](well_known) codec ID is *highly* recommended to maximize compatibility
between components, libraries, and applications. However, if a codec requires custom codec ID,
or there is no well-known ID, then the [`FourCc`] for the codec may be converted into a codec
ID.

**Tuple Struct**: `()`

**Methods:**

- `fn new(cc: FourCc) -> SubtitleCodecId` - Create a new subtitle codec ID from a FourCC.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> $crate::cmp::Ordering`
- **From**
  - `fn from(value: FourCc) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Default**
  - `fn default() -> Self`



## symphonia_core::codecs::subtitle::SubtitleCodecParameters

*Struct*

Codec parameters for subtitle codecs.

**Fields:**
- `codec: SubtitleCodecId` - The codec ID.
- `extra_data: Option<Box<[u8]>>` - Extra data (defined by the codec).

**Methods:**

- `fn new() -> SubtitleCodecParameters`
- `fn for_codec(self: & mut Self, codec: SubtitleCodecId) -> & mut Self` - Provide the `VideoCodecId`.
- `fn with_extra_data(self: & mut Self, data: Box<[u8]>) -> & mut Self` - Provide codec extra data.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## Module: well_known

IDs for well-known subtitle codecs.



