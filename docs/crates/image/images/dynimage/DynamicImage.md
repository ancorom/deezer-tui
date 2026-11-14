**image > images > dynimage > DynamicImage**

# Module: images::dynimage::DynamicImage

## Contents

**Functions**

- [`apply_color_space`](#apply_color_space) - Change the color space, modifying pixel values to refer to the same colors.
- [`apply_orientation`](#apply_orientation) - Rotates and/or flips the image as indicated by [Orientation].
- [`color`](#color) - Return this image's color type.
- [`convert_color_space`](#convert_color_space) - Change the color space and pixel type of this image.
- [`copy_from_color_space`](#copy_from_color_space) - Copy pixel data from one buffer to another.
- [`fast_blur`](#fast_blur) - Performs a fast blur on this image.
- [`to`](#to) - Encodes a dynamic image into a buffer.

---

## image::images::dynimage::DynamicImage::apply_color_space

*Function*

Change the color space, modifying pixel values to refer to the same colors.

On success, this dynamic image contains color data equivalent to its previous color data.
The sample type of `self` is not changed, the data representation is transformed within the
current buffer.

Returns `Ok` if:
- The primaries and transfer functions of both image's color spaces must be supported,
  otherwise returns a [`ImageError::Unsupported`].
- The target `Cicp` must have full range and an `Identity` matrix. (This library's
  [`Luma`][`crate::Luma`] refers implicity to a chromaticity derived non-constant luminance
  color).

See also [`Self::copy_from_color_space`].

```rust
fn apply_color_space(self: & mut Self, cicp: Cicp, options: ConvertColorOptions) -> ImageResult<()>
```



## image::images::dynimage::DynamicImage::apply_orientation

*Function*

Rotates and/or flips the image as indicated by [Orientation].

This can be used to apply Exif orientation to an image,
e.g. to correctly display a photo taken by a smartphone camera:

```
# fn only_check_if_this_compiles() -> Result<(), Box<dyn std::error::Error>> {
use image::{DynamicImage, ImageReader, ImageDecoder};

let mut decoder = ImageReader::open("file.jpg")?.into_decoder()?;
let orientation = decoder.orientation()?;
let mut image = DynamicImage::from_decoder(decoder)?;
image.apply_orientation(orientation);
# Ok(())
# }
```

Note that for some orientations cannot be efficiently applied in-place.
In that case this function will make a copy of the image internally.

If this matters to you, please see the documentation on the variants of [Orientation]
to learn which orientations can and cannot be applied without copying.

```rust
fn apply_orientation(self: & mut Self, orientation: Orientation)
```



## image::images::dynimage::DynamicImage::color

*Function*

Return this image's color type.

```rust
fn color(self: &Self) -> color::ColorType
```



## image::images::dynimage::DynamicImage::convert_color_space

*Function*

Change the color space and pixel type of this image.

On success, this dynamic image contains color data equivalent to its previous color data
with another type of pixels.

Returns `Ok` if:
- The primaries and transfer functions of both image's color spaces must be supported,
  otherwise returns a [`ImageError::Unsupported`].
- The target `Cicp` must have full range and an `Identity` matrix. (This library's
  [`Luma`][`crate::Luma`] refers implicity to a chromaticity derived non-constant luminance
  color).

See also [`Self::copy_from_color_space`].

```rust
fn convert_color_space(self: & mut Self, cicp: Cicp, options: ConvertColorOptions, color: color::ColorType) -> ImageResult<()>
```



## image::images::dynimage::DynamicImage::copy_from_color_space

*Function*

Copy pixel data from one buffer to another.

On success, this dynamic image contains color data equivalent to the sources color data.
Neither the color space nor the sample type of `self` is changed, the data representation
is transformed and copied into the current buffer.

Returns `Ok` if:
- Both images to have the same dimensions, otherwise returns a [`ImageError::Parameter`].
- The primaries and transfer functions of both image's color spaces must be supported,
  otherwise returns a [`ImageError::Unsupported`].

See also [`Self::apply_color_space`] and [`Self::convert_color_space`] to modify an image
directly.

## Accuracy

All color values are subject to change to their _intended_ values. Please do not rely on
them further than your own colorimetric understanding shows them correct. For instance,
conversion of RGB to their corresponding Luma values needs to be modified in future
versions of this library. Expect colors to be too bright or too dark until further notice.

```rust
fn copy_from_color_space(self: & mut Self, other: &DynamicImage, options: ConvertColorOptions) -> ImageResult<()>
```



## image::images::dynimage::DynamicImage::fast_blur

*Function*

Performs a fast blur on this image.

# Arguments

* `sigma` - value controls image flattening level.

This method typically assumes that the input is scene-linear light.
If it is not, color distortion may occur.

This method operates on pixel channel values directly without taking into account color
space data.

```rust
fn fast_blur(self: &Self, sigma: f32) -> DynamicImage
```



## image::images::dynimage::DynamicImage::to

*Function*

Encodes a dynamic image into a buffer.

**WARNING**: Conversion between RGB and Luma is not aware of the color space and always
uses sRGB coefficients to determine a non-constant luminance from an RGB color (and
conversely).

This unfortunately owes to the public bounds of `T` which does not allow for passing a
color space as a parameter. This function will likely be deprecated and replaced.

```rust
fn to<T>(self: &Self) -> ImageBuffer<T, Vec<<T as >::Subpixel>>
```



