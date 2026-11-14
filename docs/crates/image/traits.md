**image > traits**

# Module: traits

## Contents

**Traits**

- [`EncodableLayout`](#encodablelayout) - Types which are safe to treat as an immutable byte slice in a pixel layout
- [`Enlargeable`](#enlargeable) - An `Enlargable::Larger` value should be enough to calculate
- [`Lerp`](#lerp) - Linear interpolation without involving floating numbers.
- [`Pixel`](#pixel) - A generalized pixel.
- [`PixelWithColorType`](#pixelwithcolortype) - The pixel with an associated `ColorType`.
- [`Primitive`](#primitive) - The type of each channel in a pixel. For example, this can be `u8`, `u16`, `f32`.

---

## image::traits::EncodableLayout

*Trait*

Types which are safe to treat as an immutable byte slice in a pixel layout
for image encoding.

**Methods:**

- `as_bytes`: Get the bytes of this value.



## image::traits::Enlargeable

*Trait*

An `Enlargable::Larger` value should be enough to calculate
the sum (average) of a few hundred or thousand Enlargeable values.

**Methods:**

- `Larger`
- `clamp_from`
- `to_larger`



## image::traits::Lerp

*Trait*

Linear interpolation without involving floating numbers.

**Methods:**

- `Ratio`
- `lerp`



## image::traits::Pixel

*Trait*

A generalized pixel.

A pixel object is usually not used standalone but as a view into an image buffer.

**Methods:**

- `Subpixel`: The scalar type that is used to store each channel in this pixel.
- `CHANNEL_COUNT`: The number of channels of this pixel type.
- `channels`: Returns the components as a slice.
- `channels_mut`: Returns the components as a mutable slice
- `COLOR_MODEL`: A string that can help to interpret the meaning each channel
- `HAS_ALPHA`: Returns true if the alpha channel is contained.
- `alpha`: Retrieve the value of the alpha channel for this pixel.
- `channels4`: Returns the channels of this pixel as a 4 tuple. If the pixel
- `from_channels`: Construct a pixel from the 4 channels a, b, c and d.
- `from_slice`: Returns a view into a slice.
- `from_slice_mut`: Returns mutable view into a mutable slice.
- `to_rgb`: Convert this pixel to RGB
- `to_rgba`: Convert this pixel to RGB with an alpha channel
- `to_luma`: Convert this pixel to luma
- `to_luma_alpha`: Convert this pixel to luma with an alpha channel
- `map`: Apply the function ```f``` to each channel of this pixel.
- `apply`: Apply the function ```f``` to each channel of this pixel.
- `map_with_alpha`: Apply the function ```f``` to each channel except the alpha channel.
- `apply_with_alpha`: Apply the function ```f``` to each channel except the alpha channel.
- `map_without_alpha`: Apply the function ```f``` to each channel except the alpha channel.
- `apply_without_alpha`: Apply the function ```f``` to each channel except the alpha channel.
- `map2`: Apply the function ```f``` to each channel of this pixel and
- `apply2`: Apply the function ```f``` to each channel of this pixel and
- `invert`: Invert this pixel
- `blend`: Blend the color of a given pixel into ourself, taking into account alpha channels



## image::traits::PixelWithColorType

*Trait*

The pixel with an associated `ColorType`.
Not all possible pixels represent one of the predefined `ColorType`s.

**Methods:**

- `COLOR_TYPE`: This pixel has the format of one of the predefined `ColorType`s,



## image::traits::Primitive

*Trait*

The type of each channel in a pixel. For example, this can be `u8`, `u16`, `f32`.

**Methods:**

- `DEFAULT_MAX_VALUE`: The maximum value for this type of primitive within the context of color.
- `DEFAULT_MIN_VALUE`: The minimum value for this type of primitive within the context of color.



