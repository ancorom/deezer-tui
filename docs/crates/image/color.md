**image > color**

# Module: color

## Contents

**Structs**

- [`Luma`](#luma) - Grayscale colors.
- [`LumaA`](#lumaa) - Grayscale colors + alpha channel
- [`Rgb`](#rgb) - RGB colors.
- [`Rgba`](#rgba) - RGB colors + alpha channel

**Enums**

- [`ColorType`](#colortype) - An enumeration over supported color types and bit depths
- [`ExtendedColorType`](#extendedcolortype) - An enumeration of color types encountered in image formats.

**Traits**

- [`FromColor`](#fromcolor) - Provides color conversions for the different pixel types.
- [`FromPrimitive`](#fromprimitive) - Convert from one pixel component type to another. For example, convert from `u8` to `f32` pixel values.

---

## image::color::ColorType

*Enum*

An enumeration over supported color types and bit depths

**Variants:**
- `L8` - Pixel is 8-bit luminance
- `La8` - Pixel is 8-bit luminance with an alpha channel
- `Rgb8` - Pixel contains 8-bit R, G and B channels
- `Rgba8` - Pixel is 8-bit RGB with an alpha channel
- `L16` - Pixel is 16-bit luminance
- `La16` - Pixel is 16-bit luminance with an alpha channel
- `Rgb16` - Pixel is 16-bit RGB
- `Rgba16` - Pixel is 16-bit RGBA
- `Rgb32F` - Pixel is 32-bit float RGB
- `Rgba32F` - Pixel is 32-bit float RGBA

**Methods:**

- `fn bytes_per_pixel(self: Self) -> u8` - Returns the number of bytes contained in a pixel of `ColorType` ```c```
- `fn has_alpha(self: Self) -> bool` - Returns if there is an alpha channel.
- `fn has_color(self: Self) -> bool` - Returns false if the color scheme is grayscale, true otherwise.
- `fn bits_per_pixel(self: Self) -> u16` - Returns the number of bits contained in a pixel of `ColorType` ```c``` (which will always be
- `fn channel_count(self: Self) -> u8` - Returns the number of color channels that make up this pixel

**Traits:** Copy, Eq

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **TryFrom**
  - `fn try_from(value: ExtendedColorType) -> Result<ColorType, <Self as >::Error>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## image::color::ExtendedColorType

*Enum*

An enumeration of color types encountered in image formats.

This is not exhaustive over all existing image formats but should be granular enough to allow
round tripping of decoding and encoding as much as possible. The variants will be extended as
necessary to enable this.

Another purpose is to advise users of a rough estimate of the accuracy and effort of the
decoding from and encoding to such an image format.

**Variants:**
- `A8` - Pixel is 8-bit alpha
- `L1` - Pixel is 1-bit luminance
- `La1` - Pixel is 1-bit luminance with an alpha channel
- `Rgb1` - Pixel contains 1-bit R, G and B channels
- `Rgba1` - Pixel is 1-bit RGB with an alpha channel
- `L2` - Pixel is 2-bit luminance
- `La2` - Pixel is 2-bit luminance with an alpha channel
- `Rgb2` - Pixel contains 2-bit R, G and B channels
- `Rgba2` - Pixel is 2-bit RGB with an alpha channel
- `L4` - Pixel is 4-bit luminance
- `La4` - Pixel is 4-bit luminance with an alpha channel
- `Rgb4` - Pixel contains 4-bit R, G and B channels
- `Rgba4` - Pixel is 4-bit RGB with an alpha channel
- `Rgb5x1` - Pixel contains 5-bit R, G and B channels packed into 2 bytes
- `L8` - Pixel is 8-bit luminance
- `La8` - Pixel is 8-bit luminance with an alpha channel
- `Rgb8` - Pixel contains 8-bit R, G and B channels
- `Rgba8` - Pixel is 8-bit RGB with an alpha channel
- `L16` - Pixel is 16-bit luminance
- `La16` - Pixel is 16-bit luminance with an alpha channel
- `Rgb16` - Pixel contains 16-bit R, G and B channels
- `Rgba16` - Pixel is 16-bit RGB with an alpha channel
- `Bgr8` - Pixel contains 8-bit B, G and R channels
- `Bgra8` - Pixel is 8-bit BGR with an alpha channel
- `Rgb32F` - Pixel is 32-bit float RGB
- `Rgba32F` - Pixel is 32-bit float RGBA
- `Cmyk8` - Pixel is 8-bit CMYK
- `Cmyk16` - Pixel is 16-bit CMYK
- `Unknown(u8)` - Pixel is of unknown color type with the specified bits per pixel. This can apply to pixels

**Methods:**

- `fn channel_count(self: Self) -> u8` - Get the number of channels for colors of this type.
- `fn bits_per_pixel(self: &Self) -> u16` - Returns the number of bits per pixel for this color type.
- `fn color_type(self: &Self) -> Option<ColorType>` - Returns the ColorType that is equivalent to this ExtendedColorType.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(c: ColorType) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## image::color::FromColor

*Trait*

Provides color conversions for the different pixel types.

**Methods:**

- `from_color`: Changes `self` to represent `Other` in the color space of `Self`



## image::color::FromPrimitive

*Trait*

Convert from one pixel component type to another. For example, convert from `u8` to `f32` pixel values.

**Methods:**

- `from_primitive`: Converts from any pixel component type to this type.



## image::color::Luma

*Struct*

Grayscale colors.

**Generic Parameters:**
- T

**Tuple Struct**: `([T; 1])`

**Traits:** PixelWithColorType, Copy, PixelWithColorType, Eq

**Trait Implementations:**

- **IndexMut**
  - `fn index_mut(self: & mut Self, _index: usize) -> & mut T`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **From**
  - `fn from(c: [T; 1]) -> Self`
- **Pixel**
  - `fn channels(self: &Self) -> &[T]`
  - `fn channels_mut(self: & mut Self) -> & mut [T]`
  - `fn alpha(self: &Self) -> <Self as >::Subpixel`
  - `fn channels4(self: &Self) -> (T, T, T, T)`
  - `fn from_channels(a: T, b: T, c: T, d: T) -> Luma<T>`
  - `fn from_slice(slice: &[T]) -> &Luma<T>`
  - `fn from_slice_mut(slice: & mut [T]) -> & mut Luma<T>`
  - `fn to_rgb(self: &Self) -> Rgb<T>`
  - `fn to_rgba(self: &Self) -> Rgba<T>`
  - `fn to_luma(self: &Self) -> Luma<T>`
  - `fn to_luma_alpha(self: &Self) -> LumaA<T>`
  - `fn map<F>(self: &Self, f: F) -> Luma<T>`
  - `fn apply<F>(self: & mut Self, f: F)`
  - `fn map_with_alpha<F, G>(self: &Self, f: F, g: G) -> Luma<T>`
  - `fn apply_with_alpha<F, G>(self: & mut Self, f: F, g: G)`
  - `fn map2<F>(self: &Self, other: &Self, f: F) -> Luma<T>`
  - `fn apply2<F>(self: & mut Self, other: &Luma<T>, f: F)`
  - `fn invert(self: & mut Self)`
  - `fn blend(self: & mut Self, other: &Luma<T>)`
- **Index**
  - `fn index(self: &Self, _index: usize) -> &T`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## image::color::LumaA

*Struct*

Grayscale colors + alpha channel

**Generic Parameters:**
- T

**Tuple Struct**: `([T; 2])`

**Traits:** PixelWithColorType, Copy, PixelWithColorType, Eq

**Trait Implementations:**

- **Pixel**
  - `fn channels(self: &Self) -> &[T]`
  - `fn channels_mut(self: & mut Self) -> & mut [T]`
  - `fn alpha(self: &Self) -> <Self as >::Subpixel`
  - `fn channels4(self: &Self) -> (T, T, T, T)`
  - `fn from_channels(a: T, b: T, c: T, d: T) -> LumaA<T>`
  - `fn from_slice(slice: &[T]) -> &LumaA<T>`
  - `fn from_slice_mut(slice: & mut [T]) -> & mut LumaA<T>`
  - `fn to_rgb(self: &Self) -> Rgb<T>`
  - `fn to_rgba(self: &Self) -> Rgba<T>`
  - `fn to_luma(self: &Self) -> Luma<T>`
  - `fn to_luma_alpha(self: &Self) -> LumaA<T>`
  - `fn map<F>(self: &Self, f: F) -> LumaA<T>`
  - `fn apply<F>(self: & mut Self, f: F)`
  - `fn map_with_alpha<F, G>(self: &Self, f: F, g: G) -> LumaA<T>`
  - `fn apply_with_alpha<F, G>(self: & mut Self, f: F, g: G)`
  - `fn map2<F>(self: &Self, other: &Self, f: F) -> LumaA<T>`
  - `fn apply2<F>(self: & mut Self, other: &LumaA<T>, f: F)`
  - `fn invert(self: & mut Self)`
  - `fn blend(self: & mut Self, other: &LumaA<T>)`
- **Index**
  - `fn index(self: &Self, _index: usize) -> &T`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **IndexMut**
  - `fn index_mut(self: & mut Self, _index: usize) -> & mut T`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **From**
  - `fn from(c: [T; 2]) -> Self`



## image::color::Rgb

*Struct*

RGB colors.

For the purpose of color conversion, as well as blending, the implementation of `Pixel`
assumes an `sRGB` color space of its data.

**Generic Parameters:**
- T

**Tuple Struct**: `([T; 3])`

**Traits:** PixelWithColorType, Eq, PixelWithColorType, Copy, PixelWithColorType

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Pixel**
  - `fn channels(self: &Self) -> &[T]`
  - `fn channels_mut(self: & mut Self) -> & mut [T]`
  - `fn alpha(self: &Self) -> <Self as >::Subpixel`
  - `fn channels4(self: &Self) -> (T, T, T, T)`
  - `fn from_channels(a: T, b: T, c: T, d: T) -> Rgb<T>`
  - `fn from_slice(slice: &[T]) -> &Rgb<T>`
  - `fn from_slice_mut(slice: & mut [T]) -> & mut Rgb<T>`
  - `fn to_rgb(self: &Self) -> Rgb<T>`
  - `fn to_rgba(self: &Self) -> Rgba<T>`
  - `fn to_luma(self: &Self) -> Luma<T>`
  - `fn to_luma_alpha(self: &Self) -> LumaA<T>`
  - `fn map<F>(self: &Self, f: F) -> Rgb<T>`
  - `fn apply<F>(self: & mut Self, f: F)`
  - `fn map_with_alpha<F, G>(self: &Self, f: F, g: G) -> Rgb<T>`
  - `fn apply_with_alpha<F, G>(self: & mut Self, f: F, g: G)`
  - `fn map2<F>(self: &Self, other: &Self, f: F) -> Rgb<T>`
  - `fn apply2<F>(self: & mut Self, other: &Rgb<T>, f: F)`
  - `fn invert(self: & mut Self)`
  - `fn blend(self: & mut Self, other: &Rgb<T>)`
- **Index**
  - `fn index(self: &Self, _index: usize) -> &T`
- **IndexMut**
  - `fn index_mut(self: & mut Self, _index: usize) -> & mut T`
- **From**
  - `fn from(c: [T; 3]) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## image::color::Rgba

*Struct*

RGB colors + alpha channel

**Generic Parameters:**
- T

**Tuple Struct**: `([T; 4])`

**Traits:** PixelWithColorType, Eq, PixelWithColorType, PixelWithColorType, Copy

**Trait Implementations:**

- **IndexMut**
  - `fn index_mut(self: & mut Self, _index: usize) -> & mut T`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **From**
  - `fn from(c: [T; 4]) -> Self`
- **Pixel**
  - `fn channels(self: &Self) -> &[T]`
  - `fn channels_mut(self: & mut Self) -> & mut [T]`
  - `fn alpha(self: &Self) -> <Self as >::Subpixel`
  - `fn channels4(self: &Self) -> (T, T, T, T)`
  - `fn from_channels(a: T, b: T, c: T, d: T) -> Rgba<T>`
  - `fn from_slice(slice: &[T]) -> &Rgba<T>`
  - `fn from_slice_mut(slice: & mut [T]) -> & mut Rgba<T>`
  - `fn to_rgb(self: &Self) -> Rgb<T>`
  - `fn to_rgba(self: &Self) -> Rgba<T>`
  - `fn to_luma(self: &Self) -> Luma<T>`
  - `fn to_luma_alpha(self: &Self) -> LumaA<T>`
  - `fn map<F>(self: &Self, f: F) -> Rgba<T>`
  - `fn apply<F>(self: & mut Self, f: F)`
  - `fn map_with_alpha<F, G>(self: &Self, f: F, g: G) -> Rgba<T>`
  - `fn apply_with_alpha<F, G>(self: & mut Self, f: F, g: G)`
  - `fn map2<F>(self: &Self, other: &Self, f: F) -> Rgba<T>`
  - `fn apply2<F>(self: & mut Self, other: &Rgba<T>, f: F)`
  - `fn invert(self: & mut Self)`
  - `fn blend(self: & mut Self, other: &Rgba<T>)`
- **Index**
  - `fn index(self: &Self, _index: usize) -> &T`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`



