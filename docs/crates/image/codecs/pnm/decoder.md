**image > codecs > pnm > decoder**

# Module: codecs::pnm::decoder

## Contents

**Structs**

- [`PnmDecoder`](#pnmdecoder) - PNM decoder

---

## image::codecs::pnm::decoder::PnmDecoder

*Struct*

PNM decoder

**Generic Parameters:**
- R

**Methods:**

- `fn new(buffered_read: R) -> ImageResult<PnmDecoder<R>>` - Create a new decoder that decodes from the stream ```read```
- `fn header(self: &Self) -> &PnmHeader` - Get the header of the decoded image.
- `fn into_inner(self: Self) -> (R, PnmHeader)` - Extract the reader and header after an image has been read.
- `fn subtype(self: &Self) -> PnmSubtype` - Get the pnm subtype, depending on the magic constant contained in the header

**Trait Implementations:**

- **ImageDecoder**
  - `fn dimensions(self: &Self) -> (u32, u32)`
  - `fn color_type(self: &Self) -> ColorType`
  - `fn original_color_type(self: &Self) -> ExtendedColorType`
  - `fn read_image(self: Self, buf: & mut [u8]) -> ImageResult<()>`
  - `fn read_image_boxed(self: Box<Self>, buf: & mut [u8]) -> ImageResult<()>`



