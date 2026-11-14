**image > io > format**

# Module: io::format

## Contents

**Enums**

- [`ImageFormat`](#imageformat) - An enumeration of supported image formats.

---

## image::io::format::ImageFormat

*Enum*

An enumeration of supported image formats.
Not all formats support both encoding and decoding.

**Variants:**
- `Png` - An Image in PNG Format
- `Jpeg` - An Image in JPEG Format
- `Gif` - An Image in GIF Format
- `WebP` - An Image in WEBP Format
- `Pnm` - An Image in general PNM Format
- `Tiff` - An Image in TIFF Format
- `Tga` - An Image in TGA Format
- `Dds` - An Image in DDS Format
- `Bmp` - An Image in BMP Format
- `Ico` - An Image in ICO Format
- `Hdr` - An Image in Radiance HDR Format
- `OpenExr` - An Image in OpenEXR Format
- `Farbfeld` - An Image in farbfeld Format
- `Avif` - An Image in AVIF Format
- `Qoi` - An Image in QOI Format

**Methods:**

- `fn from_extension<S>(ext: S) -> Option<Self>` - Return the image format specified by a path's file extension.
- `fn from_path<P>(path: P) -> ImageResult<Self>` - Return the image format specified by the path's file extension.
- `fn from_mime_type<M>(mime_type: M) -> Option<Self>` - Return the image format specified by a MIME type.
- `fn to_mime_type(self: &Self) -> &'static str` - Return the MIME type for this image format or "application/octet-stream" if no MIME type
- `fn can_read(self: &Self) -> bool` - Return if the `ImageFormat` can be decoded by the lib.
- `fn can_write(self: &Self) -> bool` - Return if the `ImageFormat` can be encoded by the lib.
- `fn extensions_str(self: Self) -> &'static [&'static str]` - Return a list of applicable extensions for this format.
- `fn reading_enabled(self: &Self) -> bool` - Return the `ImageFormat`s which are enabled for reading.
- `fn writing_enabled(self: &Self) -> bool` - Return the `ImageFormat`s which are enabled for writing.
- `fn all() -> impl Trait` - Return all `ImageFormat`s

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



