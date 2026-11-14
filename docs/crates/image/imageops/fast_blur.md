**image > imageops > fast_blur**

# Module: imageops::fast_blur

## Contents

**Functions**

- [`fast_blur`](#fast_blur) - Approximation of Gaussian blur.

---

## image::imageops::fast_blur::fast_blur

*Function*

Approximation of Gaussian blur.

# Arguments

* `image_buffer` - source image.
* `sigma` - value controls image flattening level.

This method assumes alpha pre-multiplication for images that contain non-constant alpha.

This method typically assumes that the input is scene-linear light.
If it is not, color distortion may occur.

Source: Kovesi, P.:  Fast Almost-Gaussian Filtering The Australian Pattern
Recognition Society Conference: DICTA 2010. December 2010. Sydney.

```rust
fn fast_blur<P>(input_buffer: &crate::ImageBuffer<P, Vec<<P as >::Subpixel>>, sigma: f32) -> crate::ImageBuffer<P, Vec<<P as >::Subpixel>>
```



