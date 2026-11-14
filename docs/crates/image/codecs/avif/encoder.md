**image > codecs > avif > encoder**

# Module: codecs::avif::encoder

## Contents

**Structs**

- [`AvifEncoder`](#avifencoder) - AVIF Encoder.

**Enums**

- [`ColorSpace`](#colorspace) - An enumeration over supported AVIF color spaces

---

## image::codecs::avif::encoder::AvifEncoder

*Struct*

AVIF Encoder.

Writes one image into the chosen output.

**Generic Parameters:**
- W

**Methods:**

- `fn new(w: W) -> Self` - Create a new encoder that writes its output to `w`.
- `fn new_with_speed_quality(w: W, speed: u8, quality: u8) -> Self` - Create a new encoder with a specified speed and quality that writes its output to `w`.
- `fn with_colorspace(self: Self, color_space: ColorSpace) -> Self` - Encode with the specified `color_space`.
- `fn with_num_threads(self: Self, num_threads: Option<usize>) -> Self` - Configures `rayon` thread pool size.

**Trait Implementations:**

- **ImageEncoder**
  - `fn write_image(self: Self, data: &[u8], width: u32, height: u32, color: ExtendedColorType) -> ImageResult<()>` - Encode image data with the indicated color type.
  - `fn set_exif_metadata(self: & mut Self, exif: Vec<u8>) -> Result<(), UnsupportedError>`



## image::codecs::avif::encoder::ColorSpace

*Enum*

An enumeration over supported AVIF color spaces

**Variants:**
- `Srgb` - sRGB colorspace
- `Bt709` - BT.709 colorspace

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



