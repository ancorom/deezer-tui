**image > codecs > dds**

# Module: codecs::dds

## Contents

**Structs**

- [`DdsDecoder`](#ddsdecoder) - The representation of a DDS decoder

---

## image::codecs::dds::DdsDecoder

*Struct*

The representation of a DDS decoder

**Generic Parameters:**
- R

**Methods:**

- `fn new(r: R) -> ImageResult<Self>` - Create a new decoder that decodes from the stream `r`

**Trait Implementations:**

- **ImageDecoder**
  - `fn dimensions(self: &Self) -> (u32, u32)`
  - `fn color_type(self: &Self) -> ColorType`
  - `fn read_image(self: Self, buf: & mut [u8]) -> ImageResult<()>`
  - `fn read_image_boxed(self: Box<Self>, buf: & mut [u8]) -> ImageResult<()>`



