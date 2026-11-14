**image > imageops > sample**

# Module: imageops::sample

## Contents

**Structs**

- [`GaussianBlurParameters`](#gaussianblurparameters) - Holds analytical gaussian blur representation

**Enums**

- [`FilterType`](#filtertype) - Available Sampling Filters.

**Functions**

- [`blur`](#blur) - Performs a Gaussian blur on the supplied image.
- [`blur_advanced`](#blur_advanced) - Performs a Gaussian blur on the supplied image.
- [`filter3x3`](#filter3x3) - Perform a 3x3 box filter on the supplied image.
- [`interpolate_bilinear`](#interpolate_bilinear) - Linearly sample from an image using coordinates in [0, w-1] and [0, h-1].
- [`interpolate_nearest`](#interpolate_nearest) - Sample from an image using coordinates in [0, w-1] and [0, h-1], taking the
- [`resize`](#resize) - Resize the supplied image to the specified dimensions.
- [`sample_bilinear`](#sample_bilinear) - Linearly sample from an image using coordinates in [0, 1].
- [`sample_nearest`](#sample_nearest) - Sample from an image using coordinates in [0, 1], taking the nearest coordinate.
- [`thumbnail`](#thumbnail) - Resize the supplied image to the specific dimensions.
- [`unsharpen`](#unsharpen) - Performs an unsharpen mask on the supplied image.

---

## image::imageops::sample::FilterType

*Enum*

Available Sampling Filters.

## Examples

To test the different sampling filters on a real example, you can find two
examples called
[`scaledown`](https://github.com/image-rs/image/tree/main/examples/scaledown)
and
[`scaleup`](https://github.com/image-rs/image/tree/main/examples/scaleup)
in the `examples` directory of the crate source code.

Here is a 3.58 MiB
[test image](https://github.com/image-rs/image/blob/main/examples/scaledown/test.jpg)
that has been scaled down to 300x225 px:

<!-- NOTE: To test new test images locally, replace the GitHub path with `../../../docs/` -->
<div style="display: flex; flex-wrap: wrap; align-items: flex-start;">
  <div style="margin: 0 8px 8px 0;">
    <img src="https://raw.githubusercontent.com/image-rs/image/main/examples/scaledown/scaledown-test-near.png" title="Nearest"><br>
    Nearest Neighbor
  </div>
  <div style="margin: 0 8px 8px 0;">
    <img src="https://raw.githubusercontent.com/image-rs/image/main/examples/scaledown/scaledown-test-tri.png" title="Triangle"><br>
    Linear: Triangle
  </div>
  <div style="margin: 0 8px 8px 0;">
    <img src="https://raw.githubusercontent.com/image-rs/image/main/examples/scaledown/scaledown-test-cmr.png" title="CatmullRom"><br>
    Cubic: Catmull-Rom
  </div>
  <div style="margin: 0 8px 8px 0;">
    <img src="https://raw.githubusercontent.com/image-rs/image/main/examples/scaledown/scaledown-test-gauss.png" title="Gaussian"><br>
    Gaussian
  </div>
  <div style="margin: 0 8px 8px 0;">
    <img src="https://raw.githubusercontent.com/image-rs/image/main/examples/scaledown/scaledown-test-lcz2.png" title="Lanczos3"><br>
    Lanczos with window 3
  </div>
</div>

## Speed

Time required to create each of the examples above, tested on an Intel
i7-4770 CPU with Rust 1.37 in release mode:

<table style="width: auto;">
  <tr>
    <th>Nearest</th>
    <td>31 ms</td>
  </tr>
  <tr>
    <th>Triangle</th>
    <td>414 ms</td>
  </tr>
  <tr>
    <th>CatmullRom</th>
    <td>817 ms</td>
  </tr>
  <tr>
    <th>Gaussian</th>
    <td>1180 ms</td>
  </tr>
  <tr>
    <th>Lanczos3</th>
    <td>1170 ms</td>
  </tr>
</table>

**Variants:**
- `Nearest` - Nearest Neighbor
- `Triangle` - Linear Filter
- `CatmullRom` - Cubic Filter
- `Gaussian` - Gaussian Filter
- `Lanczos3` - Lanczos with window 3

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## image::imageops::sample::GaussianBlurParameters

*Struct*

Holds analytical gaussian blur representation

**Methods:**

- `fn new_from_radius(radius: f32) -> GaussianBlurParameters` - Creates a new parameters set from radius only.
- `fn new_from_kernel_size(kernel_size: f32) -> GaussianBlurParameters` - Creates a new parameters set from kernel size only.
- `fn new_anisotropic_kernel_size(x_axis_kernel_size: f32, y_axis_kernel_size: f32) -> GaussianBlurParameters` - Creates a new anisotropic parameter set from kernel sizes
- `fn new_from_sigma(sigma: f32) -> GaussianBlurParameters` - Creates a new parameters set from sigma only

**Traits:** Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`



## image::imageops::sample::blur

*Function*

Performs a Gaussian blur on the supplied image.

# Arguments

 - `sigma` - gaussian bell flattening level.

Use [`crate::imageops::fast_blur()`] for a faster but less
accurate version.
This method assumes alpha pre-multiplication for images that contain non-constant alpha.
This method typically assumes that the input is scene-linear light.
If it is not, color distortion may occur.

```rust
fn blur<I>(image: &I, sigma: f32) -> crate::ImageBuffer<<I as >::Pixel, Vec<<<I as >::Pixel as Pixel>::Subpixel>>
```



## image::imageops::sample::blur_advanced

*Function*

Performs a Gaussian blur on the supplied image.

# Arguments

 - `parameters` - see [GaussianBlurParameters] for more info.

This method assumes alpha pre-multiplication for images that contain non-constant alpha.
This method typically assumes that the input is scene-linear light.
If it is not, color distortion may occur.

```rust
fn blur_advanced<I>(image: &I, parameters: GaussianBlurParameters) -> crate::ImageBuffer<<I as >::Pixel, Vec<<<I as >::Pixel as Pixel>::Subpixel>>
```



## image::imageops::sample::filter3x3

*Function*

Perform a 3x3 box filter on the supplied image.

# Arguments:

* `image` - source image.
* `kernel` - is an array of the filter weights of length 9.

This method typically assumes that the input is scene-linear light.
If it is not, color distortion may occur.

```rust
fn filter3x3<I, P, S>(image: &I, kernel: &[f32]) -> crate::ImageBuffer<P, Vec<S>>
```



## image::imageops::sample::interpolate_bilinear

*Function*

Linearly sample from an image using coordinates in [0, w-1] and [0, h-1].

```rust
fn interpolate_bilinear<P, impl GenericImageView<Pixel = P>>(img: &impl Trait, x: f32, y: f32) -> Option<P>
```



## image::imageops::sample::interpolate_nearest

*Function*

Sample from an image using coordinates in [0, w-1] and [0, h-1], taking the
nearest pixel.

Coordinates outside the image bounds will return `None`, however the
behavior for points within half a pixel of the image bounds may change in
the future.

```rust
fn interpolate_nearest<P, impl GenericImageView<Pixel = P>>(img: &impl Trait, x: f32, y: f32) -> Option<P>
```



## image::imageops::sample::resize

*Function*

Resize the supplied image to the specified dimensions.

# Arguments:

* `nwidth` - new image width.
* `nheight` - new image height.
* `filter` -  is the sampling filter to use, see [FilterType] for mor information.

This method assumes alpha pre-multiplication for images that contain non-constant alpha.

This method typically assumes that the input is scene-linear light.
If it is not, color distortion may occur.

```rust
fn resize<I>(image: &I, nwidth: u32, nheight: u32, filter: FilterType) -> crate::ImageBuffer<<I as >::Pixel, Vec<<<I as >::Pixel as Pixel>::Subpixel>>
```



## image::imageops::sample::sample_bilinear

*Function*

Linearly sample from an image using coordinates in [0, 1].

```rust
fn sample_bilinear<P, impl GenericImageView<Pixel = P>>(img: &impl Trait, u: f32, v: f32) -> Option<P>
```



## image::imageops::sample::sample_nearest

*Function*

Sample from an image using coordinates in [0, 1], taking the nearest coordinate.

```rust
fn sample_nearest<P, impl GenericImageView<Pixel = P>>(img: &impl Trait, u: f32, v: f32) -> Option<P>
```



## image::imageops::sample::thumbnail

*Function*

Resize the supplied image to the specific dimensions.

For downscaling, this method uses a fast integer algorithm where each source pixel contributes
to exactly one target pixel.  May give aliasing artifacts if new size is close to old size.

In case the current width is smaller than the new width or similar for the height, another
strategy is used instead.  For each pixel in the output, a rectangular region of the input is
determined, just as previously.  But when no input pixel is part of this region, the nearest
pixels are interpolated instead.

For speed reasons, all interpolation is performed linearly over the colour values.  It will not
take the pixel colour spaces into account.

```rust
fn thumbnail<I, P, S>(image: &I, new_width: u32, new_height: u32) -> crate::ImageBuffer<P, Vec<S>>
```



## image::imageops::sample::unsharpen

*Function*

Performs an unsharpen mask on the supplied image.

# Arguments:

* `sigma` - is the amount to blur the image by.
* `threshold` - is the threshold for minimal brightness change that will be sharpened.

This method typically assumes that the input is scene-linear light.
If it is not, color distortion may occur.

See [Digital unsharp masking](https://en.wikipedia.org/wiki/Unsharp_masking#Digital_unsharp_masking) for more information.

```rust
fn unsharpen<I, P, S>(image: &I, sigma: f32, threshold: i32) -> crate::ImageBuffer<P, Vec<S>>
```



