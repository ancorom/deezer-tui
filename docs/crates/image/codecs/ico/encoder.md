**image > codecs > ico > encoder**

# Module: codecs::ico::encoder

## Contents

**Structs**

- [`IcoEncoder`](#icoencoder) - ICO encoder
- [`IcoFrame`](#icoframe) - An ICO image entry

---

## image::codecs::ico::encoder::IcoEncoder

*Struct*

ICO encoder

**Generic Parameters:**
- W

**Methods:**

- `fn new(w: W) -> IcoEncoder<W>` - Create a new encoder that writes its output to ```w```.
- `fn encode_images(self: Self, images: &[IcoFrame]) -> ImageResult<()>` - Takes some [`IcoFrame`]s and encodes them into an ICO.

**Trait Implementations:**

- **ImageEncoder**
  - `fn write_image(self: Self, buf: &[u8], width: u32, height: u32, color_type: ExtendedColorType) -> ImageResult<()>` - Write an ICO image with the specified width, height, and color type.



## image::codecs::ico::encoder::IcoFrame

*Struct*

An ICO image entry

**Generic Parameters:**
- 'a

**Methods:**

- `fn with_encoded<impl Into<Cow<'a, [u8]>>>(encoded_image: impl Trait, width: u32, height: u32, color_type: ExtendedColorType) -> ImageResult<Self>` - Construct a new `IcoFrame` using a pre-encoded PNG or BMP
- `fn as_png(buf: &[u8], width: u32, height: u32, color_type: ExtendedColorType) -> ImageResult<Self>` - Construct a new `IcoFrame` by encoding `buf` as a PNG



