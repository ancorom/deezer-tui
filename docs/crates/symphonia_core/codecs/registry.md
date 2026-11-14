**symphonia_core > codecs > registry**

# Module: codecs::registry

## Contents

**Structs**

- [`CodecRegistry`](#codecregistry) - A `CodecRegistry` allows the registration of codecs, and provides a method to instantiate a
- [`RegisteredAudioDecoder`](#registeredaudiodecoder) - Registration details of an audio decoder for a particular audio codec.
- [`SupportedAudioCodec`](#supportedaudiocodec) - Description of a supported audio codec.

**Traits**

- [`RegisterableAudioDecoder`](#registerableaudiodecoder) - To support registration in a codec registry, an `AudioDecoder` must implement the

**Type Aliases**

- [`AudioDecoderFactoryFn`](#audiodecoderfactoryfn) - `AudioDecoder` factory function. Creates a boxed `AudioDecoder`.

---

## symphonia_core::codecs::registry::AudioDecoderFactoryFn

*Type Alias*: `fn(...)`

`AudioDecoder` factory function. Creates a boxed `AudioDecoder`.



## symphonia_core::codecs::registry::CodecRegistry

*Struct*

A `CodecRegistry` allows the registration of codecs, and provides a method to instantiate a
`Decoder` given a `CodecParameters` object.

**Methods:**

- `fn new() -> Self` - Instantiate a new `CodecRegistry`.
- `fn get_audio_decoder(self: &Self, id: AudioCodecId) -> Option<&RegisteredAudioDecoder>` - Get the registration information of the most preferred audio decoder for the specified
- `fn get_audio_decoder_at_tier(self: &Self, tier: Tier, id: AudioCodecId) -> Option<&RegisteredAudioDecoder>` - Get the registration information of the audio decoder at the specified tier for the
- `fn register_audio_decoder<C>(self: & mut Self)` - Registers all audio codecs supported by the audio decoder at the standard tier.
- `fn register_audio_decoder_at_tier<C>(self: & mut Self, tier: Tier)` - Registers all audio codecs supported by the audio decoder at a specific tier.
- `fn make_audio_decoder(self: &Self, params: &AudioCodecParameters, opts: &AudioDecoderOptions) -> Result<Box<dyn AudioDecoder>>` - Instantiate an audio decoder for the specified audio codec parameters.

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`



## symphonia_core::codecs::registry::RegisterableAudioDecoder

*Trait*

To support registration in a codec registry, an `AudioDecoder` must implement the
`RegisterableAudioDecoder` trait.

**Methods:**

- `try_registry_new`
- `supported_codecs`: Get a list of audio codecs supported by this decoder.



## symphonia_core::codecs::registry::RegisteredAudioDecoder

*Struct*

Registration details of an audio decoder for a particular audio codec.

**Fields:**
- `codec: SupportedAudioCodec` - Audio codec details.
- `factory: AudioDecoderFactoryFn` - Factory function to instantiate the audio decoder.



## symphonia_core::codecs::registry::SupportedAudioCodec

*Struct*

Description of a supported audio codec.

**Fields:**
- `id: crate::codecs::audio::AudioCodecId`
- `info: crate::codecs::CodecInfo`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`



