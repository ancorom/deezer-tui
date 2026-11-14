**symphonia_metadata > utils > images**

# Module: utils::images

## Contents

**Structs**

- [`ImageInfo`](#imageinfo) - Image information.

**Functions**

- [`try_get_image_info`](#try_get_image_info) - Try to get basic information about an image from an image buffer.

---

## symphonia_metadata::utils::images::ImageInfo

*Struct*

Image information.

**Fields:**
- `media_type: String` - The Media Type (MIME Type) of the image format.
- `dimensions: symphonia_core::meta::Size` - The dimensions of the image.
- `color_mode: symphonia_core::meta::ColorMode` - The color mode of the image.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## symphonia_metadata::utils::images::try_get_image_info

*Function*

Try to get basic information about an image from an image buffer.

```rust
fn try_get_image_info(buf: &[u8]) -> Option<ImageInfo>
```



