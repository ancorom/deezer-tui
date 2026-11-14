**image > imageops > colorops**

# Module: imageops::colorops

## Contents

**Structs**

- [`BiLevel`](#bilevel) - A bi-level color map

**Functions**

- [`brighten`](#brighten) - Brighten the supplied image.
- [`brighten_in_place`](#brighten_in_place) - Brighten the supplied image in place.
- [`contrast`](#contrast) - Adjust the contrast of the supplied image.
- [`contrast_in_place`](#contrast_in_place) - Adjust the contrast of the supplied image in place.
- [`dither`](#dither) - Reduces the colors of the image using the supplied `color_map` while applying
- [`grayscale`](#grayscale) - Convert the supplied image to grayscale. Alpha channel is discarded.
- [`grayscale_alpha`](#grayscale_alpha) - Convert the supplied image to grayscale. Alpha channel is preserved.
- [`grayscale_with_type`](#grayscale_with_type) - Convert the supplied image to a grayscale image with the specified pixel type. Alpha channel is discarded.
- [`grayscale_with_type_alpha`](#grayscale_with_type_alpha) - Convert the supplied image to a grayscale image with the specified pixel type. Alpha channel is preserved.
- [`huerotate`](#huerotate) - Hue rotate the supplied image.
- [`huerotate_in_place`](#huerotate_in_place) - Hue rotate the supplied image in place.
- [`index_colors`](#index_colors) - Reduces the colors using the supplied `color_map` and returns an image of the indices
- [`invert`](#invert) - Invert each pixel within the supplied image.

**Traits**

- [`ColorMap`](#colormap) - A color map

---

## image::imageops::colorops::BiLevel

*Struct*

A bi-level color map

# Examples
```
use image::imageops::colorops::{index_colors, BiLevel, ColorMap};
use image::{ImageBuffer, Luma};

let (w, h) = (16, 16);
// Create an image with a smooth horizontal gradient from black (0) to white (255).
let gray = ImageBuffer::from_fn(w, h, |x, y| -> Luma<u8> { [(255 * x / w) as u8].into() });
// Mapping the gray image through the `BiLevel` filter should map gray pixels less than half
// intensity (127) to black (0), and anything greater to white (255).
let cmap = BiLevel;
let palletized = index_colors(&gray, &cmap);
let mapped = ImageBuffer::from_fn(w, h, |x, y| {
    let p = palletized.get_pixel(x, y);
    cmap.lookup(p.0[0] as usize)
        .expect("indexed color out-of-range")
});
// Create an black and white image of expected output.
let bw = ImageBuffer::from_fn(w, h, |x, y| -> Luma<u8> {
    if x <= (w / 2) {
        [0].into()
    } else {
        [255].into()
    }
});
assert_eq!(mapped, bw);
```

**Unit Struct**

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ColorMap**
  - `fn index_of(self: &Self, color: &Luma<u8>) -> usize`
  - `fn lookup(self: &Self, idx: usize) -> Option<<Self as >::Color>`
  - `fn has_lookup(self: &Self) -> bool` - Indicate `NeuQuant` implements `lookup`.
  - `fn map_color(self: &Self, color: & mut Luma<u8>)`



## image::imageops::colorops::ColorMap

*Trait*

A color map

**Methods:**

- `Color`: The color type on which the map operates on
- `index_of`: Returns the index of the closest match of `color`
- `lookup`: Looks up color by index in the color map.  If `idx` is out of range for the color map, or
- `has_lookup`: Determine if this implementation of `ColorMap` overrides the default `lookup`.
- `map_color`: Maps `color` to the closest color in the color map.



## image::imageops::colorops::brighten

*Function*

Brighten the supplied image.
```value``` is the amount to brighten each pixel by.
Negative values decrease the brightness and positive values increase it.

*[See also `brighten_in_place`.][brighten_in_place]*

```rust
fn brighten<I, P, S>(image: &I, value: i32) -> crate::ImageBuffer<P, Vec<S>>
```



## image::imageops::colorops::brighten_in_place

*Function*

Brighten the supplied image in place.
```value``` is the amount to brighten each pixel by.
Negative values decrease the brightness and positive values increase it.

*[See also `brighten`.][brighten]*

```rust
fn brighten_in_place<I>(image: & mut I, value: i32)
```



## image::imageops::colorops::contrast

*Function*

Adjust the contrast of the supplied image.
```contrast``` is the amount to adjust the contrast by.
Negative values decrease the contrast and positive values increase the contrast.

*[See also `contrast_in_place`.][contrast_in_place]*

```rust
fn contrast<I, P, S>(image: &I, contrast: f32) -> crate::ImageBuffer<P, Vec<S>>
```



## image::imageops::colorops::contrast_in_place

*Function*

Adjust the contrast of the supplied image in place.
```contrast``` is the amount to adjust the contrast by.
Negative values decrease the contrast and positive values increase the contrast.

*[See also `contrast`.][contrast]*

```rust
fn contrast_in_place<I>(image: & mut I, contrast: f32)
```



## image::imageops::colorops::dither

*Function*

Reduces the colors of the image using the supplied `color_map` while applying
Floyd-Steinberg dithering to improve the visual conception

```rust
fn dither<Pix, Map>(image: & mut crate::ImageBuffer<Pix, Vec<u8>>, color_map: &Map)
```



## image::imageops::colorops::grayscale

*Function*

Convert the supplied image to grayscale. Alpha channel is discarded.

```rust
fn grayscale<I>(image: &I) -> crate::ImageBuffer<crate::color::Luma<<<I as GenericImageView>::Pixel as Pixel>::Subpixel>, Vec<<<I as GenericImageView>::Pixel as Pixel>::Subpixel>>
```



## image::imageops::colorops::grayscale_alpha

*Function*

Convert the supplied image to grayscale. Alpha channel is preserved.

```rust
fn grayscale_alpha<I>(image: &I) -> crate::ImageBuffer<crate::color::LumaA<<<I as GenericImageView>::Pixel as Pixel>::Subpixel>, Vec<<<I as GenericImageView>::Pixel as Pixel>::Subpixel>>
```



## image::imageops::colorops::grayscale_with_type

*Function*

Convert the supplied image to a grayscale image with the specified pixel type. Alpha channel is discarded.

```rust
fn grayscale_with_type<NewPixel, I>(image: &I) -> crate::ImageBuffer<NewPixel, Vec<<NewPixel as >::Subpixel>>
```



## image::imageops::colorops::grayscale_with_type_alpha

*Function*

Convert the supplied image to a grayscale image with the specified pixel type. Alpha channel is preserved.

```rust
fn grayscale_with_type_alpha<NewPixel, I>(image: &I) -> crate::ImageBuffer<NewPixel, Vec<<NewPixel as >::Subpixel>>
```



## image::imageops::colorops::huerotate

*Function*

Hue rotate the supplied image.
`value` is the degrees to rotate each pixel by.
0 and 360 do nothing, the rest rotates by the given degree value.
just like the css webkit filter hue-rotate(180)

*[See also `huerotate_in_place`.][huerotate_in_place]*

```rust
fn huerotate<I, P, S>(image: &I, value: i32) -> crate::ImageBuffer<P, Vec<S>>
```



## image::imageops::colorops::huerotate_in_place

*Function*

Hue rotate the supplied image in place.

`value` is the degrees to rotate each pixel by.
0 and 360 do nothing, the rest rotates by the given degree value.
just like the css webkit filter hue-rotate(180)

*[See also `huerotate`.][huerotate]*

```rust
fn huerotate_in_place<I>(image: & mut I, value: i32)
```



## image::imageops::colorops::index_colors

*Function*

Reduces the colors using the supplied `color_map` and returns an image of the indices

```rust
fn index_colors<Pix, Map>(image: &crate::ImageBuffer<Pix, Vec<u8>>, color_map: &Map) -> crate::ImageBuffer<crate::color::Luma<u8>, Vec<u8>>
```



## image::imageops::colorops::invert

*Function*

Invert each pixel within the supplied image.
This function operates in place.

```rust
fn invert<I>(image: & mut I)
```



