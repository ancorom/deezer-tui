**image > codecs > webp > encoder**

# Module: codecs::webp::encoder

## Contents

**Structs**

- [`WebPEncoder`](#webpencoder) - WebP Encoder.

---

## image::codecs::webp::encoder::WebPEncoder

*Struct*

WebP Encoder.

### Limitations

Right now only **lossless** encoding is supported.

If you need **lossy** encoding, you'll have to use `libwebp`.
Example code for encoding a [`DynamicImage`](crate::DynamicImage) with `libwebp`
via the [`webp`](https://docs.rs/webp/latest/webp/) crate can be found
[here](https://github.com/jaredforth/webp/blob/main/examples/convert.rs).

### Compression ratio

This encoder reaches compression ratios higher than PNG at a fraction of the encoding time.
However, it does not reach the full potential of lossless WebP for reducing file size.

If you need an even higher compression ratio at the cost of much slower encoding,
please encode the image with `libwebp` as outlined above.

**Generic Parameters:**
- W

**Methods:**

- `fn new_lossless(w: W) -> Self` - Create a new encoder that writes its output to `w`.
- `fn encode(self: Self, buf: &[u8], width: u32, height: u32, color_type: ExtendedColorType) -> ImageResult<()>` - Encode image data with the indicated color type.

**Trait Implementations:**

- **ImageEncoder**
  - `fn write_image(self: Self, buf: &[u8], width: u32, height: u32, color_type: ExtendedColorType) -> ImageResult<()>`
  - `fn set_icc_profile(self: & mut Self, icc_profile: Vec<u8>) -> Result<(), UnsupportedError>`
  - `fn set_exif_metadata(self: & mut Self, exif: Vec<u8>) -> Result<(), UnsupportedError>`



