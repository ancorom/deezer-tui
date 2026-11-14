**symphonia_core > meta**

# Module: meta

## Contents

**Modules**

- [`well_known`](#well_known) - IDs for well-known metadata formats.

**Structs**

- [`Chapter`](#chapter) - A chapter is a labelled section of a piece of media with a defined start time.
- [`ChapterGroup`](#chaptergroup) - A group of chapters and/or other chapter groups.
- [`ColorPaletteInfo`](#colorpaletteinfo) - A description of the color palette for indexed color mode.
- [`Metadata`](#metadata) - A reference to the metadata inside of a [`MetadataLog`].
- [`MetadataBuffer`](#metadatabuffer) - The decoded contents of read metadata.
- [`MetadataBuilder`](#metadatabuilder) - A builder for [`MetadataRevision`].
- [`MetadataContainer`](#metadatacontainer) - A container of metadata tags and pictures.
- [`MetadataId`](#metadataid) - A `MetadataId` is a unique identifier used to identify a specific metadata format.
- [`MetadataInfo`](#metadatainfo) - Basic information about a metadata format.
- [`MetadataLog`](#metadatalog) - A queue for time-ordered [`MetadataRevision`]s.
- [`MetadataOptions`](#metadataoptions) - A common set of options that all metadata readers use.
- [`MetadataRevision`](#metadatarevision) - A metadata revision is a container for a single discrete version of media metadata.
- [`PerTrackMetadata`](#pertrackmetadata) - Container for metadata associated with a specific track. A [`MetadataContainer`] wrapper
- [`PerTrackMetadataBuilder`](#pertrackmetadatabuilder) - A builder for [`PerTrackMetadata`].
- [`RawTag`](#rawtag) - A raw tag represents a tag in a data format that matches, as closely as possible, to the data
- [`RawTagSubField`](#rawtagsubfield) - A key-value pair of supplementary data that can be attached to a raw tag.
- [`Size`](#size) - A 2-dimensional (width and height) size type.
- [`Tag`](#tag) - A tag encapsulates a single piece of metadata.
- [`Visual`](#visual) - A `Visual` is any 2 dimensional graphic.

**Enums**

- [`ChapterGroupItem`](#chaptergroupitem) - A chapter group item is either a chapter or chapter group.
- [`ColorMode`](#colormode) - Indicates how colors are represented in the image.
- [`ColorModel`](#colormodel) - A color model describes how a color is represented.
- [`ContentAdvisory`](#contentadvisory) - A content advisory.
- [`MetadataSideData`](#metadatasidedata) - Enumeration of types of metadata side data.
- [`RawValue`](#rawvalue) - The value of a [`RawTag`].
- [`StandardTag`](#standardtag) - A standard tag is an enumeration of well-defined and well-known tags with parsed values.
- [`StandardVisualKey`](#standardvisualkey) - `StandardVisualKey` is an enumeration providing standardized keys for common visual dispositions.

**Traits**

- [`MetadataReader`](#metadatareader) - A `MetadataReader` reads and decodes metadata and produces a revision of that decoded metadata.

**Constants**

- [`METADATA_ID_NULL`](#metadata_id_null) - Null metadata format

---

## symphonia_core::meta::Chapter

*Struct*

A chapter is a labelled section of a piece of media with a defined start time.

**Fields:**
- `start_time: crate::units::Time` - The offset from the beginning of the media to the start of the chapter.
- `end_time: Option<crate::units::Time>` - The offset from the beginning of the media to the end of the chapter.
- `start_byte: Option<u64>` - The byte position from the beginning of the media source to the first byte of the first
- `end_byte: Option<u64>` - The byte position from the beginning of the media source to the first byte of the frame
- `tags: Vec<Tag>` - The tags associated with the chapter.
- `visuals: Vec<Visual>` - The visuals associated with the chapter.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## symphonia_core::meta::ChapterGroup

*Struct*

A group of chapters and/or other chapter groups.

**Fields:**
- `items: Vec<ChapterGroupItem>` - A list of chapters and/or chapter groups.
- `tags: Vec<Tag>` - The tags associated with the group of chapters.
- `visuals: Vec<Visual>` - The visuals associated with the group of chapters.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## symphonia_core::meta::ChapterGroupItem

*Enum*

A chapter group item is either a chapter or chapter group.

**Variants:**
- `Group(ChapterGroup)` - The item is a chapter group.
- `Chapter(Chapter)` - The item is a chapter.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## symphonia_core::meta::ColorMode

*Enum*

Indicates how colors are represented in the image.

**Variants:**
- `Direct(ColorModel)` - Direct colour mode. Each pixel in the image stores the value of each color model primary.
- `Indexed(ColorPaletteInfo)` - Indexed colour mode. Each pixel in the image stores an index into a color map (the palette)

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## symphonia_core::meta::ColorModel

*Enum*

A color model describes how a color is represented.

**Variants:**
- `Y(std::num::NonZeroU8)` - Grayscale (1 channel: `Y`), of the indicated bit depth.
- `YA(std::num::NonZeroU8)` - Grayscale with alpha (2 channels: `Y`, `A`), of the indicated bit depth.
- `RGB(std::num::NonZeroU8)` - RGB (3 channels: `R`,`G`,`B`), of the indicated bit depth.
- `RGBA(std::num::NonZeroU8)` - RGBA (4 channels: `R`,`G`,`B`,`A`), of the indicated bit depth.
- `CMYK(std::num::NonZeroU8)` - CMYK (4 channels: `C`,`M`,`Y`,`K`), of the indicated bit depth.

**Methods:**

- `fn bits_per_pixel(self: &Self) -> u32` - Gets the bits/pixel.
- `fn has_alpha_channel(self: &Self) -> bool` - Returns if the color model contains an alpha channel.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## symphonia_core::meta::ColorPaletteInfo

*Struct*

A description of the color palette for indexed color mode.

**Fields:**
- `bits_per_pixel: std::num::NonZeroU8` - The number of bits per pixel used to index the palette.
- `color_model: ColorModel` - The color model of the entries in the palette.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## symphonia_core::meta::ContentAdvisory

*Enum*

A content advisory.

**Variants:**
- `None` - The content is not explicit.
- `Explicit` - The content is explicit.
- `Censored` - The content has been cleaned/censored.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## symphonia_core::meta::METADATA_ID_NULL

*Constant*: `MetadataId`

Null metadata format



## symphonia_core::meta::Metadata

*Struct*

A reference to the metadata inside of a [`MetadataLog`].

**Generic Parameters:**
- 'a

**Methods:**

- `fn is_latest(self: &Self) -> bool` - Returns `true` if the current metadata revision is the newest, `false` otherwise.
- `fn current(self: &Self) -> Option<&MetadataRevision>` - Gets an immutable reference to the current, and therefore oldest, revision of the metadata.
- `fn skip_to_latest(self: & mut Self) -> Option<&MetadataRevision>` - Skips to, and gets an immutable reference to the latest, and therefore newest, revision of
- `fn pop(self: & mut Self) -> Option<MetadataRevision>` - If there are newer `Metadata` revisions, advances the `MetadataLog` by discarding the

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## symphonia_core::meta::MetadataBuffer

*Struct*

The decoded contents of read metadata.

**Fields:**
- `revision: MetadataRevision` - The revision of metadata containing tags, visuals, and vendor-specific metadata buffers.
- `side_data: Vec<MetadataSideData>` - Additional pieces of data stored in the metadata, but not part of a metadata revision. These



## symphonia_core::meta::MetadataBuilder

*Struct*

A builder for [`MetadataRevision`].

**Methods:**

- `fn new(info: MetadataInfo) -> Self` - Instantiate a new `MetadataBuilder`.
- `fn add_tag(self: & mut Self, tag: Tag) -> & mut Self` - Add a media-level `Tag` to the metadata.
- `fn add_visual(self: & mut Self, visual: Visual) -> & mut Self` - Add a media-level `Visual` to the metadata.
- `fn add_track(self: & mut Self, per_track: PerTrackMetadata) -> & mut Self` - Add track-specific metadata.
- `fn build(self: Self) -> MetadataRevision` - Build and return the metadata revision.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## symphonia_core::meta::MetadataContainer

*Struct*

A container of metadata tags and pictures.

**Fields:**
- `tags: Vec<Tag>` - Key-value pairs of metadata.
- `visuals: Vec<Visual>` - Attached pictures.

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## symphonia_core::meta::MetadataId

*Struct*

A `MetadataId` is a unique identifier used to identify a specific metadata format.

**Tuple Struct**: `()`

**Methods:**

- `fn new(cc: FourCc) -> MetadataId` - Create a new metadata ID from a FourCC.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **From**
  - `fn from(value: FourCc) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## symphonia_core::meta::MetadataInfo

*Struct*

Basic information about a metadata format.

**Fields:**
- `metadata: MetadataId` - The `MetadataType` identifier.
- `short_name: &'static str` - A short ASCII-only string identifying the format.
- `long_name: &'static str` - A longer, more descriptive, string identifying the format.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## symphonia_core::meta::MetadataLog

*Struct*

A queue for time-ordered [`MetadataRevision`]s.

**Methods:**

- `fn metadata(self: & mut Self) -> Metadata` - Returns a reference to the metadata revisions inside the log.
- `fn push(self: & mut Self, rev: MetadataRevision)` - Push a new metadata revision to the end of the log.
- `fn append(self: & mut Self, other: & mut MetadataLog)` - Moves all metadata revisions from another metadata log to the end of this log.
- `fn push_front(self: & mut Self, rev: MetadataRevision)` - Push a metadata revision to the front of the log.
- `fn append_front(self: & mut Self, other: & mut MetadataLog)` - Moves all metadata revisions from another metadata log to the front of this log.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## symphonia_core::meta::MetadataOptions

*Struct*

A common set of options that all metadata readers use.

**Fields:**
- `limit_tag_bytes: crate::common::Limit` - The maximum size in bytes that a tag may occupy in memory once decoded. Tags exceeding this
- `limit_visual_bytes: crate::common::Limit` - The maximum size in bytes that a visual (picture) may occupy.

**Methods:**

- `fn limit_tag_bytes(self: Self, limit: Limit) -> Self` - The maximum size in bytes that a tag may occupy in memory once decoded. Tags exceeding this
- `fn limit_visual_bytes(self: Self, limit: Limit) -> Self` - The maximum size in bytes that a visual (picture) may occupy.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## symphonia_core::meta::MetadataReader

*Trait*

A `MetadataReader` reads and decodes metadata and produces a revision of that decoded metadata.

**Methods:**

- `metadata_info`: Get basic information about the metadata format.
- `read_all`: Read all metadata and return it if successful.
- `into_inner`: Consumes the `MetadataReader` and returns the underlying media source stream



## symphonia_core::meta::MetadataRevision

*Struct*

A metadata revision is a container for a single discrete version of media metadata.

A metadata revision contains what a user typically associates with metadata: tags, pictures,
etc., for the media as a whole and, optionally, for each contained track.

**Fields:**
- `info: MetadataInfo` - Information about the source of this revision of metadata.
- `media: MetadataContainer` - Media-level, global, metadata.
- `per_track: Vec<PerTrackMetadata>` - Per-track metadata.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## symphonia_core::meta::MetadataSideData

*Enum*

Enumeration of types of metadata side data.

**Variants:**
- `Chapters(ChapterGroup)` - Chapter information.



## symphonia_core::meta::PerTrackMetadata

*Struct*

Container for metadata associated with a specific track. A [`MetadataContainer`] wrapper
associating it with a track ID.

**Fields:**
- `track_id: u64` - The ID of the track the metadata is associated with.
- `metadata: MetadataContainer` - Track-specific metadata.

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## symphonia_core::meta::PerTrackMetadataBuilder

*Struct*

A builder for [`PerTrackMetadata`].

**Methods:**

- `fn new(track_id: u64) -> Self` - Instantiate a new `MetadataBuilder`.
- `fn add_tag(self: & mut Self, tag: Tag) -> & mut Self` - Add a `Tag` to the per-track metadata.
- `fn add_visual(self: & mut Self, visual: Visual) -> & mut Self` - Add a `Visual` to the per-track metadata.
- `fn build(self: Self) -> PerTrackMetadata` - Build and return the track-specific metadata.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## symphonia_core::meta::RawTag

*Struct*

A raw tag represents a tag in a data format that matches, as closely as possible, to the data
format that the tag was written in.

**Fields:**
- `key: String` - The name of the tag's key.
- `value: RawValue` - The value of the tag.
- `sub_fields: Option<Box<[RawTagSubField]>>` - The tag's sub-fields, if any.

**Methods:**

- `fn new<K, V>(key: K, value: V) -> Self` - Create a new raw tag from the provided key and value, with no sub-fields. Consumes the
- `fn new_with_sub_fields<K, V>(key: K, value: V, sub_fields: Box<[RawTagSubField]>) -> Self` - Create a new raw tag with sub-fields from the provided key, value, and sub-fields. Consumes

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## symphonia_core::meta::RawTagSubField

*Struct*

A key-value pair of supplementary data that can be attached to a raw tag.

**Fields:**
- `field: String` - The name of the sub-field.
- `value: RawValue` - The value of the sub-field.

**Methods:**

- `fn new<F, V>(field: F, value: V) -> Self` - Create a new sub-field from the provided field and value. Consumes the inputs.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## symphonia_core::meta::RawValue

*Enum*

The value of a [`RawTag`].

Note: The data types in this enumeration are an abstraction. Depending on the particular tagging
format, the actual data type of a specific tag may have a lesser width or different encoding
than the data type stored here.

**Variants:**
- `Binary(std::sync::Arc<Box<[u8]>>)` - A binary buffer.
- `Boolean(bool)` - A boolean value.
- `Flag` - A flag or indicator. A flag carries no data, but the presence of the tag has an implicit
- `Float(f64)` - A floating point number.
- `SignedInt(i64)` - A signed integer.
- `String(std::sync::Arc<String>)` - A string.
- `StringList(std::sync::Arc<Vec<String>>)` - A list of strings.
- `UnsignedInt(u64)` - An unsigned integer.

**Trait Implementations:**

- **From**
  - `fn from(v: u16) -> Self`
- **From**
  - `fn from(v: bool) -> Self`
- **From**
  - `fn from(v: &str) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **From**
  - `fn from(v: i8) -> Self`
- **From**
  - `fn from(v: i64) -> Self`
- **From**
  - `fn from(v: Vec<String>) -> Self`
- **From**
  - `fn from(v: Box<[u8]>) -> Self`
- **From**
  - `fn from(v: u32) -> Self`
- **From**
  - `fn from(v: f32) -> Self`
- **From**
  - `fn from(v: String) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(v: i16) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **From**
  - `fn from(v: u8) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **From**
  - `fn from(v: Arc<Box<[u8]>>) -> Self`
- **From**
  - `fn from(v: u64) -> Self`
- **From**
  - `fn from(v: f64) -> Self`
- **From**
  - `fn from(v: Arc<String>) -> Self`
- **From**
  - `fn from(v: i32) -> Self`
- **From**
  - `fn from(v: Cow<str>) -> Self`
- **From**
  - `fn from(v: &[u8]) -> Self`



## symphonia_core::meta::Size

*Struct*

A 2-dimensional (width and height) size type.

**Fields:**
- `width: u32` - The width in pixels.
- `height: u32` - The height in pixels.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`



## symphonia_core::meta::StandardTag

*Enum*

A standard tag is an enumeration of well-defined and well-known tags with parsed values.

**Variants:**
- `AccurateRipCount(std::sync::Arc<String>)`
- `AccurateRipCountAllOffsets(std::sync::Arc<String>)`
- `AccurateRipCountWithOffset(std::sync::Arc<String>)`
- `AccurateRipCrc(std::sync::Arc<String>)`
- `AccurateRipDiscId(std::sync::Arc<String>)`
- `AccurateRipId(std::sync::Arc<String>)`
- `AccurateRipOffset(std::sync::Arc<String>)`
- `AccurateRipResult(std::sync::Arc<String>)`
- `AccurateRipTotal(std::sync::Arc<String>)`
- `AcoustIdFingerprint(std::sync::Arc<String>)`
- `AcoustIdId(std::sync::Arc<String>)`
- `Actor(std::sync::Arc<String>)`
- `Album(std::sync::Arc<String>)`
- `AlbumArtist(std::sync::Arc<String>)`
- `Arranger(std::sync::Arc<String>)`
- `ArtDirector(std::sync::Arc<String>)`
- `Artist(std::sync::Arc<String>)`
- `AssistantDirector(std::sync::Arc<String>)`
- `Author(std::sync::Arc<String>)`
- `Bpm(u64)`
- `CdToc(std::sync::Arc<String>)`
- `CdTrackIndex(u8)`
- `ChapterTitle(std::sync::Arc<String>)`
- `Choregrapher(std::sync::Arc<String>)`
- `Cinematographer(std::sync::Arc<String>)`
- `CollectionTitle(std::sync::Arc<String>)`
- `Comment(std::sync::Arc<String>)`
- `CompilationFlag(bool)`
- `Composer(std::sync::Arc<String>)`
- `Conductor(std::sync::Arc<String>)`
- `ContentAdvisory(ContentAdvisory)`
- `ContentRating(std::sync::Arc<String>)`
- `ContentType(std::sync::Arc<String>)`
- `Coproducer(std::sync::Arc<String>)`
- `Copyright(std::sync::Arc<String>)`
- `CostumeDesigner(std::sync::Arc<String>)`
- `CueToolsDbDiscConfidence(std::sync::Arc<String>)`
- `CueToolsDbTrackConfidence(std::sync::Arc<String>)`
- `Description(std::sync::Arc<String>)`
- `DigitizedDate(std::sync::Arc<String>)`
- `Director(std::sync::Arc<String>)`
- `DiscNumber(u64)`
- `DiscSubtitle(std::sync::Arc<String>)`
- `DiscTotal(u64)`
- `Distributor(std::sync::Arc<String>)`
- `EditedBy(std::sync::Arc<String>)`
- `EditionTitle(std::sync::Arc<String>)`
- `EncodedBy(std::sync::Arc<String>)`
- `Encoder(std::sync::Arc<String>)`
- `EncoderSettings(std::sync::Arc<String>)`
- `EncodingDate(std::sync::Arc<String>)`
- `Engineer(std::sync::Arc<String>)`
- `Ensemble(std::sync::Arc<String>)`
- `ExecutiveProducer(std::sync::Arc<String>)`
- `Genre(std::sync::Arc<String>)`
- `Grouping(std::sync::Arc<String>)`
- `IdentAsin(std::sync::Arc<String>)`
- `IdentBarcode(std::sync::Arc<String>)`
- `IdentCatalogNumber(std::sync::Arc<String>)`
- `IdentEanUpn(std::sync::Arc<String>)`
- `IdentIsbn(std::sync::Arc<String>)`
- `IdentIsrc(std::sync::Arc<String>)`
- `IdentLccn(std::sync::Arc<String>)`
- `IdentPn(std::sync::Arc<String>)`
- `IdentPodcast(std::sync::Arc<String>)`
- `IdentUpc(std::sync::Arc<String>)`
- `ImdbTitleId(std::sync::Arc<String>)`
- `InitialKey(std::sync::Arc<String>)`
- `InternetRadioName(std::sync::Arc<String>)`
- `InternetRadioOwner(std::sync::Arc<String>)`
- `Keywords(std::sync::Arc<String>)`
- `Label(std::sync::Arc<String>)`
- `LabelCode(std::sync::Arc<String>)`
- `Language(std::sync::Arc<String>)`
- `License(std::sync::Arc<String>)`
- `Lyricist(std::sync::Arc<String>)`
- `Lyrics(std::sync::Arc<String>)`
- `Measure(std::sync::Arc<String>)`
- `MediaFormat(std::sync::Arc<String>)`
- `MixDj(std::sync::Arc<String>)`
- `MixEngineer(std::sync::Arc<String>)`
- `Mood(std::sync::Arc<String>)`
- `MovementName(std::sync::Arc<String>)`
- `MovementNumber(u64)`
- `MovementTotal(u64)`
- `MovieTitle(std::sync::Arc<String>)`
- `Mp3GainAlbumMinMax(std::sync::Arc<String>)`
- `Mp3GainMinMax(std::sync::Arc<String>)`
- `Mp3GainUndo(std::sync::Arc<String>)`
- `MusicBrainzAlbumArtistId(std::sync::Arc<String>)`
- `MusicBrainzAlbumId(std::sync::Arc<String>)`
- `MusicBrainzArtistId(std::sync::Arc<String>)`
- `MusicBrainzDiscId(std::sync::Arc<String>)`
- `MusicBrainzGenreId(std::sync::Arc<String>)`
- `MusicBrainzLabelId(std::sync::Arc<String>)`
- `MusicBrainzOriginalAlbumId(std::sync::Arc<String>)`
- `MusicBrainzOriginalArtistId(std::sync::Arc<String>)`
- `MusicBrainzRecordingId(std::sync::Arc<String>)`
- `MusicBrainzReleaseGroupId(std::sync::Arc<String>)`
- `MusicBrainzReleaseStatus(std::sync::Arc<String>)`
- `MusicBrainzReleaseTrackId(std::sync::Arc<String>)`
- `MusicBrainzReleaseType(std::sync::Arc<String>)`
- `MusicBrainzTrackId(std::sync::Arc<String>)`
- `MusicBrainzTrmId(std::sync::Arc<String>)`
- `MusicBrainzWorkId(std::sync::Arc<String>)`
- `Narrator(std::sync::Arc<String>)`
- `Opus(std::sync::Arc<String>)`
- `OpusNumber(u64)`
- `OriginalAlbum(std::sync::Arc<String>)`
- `OriginalArtist(std::sync::Arc<String>)`
- `OriginalFile(std::sync::Arc<String>)`
- `OriginalLyricist(std::sync::Arc<String>)`
- `OriginalRecordingDate(std::sync::Arc<String>)`
- `OriginalRecordingTime(std::sync::Arc<String>)`
- `OriginalRecordingYear(u16)`
- `OriginalReleaseDate(std::sync::Arc<String>)`
- `OriginalReleaseTime(std::sync::Arc<String>)`
- `OriginalReleaseYear(u16)`
- `OriginalWriter(std::sync::Arc<String>)`
- `Owner(std::sync::Arc<String>)`
- `Part(std::sync::Arc<String>)`
- `PartNumber(u64)`
- `PartTitle(std::sync::Arc<String>)`
- `PartTotal(u64)`
- `Performer(std::sync::Arc<String>)`
- `Period(std::sync::Arc<String>)`
- `PlayCounter(u64)`
- `PodcastCategory(std::sync::Arc<String>)`
- `PodcastDescription(std::sync::Arc<String>)`
- `PodcastFlag(bool)`
- `PodcastKeywords(std::sync::Arc<String>)`
- `Producer(std::sync::Arc<String>)`
- `ProductionCopyright(std::sync::Arc<String>)`
- `ProductionDesigner(std::sync::Arc<String>)`
- `ProductionStudio(std::sync::Arc<String>)`
- `PurchaseDate(std::sync::Arc<String>)`
- `Rating(u32)`
- `RecordingDate(std::sync::Arc<String>)`
- `RecordingLocation(std::sync::Arc<String>)`
- `RecordingTime(std::sync::Arc<String>)`
- `RecordingYear(u16)`
- `ReleaseCountry(std::sync::Arc<String>)`
- `ReleaseDate(std::sync::Arc<String>)`
- `ReleaseTime(std::sync::Arc<String>)`
- `ReleaseYear(u16)`
- `Remixer(std::sync::Arc<String>)`
- `ReplayGainAlbumGain(std::sync::Arc<String>)`
- `ReplayGainAlbumPeak(std::sync::Arc<String>)`
- `ReplayGainAlbumRange(std::sync::Arc<String>)`
- `ReplayGainReferenceLoudness(std::sync::Arc<String>)`
- `ReplayGainTrackGain(std::sync::Arc<String>)`
- `ReplayGainTrackPeak(std::sync::Arc<String>)`
- `ReplayGainTrackRange(std::sync::Arc<String>)`
- `ScreenplayAuthor(std::sync::Arc<String>)`
- `Script(std::sync::Arc<String>)`
- `Soloist(std::sync::Arc<String>)`
- `SortAlbum(std::sync::Arc<String>)`
- `SortAlbumArtist(std::sync::Arc<String>)`
- `SortArtist(std::sync::Arc<String>)`
- `SortCollectionTitle(std::sync::Arc<String>)`
- `SortComposer(std::sync::Arc<String>)`
- `SortEditionTitle(std::sync::Arc<String>)`
- `SortMovieTitle(std::sync::Arc<String>)`
- `SortOpusTitle(std::sync::Arc<String>)`
- `SortPartTitle(std::sync::Arc<String>)`
- `SortTrackTitle(std::sync::Arc<String>)`
- `SortTvEpisodeTitle(std::sync::Arc<String>)`
- `SortTvSeasonTitle(std::sync::Arc<String>)`
- `SortTvSeriesTitle(std::sync::Arc<String>)`
- `SortVolumeTitle(std::sync::Arc<String>)`
- `Subject(std::sync::Arc<String>)`
- `Summary(std::sync::Arc<String>)`
- `Synopsis(std::sync::Arc<String>)`
- `TaggingDate(std::sync::Arc<String>)`
- `TermsOfUse(std::sync::Arc<String>)`
- `Thanks(std::sync::Arc<String>)`
- `TmdbMovieId(std::sync::Arc<String>)`
- `TmdbSeriesId(std::sync::Arc<String>)`
- `TrackNumber(u64)`
- `TrackSubtitle(std::sync::Arc<String>)`
- `TrackTitle(std::sync::Arc<String>)`
- `TrackTotal(u64)`
- `Tuning(std::sync::Arc<String>)`
- `TvdbEpisodeId(std::sync::Arc<String>)`
- `TvdbMovieId(std::sync::Arc<String>)`
- `TvdbSeriesId(std::sync::Arc<String>)`
- `TvEpisodeNumber(u64)`
- `TvEpisodeTitle(std::sync::Arc<String>)`
- `TvEpisodeTotal(u64)`
- `TvNetwork(std::sync::Arc<String>)`
- `TvSeasonNumber(u64)`
- `TvSeasonTitle(std::sync::Arc<String>)`
- `TvSeasonTotal(u64)`
- `TvSeriesTitle(std::sync::Arc<String>)`
- `Url(std::sync::Arc<String>)`
- `UrlArtist(std::sync::Arc<String>)`
- `UrlCopyright(std::sync::Arc<String>)`
- `UrlInternetRadio(std::sync::Arc<String>)`
- `UrlLabel(std::sync::Arc<String>)`
- `UrlOfficial(std::sync::Arc<String>)`
- `UrlPayment(std::sync::Arc<String>)`
- `UrlPodcast(std::sync::Arc<String>)`
- `UrlPurchase(std::sync::Arc<String>)`
- `UrlSource(std::sync::Arc<String>)`
- `Version(std::sync::Arc<String>)`
- `VolumeNumber(u64)`
- `VolumeTitle(std::sync::Arc<String>)`
- `VolumeTotal(u64)`
- `Work(std::sync::Arc<String>)`
- `Writer(std::sync::Arc<String>)`
- `WrittenDate(std::sync::Arc<String>)`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## symphonia_core::meta::StandardVisualKey

*Enum*

`StandardVisualKey` is an enumeration providing standardized keys for common visual dispositions.
A demuxer may assign a `StandardVisualKey` to a `Visual` if the disposition of the attached
visual is known and can be mapped to a standard key.

The visual types listed here are derived from, though do not entirely cover, the ID3v2 APIC
frame specification.

**Variants:**
- `FileIcon`
- `OtherIcon`
- `FrontCover`
- `BackCover`
- `Leaflet`
- `Media`
- `LeadArtistPerformerSoloist`
- `ArtistPerformer`
- `Conductor`
- `BandOrchestra`
- `Composer`
- `Lyricist`
- `RecordingLocation`
- `RecordingSession`
- `Performance`
- `ScreenCapture`
- `Illustration`
- `BandArtistLogo`
- `PublisherStudioLogo`
- `Other`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## symphonia_core::meta::Tag

*Struct*

A tag encapsulates a single piece of metadata.

**Fields:**
- `raw: RawTag` - The raw tag.
- `std: Option<StandardTag>` - An optional standard tag.

**Methods:**

- `fn new(raw: RawTag) -> Self` - Create a new tag from a raw tag. Consumes the inputs.
- `fn new_std(raw: RawTag, std: StandardTag) -> Self` - Create a new tag from a raw tag with a standard tag. Consumes the inputs.
- `fn new_from_parts<K, V>(key: K, value: V, std: Option<StandardTag>) -> Self` - Create a new tag from its constituent parts: a key, value, and optional standard tag.
- `fn has_std_tag(self: &Self) -> bool` - Returns `true` if the tag was recognized as a well-known tag and has a standard tag

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## symphonia_core::meta::Visual

*Struct*

A `Visual` is any 2 dimensional graphic.

**Fields:**
- `media_type: Option<String>` - The Media Type (MIME Type) used to encode the `Visual`.
- `dimensions: Option<Size>` - The dimensions of the `Visual`.
- `color_mode: Option<ColorMode>` - The color mode of the `Visual`.
- `usage: Option<StandardVisualKey>` - The usage and/or content of the `Visual`.
- `tags: Vec<Tag>` - Any tags associated with the `Visual`.
- `data: Box<[u8]>` - The data of the `Visual`, encoded as per `media_type`.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## Module: well_known

IDs for well-known metadata formats.



