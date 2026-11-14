**symphonia_metadata > id3v1**

# Module: id3v1

## Contents

**Structs**

- [`Id3v1Reader`](#id3v1reader) - ID3v1 tag reader.

---

## symphonia_metadata::id3v1::Id3v1Reader

*Struct*

ID3v1 tag reader.

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



