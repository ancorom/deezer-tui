**image > metadata > cicp > CicpTransform**

# Module: metadata::cicp::CicpTransform

## Contents

**Functions**

- [`new`](#new) - Construct a transform between two color spaces.

---

## image::metadata::cicp::CicpTransform::new

*Function*

Construct a transform between two color spaces.

Returns `Some` if the transform is guaranteed to be supported by `image`. Both color spaces
are well understood and can be expected to be supported in future versions. However, we do
not make guarantees about adjusting the rounding modes, accuracy, and exact numeric values
used in the transform. Also, out-of-gamut colors may be handled differently per API.

Returns `None` if the transformation is not (yet) supported.

This is used with [`ConvertColorOptions`][`crate::ConvertColorOptions`] in
[`ImageBuffer::copy_from_color_space`][`crate::ImageBuffer::copy_from_color_space`],
[`DynamicImage::copy_from_color_space`][`DynamicImage::copy_from_color_space`].

```rust
fn new(from: Cicp, into: Cicp) -> Option<Self>
```



