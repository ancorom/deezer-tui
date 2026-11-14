**symphonia_metadata > embedded > riff > info**

# Module: embedded::riff::info

## Contents

**Functions**

- [`parse_riff_info_chunk`](#parse_riff_info_chunk) - Parse a RIFF INFO chunk into a `Tag` using the chunk's identifier and a slice containing the

---

## symphonia_metadata::embedded::riff::info::parse_riff_info_chunk

*Function*

Parse a RIFF INFO chunk into a `Tag` using the chunk's identifier and a slice containing the
chunk's payload.

```rust
fn parse_riff_info_chunk(chunk_id: [u8; 4], buf: &[u8], builder: & mut symphonia_core::meta::MetadataBuilder) -> symphonia_core::errors::Result<()>
```



