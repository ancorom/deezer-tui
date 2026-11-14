**image > images > generic_image**

# Module: images::generic_image

## Contents

**Structs**

- [`Pixels`](#pixels) - Immutable pixel iterator

**Traits**

- [`GenericImage`](#genericimage) - A trait for manipulating images.
- [`GenericImageView`](#genericimageview) - Trait to inspect an image.

---

## image::images::generic_image::GenericImage

*Trait*

A trait for manipulating images.

**Methods:**

- `get_pixel_mut`: Gets a reference to the mutable pixel at location `(x, y)`. Indexed from top left.
- `put_pixel`: Put a pixel at location (x, y). Indexed from top left.
- `unsafe_put_pixel`: Puts a pixel at location (x, y). Indexed from top left.
- `blend_pixel`: Put a pixel at location (x, y), taking into account alpha channels
- `copy_from`: Copies all of the pixels from another image into this image.
- `copy_from_samples`: Copy pixels from a regular strided matrix of pixels.
- `copy_within`: Copies all of the pixels from one part of this image to another part of this image.
- `sub_image`: Returns a mutable subimage that is a view into this image.



## image::images::generic_image::GenericImageView

*Trait*

Trait to inspect an image.

```
use image::{GenericImageView, Rgb, RgbImage};

let buffer = RgbImage::new(10, 10);
let image: &dyn GenericImageView<Pixel = Rgb<u8>> = &buffer;
```

**Methods:**

- `Pixel`: The type of pixel.
- `dimensions`: The width and height of this image.
- `width`: The width of this image.
- `height`: The height of this image.
- `in_bounds`: Returns true if this x, y coordinate is contained inside the image.
- `get_pixel`: Returns the pixel located at (x, y). Indexed from top left.
- `unsafe_get_pixel`: Returns the pixel located at (x, y). Indexed from top left.
- `pixels`: Returns an Iterator over the pixels of this image.
- `view`: Returns a subimage that is an immutable view into this image.
- `try_view`: Returns a subimage that is an immutable view into this image so long as
- `buffer_like`: Create an empty [`ImageBuffer`] with the same pixel type as this image.
- `buffer_with_dimensions`: Create an empty [`ImageBuffer`] with different dimensions.
- `to_pixel_view`: If the buffer has a fitting layout, return a canonical view of the samples.



## image::images::generic_image::Pixels

*Struct*

Immutable pixel iterator

**Generic Parameters:**
- 'a
- I

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<(u32, u32, <I as >::Pixel)>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



