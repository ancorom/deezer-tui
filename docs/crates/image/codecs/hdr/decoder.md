**image > codecs > hdr > decoder**

# Module: codecs::hdr::decoder

## Contents

**Structs**

- [`HdrDecoder`](#hdrdecoder) - An Radiance HDR decoder
- [`HdrMetadata`](#hdrmetadata) - Metadata for Radiance HDR image

**Constants**

- [`SIGNATURE`](#signature) - Radiance HDR file signature

---

## image::codecs::hdr::decoder::HdrDecoder

*Struct*

An Radiance HDR decoder

**Generic Parameters:**
- R

**Methods:**

- `fn new(reader: R) -> ImageResult<Self>` - Reads Radiance HDR image header from stream ```r```
- `fn new_nonstrict(reader: R) -> ImageResult<Self>` - Allows reading old Radiance HDR images
- `fn with_strictness(reader: R, strict: bool) -> ImageResult<HdrDecoder<R>>` - Reads Radiance HDR image header from stream `reader`,
- `fn metadata(self: &Self) -> HdrMetadata` - Returns file metadata. Refer to `HdrMetadata` for details.

**Trait Implementations:**

- **ImageDecoder**
  - `fn dimensions(self: &Self) -> (u32, u32)`
  - `fn color_type(self: &Self) -> ColorType`
  - `fn read_image(self: Self, buf: & mut [u8]) -> ImageResult<()>`
  - `fn read_image_boxed(self: Box<Self>, buf: & mut [u8]) -> ImageResult<()>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## image::codecs::hdr::decoder::HdrMetadata

*Struct*

Metadata for Radiance HDR image

**Fields:**
- `width: u32` - Width of decoded image. It could be either scanline length,
- `height: u32` - Height of decoded image. It depends on orientation too.
- `orientation: ((i8, i8), (i8, i8))` - Orientation matrix. For standard orientation it is ((1,0),(0,1)) - left to right, top to bottom.
- `exposure: Option<f32>` - Divide color values by exposure to get to get physical radiance in
- `color_correction: Option<(f32, f32, f32)>` - Divide color values by corresponding tuple member (r, g, b) to get to get physical radiance
- `pixel_aspect_ratio: Option<f32>` - Pixel height divided by pixel width
- `custom_attributes: Vec<(String, String)>` - All lines contained in image header are put here. Ordering of lines is preserved.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## image::codecs::hdr::decoder::SIGNATURE

*Constant*: `&[u8]`

Radiance HDR file signature



