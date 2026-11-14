# symphonia

# Project Symphonia

Symphonia is a 100% pure Rust audio decoding and multimedia format demuxing framework.

# Support

Supported formats, codecs, and metadata tags are listed below. By default Symphonia only enables
royalty-free open standard media formats and codecs. Other formats and codecs must be enabled
using feature flags.

**Tip:** All format, codec, and metadata support can be enabled with the `all` feature flag.

## Formats

The following container formats are supported.

| Format   | Feature Flag | Gapless* | Default |
|----------|--------------|----------|---------|
| AIFF     | `aiff`       | Yes      | No      |
| CAF      | `caf`        | No       | No      |
| ISO/MP4  | `isomp4`     | No       | No      |
| MKV/WebM | `mkv`        | No       | Yes     |
| OGG      | `ogg`        | Yes      | Yes     |
| Wave     | `wav`        | Yes      | Yes     |

\* Gapless playback requires support from both the demuxer and decoder.

**Tip:** All formats can be enabled with the `all-codecs` feature flag.

## Codecs

The following codecs are supported.

| Codec    | Feature Flag | Gapless | Default |
|----------|--------------|---------|---------|
| AAC-LC   | `aac`        | No      | No      |
| ADPCM    | `adpcm`      | Yes     | Yes     |
| ALAC     | `alac`       | Yes     | No      |
| FLAC     | `flac`       | Yes     | Yes     |
| MP1      | `mp1`, `mpa` | No      | No      |
| MP2      | `mp2`, `mpa` | No      | No      |
| MP3      | `mp3`, `mpa` | Yes     | No      |
| PCM      | `pcm`        | Yes     | Yes     |
| Vorbis   | `vorbis`     | Yes     | Yes     |

**Tip:** All codecs can be enabled with the `all-codecs` feature flag. Similarly, all MPEG
audio codecs can be enabled with the `mpa` feature flag.

## Metadata

For metadata formats that are standalone and not part of the media container, a feature flag may
be used to toggle support.

| Format                | Feature Flag | Default |
|-----------------------|--------------|---------|
| APEv1                 | `ape`        | Yes     |
| APEv2                 | `ape`        | Yes     |
| ID3v1                 | `id3v1`      | Yes     |
| ID3v2                 | `id3v2`      | Yes     |
| ISO/MP4               | N/A          | N/A     |
| RIFF                  | N/A          | N/A     |
| Vorbis comment (FLAC) | N/A          | N/A     |
| Vorbis comment (OGG)  | N/A          | N/A     |

**Tip:** All metadata formats can be enabled with the `all-meta` feature flag.

## Optimizations

SIMD optimizations are enabled by default. Precise control over which SIMD instruction sets are
supported may be controlled using the following feature flags. Enabling any SIMD support feature
flag will pull in the `rustfft` dependency.

| Instruction Set | Feature Flag    | Default |
|-----------------|-----------------|---------|
| SSE             | `opt-simd-sse`  | Yes     |
| AVX             | `opt-simd-avx`  | Yes     |
| Neon            | `opt-simd-neon` | Yes     |

**Tip:** All SIMD optimizations can be enabled with the `opt-simd` feature flag.

# Experimental Features

Previews of experimental new features may be enabled by using feature flags. Experimental
features should be used for development purposes only. Before using an experimental feature,
please observe the warnings below. Never use experimental features in a production application.

| Experimental Feature   | Feature Flag          |
|------------------------|-----------------------|
| Subtitle codec support | `exp-subtitle-codecs` |
| Video codec support    | `exp-video-codecs`    |

## Warnings

* SemVer compatibilty is **not** guaranteed. Be prepared for build failures.
* Experimental features and their associated feature flags **may be removed at any time.**
* Functionality **may change or break at any time.**
* Again, **never** use in any production application.

# Usage

An example implementation of a simple audio player (symphonia-play) can be found in the
Project Symphonia git repository.

# Adding new formats and codecs

Simply implement the [`AudioDecoder`][core::codecs::audio::AudioDecoder] trait for an audio
decoder or the [`FormatReader`][core::formats::FormatReader] trait for a demuxer trait and
register with the appropriate registry or probe!

## Modules

### [`symphonia`](symphonia.md)

*1 module*

### [`default`](default.md)

*3 modules, 4 functions*

### [`default::codecs`](default/codecs.md)

*1 type alias*

### [`default::formats`](default/formats.md)

*1 type alias*



## Project usage (deezer-tui) — Phase 2 audio decoder
- Cargo features: `flac, mp3, aac, isomp4`.
- Decode pipeline (verified 2026): `get_probe().probe(hint, MediaSourceStream::new(reader), fmt_opts, meta_opts)` -> format
  - `format.default_track(TrackType::Audio).codec_params.audio()` -> track
  - `get_codecs().make_audio_decoder(track, dec_opts)` -> decoder
- Decode loop: `format.next_packet()` -> `Ok(None)=EOF | Ok(Some(pkt))`; skip non-audio by `pkt.track_id != track.id`; `decoder.decode(pkt)` -> generic buffer; f32 interleaved via `buf.copy_to_slice_interleaved(&mut out)` (resize `out` to `buf.samples_interleaved()` first).
- Source: `src/audio/decoder.rs`.

### Verified API signatures (symphonia 0.6, `src/audio/decoder.rs`)
- `FormatReader` is a **trait**, not a type: `probe()` returns `Result<Box<dyn FormatReader>, ProbeError>` — keep it as `Box<dyn FormatReader>` in `Decoder::format`.
- `MediaSourceStream::new(Box<dyn MediaSource>, MediaSourceStreamOptions)` — wrap `Box<dyn Read + Send + Sync>` in `ReadOnlySource` to get the `MediaSource`.
- `FormatReader::default_track(TrackType::Audio)` is the decoder entry point; the track's `codec_params: Option<CodecParameters>` → `.as_ref().expect("…")`.
- `CodecParameters.audio()` returns `Option<&AudioCodecParameters>` (the codec's sample rate, channels, bit depth).
- `get_codecs()` (a `Codecs` collector) → `make_audio_decoder(codec: &AudioCodecParameters, &AudioDecoderOptions::default()) -> Box<dyn AudioDecoder>`.
- The decoder keeps `format` (reader) alive; `Decoder` must not double-move the reader — do NOT add a separate `reader` field to `Decoder`.
- `AudioDecoder::decode(&mut self, pkt) -> Result<AudioBuffer, SymphoniaError>`; `AudioBuffer::copy_to_slice_interleaved(&mut out)` writes interleaved f32 (size must be `buf.samples_interleaved()`).