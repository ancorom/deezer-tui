**image > images > flat**

# Module: images::flat

## Contents

**Structs**

- [`FlatSamples`](#flatsamples) - A flat buffer over a (multi channel) image.
- [`SampleLayout`](#samplelayout) - A ffi compatible description of a sample buffer.
- [`View`](#view) - A flat buffer that can be used as an image view.
- [`ViewMut`](#viewmut) - A mutable owning version of a flat buffer.

**Enums**

- [`Error`](#error) - Denotes invalid flat sample buffers when trying to convert to stricter types.
- [`NormalForm`](#normalform) - Different normal forms of buffers.

**Type Aliases**

- [`ViewMutOfPixel`](#viewmutofpixel) - Type alias for a mutable view based on a pixel's channels.
- [`ViewOfPixel`](#viewofpixel) - Type alias for a view based on a pixel's channels.

---

## image::images::flat::Error

*Enum*

Denotes invalid flat sample buffers when trying to convert to stricter types.

The biggest use case being `ImageBuffer` which expects closely packed
samples in a row major matrix representation. But this error type may be
reused for other import functions. A more versatile user may also try to
correct the underlying representation depending on the error variant.

**Variants:**
- `TooLarge` - The represented image was too large.
- `NormalFormRequired(NormalForm)` - The represented image can not use this representation.
- `ChannelCountMismatch(u8, u8)` - The color format did not match the channel count.
- `WrongColor(crate::color::ColorType)` - Deprecated - `ChannelCountMismatch` is used instead

**Traits:** Copy, Eq, Error

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## image::images::flat::FlatSamples

*Struct*

A flat buffer over a (multi channel) image.

In contrast to `ImageBuffer`, this representation of a sample collection is much more lenient
in the layout thereof. It also allows grouping by color planes instead of by pixel as long as
the strides of each extent are constant. This struct itself has no invariants on the strides
but not every possible configuration can be interpreted as a [`GenericImageView`] or
[`GenericImage`]. The methods [`as_view`] and [`as_view_mut`] construct the actual implementors
of these traits and perform necessary checks. To manually perform this and other layout checks
use [`is_normal`] or [`has_aliased_samples`].

Instances can be constructed not only by hand. The buffer instances returned by library
functions such as [`ImageBuffer::as_flat_samples`] guarantee that the conversion to a generic
image or generic view succeeds. A very different constructor is [`with_monocolor`]. It uses a
single pixel as the backing storage for an arbitrarily sized read-only raster by mapping each
pixel to the same samples by setting some strides to `0`.

[`GenericImage`]: ../trait.GenericImage.html
[`GenericImageView`]: ../trait.GenericImageView.html
[`ImageBuffer::as_flat_samples`]: ../struct.ImageBuffer.html#method.as_flat_samples
[`is_normal`]: #method.is_normal
[`has_aliased_samples`]: #method.has_aliased_samples
[`as_view`]: #method.as_view
[`as_view_mut`]: #method.as_view_mut
[`with_monocolor`]: #method.with_monocolor

**Generic Parameters:**
- Buffer

**Fields:**
- `samples: Buffer` - Underlying linear container holding sample values.
- `layout: SampleLayout` - A `repr(C)` description of the layout of buffer samples.
- `color_hint: Option<crate::color::ColorType>` - Supplementary color information.

**Methods:**

- `fn with_monocolor<P>(pixel: &'buf P, width: u32, height: u32) -> Self` - Create a monocolor image from a single pixel.
- `fn strides_cwh(self: &Self) -> (usize, usize, usize)` - Get the strides for indexing matrix-like `[(c, w, h)]`.
- `fn extents(self: &Self) -> (usize, usize, usize)` - Get the dimensions `(channels, width, height)`.
- `fn bounds(self: &Self) -> (u8, u32, u32)` - Tuple of bounds in the order of coordinate inputs.
- `fn as_ref<T>(self: &Self) -> FlatSamples<&[T]>` - Get a reference based version.
- `fn as_mut<T>(self: & mut Self) -> FlatSamples<& mut [T]>` - Get a mutable reference based version.
- `fn to_vec<T>(self: &Self) -> FlatSamples<Vec<T>>` - Copy the data into an owned vector.
- `fn get_sample<T>(self: &Self, channel: u8, x: u32, y: u32) -> Option<&T>` - Get a reference to a single sample.
- `fn get_mut_sample<T>(self: & mut Self, channel: u8, x: u32, y: u32) -> Option<& mut T>` - Get a mutable reference to a single sample.
- `fn as_view<P>(self: &Self) -> Result<View<&[<P as >::Subpixel], P>, Error>` - View this buffer as an image over some type of pixel.
- `fn as_view_with_mut_samples<P>(self: & mut Self) -> Result<View<& mut [<P as >::Subpixel], P>, Error>` - View this buffer but keep mutability at a sample level.
- `fn as_view_mut<P>(self: & mut Self) -> Result<ViewMut<& mut [<P as >::Subpixel], P>, Error>` - Interpret this buffer as a mutable image.
- `fn as_slice<T>(self: &Self) -> &[T]` - View the samples as a slice.
- `fn as_mut_slice<T>(self: & mut Self) -> & mut [T]` - View the samples as a slice.
- `fn image_slice<T>(self: &Self) -> Option<&[T]>` - Return the portion of the buffer that holds sample values.
- `fn image_mut_slice<T>(self: & mut Self) -> Option<& mut [T]>` - Mutable portion of the buffer that holds sample values.
- `fn try_into_buffer<P>(self: Self) -> Result<ImageBuffer<P, Buffer>, (Error, Self)>` - Move the data into an image buffer.
- `fn min_length(self: &Self) -> Option<usize>` - Get the minimum length of a buffer such that all in-bounds samples have valid indices.
- `fn fits(self: &Self, len: usize) -> bool` - Check if a buffer of length `len` is large enough.
- `fn has_aliased_samples(self: &Self) -> bool` - If there are any samples aliasing each other.
- `fn is_normal(self: &Self, form: NormalForm) -> bool` - Check if a buffer fulfills the requirements of a normal form.
- `fn in_bounds(self: &Self, channel: u8, x: u32, y: u32) -> bool` - Check that the pixel and the channel index are in bounds.
- `fn index(self: &Self, channel: u8, x: u32, y: u32) -> Option<usize>` - Resolve the index of a particular sample.
- `fn index_ignoring_bounds(self: &Self, channel: usize, x: usize, y: usize) -> Option<usize>` - Get the theoretical position of sample (x, y, channel).
- `fn in_bounds_index(self: &Self, channel: u8, x: u32, y: u32) -> usize` - Get an index provided it is inbouds.
- `fn shrink_to(self: & mut Self, channels: u8, width: u32, height: u32)` - Shrink the image to the minimum of current and given extents.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Index**
  - `fn index(self: &Self, (c, x, y): (u8, u32, u32)) -> &<Self as >::Output` - Return a reference to a single sample at specified coordinates.
- **IndexMut**
  - `fn index_mut(self: & mut Self, (c, x, y): (u8, u32, u32)) -> & mut <Self as >::Output` - Return a mutable reference to a single sample at specified coordinates.



## image::images::flat::NormalForm

*Enum*

Different normal forms of buffers.

A normal form is an unaliased buffer with some additional constraints.  The `ÌmageBuffer` uses
row major form with packed samples.

**Variants:**
- `Unaliased` - No pixel aliases another.
- `PixelPacked` - At least pixels are packed.
- `ImagePacked` - All samples are packed.
- `RowMajorPacked` - The samples are in row-major form and all samples are packed.
- `ColumnMajorPacked` - The samples are in column-major form and all samples are packed.

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering>` - Compares the logical preconditions.
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## image::images::flat::SampleLayout

*Struct*

A ffi compatible description of a sample buffer.

**Fields:**
- `channels: u8` - The number of channels in the color representation of the image.
- `channel_stride: usize` - Add this to an index to get to the sample in the next channel.
- `width: u32` - The width of the represented image.
- `width_stride: usize` - Add this to an index to get to the next sample in x-direction.
- `height: u32` - The height of the represented image.
- `height_stride: usize` - Add this to an index to get to the next sample in y-direction.

**Methods:**

- `fn row_major_packed(channels: u8, width: u32, height: u32) -> Self` - Describe a row-major image packed in all directions.
- `fn column_major_packed(channels: u8, width: u32, height: u32) -> Self` - Describe a column-major image packed in all directions.
- `fn strides_cwh(self: &Self) -> (usize, usize, usize)` - Get the strides for indexing matrix-like `[(c, w, h)]`.
- `fn extents(self: &Self) -> (usize, usize, usize)` - Get the dimensions `(channels, width, height)`.
- `fn bounds(self: &Self) -> (u8, u32, u32)` - Tuple of bounds in the order of coordinate inputs.
- `fn min_length(self: &Self) -> Option<usize>` - Get the minimum length of a buffer such that all in-bounds samples have valid indices.
- `fn fits(self: &Self, len: usize) -> bool` - Check if a buffer of length `len` is large enough.
- `fn has_aliased_samples(self: &Self) -> bool` - If there are any samples aliasing each other.
- `fn is_normal(self: &Self, form: NormalForm) -> bool` - Check if a buffer fulfills the requirements of a normal form.
- `fn in_bounds(self: &Self, channel: u8, x: u32, y: u32) -> bool` - Check that the pixel and the channel index are in bounds.
- `fn index(self: &Self, channel: u8, x: u32, y: u32) -> Option<usize>` - Resolve the index of a particular sample.
- `fn index_ignoring_bounds(self: &Self, channel: usize, x: usize, y: usize) -> Option<usize>` - Get the theoretical position of sample (channel, x, y).
- `fn in_bounds_index(self: &Self, c: u8, x: u32, y: u32) -> usize` - Get an index provided it is inbouds.
- `fn shrink_to(self: & mut Self, channels: u8, width: u32, height: u32)` - Shrink the image to the minimum of current and given extents.

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



## image::images::flat::View

*Struct*

A flat buffer that can be used as an image view.

This is a nearly trivial wrapper around a buffer but at least sanitizes by checking the buffer
length first and constraining the pixel type.

Note that this does not eliminate panics as the `AsRef<[T]>` implementation of `Buffer` may be
unreliable, i.e. return different buffers at different times. This of course is a non-issue for
all common collections where the bounds check once must be enough.

# Inner invariants

* For all indices inside bounds, the corresponding index is valid in the buffer
* `P::channel_count()` agrees with `self.inner.layout.channels`

**Generic Parameters:**
- Buffer
- P

**Methods:**

- `fn into_inner(self: Self) -> FlatSamples<Buffer>` - Take out the sample buffer.
- `fn flat(self: &Self) -> &FlatSamples<Buffer>` - Get a reference on the inner sample descriptor.
- `fn samples(self: &Self) -> &Buffer` - Get a reference on the inner buffer.
- `fn get_sample(self: &Self, channel: u8, x: u32, y: u32) -> Option<&<P as >::Subpixel>` - Get a reference to a selected subpixel if it is in-bounds.
- `fn get_mut_sample(self: & mut Self, channel: u8, x: u32, y: u32) -> Option<& mut <P as >::Subpixel>` - Get a mutable reference to a selected subpixel if it is in-bounds.
- `fn min_length(self: &Self) -> usize` - Get the minimum length of a buffer such that all in-bounds samples have valid indices.
- `fn image_slice(self: &Self) -> &[<P as >::Subpixel]` - Return the portion of the buffer that holds sample values.
- `fn image_mut_slice(self: & mut Self) -> & mut [<P as >::Subpixel]` - Return the mutable portion of the buffer that holds sample values.
- `fn shrink_to(self: & mut Self, width: u32, height: u32)` - Shrink the inner image.
- `fn try_upgrade(self: Self) -> Result<ViewMut<Buffer, P>, (Error, Self)>` - Try to convert this into an image with mutable pixels.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **GenericImageView**
  - `fn dimensions(self: &Self) -> (u32, u32)`
  - `fn get_pixel(self: &Self, x: u32, y: u32) -> <Self as >::Pixel`
  - `fn to_pixel_view(self: &Self) -> Option<ViewOfPixel<<Self as >::Pixel>>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## image::images::flat::ViewMut

*Struct*

A mutable owning version of a flat buffer.

While this wraps a buffer similar to `ImageBuffer`, this is mostly intended as a utility. The
library endorsed normalized representation is still `ImageBuffer`. Also, the implementation of
`AsMut<[P::Subpixel]>` must always yield the same buffer. Therefore there is no public way to
construct this with an owning buffer.

# Inner invariants

* For all indices inside bounds, the corresponding index is valid in the buffer
* There is no aliasing of samples
* The samples are packed, i.e. `self.inner.layout.sample_stride == 1`
* `P::channel_count()` agrees with `self.inner.layout.channels`

**Generic Parameters:**
- Buffer
- P

**Methods:**

- `fn into_inner(self: Self) -> FlatSamples<Buffer>` - Take out the sample buffer.
- `fn flat(self: &Self) -> &FlatSamples<Buffer>` - Get a reference on the sample buffer descriptor.
- `fn samples(self: &Self) -> &Buffer` - Get a reference on the inner buffer.
- `fn min_length(self: &Self) -> usize` - Get the minimum length of a buffer such that all in-bounds samples have valid indices.
- `fn get_sample(self: &Self, channel: u8, x: u32, y: u32) -> Option<&<P as >::Subpixel>` - Get a reference to a selected subpixel.
- `fn get_mut_sample(self: & mut Self, channel: u8, x: u32, y: u32) -> Option<& mut <P as >::Subpixel>` - Get a mutable reference to a selected sample.
- `fn image_slice(self: &Self) -> &[<P as >::Subpixel]` - Return the portion of the buffer that holds sample values.
- `fn image_mut_slice(self: & mut Self) -> & mut [<P as >::Subpixel]` - Return the mutable buffer that holds sample values.
- `fn shrink_to(self: & mut Self, width: u32, height: u32)` - Shrink the inner image.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **GenericImage**
  - `fn get_pixel_mut(self: & mut Self, x: u32, y: u32) -> & mut <Self as >::Pixel`
  - `fn put_pixel(self: & mut Self, x: u32, y: u32, pixel: <Self as >::Pixel)`
  - `fn blend_pixel(self: & mut Self, x: u32, y: u32, pixel: <Self as >::Pixel)`
- **GenericImageView**
  - `fn dimensions(self: &Self) -> (u32, u32)`
  - `fn get_pixel(self: &Self, x: u32, y: u32) -> <Self as >::Pixel`
  - `fn to_pixel_view(self: &Self) -> Option<ViewOfPixel<<Self as >::Pixel>>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## image::images::flat::ViewMutOfPixel

*Type Alias*: `ViewMut<&'lt  mut [<P as Pixel>::Subpixel], P>`

Type alias for a mutable view based on a pixel's channels.



## image::images::flat::ViewOfPixel

*Type Alias*: `View<&'lt [<P as Pixel>::Subpixel], P>`

Type alias for a view based on a pixel's channels.



