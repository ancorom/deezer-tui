**image > codecs > bmp > decoder**

# Module: codecs::bmp::decoder

## Contents

**Structs**

- [`BmpDecoder`](#bmpdecoder) - A bmp decoder

---

## image::codecs::bmp::decoder::BmpDecoder

*Struct*

A bmp decoder

**Generic Parameters:**
- R

**Methods:**

- `fn new(reader: R) -> ImageResult<BmpDecoder<R>>` - Create a new decoder that decodes from the stream ```r```
- `fn new_without_file_header(reader: R) -> ImageResult<BmpDecoder<R>>` - Create a new decoder that decodes from the stream ```r``` without first
- `fn set_indexed_color(self: & mut Self, indexed_color: bool)` - If true, the palette in BMP does not apply to the image even if it is found.
- `fn get_palette(self: &Self) -> Option<&[[u8; 3]]>` - Get the palette that is embedded in the BMP image, if any.

**Trait Implementations:**

- **ImageDecoderRect**
  - `fn read_rect(self: & mut Self, x: u32, y: u32, width: u32, height: u32, buf: & mut [u8], row_pitch: usize) -> ImageResult<()>`
- **ImageDecoder**
  - `fn dimensions(self: &Self) -> (u32, u32)`
  - `fn color_type(self: &Self) -> ColorType`
  - `fn read_image(self: Self, buf: & mut [u8]) -> ImageResult<()>`
  - `fn read_image_boxed(self: Box<Self>, buf: & mut [u8]) -> ImageResult<()>`



