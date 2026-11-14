**symphonia_core > codecs > video**

# Module: codecs::video

## Contents

**Modules**

- [`well_known`](#well_known) - Codec IDs and profiles for well-known video codecs.

**Structs**

- [`VideoCodecId`](#videocodecid) - An `VideoCodecId` is a unique identifier used to identify a specific video codec.
- [`VideoCodecParameters`](#videocodecparameters) - Codec parameters for video codecs.
- [`VideoExtraData`](#videoextradata) - Extra data for a video codec.
- [`VideoExtraDataId`](#videoextradataid) - An `VideoExtraDataId` is a unique identifier used to identify a specific video extra data.

**Constants**

- [`CODEC_ID_NULL_VIDEO`](#codec_id_null_video) - Null video codec ID
- [`VIDEO_EXTRA_DATA_ID_NULL`](#video_extra_data_id_null) - Null video extra data ID.

---

## symphonia_core::codecs::video::CODEC_ID_NULL_VIDEO

*Constant*: `VideoCodecId`

Null video codec ID



## symphonia_core::codecs::video::VIDEO_EXTRA_DATA_ID_NULL

*Constant*: `VideoExtraDataId`

Null video extra data ID.



## symphonia_core::codecs::video::VideoCodecId

*Struct*

An `VideoCodecId` is a unique identifier used to identify a specific video codec.

# Creating a Codec ID

Using a [well-known](well_known) codec ID is *highly* recommended to maximize compatibility
between components, libraries, and applications. However, if a codec requires custom codec ID,
or there is no well-known ID, then the [`FourCc`] for the codec may be converted into a codec
ID.

**Tuple Struct**: `()`

**Methods:**

- `fn new(cc: FourCc) -> VideoCodecId` - Create a new video codec ID from a FourCC.

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



## symphonia_core::codecs::video::VideoCodecParameters

*Struct*

Codec parameters for video codecs.

**Fields:**
- `codec: VideoCodecId` - The codec ID.
- `profile: Option<crate::codecs::CodecProfile>` - The codec-defined profile.
- `level: Option<u32>` - The codec-defined level.
- `width: Option<u16>` - Video width.
- `height: Option<u16>` - Video height.
- `extra_data: Vec<VideoExtraData>` - Extra data (defined by the codec).

**Methods:**

- `fn for_codec(self: & mut Self, codec: VideoCodecId) -> & mut Self` - Provide the `VideoCodecId`.
- `fn with_profile(self: & mut Self, profile: CodecProfile) -> & mut Self` - Provide codec profile.
- `fn with_level(self: & mut Self, level: u32) -> & mut Self` - Provide codec level.
- `fn with_width(self: & mut Self, width: u16) -> & mut Self` - Provide video width.
- `fn with_height(self: & mut Self, height: u16) -> & mut Self` - Provide video height.
- `fn add_extra_data(self: & mut Self, data: VideoExtraData) -> & mut Self` - Adds codec's extra data.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## symphonia_core::codecs::video::VideoExtraData

*Struct*

Extra data for a video codec.

**Fields:**
- `id: VideoExtraDataId` - The extra data ID.
- `data: Box<[u8]>` - Extra data (defined by codec)

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`



## symphonia_core::codecs::video::VideoExtraDataId

*Struct*

An `VideoExtraDataId` is a unique identifier used to identify a specific video extra data.

**Tuple Struct**: `()`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`



## Module: well_known

Codec IDs and profiles for well-known video codecs.



