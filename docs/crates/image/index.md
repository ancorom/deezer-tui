# image

# Overview

This crate provides native rust implementations of image encoding and decoding as well as some
basic image manipulation functions. Additional documentation can currently also be found in the
[README.md file which is most easily viewed on
github](https://github.com/image-rs/image/blob/main/README.md).

There are two core problems for which this library provides solutions: a unified interface for image
encodings and simple generic buffers for their content. It's possible to use either feature
without the other. The focus is on a small and stable set of common operations that can be
supplemented by other specialized crates. The library also prefers safe solutions with few
dependencies.

# High level API

Load images using [`ImageReader`](crate::ImageReader):

```rust,no_run
use std::io::Cursor;
use image::ImageReader;
# fn main() -> Result<(), image::ImageError> {
# let bytes = vec![0u8];

let img = ImageReader::open("myimage.png")?.decode()?;
let img2 = ImageReader::new(Cursor::new(bytes)).with_guessed_format()?.decode()?;
# Ok(())
# }
```

And save them using [`save`] or [`write_to`] methods:

```rust,no_run
# use std::io::{Write, Cursor};
# use image::{DynamicImage, ImageFormat};
# #[cfg(feature = "png")]
# fn main() -> Result<(), image::ImageError> {
# let img: DynamicImage = unimplemented!();
# let img2: DynamicImage = unimplemented!();
img.save("empty.jpg")?;

let mut bytes: Vec<u8> = Vec::new();
img2.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)?;
# Ok(())
# }
# #[cfg(not(feature = "png"))] fn main() {}
```

With default features, the crate includes support for [many common image formats](codecs/index.html#supported-formats).

[`save`]: enum.DynamicImage.html#method.save
[`write_to`]: enum.DynamicImage.html#method.write_to
[`ImageReader`]: struct.Reader.html

# Image buffers

The two main types for storing images:
* [`ImageBuffer`] which holds statically typed image contents.
* [`DynamicImage`] which is an enum over the supported `ImageBuffer` formats
  and supports conversions between them.

As well as a few more specialized options:
* [`GenericImage`] trait for a mutable image buffer.
* [`GenericImageView`] trait for read only references to a `GenericImage`.
* [`flat`] module containing types for interoperability with generic channel
  matrices and foreign interfaces.

[`GenericImageView`]: trait.GenericImageView.html
[`GenericImage`]: trait.GenericImage.html
[`ImageBuffer`]: struct.ImageBuffer.html
[`DynamicImage`]: enum.DynamicImage.html
[`flat`]: flat/index.html

# Low level encoding/decoding API

Implementations of [`ImageEncoder`] provides low level control over encoding:
```rust,no_run
# use std::io::Write;
# use image::DynamicImage;
# use image::ImageEncoder;
# #[cfg(feature = "jpeg")]
# fn main() -> Result<(), image::ImageError> {
# use image::codecs::jpeg::JpegEncoder;
# let img: DynamicImage = unimplemented!();
# let writer: Box<dyn Write> = unimplemented!();
let encoder = JpegEncoder::new_with_quality(&mut writer, 95);
img.write_with_encoder(encoder)?;
# Ok(())
# }
# #[cfg(not(feature = "jpeg"))] fn main() {}
```
While [`ImageDecoder`] and [`ImageDecoderRect`] give access to more advanced decoding options:

```rust,no_run
# use std::io::{BufReader, Cursor};
# use image::DynamicImage;
# use image::ImageDecoder;
# #[cfg(feature = "png")]
# fn main() -> Result<(), image::ImageError> {
# use image::codecs::png::PngDecoder;
# let img: DynamicImage = unimplemented!();
# let reader: BufReader<Cursor<&[u8]>> = unimplemented!();
let decoder = PngDecoder::new(&mut reader)?;
let icc = decoder.icc_profile();
let img = DynamicImage::from_decoder(decoder)?;
# Ok(())
# }
# #[cfg(not(feature = "png"))] fn main() {}
```

[`DynamicImage::from_decoder`]: enum.DynamicImage.html#method.from_decoder
[`ImageDecoderRect`]: trait.ImageDecoderRect.html
[`ImageDecoder`]: trait.ImageDecoder.html
[`ImageEncoder`]: trait.ImageEncoder.html

## Modules

### [`image`](image.md)

*8 modules*

### [`animation`](animation.md)

*3 structs*

### [`animation::Delay`](animation/Delay.md)

*1 function*

### [`codecs`](codecs.md)

*15 modules*

### [`codecs::avif::encoder`](codecs/avif/encoder.md)

*1 enum, 1 struct*

### [`codecs::bmp::decoder`](codecs/bmp/decoder.md)

*1 struct*

### [`codecs::bmp::encoder`](codecs/bmp/encoder.md)

*1 struct*

### [`codecs::dds`](codecs/dds.md)

*1 struct*

### [`codecs::farbfeld`](codecs/farbfeld.md)

*3 structs*

### [`codecs::gif`](codecs/gif.md)

*1 enum, 3 structs*

### [`codecs::hdr::decoder`](codecs/hdr/decoder.md)

*1 constant, 2 structs*

### [`codecs::hdr::encoder`](codecs/hdr/encoder.md)

*1 struct*

### [`codecs::ico::decoder`](codecs/ico/decoder.md)

*1 struct*

### [`codecs::ico::encoder`](codecs/ico/encoder.md)

*2 structs*

### [`codecs::jpeg::decoder`](codecs/jpeg/decoder.md)

*1 struct*

### [`codecs::jpeg::encoder`](codecs/jpeg/encoder.md)

*1 enum, 2 structs*

### [`codecs::openexr`](codecs/openexr.md)

*2 structs*

### [`codecs::png`](codecs/png.md)

*2 enums, 3 structs*

### [`codecs::pnm::decoder`](codecs/pnm/decoder.md)

*1 struct*

### [`codecs::pnm::encoder`](codecs/pnm/encoder.md)

*1 enum, 1 struct*

### [`codecs::pnm::header`](codecs/pnm/header.md)

*3 enums, 5 structs*

### [`codecs::qoi`](codecs/qoi.md)

*2 structs*

### [`codecs::tga::decoder`](codecs/tga/decoder.md)

*1 struct*

### [`codecs::tga::encoder`](codecs/tga/encoder.md)

*1 struct*

### [`codecs::tiff`](codecs/tiff.md)

*3 structs*

### [`codecs::webp::decoder`](codecs/webp/decoder.md)

*1 struct*

### [`codecs::webp::encoder`](codecs/webp/encoder.md)

*1 struct*

### [`color`](color.md)

*2 enums, 2 traits, 4 structs*

### [`error`](error.md)

*1 type alias, 5 enums, 6 structs*

### [`hooks`](hooks.md)

*1 struct, 1 type alias, 3 functions*

### [`imageops`](imageops.md)

*1 module, 8 functions*

### [`imageops::affine`](imageops/affine.md)

*13 functions*

### [`imageops::colorops`](imageops/colorops.md)

*1 struct, 1 trait, 13 functions*

### [`imageops::fast_blur`](imageops/fast_blur.md)

*1 function*

### [`imageops::sample`](imageops/sample.md)

*1 enum, 1 struct, 10 functions*

### [`images`](images.md)

*1 module*

### [`images::buffer`](images/buffer.md)

*1 trait, 10 structs, 6 type aliases*

### [`images::buffer::ImageBuffer`](images/buffer/ImageBuffer.md)

*1 function*

### [`images::buffer_par`](images/buffer_par.md)

*4 structs*

### [`images::dynimage`](images/dynimage.md)

*1 enum, 5 functions*

### [`images::dynimage::DynamicImage`](images/dynimage/DynamicImage.md)

*7 functions*

### [`images::flat`](images/flat.md)

*2 enums, 2 type aliases, 4 structs*

### [`images::generic_image`](images/generic_image.md)

*1 struct, 2 traits*

### [`images::sub_image`](images/sub_image.md)

*2 structs*

### [`images::sub_image::SubImage`](images/sub_image/SubImage.md)

*1 function*

### [`io`](io.md)

*3 type aliases*

### [`io::decoder`](io/decoder.md)

*3 traits*

### [`io::encoder`](io/encoder.md)

*1 struct, 1 trait*

### [`io::format`](io/format.md)

*1 enum*

### [`io::free_functions`](io/free_functions.md)

*4 functions*

### [`io::image_reader_type`](io/image_reader_type.md)

*1 struct*

### [`io::image_reader_type::ImageReader`](io/image_reader_type/ImageReader.md)

*1 function*

### [`io::limits`](io/limits.md)

*2 structs*

### [`math::rect`](math/rect.md)

*1 struct*

### [`metadata`](metadata.md)

*2 enums*

### [`metadata::cicp`](metadata/cicp.md)

*2 structs, 4 enums*

### [`metadata::cicp::CicpTransform`](metadata/cicp/CicpTransform.md)

*1 function*

### [`traits`](traits.md)

*6 traits*

### [`traits::private`](traits/private.md)

*1 enum, 1 struct, 1 trait*

### [`traits::seals`](traits/seals.md)

*1 trait*

