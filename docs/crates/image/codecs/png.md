**image > codecs > png**

# Module: codecs::png

## Contents

**Structs**

- [`ApngDecoder`](#apngdecoder) - An [`AnimationDecoder`] adapter of [`PngDecoder`].
- [`PngDecoder`](#pngdecoder) - PNG decoder
- [`PngEncoder`](#pngencoder) - PNG encoder

**Enums**

- [`CompressionType`](#compressiontype) - DEFLATE compression level of a PNG encoder. The default setting is `Fast`.
- [`FilterType`](#filtertype) - Filter algorithms used to process image data to improve compression.

---

## image::codecs::png::ApngDecoder

*Struct*

An [`AnimationDecoder`] adapter of [`PngDecoder`].

See [`PngDecoder::apng`] for more information.

[`AnimationDecoder`]: ../trait.AnimationDecoder.html
[`PngDecoder`]: struct.PngDecoder.html
[`PngDecoder::apng`]: struct.PngDecoder.html#method.apng

**Generic Parameters:**
- R

**Trait Implementations:**

- **AnimationDecoder**
  - `fn loop_count(self: &Self) -> LoopCount`
  - `fn into_frames(self: Self) -> Frames<'a>`



## image::codecs::png::CompressionType

*Enum*

DEFLATE compression level of a PNG encoder. The default setting is `Fast`.

**Variants:**
- `Default` - Default compression level
- `Fast` - Fast, minimal compression
- `Best` - High compression level
- `Uncompressed` - No compression whatsoever
- `Level(u8)` - Detailed compression level between 1 and 9

**Traits:** Copy, Eq

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## image::codecs::png::FilterType

*Enum*

Filter algorithms used to process image data to improve compression.

The default filter is `Adaptive`.

**Variants:**
- `NoFilter` - No processing done, best used for low bit depth grayscale or data with a
- `Sub` - Filters based on previous pixel in the same scanline
- `Up` - Filters based on the scanline above
- `Avg` - Filters based on the average of left and right neighbor pixels
- `Paeth` - Algorithm that takes into account the left, upper left, and above pixels
- `Adaptive` - Uses a heuristic to select one of the preceding filters for each

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## image::codecs::png::PngDecoder

*Struct*

PNG decoder

**Generic Parameters:**
- R

**Methods:**

- `fn new(r: R) -> ImageResult<PngDecoder<R>>` - Creates a new decoder that decodes from the stream ```r```
- `fn with_limits(r: R, limits: Limits) -> ImageResult<PngDecoder<R>>` - Creates a new decoder that decodes from the stream ```r``` with the given limits.
- `fn gamma_value(self: &Self) -> ImageResult<Option<f64>>` - Returns the gamma value of the image or None if no gamma value is indicated.
- `fn apng(self: Self) -> ImageResult<ApngDecoder<R>>` - Turn this into an iterator over the animation frames.
- `fn is_apng(self: &Self) -> ImageResult<bool>` - Returns if the image contains an animation.

**Trait Implementations:**

- **ImageDecoder**
  - `fn dimensions(self: &Self) -> (u32, u32)`
  - `fn color_type(self: &Self) -> ColorType`
  - `fn icc_profile(self: & mut Self) -> ImageResult<Option<Vec<u8>>>`
  - `fn exif_metadata(self: & mut Self) -> ImageResult<Option<Vec<u8>>>`
  - `fn xmp_metadata(self: & mut Self) -> ImageResult<Option<Vec<u8>>>`
  - `fn iptc_metadata(self: & mut Self) -> ImageResult<Option<Vec<u8>>>`
  - `fn read_image(self: Self, buf: & mut [u8]) -> ImageResult<()>`
  - `fn read_image_boxed(self: Box<Self>, buf: & mut [u8]) -> ImageResult<()>`
  - `fn set_limits(self: & mut Self, limits: Limits) -> ImageResult<()>`



## image::codecs::png::PngEncoder

*Struct*

PNG encoder

**Generic Parameters:**
- W

**Methods:**

- `fn new(w: W) -> PngEncoder<W>` - Create a new encoder that writes its output to ```w```
- `fn new_with_quality(w: W, compression: CompressionType, filter: FilterType) -> PngEncoder<W>` - Create a new encoder that writes its output to `w` with `CompressionType` `compression` and

**Trait Implementations:**

- **ImageEncoder**
  - `fn write_image(self: Self, buf: &[u8], width: u32, height: u32, color_type: ExtendedColorType) -> ImageResult<()>` - Write a PNG image with the specified width, height, and color type.
  - `fn set_icc_profile(self: & mut Self, icc_profile: Vec<u8>) -> Result<(), UnsupportedError>`
  - `fn set_exif_metadata(self: & mut Self, exif: Vec<u8>) -> Result<(), UnsupportedError>`



