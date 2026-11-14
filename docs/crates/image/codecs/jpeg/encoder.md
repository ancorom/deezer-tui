**image > codecs > jpeg > encoder**

# Module: codecs::jpeg::encoder

## Contents

**Structs**

- [`JpegEncoder`](#jpegencoder) - The representation of a JPEG encoder
- [`PixelDensity`](#pixeldensity) - Represents the pixel density of an image

**Enums**

- [`PixelDensityUnit`](#pixeldensityunit) - Represents a unit in which the density of an image is measured

---

## image::codecs::jpeg::encoder::JpegEncoder

*Struct*

The representation of a JPEG encoder

**Generic Parameters:**
- W

**Methods:**

- `fn new(w: W) -> JpegEncoder<W>` - Create a new encoder that writes its output to ```w```
- `fn new_with_quality(w: W, quality: u8) -> JpegEncoder<W>` - Create a new encoder that writes its output to ```w```, and has
- `fn set_pixel_density(self: & mut Self, pixel_density: PixelDensity)` - Set the pixel density of the images the encoder will encode.
- `fn encode(self: & mut Self, image: &[u8], width: u32, height: u32, color_type: ExtendedColorType) -> ImageResult<()>` - Encodes the image stored in the raw byte buffer ```image```
- `fn encode_image<I>(self: & mut Self, image: &I) -> ImageResult<()>` - Encodes the given image.

**Trait Implementations:**

- **ImageEncoder**
  - `fn write_image(self: Self, buf: &[u8], width: u32, height: u32, color_type: ExtendedColorType) -> ImageResult<()>`
  - `fn set_icc_profile(self: & mut Self, icc_profile: Vec<u8>) -> Result<(), UnsupportedError>`
  - `fn set_exif_metadata(self: & mut Self, exif: Vec<u8>) -> Result<(), UnsupportedError>`



## image::codecs::jpeg::encoder::PixelDensity

*Struct*

Represents the pixel density of an image

For example, a 300 DPI image is represented by:

```rust
use image::codecs::jpeg::*;
let hdpi = PixelDensity::dpi(300);
assert_eq!(hdpi, PixelDensity {density: (300,300), unit: PixelDensityUnit::Inches})
```

**Fields:**
- `density: (u16, u16)` - A couple of values for (Xdensity, Ydensity)
- `unit: PixelDensityUnit` - The unit in which the density is measured

**Methods:**

- `fn dpi(density: u16) -> Self` - Creates the most common pixel density type:

**Traits:** Copy, Eq

**Trait Implementations:**

- **Default**
  - `fn default() -> Self` - Returns a pixel density with a pixel aspect ratio of 1
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## image::codecs::jpeg::encoder::PixelDensityUnit

*Enum*

Represents a unit in which the density of an image is measured

**Variants:**
- `PixelAspectRatio` - Represents the absence of a unit, the values indicate only a
- `Inches` - Pixels per inch (2.54 cm)
- `Centimeters` - Pixels per centimeter

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



