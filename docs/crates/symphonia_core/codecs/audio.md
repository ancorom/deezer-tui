**symphonia_core > codecs > audio**

# Module: codecs::audio

## Contents

**Modules**

- [`well_known`](#well_known) - Codec IDs and profiles for well-known audio codecs.

**Structs**

- [`AudioCodecId`](#audiocodecid) - An `AudioCodecId` is a unique identifier used to identify a specific audio codec.
- [`AudioCodecParameters`](#audiocodecparameters) - Codec parameters for audio codecs.
- [`AudioDecoderOptions`](#audiodecoderoptions) - `AudioDecoderOptions` is a common set of options that all audio decoders use.
- [`FinalizeResult`](#finalizeresult) - `FinalizeResult` contains optional information that can only be found, calculated, or

**Enums**

- [`VerificationCheck`](#verificationcheck) - A method and expected value to perform verification on the decoded audio.

**Traits**

- [`AudioDecoder`](#audiodecoder) - An `AudioDecoder` implements an audio codec's decode algorithm. It consumes `Packet`s and

**Constants**

- [`CODEC_ID_NULL_AUDIO`](#codec_id_null_audio) - Null audio codec ID

---

## symphonia_core::codecs::audio::AudioCodecId

*Struct*

An `AudioCodecId` is a unique identifier used to identify a specific audio codec.

# Creating a Codec ID

Using a [well-known](well_known) codec ID is *highly* recommended to maximize compatibility
between components, libraries, and applications. However, if a codec requires custom codec ID,
or there is no well-known ID, then the [`FourCc`] for the codec may be converted into a codec
ID.

**Tuple Struct**: `()`

**Methods:**

- `fn new(cc: FourCc) -> AudioCodecId` - Create a new audio codec ID from a FourCC.

**Traits:** Eq, Copy

**Trait Implementations:**

- **From**
  - `fn from(value: FourCc) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> $crate::cmp::Ordering`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## symphonia_core::codecs::audio::AudioCodecParameters

*Struct*

Codec parameters for audio codecs.

**Fields:**
- `codec: AudioCodecId` - The codec ID.
- `profile: Option<crate::codecs::CodecProfile>` - The codec-defined profile.
- `sample_rate: Option<u32>` - The sample rate of the audio in Hz.
- `sample_format: Option<crate::audio::sample::SampleFormat>` - The sample format of an audio sample.
- `bits_per_sample: Option<u32>` - The number of bits per one decoded audio sample.
- `bits_per_coded_sample: Option<u32>` - The number of bits per one encoded audio sample.
- `channels: Option<crate::audio::Channels>` - Audio channels.
- `max_frames_per_packet: Option<u64>` - The maximum number of frames a packet will contain.
- `verification_check: Option<VerificationCheck>` - A method and expected value that may be used to perform verification on the decoded audio.
- `frames_per_block: Option<u64>` - The number of frames per block, in case packets are seperated in multiple blocks.
- `extra_data: Option<Box<[u8]>>` - Extra data (defined by the codec).

**Methods:**

- `fn new() -> AudioCodecParameters`
- `fn for_codec(self: & mut Self, codec: AudioCodecId) -> & mut Self` - Provide the `AudioCodecId`.
- `fn with_profile(self: & mut Self, profile: CodecProfile) -> & mut Self` - Provide codec profile.
- `fn with_sample_rate(self: & mut Self, sample_rate: u32) -> & mut Self` - Provide the sample rate in Hz.
- `fn with_sample_format(self: & mut Self, sample_format: SampleFormat) -> & mut Self` - Provide the codec's decoded audio sample format.
- `fn with_bits_per_sample(self: & mut Self, bits_per_sample: u32) -> & mut Self` - Provide the bit per sample of a decoded audio sample.
- `fn with_bits_per_coded_sample(self: & mut Self, bits_per_coded_sample: u32) -> & mut Self` - Provide the bits per sample of an encoded audio sample.
- `fn with_channels(self: & mut Self, channels: Channels) -> & mut Self` - Provide the channel map.
- `fn with_max_frames_per_packet(self: & mut Self, len: u64) -> & mut Self` - Provide the maximum number of frames per packet.
- `fn with_frames_per_block(self: & mut Self, len: u64) -> & mut Self` - Provide the maximum number of frames per packet.
- `fn with_extra_data(self: & mut Self, data: Box<[u8]>) -> & mut Self` - Provide codec extra data.
- `fn with_verification_code(self: & mut Self, code: VerificationCheck) -> & mut Self` - Provide a verification code of the final decoded audio.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## symphonia_core::codecs::audio::AudioDecoder

*Trait*

An `AudioDecoder` implements an audio codec's decode algorithm. It consumes `Packet`s and
produces buffers of PCM audio.

**Methods:**

- `reset`: Reset the decoder.
- `codec_info`: Get basic information about the codec.
- `codec_params`: Gets a reference to an updated set of `AudioCodecParameters` based on the codec parameters
- `decode`: Decodes a `Packet` of audio data and returns a generic (untyped) audio buffer reference
- `decode_ref`: Decodes a `PacketRef` of audio data and returns a generic (untyped) audio buffer reference
- `finalize`: Optionally, obtain post-decode information such as the verification status.
- `last_decoded`: Allows read access to the internal audio buffer.



## symphonia_core::codecs::audio::AudioDecoderOptions

*Struct*

`AudioDecoderOptions` is a common set of options that all audio decoders use.

**Fields:**
- `gapless: bool` - Enable support for gapless playback.
- `verify: bool` - The decoded audio should be verified if possible during the decode process.

**Methods:**

- `fn gapless(self: Self, enable: bool) -> Self` - Enable or disable support for gapless playback.
- `fn verify(self: Self, verify: bool) -> Self` - Enable or disable decoded audio verification if the decoder supports it.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`



## symphonia_core::codecs::audio::CODEC_ID_NULL_AUDIO

*Constant*: `AudioCodecId`

Null audio codec ID



## symphonia_core::codecs::audio::FinalizeResult

*Struct*

`FinalizeResult` contains optional information that can only be found, calculated, or
determined after decoding is complete.

**Fields:**
- `verify_ok: Option<bool>` - If verification is enabled and supported by the decoder, provides the verification result

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`



## symphonia_core::codecs::audio::VerificationCheck

*Enum*

A method and expected value to perform verification on the decoded audio.

**Variants:**
- `Crc8(u8)` - CRC8 of interleaved PCM audio samples.
- `Crc16([u8; 2])` - CRC16 of interleaved PCM audio samples.
- `Crc32([u8; 4])` - CRC32 of interleaved PCM audio samples.
- `Md5([u8; 16])` - MD5 of interleaved PCM audio samples.
- `Other([u8; 16])` - Codec defined, up-to 16-byte code.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## Module: well_known

Codec IDs and profiles for well-known audio codecs.



