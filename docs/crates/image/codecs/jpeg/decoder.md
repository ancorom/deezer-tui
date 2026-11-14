**image > codecs > jpeg > decoder**

# Module: codecs::jpeg::decoder

## Contents

**Structs**

- [`JpegDecoder`](#jpegdecoder) - JPEG decoder

---

## image::codecs::jpeg::decoder::JpegDecoder

*Struct*

JPEG decoder

**Generic Parameters:**
- R

**Methods:**

- `fn new(r: R) -> ImageResult<JpegDecoder<R>>` - Create a new decoder that decodes from the stream ```r```

**Trait Implementations:**

- **ImageDecoder**
  - `fn dimensions(self: &Self) -> (u32, u32)`
  - `fn color_type(self: &Self) -> ColorType`
  - `fn icc_profile(self: & mut Self) -> ImageResult<Option<Vec<u8>>>`
  - `fn exif_metadata(self: & mut Self) -> ImageResult<Option<Vec<u8>>>`
  - `fn xmp_metadata(self: & mut Self) -> ImageResult<Option<Vec<u8>>>`
  - `fn iptc_metadata(self: & mut Self) -> ImageResult<Option<Vec<u8>>>`
  - `fn orientation(self: & mut Self) -> ImageResult<Orientation>`
  - `fn read_image(self: Self, buf: & mut [u8]) -> ImageResult<()>`
  - `fn set_limits(self: & mut Self, limits: Limits) -> ImageResult<()>`
  - `fn read_image_boxed(self: Box<Self>, buf: & mut [u8]) -> ImageResult<()>`



