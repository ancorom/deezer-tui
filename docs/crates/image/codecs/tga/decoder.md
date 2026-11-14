**image > codecs > tga > decoder**

# Module: codecs::tga::decoder

## Contents

**Structs**

- [`TgaDecoder`](#tgadecoder) - The representation of a TGA decoder

---

## image::codecs::tga::decoder::TgaDecoder

*Struct*

The representation of a TGA decoder

**Generic Parameters:**
- R

**Methods:**

- `fn new(r: R) -> ImageResult<TgaDecoder<R>>` - Create a new decoder that decodes from the stream `r`

**Trait Implementations:**

- **ImageDecoder**
  - `fn dimensions(self: &Self) -> (u32, u32)`
  - `fn color_type(self: &Self) -> ColorType`
  - `fn original_color_type(self: &Self) -> ExtendedColorType`
  - `fn read_image(self: Self, buf: & mut [u8]) -> ImageResult<()>`
  - `fn read_image_boxed(self: Box<Self>, buf: & mut [u8]) -> ImageResult<()>`



