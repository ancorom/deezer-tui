**symphonia_core > subtitle**

# Module: subtitle

## Contents

**Structs**

- [`PlainTextSubtitleBuffer`](#plaintextsubtitlebuffer)
- [`RawTextSubtitleBuffer`](#rawtextsubtitlebuffer)
- [`RenderedSubtitleBuffer`](#renderedsubtitlebuffer)

**Enums**

- [`GenericSubtitleBufferRef`](#genericsubtitlebufferref)

---

## symphonia_core::subtitle::GenericSubtitleBufferRef

*Enum*

**Generic Parameters:**
- 'a

**Variants:**
- `RawText(&'a RawTextSubtitleBuffer)` - Raw, unparsed, subtitle text.
- `PlainText(&'a PlainTextSubtitleBuffer)` - Plain text subtitle text.
- `Rendered(&'a RenderedSubtitleBuffer)` - Rendered subtitle picture buffer.



## symphonia_core::subtitle::PlainTextSubtitleBuffer

*Struct*



## symphonia_core::subtitle::RawTextSubtitleBuffer

*Struct*



## symphonia_core::subtitle::RenderedSubtitleBuffer

*Struct*



