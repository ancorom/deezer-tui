**image > codecs > webp > decoder**

# Module: codecs::webp::decoder

## Contents

**Structs**

- [`WebPDecoder`](#webpdecoder) - WebP Image format decoder.

---

## image::codecs::webp::decoder::WebPDecoder

*Struct*

WebP Image format decoder.

Supports both lossless and lossy WebP images.

**Generic Parameters:**
- R

**Methods:**

- `fn new(r: R) -> ImageResult<Self>` - Create a new `WebPDecoder` from the Reader `r`.
- `fn has_animation(self: &Self) -> bool` - Returns true if the image as described by the bitstream is animated.
- `fn set_background_color(self: & mut Self, color: Rgba<u8>) -> ImageResult<()>` - Sets the background color if the image is an extended and animated webp.

**Trait Implementations:**

- **ImageDecoder**
  - `fn dimensions(self: &Self) -> (u32, u32)`
  - `fn color_type(self: &Self) -> ColorType`
  - `fn read_image(self: Self, buf: & mut [u8]) -> ImageResult<()>`
  - `fn read_image_boxed(self: Box<Self>, buf: & mut [u8]) -> ImageResult<()>`
  - `fn icc_profile(self: & mut Self) -> ImageResult<Option<Vec<u8>>>`
  - `fn exif_metadata(self: & mut Self) -> ImageResult<Option<Vec<u8>>>`
  - `fn xmp_metadata(self: & mut Self) -> ImageResult<Option<Vec<u8>>>`
  - `fn orientation(self: & mut Self) -> ImageResult<Orientation>`
- **AnimationDecoder**
  - `fn loop_count(self: &Self) -> crate::metadata::LoopCount`
  - `fn into_frames(self: Self) -> Frames<'a>`



