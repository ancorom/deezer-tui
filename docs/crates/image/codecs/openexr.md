**image > codecs > openexr**

# Module: codecs::openexr

## Contents

**Structs**

- [`OpenExrDecoder`](#openexrdecoder) - An OpenEXR decoder. Immediately reads the meta data from the file.
- [`OpenExrEncoder`](#openexrencoder) - A thin wrapper that implements `ImageEncoder` for OpenEXR images. Will behave like `image::codecs::openexr::write_buffer`.

---

## image::codecs::openexr::OpenExrDecoder

*Struct*

An OpenEXR decoder. Immediately reads the meta data from the file.

**Generic Parameters:**
- R

**Methods:**

- `fn new(source: R) -> ImageResult<Self>` - Create a decoder. Consumes the first few bytes of the source to extract image dimensions.
- `fn with_alpha_preference(source: R, alpha_preference: Option<bool>) -> ImageResult<Self>` - Create a decoder. Consumes the first few bytes of the source to extract image dimensions.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **ImageDecoder**
  - `fn dimensions(self: &Self) -> (u32, u32)`
  - `fn color_type(self: &Self) -> ColorType`
  - `fn original_color_type(self: &Self) -> ExtendedColorType`
  - `fn read_image(self: Self, unaligned_bytes: & mut [u8]) -> ImageResult<()>`
  - `fn read_image_boxed(self: Box<Self>, buf: & mut [u8]) -> ImageResult<()>`



## image::codecs::openexr::OpenExrEncoder

*Struct*

A thin wrapper that implements `ImageEncoder` for OpenEXR images. Will behave like `image::codecs::openexr::write_buffer`.

**Generic Parameters:**
- W

**Tuple Struct**: `()`

**Methods:**

- `fn new(write: W) -> Self` - Create an `ImageEncoder`. Does not write anything yet. Writing later will behave like `image::codecs::openexr::write_buffer`.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **ImageEncoder**
  - `fn write_image(self: Self, buf: &[u8], width: u32, height: u32, color_type: ExtendedColorType) -> ImageResult<()>` - Writes the complete image.



