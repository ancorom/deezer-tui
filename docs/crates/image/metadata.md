**image > metadata**

# Module: metadata

## Contents

**Enums**

- [`LoopCount`](#loopcount) - The number of times animated image should loop over.
- [`Orientation`](#orientation) - Describes the transformations to be applied to the image.

---

## image::metadata::LoopCount

*Enum*

The number of times animated image should loop over.

**Variants:**
- `Infinite` - Loop the image Infinitely
- `Finite(std::num::NonZeroU32)` - Loop the image within Finite times.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`



## image::metadata::Orientation

*Enum*

Describes the transformations to be applied to the image.
Compatible with [Exif orientation](https://web.archive.org/web/20200412005226/https://www.impulseadventure.com/photo/exif-orientation.html).

Orientation is specified in the file's metadata, and is often written by cameras.

You can apply it to an image via [`DynamicImage::apply_orientation`](crate::DynamicImage::apply_orientation).

**Variants:**
- `NoTransforms` - Do not perform any transformations.
- `Rotate90` - Rotate by 90 degrees clockwise.
- `Rotate180` - Rotate by 180 degrees. Can be performed in-place.
- `Rotate270` - Rotate by 270 degrees clockwise. Equivalent to rotating by 90 degrees counter-clockwise.
- `FlipHorizontal` - Flip horizontally. Can be performed in-place.
- `FlipVertical` - Flip vertically. Can be performed in-place.
- `Rotate90FlipH` - Rotate by 90 degrees clockwise and flip horizontally.
- `Rotate270FlipH` - Rotate by 270 degrees clockwise and flip horizontally.

**Methods:**

- `fn from_exif(exif_orientation: u8) -> Option<Self>` - Converts from [Exif orientation](https://web.archive.org/web/20200412005226/https://www.impulseadventure.com/photo/exif-orientation.html)
- `fn to_exif(self: Self) -> u8` - Converts into [Exif orientation](https://web.archive.org/web/20200412005226/https://www.impulseadventure.com/photo/exif-orientation.html)
- `fn from_exif_chunk(chunk: &[u8]) -> Option<Self>` - Extracts the image orientation from a raw Exif chunk.
- `fn remove_from_exif_chunk(chunk: & mut [u8]) -> Option<Self>` - Extracts the image orientation from a raw Exif chunk and sets the orientation in the Exif chunk to `Orientation::NoTransforms`.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



