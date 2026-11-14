**symphonia_metadata > embedded > flac**

# Module: embedded::flac

## Contents

**Functions**

- [`read_flac_application_block`](#read_flac_application_block) - Read a vendor-specific application metadata block as vendor data.
- [`read_flac_comment_block`](#read_flac_comment_block) - Read a comment metadata block.
- [`read_flac_cuesheet_block`](#read_flac_cuesheet_block) - Read a cuesheet metadata block as a chapter group.
- [`read_flac_picture_block`](#read_flac_picture_block) - Read a picture metadata block.
- [`read_flac_seektable_block`](#read_flac_seektable_block) - Read a seek table metadata block as a seek index.

**Constants**

- [`FLAC_METADATA_INFO`](#flac_metadata_info)

---

## symphonia_metadata::embedded::flac::FLAC_METADATA_INFO

*Constant*: `symphonia_core::meta::MetadataInfo`



## symphonia_metadata::embedded::flac::read_flac_application_block

*Function*

Read a vendor-specific application metadata block as vendor data.

```rust
fn read_flac_application_block<B>(reader: & mut B, block_length: u32) -> symphonia_core::errors::Result<symphonia_core::formats::VendorDataAttachment>
```



## symphonia_metadata::embedded::flac::read_flac_comment_block

*Function*

Read a comment metadata block.

```rust
fn read_flac_comment_block<B>(reader: & mut B, builder: & mut symphonia_core::meta::MetadataBuilder) -> symphonia_core::errors::Result<()>
```



## symphonia_metadata::embedded::flac::read_flac_cuesheet_block

*Function*

Read a cuesheet metadata block as a chapter group.

```rust
fn read_flac_cuesheet_block<B>(reader: & mut B, tb: symphonia_core::units::TimeBase) -> symphonia_core::errors::Result<symphonia_core::meta::ChapterGroup>
```



## symphonia_metadata::embedded::flac::read_flac_picture_block

*Function*

Read a picture metadata block.

```rust
fn read_flac_picture_block<B>(reader: & mut B) -> symphonia_core::errors::Result<symphonia_core::meta::Visual>
```



## symphonia_metadata::embedded::flac::read_flac_seektable_block

*Function*

Read a seek table metadata block as a seek index.

```rust
fn read_flac_seektable_block<B>(reader: & mut B, block_length: u32) -> symphonia_core::errors::Result<symphonia_core::formats::util::SeekIndex>
```



