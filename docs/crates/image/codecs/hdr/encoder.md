**image > codecs > hdr > encoder**

# Module: codecs::hdr::encoder

## Contents

**Structs**

- [`HdrEncoder`](#hdrencoder) - Radiance HDR encoder

---

## image::codecs::hdr::encoder::HdrEncoder

*Struct*

Radiance HDR encoder

**Generic Parameters:**
- W

**Methods:**

- `fn new(w: W) -> HdrEncoder<W>` - Creates encoder
- `fn encode(self: Self, rgb: &[Rgb<f32>], width: usize, height: usize) -> ImageResult<()>` - Encodes the image ```rgb```

**Trait Implementations:**

- **ImageEncoder**
  - `fn write_image(self: Self, unaligned_bytes: &[u8], width: u32, height: u32, color_type: ExtendedColorType) -> ImageResult<()>`



