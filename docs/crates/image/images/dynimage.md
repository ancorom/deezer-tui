**image > images > dynimage**

# Module: images::dynimage

## Contents

**Enums**

- [`DynamicImage`](#dynamicimage) - A Dynamic Image

**Functions**

- [`image_dimensions`](#image_dimensions) - Read a tuple containing the (width, height) of the image located at the specified path.
- [`load_from_memory`](#load_from_memory) - Create a new image from a byte slice
- [`load_from_memory_with_format`](#load_from_memory_with_format) - Create a new image from a byte slice
- [`open`](#open) - Open the image located at the path specified.
- [`write_buffer_with_format`](#write_buffer_with_format) - Writes the supplied buffer to a writer in the specified format.

---

## image::images::dynimage::DynamicImage

*Enum*

A Dynamic Image

This represents a _matrix_ of _pixels_ which are _convertible_ from and to an _RGBA_
representation. More variants that adhere to these principles may get added in the future, in
particular to cover other combinations typically used.

# Usage

This type can act as a converter between specific `ImageBuffer` instances.

```
use image::{DynamicImage, GrayImage, RgbImage};

let rgb: RgbImage = RgbImage::new(10, 10);
let luma: GrayImage = DynamicImage::ImageRgb8(rgb).into_luma8();
```

# Design

There is no goal to provide an all-encompassing type with all possible memory layouts. This
would hardly be feasible as a simple enum, due to the sheer number of combinations of channel
kinds, channel order, and bit depth. Rather, this type provides an opinionated selection with
normalized channel order which can store common pixel values without loss.

# Color space

Each image has an associated color space in the form of [CICP] data ([ITU Rec H.273]). Not all
color spaces are supported in the sense that you can compute in them ([Context][w3c-png]).
Conversion into different pixels types ([`ColorType`][`crate::ColorType`]) _generally_ take the
color space into account, with the exception of [`DynamicImage::to`] due to historical design
baggage.

The imageops functions operate in _encoded_ space, directly on the channel values, and do _not_
linearize colors internally as you might be used to from GPU shader programming. Their return
values however copy the color space annotation of the source.

The IO functions do _not yet_ write ICC or CICP indications into the result formats. We're
aware of this problem, it is tracked in [#2493] and [#1460].

[CICP]: https://www.w3.org/TR/png-3/#cICP-chunk
[w3c-png]: https://github.com/w3c/png/issues/312
[ITU Rec H.273]: https://www.itu.int/rec/T-REC-H.273-202407-I/en
[#2493]: https://github.com/image-rs/image/issues/2493
[#1460]: https://github.com/image-rs/image/issues/1460

**Variants:**
- `ImageLuma8(crate::images::buffer::GrayImage)` - Each pixel in this image is 8-bit Luma
- `ImageLumaA8(crate::images::buffer::GrayAlphaImage)` - Each pixel in this image is 8-bit Luma with alpha
- `ImageRgb8(crate::images::buffer::RgbImage)` - Each pixel in this image is 8-bit Rgb
- `ImageRgba8(crate::images::buffer::RgbaImage)` - Each pixel in this image is 8-bit Rgb with alpha
- `ImageLuma16(ImageBuffer<crate::color::Luma<u16>, Vec<u16>>)` - Each pixel in this image is 16-bit Luma
- `ImageLumaA16(ImageBuffer<crate::color::LumaA<u16>, Vec<u16>>)` - Each pixel in this image is 16-bit Luma with alpha
- `ImageRgb16(ImageBuffer<crate::color::Rgb<u16>, Vec<u16>>)` - Each pixel in this image is 16-bit Rgb
- `ImageRgba16(ImageBuffer<crate::color::Rgba<u16>, Vec<u16>>)` - Each pixel in this image is 16-bit Rgb with alpha
- `ImageRgb32F(crate::images::buffer::Rgb32FImage)` - Each pixel in this image is 32-bit float Rgb
- `ImageRgba32F(crate::images::buffer::Rgba32FImage)` - Each pixel in this image is 32-bit float Rgb with alpha

**Methods:**

- `fn new(w: u32, h: u32, color: color::ColorType) -> DynamicImage` - Creates a dynamic image backed by a buffer depending on
- `fn new_luma8(w: u32, h: u32) -> DynamicImage` - Creates a dynamic image backed by a buffer of gray pixels.
- `fn new_luma_a8(w: u32, h: u32) -> DynamicImage` - Creates a dynamic image backed by a buffer of gray
- `fn new_rgb8(w: u32, h: u32) -> DynamicImage` - Creates a dynamic image backed by a buffer of RGB pixels.
- `fn new_rgba8(w: u32, h: u32) -> DynamicImage` - Creates a dynamic image backed by a buffer of RGBA pixels.
- `fn new_luma16(w: u32, h: u32) -> DynamicImage` - Creates a dynamic image backed by a buffer of gray pixels.
- `fn new_luma_a16(w: u32, h: u32) -> DynamicImage` - Creates a dynamic image backed by a buffer of gray
- `fn new_rgb16(w: u32, h: u32) -> DynamicImage` - Creates a dynamic image backed by a buffer of RGB pixels.
- `fn new_rgba16(w: u32, h: u32) -> DynamicImage` - Creates a dynamic image backed by a buffer of RGBA pixels.
- `fn new_rgb32f(w: u32, h: u32) -> DynamicImage` - Creates a dynamic image backed by a buffer of RGB pixels.
- `fn new_rgba32f(w: u32, h: u32) -> DynamicImage` - Creates a dynamic image backed by a buffer of RGBA pixels.
- `fn from_decoder<impl ImageDecoder>(decoder: impl Trait) -> ImageResult<Self>` - Decodes an encoded image into a dynamic image.
- `fn to<T>(self: &Self) -> ImageBuffer<T, Vec<<T as >::Subpixel>>` - Encodes a dynamic image into a buffer.
- `fn to_rgb8(self: &Self) -> RgbImage` - Returns a copy of this image as an RGB image.
- `fn to_rgb16(self: &Self) -> ImageBuffer<crate::color::Rgb<u16>, Vec<u16>>` - Returns a copy of this image as an RGB image.
- `fn to_rgb32f(self: &Self) -> Rgb32FImage` - Returns a copy of this image as an RGB image.
- `fn to_rgba8(self: &Self) -> RgbaImage` - Returns a copy of this image as an RGBA image.
- `fn to_rgba16(self: &Self) -> ImageBuffer<crate::color::Rgba<u16>, Vec<u16>>` - Returns a copy of this image as an RGBA image.
- `fn to_rgba32f(self: &Self) -> Rgba32FImage` - Returns a copy of this image as an RGBA image.
- `fn to_luma8(self: &Self) -> GrayImage` - Returns a copy of this image as a Luma image.
- `fn to_luma16(self: &Self) -> ImageBuffer<crate::color::Luma<u16>, Vec<u16>>` - Returns a copy of this image as a Luma image.
- `fn to_luma32f(self: &Self) -> ImageBuffer<Luma<f32>, Vec<f32>>` - Returns a copy of this image as a Luma image.
- `fn to_luma_alpha8(self: &Self) -> GrayAlphaImage` - Returns a copy of this image as a `LumaA` image.
- `fn to_luma_alpha16(self: &Self) -> ImageBuffer<crate::color::LumaA<u16>, Vec<u16>>` - Returns a copy of this image as a `LumaA` image.
- `fn to_luma_alpha32f(self: &Self) -> ImageBuffer<LumaA<f32>, Vec<f32>>` - Returns a copy of this image as a `LumaA` image.
- `fn into_rgb8(self: Self) -> RgbImage` - Consume the image and returns a RGB image.
- `fn into_rgb16(self: Self) -> ImageBuffer<crate::color::Rgb<u16>, Vec<u16>>` - Consume the image and returns a RGB image.
- `fn into_rgb32f(self: Self) -> Rgb32FImage` - Consume the image and returns a RGB image.
- `fn into_rgba8(self: Self) -> RgbaImage` - Consume the image and returns a RGBA image.
- `fn into_rgba16(self: Self) -> ImageBuffer<crate::color::Rgba<u16>, Vec<u16>>` - Consume the image and returns a RGBA image.
- `fn into_rgba32f(self: Self) -> Rgba32FImage` - Consume the image and returns a RGBA image.
- `fn into_luma8(self: Self) -> GrayImage` - Consume the image and returns a Luma image.
- `fn into_luma16(self: Self) -> ImageBuffer<crate::color::Luma<u16>, Vec<u16>>` - Consume the image and returns a Luma image.
- `fn into_luma_alpha8(self: Self) -> GrayAlphaImage` - Consume the image and returns a `LumaA` image.
- `fn into_luma_alpha16(self: Self) -> ImageBuffer<crate::color::LumaA<u16>, Vec<u16>>` - Consume the image and returns a `LumaA` image.
- `fn crop(self: & mut Self, x: u32, y: u32, width: u32, height: u32) -> DynamicImage` - Return a cut-out of this image delimited by the bounding rectangle.
- `fn crop_imm(self: &Self, x: u32, y: u32, width: u32, height: u32) -> DynamicImage` - Return a cut-out of this image delimited by the bounding rectangle.
- `fn as_rgb8(self: &Self) -> Option<&RgbImage>` - Return a reference to an 8bit RGB image
- `fn as_mut_rgb8(self: & mut Self) -> Option<& mut RgbImage>` - Return a mutable reference to an 8bit RGB image
- `fn as_rgba8(self: &Self) -> Option<&RgbaImage>` - Return a reference to an 8bit RGBA image
- `fn as_mut_rgba8(self: & mut Self) -> Option<& mut RgbaImage>` - Return a mutable reference to an 8bit RGBA image
- `fn as_luma8(self: &Self) -> Option<&GrayImage>` - Return a reference to an 8bit Grayscale image
- `fn as_mut_luma8(self: & mut Self) -> Option<& mut GrayImage>` - Return a mutable reference to an 8bit Grayscale image
- `fn as_luma_alpha8(self: &Self) -> Option<&GrayAlphaImage>` - Return a reference to an 8bit Grayscale image with an alpha channel
- `fn as_mut_luma_alpha8(self: & mut Self) -> Option<& mut GrayAlphaImage>` - Return a mutable reference to an 8bit Grayscale image with an alpha channel
- `fn as_rgb16(self: &Self) -> Option<&ImageBuffer<crate::color::Rgb<u16>, Vec<u16>>>` - Return a reference to an 16bit RGB image
- `fn as_mut_rgb16(self: & mut Self) -> Option<& mut ImageBuffer<crate::color::Rgb<u16>, Vec<u16>>>` - Return a mutable reference to an 16bit RGB image
- `fn as_rgba16(self: &Self) -> Option<&ImageBuffer<crate::color::Rgba<u16>, Vec<u16>>>` - Return a reference to an 16bit RGBA image
- `fn as_mut_rgba16(self: & mut Self) -> Option<& mut ImageBuffer<crate::color::Rgba<u16>, Vec<u16>>>` - Return a mutable reference to an 16bit RGBA image
- `fn as_rgb32f(self: &Self) -> Option<&Rgb32FImage>` - Return a reference to an 32bit RGB image
- `fn as_mut_rgb32f(self: & mut Self) -> Option<& mut Rgb32FImage>` - Return a mutable reference to an 32bit RGB image
- `fn as_rgba32f(self: &Self) -> Option<&Rgba32FImage>` - Return a reference to an 32bit RGBA image
- `fn as_mut_rgba32f(self: & mut Self) -> Option<& mut Rgba32FImage>` - Return a mutable reference to an 32bit RGBA image
- `fn as_luma16(self: &Self) -> Option<&ImageBuffer<crate::color::Luma<u16>, Vec<u16>>>` - Return a reference to an 16bit Grayscale image
- `fn as_mut_luma16(self: & mut Self) -> Option<& mut ImageBuffer<crate::color::Luma<u16>, Vec<u16>>>` - Return a mutable reference to an 16bit Grayscale image
- `fn as_luma_alpha16(self: &Self) -> Option<&ImageBuffer<crate::color::LumaA<u16>, Vec<u16>>>` - Return a reference to an 16bit Grayscale image with an alpha channel
- `fn as_mut_luma_alpha16(self: & mut Self) -> Option<& mut ImageBuffer<crate::color::LumaA<u16>, Vec<u16>>>` - Return a mutable reference to an 16bit Grayscale image with an alpha channel
- `fn as_flat_samples_u8(self: &Self) -> Option<FlatSamples<&[u8]>>` - Return a view on the raw sample buffer for 8 bit per channel images.
- `fn as_flat_samples_u16(self: &Self) -> Option<FlatSamples<&[u16]>>` - Return a view on the raw sample buffer for 16 bit per channel images.
- `fn as_flat_samples_f32(self: &Self) -> Option<FlatSamples<&[f32]>>` - Return a view on the raw sample buffer for 32bit per channel images.
- `fn as_bytes(self: &Self) -> &[u8]` - Return this image's pixels as a native endian byte slice.
- `fn into_bytes(self: Self) -> Vec<u8>` - Return this image's pixels as a byte vector. If the `ImageBuffer`
- `fn color(self: &Self) -> color::ColorType` - Return this image's color type.
- `fn width(self: &Self) -> u32` - Returns the width of the underlying image
- `fn height(self: &Self) -> u32` - Returns the height of the underlying image
- `fn set_rgb_primaries(self: & mut Self, color: CicpColorPrimaries)` - Define the color space for the image.
- `fn set_transfer_function(self: & mut Self, tf: CicpTransferCharacteristics)` - Define the transfer function for the image.
- `fn color_space(self: &Self) -> Cicp` - Get the Cicp encoding of this buffer's color data.
- `fn set_color_space(self: & mut Self, cicp: Cicp) -> ImageResult<()>` - Set primaries and transfer characteristics from a Cicp color space.
- `fn has_alpha(self: &Self) -> bool` - Whether the image contains an alpha channel
- `fn grayscale(self: &Self) -> DynamicImage` - Return a grayscale version of this image.
- `fn invert(self: & mut Self)` - Invert the colors of this image.
- `fn resize(self: &Self, nwidth: u32, nheight: u32, filter: imageops::FilterType) -> DynamicImage` - Resize this image using the specified filter algorithm.
- `fn resize_exact(self: &Self, nwidth: u32, nheight: u32, filter: imageops::FilterType) -> DynamicImage` - Resize this image using the specified filter algorithm.
- `fn thumbnail(self: &Self, nwidth: u32, nheight: u32) -> DynamicImage` - Scale this image down to fit within a specific size.
- `fn thumbnail_exact(self: &Self, nwidth: u32, nheight: u32) -> DynamicImage` - Scale this image down to a specific size.
- `fn resize_to_fill(self: &Self, nwidth: u32, nheight: u32, filter: imageops::FilterType) -> DynamicImage` - Resize this image using the specified filter algorithm.
- `fn blur(self: &Self, sigma: f32) -> DynamicImage` - Performs a Gaussian blur on this image.
- `fn blur_advanced(self: &Self, parameters: GaussianBlurParameters) -> DynamicImage` - Performs a Gaussian blur on this image.
- `fn fast_blur(self: &Self, sigma: f32) -> DynamicImage` - Performs a fast blur on this image.
- `fn unsharpen(self: &Self, sigma: f32, threshold: i32) -> DynamicImage` - Performs an unsharpen mask on this image.
- `fn filter3x3(self: &Self, kernel: &[f32]) -> DynamicImage` - Filters this image with the specified 3x3 kernel.
- `fn adjust_contrast(self: &Self, c: f32) -> DynamicImage` - Adjust the contrast of this image.
- `fn brighten(self: &Self, value: i32) -> DynamicImage` - Brighten the pixels of this image.
- `fn huerotate(self: &Self, value: i32) -> DynamicImage` - Hue rotate the supplied image.
- `fn flipv(self: &Self) -> DynamicImage` - Flip this image vertically
- `fn fliph(self: &Self) -> DynamicImage` - Flip this image horizontally
- `fn rotate90(self: &Self) -> DynamicImage` - Rotate this image 90 degrees clockwise.
- `fn rotate180(self: &Self) -> DynamicImage` - Rotate this image 180 degrees.
- `fn rotate270(self: &Self) -> DynamicImage` - Rotate this image 270 degrees clockwise.
- `fn apply_orientation(self: & mut Self, orientation: Orientation)` - Rotates and/or flips the image as indicated by [Orientation].
- `fn copy_from_color_space(self: & mut Self, other: &DynamicImage, options: ConvertColorOptions) -> ImageResult<()>` - Copy pixel data from one buffer to another.
- `fn apply_color_space(self: & mut Self, cicp: Cicp, options: ConvertColorOptions) -> ImageResult<()>` - Change the color space, modifying pixel values to refer to the same colors.
- `fn convert_color_space(self: & mut Self, cicp: Cicp, options: ConvertColorOptions, color: color::ColorType) -> ImageResult<()>` - Change the color space and pixel type of this image.
- `fn write_to<W>(self: &Self, w: W, format: ImageFormat) -> ImageResult<()>` - Encode this image and write it to `w`.
- `fn write_with_encoder<impl ImageEncoder>(self: &Self, encoder: impl Trait) -> ImageResult<()>` - Encode this image with the provided encoder.
- `fn save<Q>(self: &Self, path: Q) -> ImageResult<()>` - Saves the buffer to a file with the format derived from the file extension.
- `fn save_with_format<Q>(self: &Self, path: Q, format: ImageFormat) -> ImageResult<()>` - Saves the buffer to a file with the specified format.

**Trait Implementations:**

- **From**
  - `fn from(image: Rgb32FImage) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **From**
  - `fn from(image: GrayAlphaImage) -> Self`
- **From**
  - `fn from(image: Rgba32FImage) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
  - `fn clone_from(self: & mut Self, source: &Self)`
- **From**
  - `fn from(image: RgbImage) -> Self`
- **From**
  - `fn from(image: ImageBuffer<crate::color::LumaA<u16>, Vec<u16>>) -> Self`
- **From**
  - `fn from(image: ImageBuffer<Luma<f32>, Vec<f32>>) -> Self`
- **From**
  - `fn from(image: RgbaImage) -> Self`
- **GenericImage**
  - `fn put_pixel(self: & mut Self, x: u32, y: u32, pixel: color::Rgba<u8>)`
  - `fn blend_pixel(self: & mut Self, x: u32, y: u32, pixel: color::Rgba<u8>)`
  - `fn get_pixel_mut(self: & mut Self, _: u32, _: u32) -> & mut color::Rgba<u8>` - Do not use is function: It is unimplemented!
- **From**
  - `fn from(image: ImageBuffer<crate::color::Rgb<u16>, Vec<u16>>) -> Self`
- **From**
  - `fn from(image: ImageBuffer<LumaA<f32>, Vec<f32>>) -> Self`
- **GenericImageView**
  - `fn dimensions(self: &Self) -> (u32, u32)`
  - `fn get_pixel(self: &Self, x: u32, y: u32) -> color::Rgba<u8>`
- **From**
  - `fn from(image: ImageBuffer<crate::color::Luma<u16>, Vec<u16>>) -> Self`
- **Default**
  - `fn default() -> Self`
- **From**
  - `fn from(image: ImageBuffer<crate::color::Rgba<u16>, Vec<u16>>) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(image: GrayImage) -> Self`



## image::images::dynimage::image_dimensions

*Function*

Read a tuple containing the (width, height) of the image located at the specified path.
This is faster than fully loading the image and then getting its dimensions.

Try [`ImageReader`] for more advanced uses, including guessing the format based on the file's
content before its path or manually supplying the format.

```rust
fn image_dimensions<P>(path: P) -> crate::error::ImageResult<(u32, u32)>
```



## image::images::dynimage::load_from_memory

*Function*

Create a new image from a byte slice

Makes an educated guess about the image format.
TGA is not supported by this function.

Try [`ImageReader`] for more advanced uses.

```rust
fn load_from_memory(buffer: &[u8]) -> crate::error::ImageResult<DynamicImage>
```



## image::images::dynimage::load_from_memory_with_format

*Function*

Create a new image from a byte slice

This is just a simple wrapper that constructs an `std::io::Cursor` around the buffer and then
calls `load` with that reader.

Try [`ImageReader`] for more advanced uses.

[`load`]: fn.load.html

```rust
fn load_from_memory_with_format(buf: &[u8], format: crate::ImageFormat) -> crate::error::ImageResult<DynamicImage>
```



## image::images::dynimage::open

*Function*

Open the image located at the path specified.
The image's format is determined from the path's file extension.

Try [`ImageReader`] for more advanced uses, including guessing the format based on the file's
content before its path.

```rust
fn open<P>(path: P) -> crate::error::ImageResult<DynamicImage>
```



## image::images::dynimage::write_buffer_with_format

*Function*

Writes the supplied buffer to a writer in the specified format.

The buffer is assumed to have the correct format according to the specified color type. This
will lead to corrupted writers if the buffer contains malformed data.

Assumes the writer is buffered. In most cases, you should wrap your writer in a `BufWriter` for
best performance.

```rust
fn write_buffer_with_format<W, impl Into<ExtendedColorType>>(buffered_writer: & mut W, buf: &[u8], width: u32, height: u32, color: impl Trait, format: crate::ImageFormat) -> crate::error::ImageResult<()>
```



