**image > codecs > tga > encoder**

# Module: codecs::tga::encoder

## Contents

**Structs**

- [`TgaEncoder`](#tgaencoder) - TGA encoder.

---

## image::codecs::tga::encoder::TgaEncoder

*Struct*

TGA encoder.

**Generic Parameters:**
- W

**Methods:**

- `fn new(w: W) -> TgaEncoder<W>` - Create a new encoder that writes its output to ```w```.
- `fn disable_rle(self: Self) -> TgaEncoder<W>` - Disables run-length encoding
- `fn encode(self: Self, buf: &[u8], width: u32, height: u32, color_type: ExtendedColorType) -> ImageResult<()>` - Encodes the image ```buf``` that has dimensions ```width```

**Trait Implementations:**

- **ImageEncoder**
  - `fn write_image(self: Self, buf: &[u8], width: u32, height: u32, color_type: ExtendedColorType) -> ImageResult<()>`



