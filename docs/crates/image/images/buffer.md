**image > images > buffer**

# Module: images::buffer

## Contents

**Structs**

- [`ConvertColorOptions`](#convertcoloroptions) - Inputs to [`ImageBuffer::copy_from_color_space`].
- [`EnumeratePixels`](#enumeratepixels) - Enumerate the pixels of an image.
- [`EnumeratePixelsMut`](#enumeratepixelsmut) - Enumerate the pixels of an image.
- [`EnumerateRows`](#enumeraterows) - Enumerate the rows of an image.
- [`EnumerateRowsMut`](#enumeraterowsmut) - Enumerate the rows of an image.
- [`ImageBuffer`](#imagebuffer) - Generic image buffer
- [`Pixels`](#pixels) - Iterate over pixel refs.
- [`PixelsMut`](#pixelsmut) - Iterate over mutable pixel refs.
- [`Rows`](#rows) - Iterate over rows of an image
- [`RowsMut`](#rowsmut) - Iterate over mutable rows of an image

**Traits**

- [`ConvertBuffer`](#convertbuffer) - Provides color conversions for whole image buffers.

**Type Aliases**

- [`GrayAlphaImage`](#grayalphaimage) - Sendable grayscale + alpha channel image buffer
- [`GrayImage`](#grayimage) - Sendable grayscale image buffer
- [`Rgb32FImage`](#rgb32fimage) - An image buffer for 32-bit float RGB pixels,
- [`RgbImage`](#rgbimage) - Sendable Rgb image buffer
- [`Rgba32FImage`](#rgba32fimage) - An image buffer for 32-bit float RGBA pixels,
- [`RgbaImage`](#rgbaimage) - Sendable Rgb + alpha channel image buffer

---

## image::images::buffer::ConvertBuffer

*Trait*

Provides color conversions for whole image buffers.

**Methods:**

- `convert`: Converts `self` to a buffer of type T



## image::images::buffer::ConvertColorOptions

*Struct*

Inputs to [`ImageBuffer::copy_from_color_space`].

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`



## image::images::buffer::EnumeratePixels

*Struct*

Enumerate the pixels of an image.

**Generic Parameters:**
- 'a
- P

**Trait Implementations:**

- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<(u32, u32, &'a P)>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## image::images::buffer::EnumeratePixelsMut

*Struct*

Enumerate the pixels of an image.

**Generic Parameters:**
- 'a
- P

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<(u32, u32, &'a  mut P)>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`



## image::images::buffer::EnumerateRows

*Struct*

Enumerate the rows of an image.

**Generic Parameters:**
- 'a
- P

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<(u32, EnumeratePixels<'a, P>)>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



## image::images::buffer::EnumerateRowsMut

*Struct*

Enumerate the rows of an image.

**Generic Parameters:**
- 'a
- P

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<(u32, EnumeratePixelsMut<'a, P>)>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`



## image::images::buffer::GrayAlphaImage

*Type Alias*: `ImageBuffer<crate::color::LumaA<u8>, Vec<u8>>`

Sendable grayscale + alpha channel image buffer



## image::images::buffer::GrayImage

*Type Alias*: `ImageBuffer<crate::color::Luma<u8>, Vec<u8>>`

Sendable grayscale image buffer



## image::images::buffer::ImageBuffer

*Struct*

Generic image buffer

This is an image parameterised by its Pixel types, represented by a width and height and a
container of channel data. It provides direct access to its pixels and implements the
[`GenericImageView`] and [`GenericImage`] traits. In many ways, this is the standard buffer
implementing those traits. Using this concrete type instead of a generic type parameter has
been shown to improve performance.

The crate defines a few type aliases with regularly used pixel types for your convenience, such
as [`RgbImage`], [`GrayImage`] etc.

[`GenericImage`]: trait.GenericImage.html
[`GenericImageView`]: trait.GenericImageView.html
[`RgbImage`]: type.RgbImage.html
[`GrayImage`]: type.GrayImage.html

To convert between images of different Pixel types use [`DynamicImage`].

You can retrieve a complete description of the buffer's layout and contents through
[`as_flat_samples`] and [`as_flat_samples_mut`]. This can be handy to also use the contents in
a foreign language, map it as a GPU host buffer or other similar tasks.

[`DynamicImage`]: enum.DynamicImage.html
[`as_flat_samples`]: #method.as_flat_samples
[`as_flat_samples_mut`]: #method.as_flat_samples_mut

## Examples

Create a simple canvas and paint a small cross.

```
use image::{RgbImage, Rgb};

let mut img = RgbImage::new(32, 32);

for x in 15..=17 {
    for y in 8..24 {
        img.put_pixel(x, y, Rgb([255, 0, 0]));
        img.put_pixel(y, x, Rgb([255, 0, 0]));
    }
}
```

Overlays an image on top of a larger background raster.

```no_run
use image::{GenericImage, GenericImageView, ImageBuffer, open};

let on_top = open("path/to/some.png").unwrap().into_rgb8();
let mut img = ImageBuffer::from_fn(512, 512, |x, y| {
    if (x + y) % 2 == 0 {
        image::Rgb([0, 0, 0])
    } else {
        image::Rgb([255, 255, 255])
    }
});

image::imageops::overlay(&mut img, &on_top, 128, 128);
```

Convert an `RgbaImage` to a `GrayImage`.

```no_run
use image::{open, DynamicImage};

let rgba = open("path/to/some.png").unwrap().into_rgba8();
let gray = DynamicImage::ImageRgba8(rgba).into_luma8();
```

**Generic Parameters:**
- P
- Container

**Methods:**

- `fn set_rgb_primaries(self: & mut Self, color: CicpColorPrimaries)` - Define the color space for the image.
- `fn set_transfer_function(self: & mut Self, tf: CicpTransferCharacteristics)` - Define the transfer function for the image.
- `fn color_space(self: &Self) -> Cicp` - Get the Cicp encoding of this buffer's color data.
- `fn set_color_space(self: & mut Self, cicp: Cicp) -> ImageResult<()>` - Set primaries and transfer characteristics from a Cicp color space.
- `fn par_pixels_mut(self: & mut Self) -> PixelsMutPar<P>` - Returns a parallel iterator over the mutable pixels of this image, usable with `rayon`.
- `fn par_enumerate_pixels_mut(self: & mut Self) -> EnumeratePixelsMutPar<P>` - Returns a parallel iterator over the mutable pixels of this image and their coordinates, usable with `rayon`.
- `fn write_with_encoder<E>(self: &Self, encoder: E) -> ImageResult<()>` - Writes the buffer with the given encoder.
- `fn copy_from_color_space<FromType, D>(self: & mut Self, from: &ImageBuffer<FromType, D>, options: ConvertColorOptions) -> ImageResult<()>` - Copy pixel data from one buffer to another, calculating equivalent color representations
- `fn to_color_space<IntoType>(self: &Self, color: Cicp, options: ConvertColorOptions) -> Result<ImageBuffer<IntoType, Vec<<SelfPixel as >::Subpixel>>, ImageError>` - Convert this buffer into a newly allocated buffer, changing the color representation.
- `fn apply_color_space(self: & mut Self, color: Cicp, options: ConvertColorOptions) -> ImageResult<()>` - Apply a color space to an image, transforming the pixel representation.
- `fn new(width: u32, height: u32) -> ImageBuffer<P, Vec<<P as >::Subpixel>>` - Creates a new image buffer based on a `Vec<P::Subpixel>`.
- `fn from_pixel(width: u32, height: u32, pixel: P) -> ImageBuffer<P, Vec<<P as >::Subpixel>>` - Constructs a new `ImageBuffer` by copying a pixel
- `fn from_fn<F>(width: u32, height: u32, f: F) -> ImageBuffer<P, Vec<<P as >::Subpixel>>` - Constructs a new `ImageBuffer` by repeated application of the supplied function.
- `fn from_vec(width: u32, height: u32, buf: Vec<<P as >::Subpixel>) -> Option<ImageBuffer<P, Vec<<P as >::Subpixel>>>` - Creates an image buffer out of an existing buffer.
- `fn into_vec(self: Self) -> Vec<<P as >::Subpixel>` - Consumes the image buffer and returns the underlying data
- `fn from_raw_bgr(width: u32, height: u32, container: Container) -> Option<Self>` - Construct an image by swapping `Bgr` channels into an `Rgb` order.
- `fn into_raw_bgr(self: Self) -> Container` - Return the underlying raw buffer after converting it into `Bgr` channel order.
- `fn save<Q>(self: &Self, path: Q) -> ImageResult<()>` - Saves the buffer to a file at the path specified.
- `fn from_raw_bgra(width: u32, height: u32, container: Container) -> Option<Self>` - Construct an image by swapping `BgrA` channels into an `RgbA` order.
- `fn into_raw_bgra(self: Self) -> Container` - Return the underlying raw buffer after converting it into `BgrA` channel order.
- `fn from_par_fn<F>(width: u32, height: u32, f: F) -> ImageBuffer<P, Vec<<P as >::Subpixel>>` - Constructs a new `ImageBuffer` by repeated application of the supplied function,
- `fn save_with_format<Q>(self: &Self, path: Q, format: ImageFormat) -> ImageResult<()>` - Saves the buffer to a file at the specified path in
- `fn par_pixels(self: &Self) -> PixelsPar<P>` - Returns a parallel iterator over the pixels of this image, usable with `rayon`.
- `fn par_enumerate_pixels(self: &Self) -> EnumeratePixelsPar<P>` - Returns a parallel iterator over the pixels of this image and their coordinates, usable with `rayon`.
- `fn from_raw(width: u32, height: u32, buf: Container) -> Option<ImageBuffer<P, Container>>` - Constructs a buffer from a generic container
- `fn into_raw(self: Self) -> Container` - Returns the underlying raw buffer
- `fn as_raw(self: &Self) -> &Container` - Returns the underlying raw buffer
- `fn dimensions(self: &Self) -> (u32, u32)` - The width and height of this image.
- `fn width(self: &Self) -> u32` - The width of this image.
- `fn height(self: &Self) -> u32` - The height of this image.
- `fn pixels(self: &Self) -> Pixels<P>` - Returns an iterator over the pixels of this image.
- `fn rows(self: &Self) -> Rows<P>` - Returns an iterator over the rows of this image.
- `fn enumerate_pixels(self: &Self) -> EnumeratePixels<P>` - Enumerates over the pixels of the image.
- `fn enumerate_rows(self: &Self) -> EnumerateRows<P>` - Enumerates over the rows of the image.
- `fn get_pixel(self: &Self, x: u32, y: u32) -> &P` - Gets a reference to the pixel at location `(x, y)`
- `fn get_pixel_checked(self: &Self, x: u32, y: u32) -> Option<&P>` - Gets a reference to the pixel at location `(x, y)` or returns `None` if
- `fn sample_layout(self: &Self) -> SampleLayout` - Get the format of the buffer when viewed as a matrix of samples.
- `fn into_flat_samples(self: Self) -> FlatSamples<Container>` - Return the raw sample buffer with its stride an dimension information.
- `fn as_flat_samples(self: &Self) -> FlatSamples<&[<P as >::Subpixel]>` - Return a view on the raw sample buffer.
- `fn as_flat_samples_mut(self: & mut Self) -> FlatSamples<& mut [<P as >::Subpixel]>` - Return a mutable view on the raw sample buffer.
- `fn expand_palette(self: Self, palette: &[(u8, u8, u8)], transparent_idx: Option<u8>) -> RgbaImage` - Expands a color palette by re-using the existing buffer.
- `fn write_to<W>(self: &Self, writer: & mut W, format: ImageFormat) -> ImageResult<()>` - Writes the buffer to a writer in the specified format.
- `fn pixels_mut(self: & mut Self) -> PixelsMut<P>` - Returns an iterator over the mutable pixels of this image.
- `fn rows_mut(self: & mut Self) -> RowsMut<P>` - Returns an iterator over the mutable rows of this image.
- `fn enumerate_pixels_mut(self: & mut Self) -> EnumeratePixelsMut<P>` - Enumerates over the pixels of the image.
- `fn enumerate_rows_mut(self: & mut Self) -> EnumerateRowsMut<P>` - Enumerates over the rows of the image.
- `fn get_pixel_mut(self: & mut Self, x: u32, y: u32) -> & mut P` - Gets a reference to the mutable pixel at location `(x, y)`
- `fn get_pixel_mut_checked(self: & mut Self, x: u32, y: u32) -> Option<& mut P>` - Gets a reference to the mutable pixel at location `(x, y)` or returns
- `fn put_pixel(self: & mut Self, x: u32, y: u32, pixel: P)` - Puts a pixel at location `(x, y)`

**Traits:** Eq

**Trait Implementations:**

- **GenericImage**
  - `fn get_pixel_mut(self: & mut Self, x: u32, y: u32) -> & mut P`
  - `fn put_pixel(self: & mut Self, x: u32, y: u32, pixel: P)`
  - `fn unsafe_put_pixel(self: & mut Self, x: u32, y: u32, pixel: P)` - Puts a pixel at location (x, y), ignoring bounds checking.
  - `fn blend_pixel(self: & mut Self, x: u32, y: u32, p: P)` - Put a pixel at location (x, y), taking into account alpha channels
  - `fn copy_from_samples(self: & mut Self, view: ViewOfPixel<<Self as >::Pixel>, x: u32, y: u32) -> ImageResult<()>`
  - `fn copy_within(self: & mut Self, source: Rect, x: u32, y: u32) -> bool`
- **From**
  - `fn from(value: DynamicImage) -> Self`
- **GenericImageView**
  - `fn dimensions(self: &Self) -> (u32, u32)`
  - `fn get_pixel(self: &Self, x: u32, y: u32) -> P`
  - `fn to_pixel_view(self: &Self) -> Option<ViewOfPixel<<Self as >::Pixel>>`
  - `fn unsafe_get_pixel(self: &Self, x: u32, y: u32) -> P` - Returns the pixel located at (x, y), ignoring bounds checking.
  - `fn buffer_with_dimensions(self: &Self, width: u32, height: u32) -> ImageBuffer<P, Vec<<P as >::Subpixel>>`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as Deref>::Target`
- **Index**
  - `fn index(self: &Self, (x, y): (u32, u32)) -> &P`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **ConvertBuffer**
  - `fn convert(self: &Self) -> ImageBuffer<ToType, Vec<<ToType as >::Subpixel>>` - # Examples
- **From**
  - `fn from(value: DynamicImage) -> Self`
- **IndexMut**
  - `fn index_mut(self: & mut Self, (x, y): (u32, u32)) -> & mut P`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> ImageBuffer<P, Container>`
  - `fn clone_from(self: & mut Self, source: &Self)`
- **From**
  - `fn from(value: DynamicImage) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Default**
  - `fn default() -> Self`
- **From**
  - `fn from(value: DynamicImage) -> Self`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as Deref>::Target`



## image::images::buffer::Pixels

*Struct*

Iterate over pixel refs.

**Generic Parameters:**
- 'a
- P

**Trait Implementations:**

- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<&'a P>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<&'a P>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`



## image::images::buffer::PixelsMut

*Struct*

Iterate over mutable pixel refs.

**Generic Parameters:**
- 'a
- P

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<&'a  mut P>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<&'a  mut P>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



## image::images::buffer::Rgb32FImage

*Type Alias*: `ImageBuffer<crate::color::Rgb<f32>, Vec<f32>>`

An image buffer for 32-bit float RGB pixels,
where the backing container is a flattened vector of floats.



## image::images::buffer::RgbImage

*Type Alias*: `ImageBuffer<crate::color::Rgb<u8>, Vec<u8>>`

Sendable Rgb image buffer



## image::images::buffer::Rgba32FImage

*Type Alias*: `ImageBuffer<crate::color::Rgba<f32>, Vec<f32>>`

An image buffer for 32-bit float RGBA pixels,
where the backing container is a flattened vector of floats.



## image::images::buffer::RgbaImage

*Type Alias*: `ImageBuffer<crate::color::Rgba<u8>, Vec<u8>>`

Sendable Rgb + alpha channel image buffer



## image::images::buffer::Rows

*Struct*

Iterate over rows of an image

This iterator is created with [`ImageBuffer::rows`]. See its document for details.

[`ImageBuffer::rows`]: ../struct.ImageBuffer.html#method.rows

**Generic Parameters:**
- 'a
- P

**Trait Implementations:**

- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<Pixels<'a, P>>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<Pixels<'a, P>>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## image::images::buffer::RowsMut

*Struct*

Iterate over mutable rows of an image

This iterator is created with [`ImageBuffer::rows_mut`]. See its document for details.

[`ImageBuffer::rows_mut`]: ../struct.ImageBuffer.html#method.rows_mut

**Generic Parameters:**
- 'a
- P

**Trait Implementations:**

- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<PixelsMut<'a, P>>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<PixelsMut<'a, P>>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



