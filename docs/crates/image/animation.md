**image > animation**

# Module: animation

## Contents

**Structs**

- [`Delay`](#delay) - The delay of a frame relative to the previous one.
- [`Frame`](#frame) - A single animation frame
- [`Frames`](#frames) - An implementation dependent iterator, reading the frames as requested

---

## image::animation::Delay

*Struct*

The delay of a frame relative to the previous one.

The ratio is reduced on construction which means equality comparisons is reliable even when
mixing different bases. Note however that there is an upper limit to the delays that can be
represented exactly when using [`Self::from_saturating_duration`] which depends on the
granularity of the interval.

```
use image::Delay;
let delay_10ms = Delay::from_numer_denom_ms(10, 1);
let delay_10000us = Delay::from_numer_denom_ms(10_000, 1_000);

assert_eq!(delay_10ms, delay_10000us);
```

**Methods:**

- `fn from_numer_denom_ms(numerator: u32, denominator: u32) -> Self` - Create a delay from a ratio of milliseconds.
- `fn from_saturating_duration(duration: Duration) -> Self` - Convert from a duration, clamped between 0 and an implemented defined maximum.
- `fn numer_denom_ms(self: Self) -> (u32, u32)` - The numerator and denominator of the delay in milliseconds.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`



## image::animation::Frame

*Struct*

A single animation frame

**Methods:**

- `fn new(buffer: RgbaImage) -> Frame` - Constructs a new frame without any delay.
- `fn from_parts(buffer: RgbaImage, left: u32, top: u32, delay: Delay) -> Frame` - Constructs a new frame
- `fn delay(self: &Self) -> Delay` - Delay of this frame
- `fn buffer(self: &Self) -> &RgbaImage` - Returns the image buffer
- `fn buffer_mut(self: & mut Self) -> & mut RgbaImage` - Returns a mutable image buffer
- `fn into_buffer(self: Self) -> RgbaImage` - Returns the image buffer
- `fn left(self: &Self) -> u32` - Returns the x offset
- `fn top(self: &Self) -> u32` - Returns the y offset

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
  - `fn clone_from(self: & mut Self, source: &Self)`



## image::animation::Frames

*Struct*

An implementation dependent iterator, reading the frames as requested

**Generic Parameters:**
- 'a

**Methods:**

- `fn new(iterator: Box<dyn Iterator>) -> Self` - Creates a new `Frames` from an implementation specific iterator.
- `fn collect_frames(self: Self) -> ImageResult<Vec<Frame>>` - Steps through the iterator from the current frame until the end and pushes each frame into

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<ImageResult<Frame>>`



