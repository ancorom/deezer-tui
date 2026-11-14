**symphonia_core > formats**

# Module: formats

## Contents

**Modules**

- [`prelude`](#prelude) - The `formats` module prelude for format reader implementers.
- [`probe`](#probe) - The `probe` module provides methods and traits to support auto-detection of media formats from
- [`util`](#util) - Helper utilities for implementing media demuxers.
- [`well_known`](#well_known) - IDs for well-known container formats.

**Structs**

- [`ExternalFormatData`](#externalformatdata) - `ExternalFormatData` contains supplementary data related to the media container that was read
- [`FileAttachment`](#fileattachment) - A file attachment.
- [`FormatId`](#formatid) - A `FormatId` is a unique identifier used to identify a specific container format.
- [`FormatInfo`](#formatinfo) - Basic information about a container format.
- [`FormatOptions`](#formatoptions) - `FormatOptions` is a common set of options that all demuxers use.
- [`MediaInfo`](#mediainfo) - Information about a piece of media as a whole.
- [`SeekedTo`](#seekedto) - `SeekedTo` is the result of a seek.
- [`Track`](#track) - A `Track` is an independently coded media bitstream. A media format may contain multiple tracks
- [`TrackFlags`](#trackflags) - Flags indicating certain attributes about a track.
- [`VendorDataAttachment`](#vendordataattachment) - Application or vendor-specific proprietary binary data attachment.

**Enums**

- [`Attachment`](#attachment) - An attachment is additional data that is carried along with the container format.
- [`SeekMode`](#seekmode) - `SeekMode` selects the precision of a seek.
- [`SeekTo`](#seekto) - `SeekTo` specifies a position to seek to.
- [`TrackType`](#tracktype) - The track type.

**Traits**

- [`FormatReader`](#formatreader) - A `FormatReader` is a media container demuxer. It provides methods to read a media container

**Constants**

- [`FORMAT_ID_NULL`](#format_id_null) - Null container format

---

## symphonia_core::formats::Attachment

*Enum*

An attachment is additional data that is carried along with the container format.

**Variants:**
- `File(FileAttachment)` - A file.
- `VendorData(VendorDataAttachment)` - Application or vendor-specific data.



## symphonia_core::formats::ExternalFormatData

*Struct*

`ExternalFormatData` contains supplementary data related to the media container that was read
before the start of the container, or provided through some other side-channel.

**Fields:**
- `metadata: Option<crate::meta::MetadataLog>` - Optional metadata.
- `chapters: Option<crate::meta::ChapterGroup>` - Optional chapter information.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## symphonia_core::formats::FORMAT_ID_NULL

*Constant*: `FormatId`

Null container format



## symphonia_core::formats::FileAttachment

*Struct*

A file attachment.

**Fields:**
- `name: String` - The file name.
- `description: Option<String>` - An optional description of the file.
- `media_type: Option<String>` - An optional media-type describing the file data.
- `data: Box<[u8]>` - The file data.



## symphonia_core::formats::FormatId

*Struct*

A `FormatId` is a unique identifier used to identify a specific container format.

**Tuple Struct**: `()`

**Methods:**

- `fn new(cc: FourCc) -> FormatId` - Create a new format ID from a FourCC.

**Traits:** Copy, Eq

**Trait Implementations:**

- **From**
  - `fn from(value: FourCc) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## symphonia_core::formats::FormatInfo

*Struct*

Basic information about a container format.

**Fields:**
- `format: FormatId` - The `FormatId` identifier.
- `short_name: &'static str` - A short ASCII-only string identifying the format.
- `long_name: &'static str` - A longer, more descriptive, string identifying the format.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`



## symphonia_core::formats::FormatOptions

*Struct*

`FormatOptions` is a common set of options that all demuxers use.

**Fields:**
- `prebuild_seek_index: bool` - If a `FormatReader` requires a seek index, but the container does not provide one, build the
- `seek_index_fill_period_ms: u16` - If a seek index needs to be built, this value determines the period, in milliseconds, at
- `external_data: ExternalFormatData` - External, supplementary, data related to the media container read before the start of the

**Methods:**

- `fn prebuild_seek_index(self: Self, prebuild: bool) -> Self` - If a `FormatReader` requires a seek index, but the container does not provide one, build the
- `fn seek_index_fill_period_ms(self: Self, period: u16) -> Self` - If a seek index needs to be built, this value determines the period, in milliseconds, at

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## symphonia_core::formats::FormatReader

*Trait*

A `FormatReader` is a media container demuxer. It provides methods to read a media container
and iterate over the codec bitstream packets of all encapsulated tracks. Additionally, it
provides methods to access any metadata, chapters, or attachments.

Most, if not all, media containers contain metadata, then a number of packetized, and
interleaved codec bitstreams. These bitstreams are usually referred to as tracks. Generally,
the encapsulated bitstreams are independently encoded using some codec. The allowed codecs for a
container are defined in the specification of the container format.

While demuxing, packets are read one-by-one and may be discarded or decoded at the choice of
the caller. The contents of a packet is undefined: it may be a frame of video, a millisecond
of audio, or a subtitle, but a packet will never contain data from two different bitstreams.
Therefore the caller can be selective in what tracks(s) should be decoded and consumed.

`FormatReader` provides an Iterator-like interface over packets for easy consumption and
filtering. Seeking will invalidate the state of any `Decoder` processing packets from the
`FormatReader` and should be reset after a successful seek operation.

**Methods:**

- `format_info`: Get basic information about the container format.
- `media_info`: Get information about the media as a whole.
- `attachments`: Get a list of all attachments.
- `chapters`: Get media chapters, if available.
- `metadata`: Gets the metadata revision log.
- `seek`: Seek, as precisely as possible depending on the mode, to the `Time` or track `TimeStamp`
- `tracks`: Gets a list of tracks in the container.
- `first_track`: Get the first track of a certain track type.
- `first_track_known_codec`: Get the first track of a certain track type with a known (non-null) codec.
- `default_track`: Get the default track of a certain track type.
- `next_packet`: Reader the next packet from the container.
- `into_inner`: Consumes the `FormatReader` and returns the underlying media source stream



## symphonia_core::formats::MediaInfo

*Struct*

Information about a piece of media as a whole.

**Fields:**
- `time_base: Option<crate::units::TimeBase>` - The timebase of the media.
- `duration: Option<crate::units::Duration>` - The duration of the media in timebase units.
- `start_ts: crate::units::Timestamp` - The timestamp of the first frame in timebase units.

**Methods:**

- `fn from_track(track: &Track) -> Self` - For media that contains a single track, populates and returns `MediaInfo` from that track.
- `fn from_tracks(tracks: &[Track]) -> Self` - For media that contains multiple tracks, populates and returns `MediaInfo` using the
- `fn new() -> Self`
- `fn with_time_base(self: & mut Self, time_base: TimeBase) -> & mut Self` - Provide the `TimeBase`.
- `fn with_duration(self: & mut Self, duration: Duration) -> & mut Self` - Provide the duration in timebase units.
- `fn with_start_ts(self: & mut Self, start_ts: Timestamp) -> & mut Self` - Provide the timestamp of the first frame.

**Traits:** Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## symphonia_core::formats::SeekMode

*Enum*

`SeekMode` selects the precision of a seek.

**Variants:**
- `Coarse` - Coarse seek mode is a best-effort attempt to seek to the requested position. The actual
- `Accurate` - Accurate (aka sample-accurate) seek mode will be always seek to a position before the

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## symphonia_core::formats::SeekTo

*Enum*

`SeekTo` specifies a position to seek to.

**Variants:**
- `Time{ time: crate::units::Time, track_id: Option<u32> }` - Seek to a `Time` in regular time units (i.e., seconds).
- `Timestamp{ ts: crate::units::Timestamp, track_id: u32 }` - Seek to a track's `Timestamp` in that track's timebase units.



## symphonia_core::formats::SeekedTo

*Struct*

`SeekedTo` is the result of a seek.

**Fields:**
- `track_id: u32` - The track the seek was relative to.
- `required_ts: crate::units::Timestamp` - The `TimeStamp` required for the requested seek.
- `actual_ts: crate::units::Timestamp` - The `TimeStamp` that was seeked to.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## symphonia_core::formats::Track

*Struct*

A `Track` is an independently coded media bitstream. A media format may contain multiple tracks
in one container. Each of those tracks are represented by one `Track`.

**Fields:**
- `id: u32` - A unique identifier for the track.
- `codec_params: Option<crate::codecs::CodecParameters>` - The codec parameters for the track.
- `language: Option<String>` - The language of the track. May be unknown or not set.
- `time_base: Option<crate::units::TimeBase>` - The timebase of the track.
- `num_frames: Option<u64>` - The length of the track in number of audio, video, or subtitle frames.
- `duration: Option<crate::units::Duration>` - The duration of the track in timebase units.
- `start_ts: crate::units::Timestamp` - The timestamp of the first frame.
- `delay: Option<u32>` - The number of leading frames inserted by the encoder that should be skipped during playback.
- `padding: Option<u32>` - The number of trailing frames inserted by the encoder for padding that should be skipped
- `flags: TrackFlags` - Flags indicating track attributes.

**Methods:**

- `fn new(id: u32) -> Self` - Instantiate a new track with a given ID.
- `fn with_codec_params(self: & mut Self, codec_params: CodecParameters) -> & mut Self` - Provide the codec parameters.
- `fn with_language(self: & mut Self, language: &str) -> & mut Self` - Provide the track language.
- `fn with_time_base(self: & mut Self, time_base: TimeBase) -> & mut Self` - Provide the `TimeBase`.
- `fn with_num_frames(self: & mut Self, num_frames: u64) -> & mut Self` - Provide the total number of frames.
- `fn with_duration(self: & mut Self, duration: Duration) -> & mut Self` - Provide the duration in timebase units.
- `fn with_start_ts(self: & mut Self, start_ts: Timestamp) -> & mut Self` - Provide the timestamp of the first frame.
- `fn with_delay(self: & mut Self, delay: u32) -> & mut Self` - Provide the number of delay frames.
- `fn with_padding(self: & mut Self, padding: u32) -> & mut Self` - Provide the number of padding frames.
- `fn with_flags(self: & mut Self, flags: TrackFlags) -> & mut Self` - Append provided track flags.
- `fn track_type(self: &Self) -> Option<TrackType>` - Get the track type.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## symphonia_core::formats::TrackFlags

*Struct*

Flags indicating certain attributes about a track.

**Tuple Struct**: `()`

**Methods:**

- `fn iter(self: &Self) -> $crate::iter::Iter<TrackFlags>` - Yield a set of contained flags values.
- `fn iter_names(self: &Self) -> $crate::iter::IterNames<TrackFlags>` - Yield a set of contained named flags values.
- `fn empty() -> Self` - Get a flags value with all bits unset.
- `fn all() -> Self` - Get a flags value with all known bits set.
- `fn bits(self: &Self) -> u32` - Get the underlying bits value.
- `fn from_bits(bits: u32) -> $crate::__private::core::option::Option<Self>` - Convert from a bits value.
- `fn from_bits_truncate(bits: u32) -> Self` - Convert from a bits value, unsetting any unknown bits.
- `fn from_bits_retain(bits: u32) -> Self` - Convert from a bits value exactly.
- `fn from_name(name: &str) -> $crate::__private::core::option::Option<Self>` - Get a flags value with the bits of a flag with the given name set.
- `fn is_empty(self: &Self) -> bool` - Whether all bits in `self` are unset.
- `fn is_all(self: &Self) -> bool` - Whether all known bits in this flags value are set.
- `fn intersects(self: &Self, other: Self) -> bool` - Whether any set bits in `other` are also set in `self`.
- `fn contains(self: &Self, other: Self) -> bool` - Whether all set bits in `other` are also set in `self`.
- `fn insert(self: & mut Self, other: Self)` - The bitwise or (`|`) of the bits in `self` and `other`.
- `fn remove(self: & mut Self, other: Self)` - The intersection of `self` with the complement of `other` (`&!`).
- `fn toggle(self: & mut Self, other: Self)` - The bitwise exclusive-or (`^`) of the bits in `self` and `other`.
- `fn set(self: & mut Self, other: Self, value: bool)` - Call `insert` when `value` is `true` or `remove` when `value` is `false`.
- `fn intersection(self: Self, other: Self) -> Self` - The bitwise and (`&`) of the bits in `self` and `other`.
- `fn union(self: Self, other: Self) -> Self` - The bitwise or (`|`) of the bits in `self` and `other`.
- `fn difference(self: Self, other: Self) -> Self` - The intersection of `self` with the complement of `other` (`&!`).
- `fn symmetric_difference(self: Self, other: Self) -> Self` - The bitwise exclusive-or (`^`) of the bits in `self` and `other`.
- `fn complement(self: Self) -> Self` - The bitwise negation (`!`) of the bits in `self`, truncating the result.

**Traits:** PublicFlags, Eq, Copy

**Trait Implementations:**

- **Extend**
  - `fn extend<T>(self: & mut Self, iterator: T)` - The bitwise or (`|`) of the bits in each flags value.
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`
- **Octal**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: Self)` - The bitwise and (`&`) of the bits in `self` and `other`.
- **Sub**
  - `fn sub(self: Self, other: Self) -> Self` - The intersection of `self` with the complement of `other` (`&!`).
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: Self)` - The bitwise or (`|`) of the bits in `self` and `other`.
- **BitXor**
  - `fn bitxor(self: Self, other: Self) -> Self` - The bitwise exclusive-or (`^`) of the bits in `self` and `other`.
- **FromIterator**
  - `fn from_iter<T>(iterator: T) -> Self` - The bitwise or (`|`) of the bits in each flags value.
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: Self)` - The intersection of `self` with the complement of `other` (`&!`).
- **Not**
  - `fn not(self: Self) -> Self` - The bitwise negation (`!`) of the bits in `self`, truncating the result.
- **Flags**
  - `fn bits(self: &Self) -> u32`
  - `fn from_bits_retain(bits: u32) -> TrackFlags`
  - `fn all_named() -> TrackFlags`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: Self)` - The bitwise exclusive-or (`^`) of the bits in `self` and `other`.
- **Binary**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: Self) -> Self` - The bitwise and (`&`) of the bits in `self` and `other`.
- **Default**
  - `fn default() -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: TrackFlags) -> Self` - The bitwise or (`|`) of the bits in `self` and `other`.



## symphonia_core::formats::TrackType

*Enum*

The track type.

**Variants:**
- `Audio` - An audio track.
- `Video` - A video track.
- `Subtitle` - A subtitle track.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## symphonia_core::formats::VendorDataAttachment

*Struct*

Application or vendor-specific proprietary binary data attachment.

**Fields:**
- `ident: String` - A text representation of the vendor's application identifier.
- `data: Box<[u8]>` - The vendor data.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## Module: prelude

The `formats` module prelude for format reader implementers.



## Module: probe

The `probe` module provides methods and traits to support auto-detection of media formats from
arbitrary media streams.



## Module: util

Helper utilities for implementing media demuxers.



## Module: well_known

IDs for well-known container formats.



