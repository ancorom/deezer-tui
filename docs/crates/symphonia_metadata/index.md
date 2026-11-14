# symphonia_metadata

This crate implements a collection of metadata readers for standalone metadata formats, and read
functions for metadata formats that are embedded into another container.

# Standalone Metadata Formats

A standalone metadata format is one that exists independent of the media container.

This crate implements metadata readers (an object implementing the
[`MetadataReader`](symphonia_core::meta::MetadataReader) trait) for these metadata formats.
A metadata reader may be registered with a probe for automatic detection. Each major standalone
metadata format reader is implemented in a separate module.

# Embedded Metadata Formats

An embedded metadata format is one that is embedded into the media container. This crate
implements reading or parsing functions for these metadata formats in the [`embedded`] module.

## Modules

### [`symphonia_metadata`](symphonia_metadata.md)

*5 modules*

### [`ape`](ape.md)

*1 struct*

### [`embedded`](embedded.md)

*3 modules*

### [`embedded::flac`](embedded/flac.md)

*1 constant, 5 functions*

### [`embedded::riff::info`](embedded/riff/info.md)

*1 function*

### [`embedded::vorbis`](embedded/vorbis.md)

*1 constant, 1 function*

### [`id3v1`](id3v1.md)

*1 struct*

### [`id3v2`](id3v2.md)

*1 module, 1 struct*

### [`id3v2::sub_fields`](id3v2/sub_fields.md)

*38 constants*

### [`utils`](utils.md)

*4 modules*

### [`utils::id3v1`](utils/id3v1.md)

*1 function*

### [`utils::id3v2`](utils/id3v2.md)

*1 function*

### [`utils::images`](utils/images.md)

*1 function, 1 struct*

### [`utils::itunes`](utils/itunes.md)

*1 function*

