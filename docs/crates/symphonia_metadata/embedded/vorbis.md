**symphonia_metadata > embedded > vorbis**

# Module: embedded::vorbis

## Contents

**Functions**

- [`read_vorbis_comment`](#read_vorbis_comment)

**Constants**

- [`VORBIS_COMMENT_METADATA_INFO`](#vorbis_comment_metadata_info)

---

## symphonia_metadata::embedded::vorbis::VORBIS_COMMENT_METADATA_INFO

*Constant*: `symphonia_core::meta::MetadataInfo`



## symphonia_metadata::embedded::vorbis::read_vorbis_comment

*Function*

```rust
fn read_vorbis_comment<B>(reader: & mut B, builder: & mut symphonia_core::meta::MetadataBuilder, side_data: & mut Vec<symphonia_core::meta::MetadataSideData>) -> symphonia_core::errors::Result<()>
```



