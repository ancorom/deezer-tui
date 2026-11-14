**image > io > image_reader_type**

# Module: io::image_reader_type

## Contents

**Structs**

- [`ImageReader`](#imagereader) - A multi-format image reader.

---

## image::io::image_reader_type::ImageReader

*Struct*

A multi-format image reader.

Wraps an input reader to facilitate automatic detection of an image's format, appropriate
decoding method, and dispatches into the set of supported [`ImageDecoder`] implementations.

## Usage

Opening a file, deducing the format based on the file path automatically, and trying to decode
the image contained can be performed by constructing the reader and immediately consuming it.

```no_run
# use image::ImageError;
# use image::ImageReader;
# fn main() -> Result<(), ImageError> {
let image = ImageReader::open("path/to/image.png")?
    .decode()?;
# Ok(()) }
```

It is also possible to make a guess based on the content. This is especially handy if the
source is some blob in memory and you have constructed the reader in another way. Here is an
example with a `pnm` black-and-white subformat that encodes its pixel matrix with ascii values.

```
# use image::ImageError;
# use image::ImageReader;
# fn main() -> Result<(), ImageError> {
use std::io::Cursor;
use image::ImageFormat;

let raw_data = b"P1 2 2\n\
    0 1\n\
    1 0\n";

let mut reader = ImageReader::new(Cursor::new(raw_data))
    .with_guessed_format()
    .expect("Cursor io never fails");
assert_eq!(reader.format(), Some(ImageFormat::Pnm));

# #[cfg(feature = "pnm")]
let image = reader.decode()?;
# Ok(()) }
```

As a final fallback or if only a specific format must be used, the reader always allows manual
specification of the supposed image format with [`set_format`].

[`set_format`]: #method.set_format
[`ImageDecoder`]: ../trait.ImageDecoder.html

**Generic Parameters:**
- R

**Methods:**

- `fn new(buffered_reader: R) -> Self` - Create a new image reader without a preset format.
- `fn with_format(buffered_reader: R, format: ImageFormat) -> Self` - Construct a reader with specified format.
- `fn format(self: &Self) -> Option<ImageFormat>` - Get the currently determined format.
- `fn set_format(self: & mut Self, format: ImageFormat)` - Supply the format as which to interpret the read image.
- `fn clear_format(self: & mut Self)` - Remove the current information on the image format.
- `fn no_limits(self: & mut Self)` - Disable all decoding limits.
- `fn limits(self: & mut Self, limits: Limits)` - Set a custom set of decoding limits.
- `fn into_inner(self: Self) -> R` - Unwrap the reader.
- `fn into_decoder(self: Self) -> ImageResult<impl Trait>` - Convert the reader into a decoder.
- `fn with_guessed_format(self: Self) -> io::Result<Self>` - Make a format guess based on the content, replacing it on success.
- `fn into_dimensions(self: Self) -> ImageResult<(u32, u32)>` - Read the image dimensions.
- `fn decode(self: Self) -> ImageResult<DynamicImage>` - Read the image (replaces `load`).
- `fn open<P>(path: P) -> io::Result<Self>` - Open a file to read, format will be guessed from path.



