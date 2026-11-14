**image > codecs > tiff**

# Module: codecs::tiff

## Contents

**Structs**

- [`TiffDecoder`](#tiffdecoder) - Decoder for TIFF images.
- [`TiffEncoder`](#tiffencoder) - Encoder for tiff images
- [`TiffReader`](#tiffreader) - Wrapper struct around a `Cursor<Vec<u8>>`

---

## image::codecs::tiff::TiffDecoder

*Struct*

Decoder for TIFF images.

**Generic Parameters:**
- R

**Methods:**

- `fn new(r: R) -> Result<TiffDecoder<R>, ImageError>` - Create a new `TiffDecoder`.

**Trait Implementations:**

- **ImageDecoder**
  - `fn dimensions(self: &Self) -> (u32, u32)`
  - `fn color_type(self: &Self) -> ColorType`
  - `fn original_color_type(self: &Self) -> ExtendedColorType`
  - `fn icc_profile(self: & mut Self) -> ImageResult<Option<Vec<u8>>>`
  - `fn xmp_metadata(self: & mut Self) -> ImageResult<Option<Vec<u8>>>`
  - `fn orientation(self: & mut Self) -> ImageResult<Orientation>`
  - `fn set_limits(self: & mut Self, limits: crate::Limits) -> ImageResult<()>`
  - `fn read_image(self: Self, buf: & mut [u8]) -> ImageResult<()>`
  - `fn read_image_boxed(self: Box<Self>, buf: & mut [u8]) -> ImageResult<()>`



## image::codecs::tiff::TiffEncoder

*Struct*

Encoder for tiff images

**Generic Parameters:**
- W

**Methods:**

- `fn new(w: W) -> TiffEncoder<W>` - Create a new encoder that writes its output to `w`
- `fn encode(self: Self, buf: &[u8], width: u32, height: u32, color_type: ExtendedColorType) -> ImageResult<()>` - See the trait method [`write_image`](#method.write_image) for more details.

**Trait Implementations:**

- **ImageEncoder**
  - `fn write_image(self: Self, buf: &[u8], width: u32, height: u32, color_type: ExtendedColorType) -> ImageResult<()>` - Encodes the image `image` that has dimensions `width` and `height` and `ColorType` `c`.
  - `fn set_icc_profile(self: & mut Self, icc_profile: Vec<u8>) -> Result<(), UnsupportedError>`



## image::codecs::tiff::TiffReader

*Struct*

Wrapper struct around a `Cursor<Vec<u8>>`

**Generic Parameters:**
- R

**Tuple Struct**: `()`

**Trait Implementations:**

- **Read**
  - `fn read(self: & mut Self, buf: & mut [u8]) -> io::Result<usize>`
  - `fn read_to_end(self: & mut Self, buf: & mut Vec<u8>) -> io::Result<usize>`



