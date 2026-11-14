**symphonia_core > formats > probe**

# Module: formats::probe

## Contents

**Structs**

- [`Hint`](#hint) - A `Hint` provides additional information and context when probing a media source stream.
- [`Probe`](#probe) - `Probe` scans a `MediaSourceStream` for metadata and container formats, and provides an
- [`ProbeDataMatchSpec`](#probedatamatchspec) - The probe match specification provides declarative information that is used by a `Probe` to
- [`ProbeFormatData`](#probeformatdata) - Container format-specific probe data.
- [`ProbeMetadataData`](#probemetadatadata) - Metadata format-specific probe data.
- [`ProbeOptions`](#probeoptions) - Options for controlling the behaviour of a `Probe`.

**Enums**

- [`Anchors`](#anchors) - Anchors are explicit fixed offsets from the end of a media stream that should be searched for
- [`Score`](#score) - The result of a scoring operation.

**Traits**

- [`ProbeableFormat`](#probeableformat) - To support probing, a `FormatReader` must implement the `ProbeableFormat` trait.
- [`ProbeableMetadata`](#probeablemetadata) - To support probing, a `MetadataReader` must implement the `ProbeableMetadata` trait.
- [`Scoreable`](#scoreable) - The `Scoreable` trait defines the scoring functionality a reader must implement to support

**Type Aliases**

- [`FormatFactoryFn`](#formatfactoryfn) - `FormatReader` probe factory function. Creates a boxed `FormatReader`.
- [`MetadataFactoryFn`](#metadatafactoryfn) - `MetadataReader` probe factory function. Creates a boxed `MetadataReader`.

---

## symphonia_core::formats::probe::Anchors

*Enum*

Anchors are explicit fixed offsets from the end of a media stream that should be searched for
format markers.

**Variants:**
- `None` - No anchors.
- `Exclusive(&'static [u32])` - Exclusive anchors indicate that the format markers will only searched for at the anchor
- `Supplemental(&'static [u32])` - Supplemental anchors indicate that the format markers will first be scanned for at the

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`



## symphonia_core::formats::probe::FormatFactoryFn

*Type Alias*: `fn(...)`

`FormatReader` probe factory function. Creates a boxed `FormatReader`.



## symphonia_core::formats::probe::Hint

*Struct*

A `Hint` provides additional information and context when probing a media source stream.

For example, the `Probe` cannot examine the extension or mime-type of the media because
`MediaSourceStream` abstracts away such details. However, the embedder may have this information
from a file path, HTTP header, email attachment metadata, etc. `Hint`s are optional, and won't
lead the probe astray if they're wrong, but they may provide an informed initial guess and
optimize the guessing process siginificantly especially as more formats are registered.

**Methods:**

- `fn new() -> Self` - Instantiate an empty `Hint`.
- `fn with_extension(self: & mut Self, extension: &str) -> & mut Self` - Add a file extension hint.
- `fn mime_type(self: & mut Self, mime_type: &str) -> & mut Self` - Add a MIME/Media-type hint.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## symphonia_core::formats::probe::MetadataFactoryFn

*Type Alias*: `fn(...)`

`MetadataReader` probe factory function. Creates a boxed `MetadataReader`.



## symphonia_core::formats::probe::Probe

*Struct*

`Probe` scans a `MediaSourceStream` for metadata and container formats, and provides an
iterator-like interface to instantiate readers for the formats encountered.

**Methods:**

- `fn new() -> Self` - Instantiate a probe with default options.
- `fn new_with_options(opts: &ProbeOptions) -> Self` - Instantiate a probe with custom options.
- `fn register_format<P>(self: & mut Self)` - Register the parameterized format reader at the standard tier.
- `fn register_metadata<P>(self: & mut Self)` - Register the parameterized metadata reader at the standard tier.
- `fn register_format_at_tier<P>(self: & mut Self, tier: Tier)` - Register the parameterized format reader at a specific tier.
- `fn register_metadata_at_tier<P>(self: & mut Self, tier: Tier)` - Register the parameterized metadata reader at a specific tier.
- `fn probe<'s>(self: &Self, hint: &Hint, mss: MediaSourceStream<'s>, fmt_opts: FormatOptions, meta_opts: MetadataOptions) -> Result<Box<dyn FormatReader>>` - Searches the provided `MediaSourceStream` for a container format. Any metadata that is read

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`



## symphonia_core::formats::probe::ProbeDataMatchSpec

*Struct*

The probe match specification provides declarative information that is used by a `Probe` to
detect the presence of a specific container or metadata format while scanning a
`MediaSourceStream`.

**Fields:**
- `extensions: &'static [&'static str]` - A list of case-insensitive file extensions that are generally used by the format.
- `mime_types: &'static [&'static str]` - A list of case-insensitive MIME types that are generally used by the format.
- `markers: &'static [&'static [u8]]` - A byte-string start-of-format marker that will be searched for within the stream. Typically

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`



## symphonia_core::formats::probe::ProbeFormatData

*Struct*

Container format-specific probe data.

**Fields:**
- `spec: ProbeDataMatchSpec` - The match specification used by the probe to match against the media source stream.
- `info: crate::formats::FormatInfo` - A description of the container format and reader if a match with the basic probe data is

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`



## symphonia_core::formats::probe::ProbeMetadataData

*Struct*

Metadata format-specific probe data.

**Fields:**
- `spec: ProbeDataMatchSpec` - The match specification used by the probe to match against the media source stream.
- `info: crate::meta::MetadataInfo` - A description of the metadata format and reader if a match with the basic probe data is
- `anchors: Anchors` - Anchors to search.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`



## symphonia_core::formats::probe::ProbeOptions

*Struct*

Options for controlling the behaviour of a `Probe`.

**Fields:**
- `max_probe_depth: u32` - The maximum number of bytes that will be scanned from the media source before giving up.
- `max_score_depth: u16` - The maximum number of bytes a score operation may read before it will be forced to abort.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## symphonia_core::formats::probe::ProbeableFormat

*Trait*

To support probing, a `FormatReader` must implement the `ProbeableFormat` trait.

**Methods:**

- `try_probe_new`: Create an instance of the format reader.
- `probe_data`: Returns a list of probe data that a [`Probe`] will use to determine if the reader



## symphonia_core::formats::probe::ProbeableMetadata

*Trait*

To support probing, a `MetadataReader` must implement the `ProbeableMetadata` trait.

**Methods:**

- `try_probe_new`: Create an instance of the metadata reader.
- `probe_data`: Returns a list of probe data that a [`Probe`] will use to determine if the reader



## symphonia_core::formats::probe::Score

*Enum*

The result of a scoring operation.

**Variants:**
- `Unsupported` - The format is not supported.
- `Supported(u8)` - The format is supported with a confidence between 0 (not confident) and 255 (very



## symphonia_core::formats::probe::Scoreable

*Trait*

The `Scoreable` trait defines the scoring functionality a reader must implement to support
probing for a container or metadata format.

**Methods:**

- `score`: Using scoped access to a `MediaSourceStream`, calculate and return a value between 0 and 255



