**image > images**

# Module: images

## Contents

**Modules**

- [`flat`](#flat) - Image representations for ffi.

---

## Module: flat

Image representations for ffi.

# Usage

Imagine you want to offer a very simple ffi interface: The caller provides an image buffer and
your program creates a thumbnail from it and dumps that image as `png`. This module is designed
to help you transition from raw memory data to Rust representation.

```no_run
use std::ptr;
use std::slice;
use image::Rgb;
use image::flat::{FlatSamples, SampleLayout};
use image::imageops::thumbnail;

#[no_mangle]
pub extern "C" fn store_rgb8_compressed(
    data: *const u8, len: usize,
    layout: *const SampleLayout
)
    -> bool
{
    let samples = unsafe { slice::from_raw_parts(data, len) };
    let layout = unsafe { ptr::read(layout) };

    let buffer = FlatSamples {
        samples,
        layout,
        color_hint: None,
    };

    let view = match buffer.as_view::<Rgb<u8>>() {
        Err(_) => return false, // Invalid layout.
        Ok(view) => view,
    };

    thumbnail(&view, 64, 64)
        .save("output.png")
        .map(|_| true)
        .unwrap_or_else(|_| false)
}
```




