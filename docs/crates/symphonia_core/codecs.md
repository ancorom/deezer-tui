**symphonia_core > codecs**

# Module: codecs

## Contents

**Modules**

- [`audio`](#audio) - Audio decoder specific support.
- [`registry`](#registry) - Registry for codecs to support lookup and instantiation of decoders dynamically at runtime.
- [`subtitle`](#subtitle) - Subtitle decoder specific support.
- [`video`](#video) - Video decoder specific support.

**Structs**

- [`CodecInfo`](#codecinfo) - Basic information about a codec.
- [`CodecProfile`](#codecprofile) - A codec-specific identification code for a profile.
- [`CodecProfileInfo`](#codecprofileinfo) - Basic information about a codec profile.

**Enums**

- [`CodecId`](#codecid) - Generic wrapper around type-specific codec IDs.
- [`CodecParameters`](#codecparameters) - Generic wrapper around type-specific codec parameters.

---

## symphonia_core::codecs::CodecId

*Enum*

Generic wrapper around type-specific codec IDs.

**Variants:**
- `Audio(crate::codecs::audio::AudioCodecId)` - Codec ID for an audio codec.
- `Video(crate::codecs::video::VideoCodecId)` - Codec ID for a video codec.
- `Subtitle(crate::codecs::subtitle::SubtitleCodecId)` - Codec ID for a subtitle codec.

**Traits:** Copy, Eq

**Trait Implementations:**

- **From**
  - `fn from(value: VideoCodecId) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **From**
  - `fn from(value: SubtitleCodecId) -> Self`
- **From**
  - `fn from(value: AudioCodecId) -> Self`



## symphonia_core::codecs::CodecInfo

*Struct*

Basic information about a codec.

**Fields:**
- `short_name: &'static str` - A short ASCII-only string identifying the codec.
- `long_name: &'static str` - A longer, more descriptive, string identifying the codec.
- `profiles: &'static [CodecProfileInfo]` - A list of codec profiles.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## symphonia_core::codecs::CodecParameters

*Enum*

Generic wrapper around type-specific codec parameters.

**Variants:**
- `Audio(crate::codecs::audio::AudioCodecParameters)` - Codec parameters for an audio codec.
- `Video(crate::codecs::video::VideoCodecParameters)` - Codec parameters for a video codec.
- `Subtitle(crate::codecs::subtitle::SubtitleCodecParameters)` - Codec parameters for a subtitle codec.

**Methods:**

- `fn is_audio(self: &Self) -> bool` - Returns `true` if the codec parameters are for an audio codec.
- `fn audio(self: &Self) -> Option<&AudioCodecParameters>` - If the codec parameters are for an audio codec, returns an immutable reference to the
- `fn audio_mut(self: & mut Self) -> Option<& mut AudioCodecParameters>` - If the codec parameters are for an audio codec, returns a mutable reference to the
- `fn is_video(self: &Self) -> bool` - Returns `true` if the codec parameters are for a video codec.
- `fn video(self: &Self) -> Option<&VideoCodecParameters>` - If the codec parameters are for an video codec, returns an immutable reference to the
- `fn video_mut(self: & mut Self) -> Option<& mut VideoCodecParameters>` - If the codec parameters are for an video codec, returns a mutable reference to the
- `fn is_subtitle(self: &Self) -> bool` - Returns `true` if the codec parameters are for an subtitle codec.
- `fn subtitle(self: &Self) -> Option<&SubtitleCodecParameters>` - If the codec parameters are for an subtitle codec, returns an immutable reference to the
- `fn subtitle_mut(self: & mut Self) -> Option<& mut SubtitleCodecParameters>` - If the codec parameters are for an subtitle codec, returns a mutable reference to the

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(value: SubtitleCodecParameters) -> Self`
- **From**
  - `fn from(value: AudioCodecParameters) -> Self`
- **From**
  - `fn from(value: VideoCodecParameters) -> Self`



## symphonia_core::codecs::CodecProfile

*Struct*

A codec-specific identification code for a profile.

In general, codec profiles are designed to target specific applications, and define a set of
minimum capabilities a decoder must implement to successfully decode a bitstream. For an
encoder, a profile imposes a set of constraints upon the bitstream it produces.

**Tuple Struct**: `()`

**Methods:**

- `fn new(code: u32) -> Self` - Create a new codec profile from a profile code.
- `fn get(self: &Self) -> u32` - Get the profile code.

**Traits:** Copy, Eq

**Trait Implementations:**

- **From**
  - `fn from(value: u32) -> Self`
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



## symphonia_core::codecs::CodecProfileInfo

*Struct*

Basic information about a codec profile.

**Fields:**
- `profile: CodecProfile` - The codec profile.
- `short_name: &'static str` - A short ASCII-only string identifying the codec profile.
- `long_name: &'static str` - A longer, more descriptive, string identifying the codec profile

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## Module: audio

Audio decoder specific support.



## Module: registry

Registry for codecs to support lookup and instantiation of decoders dynamically at runtime.



## Module: subtitle

Subtitle decoder specific support.



## Module: video

Video decoder specific support.



