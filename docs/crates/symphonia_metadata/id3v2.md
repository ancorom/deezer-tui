**symphonia_metadata > id3v2**

# Module: id3v2

## Contents

**Modules**

- [`sub_fields`](#sub_fields) - Key name constants for sub-fields of well-known ID3v2 frames.

**Structs**

- [`Id3v2Reader`](#id3v2reader) - ID3v2 tag reader.

---

## symphonia_metadata::id3v2::Id3v2Reader

*Struct*

ID3v2 tag reader.

**Generic Parameters:**
- 's

**Methods:**

- `fn try_new(mss: MediaSourceStream<'s>, _opts: MetadataOptions) -> Result<Self>`

**Trait Implementations:**

- **ProbeableMetadata**
  - `fn try_probe_new(mss: MediaSourceStream, opts: MetadataOptions) -> Result<Box<dyn MetadataReader>>`
  - `fn probe_data() -> &'static [ProbeMetadataData]`
- **Scoreable**
  - `fn score(_src: ScopedStream<& mut MediaSourceStream>) -> Result<Score>`
- **MetadataReader**
  - `fn metadata_info(self: &Self) -> &MetadataInfo`
  - `fn read_all(self: & mut Self) -> Result<MetadataBuffer>`
  - `fn into_inner<'s>(self: Box<Self>) -> MediaSourceStream<'s>`



## Module: sub_fields

Key name constants for sub-fields of well-known ID3v2 frames.

For the exact meaning of these fields, and the format of their values, please consult the
official ID3v2 specification.



