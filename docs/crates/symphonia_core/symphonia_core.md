**symphonia_core**

# Module: symphonia_core

## Contents

**Modules**

- [`audio`](#audio) - The `audio` module provides primitives for working with multi-channel audio generically across
- [`checksum`](#checksum) - The `checksum` module provides implementations of common error-detecting codes and hashing
- [`codecs`](#codecs) - The `codec` module and its sub-modules provides traits and supporting infrastructure to
- [`common`](#common) - The `common` module defines common enums, structs, types, etc.
- [`dsp`](#dsp) - The `dsp` module provides efficient implementations of common signal processing algorithms.
- [`errors`](#errors) - The `errors` module defines the common error type.
- [`formats`](#formats) - The `format` module provides the traits and support structures necessary to implement media
- [`io`](#io) - The `io` module implements composable bit- and byte-level I/O.
- [`meta`](#meta) - The `meta` module defines basic metadata elements, and management structures.
- [`packet`](#packet) - The `packet` module defines the packet structure.
- [`subtitle`](#subtitle) - The `subtitle` module provides primitives for working with text-based and rendered subtitles.
- [`units`](#units) - The `units` module provides definitions for common units.
- [`util`](#util) - The `util` module provides a repository of commonly used utility functions sorted into distinct
- [`video`](#video) - The `video` module provides primitives for working with video.

**Macros**

- [`codec_profile`](#codec_profile) - Convience macro for declaring `CodecProfileInfo`.
- [`support_audio_codec`](#support_audio_codec) - Convenience macro for declaring a `SupportedAudioCodec`.
- [`support_format`](#support_format) - Convenience macro for declaring a `ProbeData` for a `FormatReader`.
- [`support_metadata`](#support_metadata) - Convenience macro for declaring a `ProbeData` for a `MetadataReader`.
- [`support_subtitle_codec`](#support_subtitle_codec) - Convenience macro for declaring a `SupportedSubtitleCodec`.
- [`support_video_codec`](#support_video_codec) - Convenience macro for declaring a `SupportedVideoCodec`.

---

## Module: audio

The `audio` module provides primitives for working with multi-channel audio generically across
sample formats.

## Standard Sample Formats

The structs and traits in this module are generic across any type implementing the [`Sample`]
trait. However, a set of standard sample formats is defined. These standard sample formats are:
U8, S8, U16, S16, U24, S24, U32, S32, F32, and F64. These sample formats correspond to the
following underlying data types, respectively: `u8`, `i8`, `u16`, `i16`, `u24`, `i24`, `u32`,
`i32`, `f32`, and `f64`.

### Generic Wrappers

To make handling varying sample formats easier, this module also implements a set of generic
wrappers around typed audio buffers and slices. These wrappers are implemented as enums whereby
each enumerator corresponds to one of the aforementioned standard sample formats. Functions on
the generic wrappers match closely with that of the typed interface and are dispatch to the
active enumerator.



## Module: checksum

The `checksum` module provides implementations of common error-detecting codes and hashing
algorithms.



## symphonia_core::codec_profile

*Declarative Macro*

Convience macro for declaring `CodecProfileInfo`.

```rust
macro_rules! codec_profile {
    ($id:expr, $short_name:expr, $long_name:expr) => { ... };
}
```



## Module: codecs

The `codec` module and its sub-modules provides traits and supporting infrastructure to
implement audio, video, and subtitle decoders.

# Nomenclature

* A codec ID refers to a unique identifier for a specific codec.
* A codec type refers to the type of media the codec is encoding: audio, video, subtitles,
  etc.
* Codec parameters refers to a set of parameters common to a particular codec type (e.g.,
  audio sample rate).



## Module: common

The `common` module defines common enums, structs, types, etc.



## Module: dsp

The `dsp` module provides efficient implementations of common signal processing algorithms.



## Module: errors

The `errors` module defines the common error type.



## Module: formats

The `format` module provides the traits and support structures necessary to implement media
demuxers.



## Module: io

The `io` module implements composable bit- and byte-level I/O.

The following nomenclature is used to denote where the data being read is sourced from:
 * A `Stream` consumes any source implementing [`ReadBytes`] one byte at a time.
 * A `Reader` consumes a `&[u8]`.

The sole exception to this rule is [`MediaSourceStream`] which consumes sources implementing
[`MediaSource`] (aka. [`std::io::Read`]).

All `Reader`s and `Stream`s operating on bytes of data at a time implement the [`ReadBytes`]
trait. Likewise, all `Reader`s and `Stream`s operating on bits of data at a time implement
either the [`ReadBitsLtr`] or [`ReadBitsRtl`] traits depending on the order in which they
consume bits.



## Module: meta

The `meta` module defines basic metadata elements, and management structures.

# Tags

Within the context of media, a tag is single piece of metadata about the media as a whole, or a
track within the media. The storage of tags, their structure, and organization, varies based on
the metadata/tagging format.

The [`Tag`] structure represents a single tag, and abstracts over the differences in tagging
formats. `Tag` is a composition of a mandatory raw tag, and an optional standard tag.

## Raw Tags

A [`RawTag`] stores a tag in a data format that matches as closely as possible to the format of
the tag as it was written. The data format depends on the tagging format, and the writer of the
tag.

A `RawTag` consists of a mandatory key-value pair. For most tagging formats, this is sufficient
to faithfully represent the original tag, however, for some more structured tagging formats, a
set of additional key-value pairs ([`RawTagSubField`]) may be populated.

The meaning of the tag can be derived from its key, however, the key may be named differently
based on the underlying tagging format and the writer of the tag.

Raw tags can be ignored by most tag consumers. Instead, standard tags should be preferred.

## Standard Tags

A [`StandardTag`] is a parsed representation of a tag. Unlike a raw tag, a standard tag has a
well-defined data type and meaning.

A metadata reader will assign a `StandardTag` to a `Tag` if it is able to identify the meaning
of the `RawTag`, and parse its value. If the `RawTag` maps to multiple `StandardTag`s, then
the `Tag` (along with the `RawTag`) will be duplicated for each `StandardTag` with each instance
being assigned one `StandardTag`.

An end-user should prefer consuming standard tags over raw tags.

## Storage Efficiency

In many cases, the value of a `RawTag` will be the same as the `StandardTag`. Since a value may
be large, duplicating it could be wasteful. For this reason, string and binary data values are
stored using an [`Arc`].



## Module: packet

The `packet` module defines the packet structure.



## Module: subtitle

The `subtitle` module provides primitives for working with text-based and rendered subtitles.



## symphonia_core::support_audio_codec

*Declarative Macro*

Convenience macro for declaring a `SupportedAudioCodec`.

```rust
macro_rules! support_audio_codec {
    ($id:expr, $short_name:expr, $long_name:expr) => { ... };
    ($id:expr, $short_name:expr, $long_name:expr, $profiles:expr) => { ... };
}
```



## symphonia_core::support_format

*Declarative Macro*

Convenience macro for declaring a `ProbeData` for a `FormatReader`.

```rust
macro_rules! support_format {
    ($info:expr, $exts:expr, $mimes:expr, $markers:expr) => { ... };
}
```



## symphonia_core::support_metadata

*Declarative Macro*

Convenience macro for declaring a `ProbeData` for a `MetadataReader`.

```rust
macro_rules! support_metadata {
    ($info:expr, $exts:expr, $mimes:expr, $markers:expr) => { ... };
    ($info:expr, $exts:expr, $mimes:expr, $markers:expr, $anchors:expr) => { ... };
}
```



## symphonia_core::support_subtitle_codec

*Declarative Macro*

Convenience macro for declaring a `SupportedSubtitleCodec`.

```rust
macro_rules! support_subtitle_codec {
    ($id:expr, $short_name:expr, $long_name:expr) => { ... };
}
```



## symphonia_core::support_video_codec

*Declarative Macro*

Convenience macro for declaring a `SupportedVideoCodec`.

```rust
macro_rules! support_video_codec {
    ($id:expr, $short_name:expr, $long_name:expr) => { ... };
    ($id:expr, $short_name:expr, $long_name:expr, $profiles:expr) => { ... };
}
```



## Module: units

The `units` module provides definitions for common units.



## Module: util

The `util` module provides a repository of commonly used utility functions sorted into distinct
categories.

If a function is used all-over the codebase, and does not belong to specific top-level module,
it should be placed here.



## Module: video

The `video` module provides primitives for working with video.



