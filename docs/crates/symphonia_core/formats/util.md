**symphonia_core > formats > util**

# Module: formats::util

## Contents

**Structs**

- [`SeekIndex`](#seekindex) - A `SeekIndex` stores `SeekPoint`s (generally a sample or frame number to byte offset) within
- [`SeekPoint`](#seekpoint) - A `SeekPoint` is a mapping between a sample or frame number to byte offset within a media

**Enums**

- [`SeekSearchResult`](#seeksearchresult) - `SeekSearchResult` is the return value for a search on a `SeekIndex`. It returns a range of

---

## symphonia_core::formats::util::SeekIndex

*Struct*

A `SeekIndex` stores `SeekPoint`s (generally a sample or frame number to byte offset) within
a media stream and provides methods to efficiently search for the nearest `SeekPoint`(s)
given a timestamp.

A `SeekIndex` does not require complete coverage of the entire media stream. However, the
better the coverage, the smaller the manual search range the `SeekIndex` will return.

**Methods:**

- `fn new() -> SeekIndex` - Create an empty `SeekIndex`
- `fn insert(self: & mut Self, ts: Timestamp, byte_offset: u64, n_frames: u32)` - Insert a `SeekPoint` into the index.
- `fn search(self: &Self, frame_ts: Timestamp) -> SeekSearchResult` - Search the index to find a bounded range of bytes wherein the specified frame timestamp

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`



## symphonia_core::formats::util::SeekPoint

*Struct*

A `SeekPoint` is a mapping between a sample or frame number to byte offset within a media
stream.

**Fields:**
- `frame_ts: crate::units::Timestamp` - The frame or sample timestamp of the `SeekPoint`.
- `byte_offset: u64` - The byte offset of the `SeekPoint`s timestamp relative to a format-specific location.
- `n_frames: u32` - The number of frames the `SeekPoint` covers.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## symphonia_core::formats::util::SeekSearchResult

*Enum*

`SeekSearchResult` is the return value for a search on a `SeekIndex`. It returns a range of
`SeekPoint`s a `FormatReader` should search to find the desired timestamp. Ranges are
lower-bound inclusive, and upper-bound exclusive.

**Variants:**
- `Stream` - The `SeekIndex` is empty so the desired timestamp could not be found. The entire stream
- `Upper(SeekPoint)` - The desired timestamp can be found before, the `SeekPoint`. The stream should be
- `Lower(SeekPoint)` - The desired timestamp can be found at, or after, the `SeekPoint`. The stream should be
- `Range(SeekPoint, SeekPoint)` - The desired timestamp can be found within the range. The stream should be searched for

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



