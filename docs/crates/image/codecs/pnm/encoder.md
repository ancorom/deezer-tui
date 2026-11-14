**image > codecs > pnm > encoder**

# Module: codecs::pnm::encoder

## Contents

**Structs**

- [`PnmEncoder`](#pnmencoder) - Encodes images to any of the `pnm` image formats.

**Enums**

- [`FlatSamples`](#flatsamples)

---

## image::codecs::pnm::encoder::FlatSamples

*Enum*

**Generic Parameters:**
- 'a

**Variants:**
- `U8(&'a [u8])`
- `U16(&'a [u16])`



## image::codecs::pnm::encoder::PnmEncoder

*Struct*

Encodes images to any of the `pnm` image formats.

**Generic Parameters:**
- W

**Methods:**

- `fn new(writer: W) -> Self` - Create new `PnmEncoder` from the `writer`.
- `fn with_subtype(self: Self, subtype: PnmSubtype) -> Self` - Encode a specific pnm subtype image.
- `fn with_header(self: Self, header: PnmHeader) -> Self` - Enforce the use of a chosen header.
- `fn with_dynamic_header(self: Self) -> Self` - Create the header dynamically for each image.
- `fn encode<'s, S>(self: & mut Self, image: S, width: u32, height: u32, color: ExtendedColorType) -> ImageResult<()>` - Encode an image whose samples are represented as a sequence of `u8` or `u16` data.

**Trait Implementations:**

- **ImageEncoder**
  - `fn write_image(self: Self, buf: &[u8], width: u32, height: u32, color_type: ExtendedColorType) -> ImageResult<()>`



