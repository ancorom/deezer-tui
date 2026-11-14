**image > codecs > bmp > encoder**

# Module: codecs::bmp::encoder

## Contents

**Structs**

- [`BmpEncoder`](#bmpencoder) - The representation of a BMP encoder.

---

## image::codecs::bmp::encoder::BmpEncoder

*Struct*

The representation of a BMP encoder.

**Generic Parameters:**
- 'a
- W

**Methods:**

- `fn new(w: &'a  mut W) -> Self` - Create a new encoder that writes its output to ```w```.
- `fn encode(self: & mut Self, image: &[u8], width: u32, height: u32, c: ExtendedColorType) -> ImageResult<()>` - Encodes the image `image` that has dimensions `width` and `height` and `ExtendedColorType` `c`.
- `fn encode_with_palette(self: & mut Self, image: &[u8], width: u32, height: u32, color_type: ExtendedColorType, palette: Option<&[[u8; 3]]>) -> ImageResult<()>` - Same as `encode`, but allow a palette to be passed in. The `palette` is ignored for color

**Trait Implementations:**

- **ImageEncoder**
  - `fn write_image(self: Self, buf: &[u8], width: u32, height: u32, color_type: ExtendedColorType) -> ImageResult<()>`



