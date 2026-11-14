**symphonia_metadata > ape**

# Module: ape

## Contents

**Structs**

- [`ApeReader`](#apereader) - APEv1 and APEv2 tag reader.

---

## symphonia_metadata::ape::ApeReader

*Struct*

APEv1 and APEv2 tag reader.

**Generic Parameters:**
- 's

**Methods:**

- `fn try_new(mss: MediaSourceStream<'s>, _opts: MetadataOptions) -> Result<Self>`

**Trait Implementations:**

- **MetadataReader**
  - `fn metadata_info(self: &Self) -> &MetadataInfo`
  - `fn read_all(self: & mut Self) -> Result<MetadataBuffer>`
  - `fn into_inner<'s>(self: Box<Self>) -> MediaSourceStream<'s>`
- **Scoreable**
  - `fn score(_src: ScopedStream<& mut MediaSourceStream>) -> Result<Score>`
- **ProbeableMetadata**
  - `fn try_probe_new(mss: MediaSourceStream, opts: MetadataOptions) -> Result<Box<dyn MetadataReader>>`
  - `fn probe_data() -> &'static [ProbeMetadataData]`



