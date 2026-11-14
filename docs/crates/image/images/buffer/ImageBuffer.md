**image > images > buffer > ImageBuffer**

# Module: images::buffer::ImageBuffer

## Contents

**Functions**

- [`copy_from_color_space`](#copy_from_color_space) - Copy pixel data from one buffer to another, calculating equivalent color representations

---

## image::images::buffer::ImageBuffer::copy_from_color_space

*Function*

Copy pixel data from one buffer to another, calculating equivalent color representations
for the target's color space.

Returns `Ok` if:
- Both images to have the same dimensions, otherwise returns a [`ImageError::Parameter`].
- The primaries and transfer functions of both image's color spaces must be supported,
  otherwise returns a [`ImageError::Unsupported`].
- The pixel's channel layout must be supported for conversion, otherwise returns a
  [`ImageError::Unsupported`]. You can rely on RGB and RGBA always being supported. If a
  layout is supported for one color space it is supported for all of them.

To copy color data of arbitrary channel layouts use `DynamicImage` with the overhead of
having data converted into and from RGB representation.

```rust
fn copy_from_color_space<FromType, D>(self: & mut Self, from: &ImageBuffer<FromType, D>, options: ConvertColorOptions) -> ImageResult<()>
```



