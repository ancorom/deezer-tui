**image > codecs**

# Module: codecs

## Contents

**Modules**

- [`avif`](#avif) - Encoding of AVIF images.
- [`bmp`](#bmp) - Decoding and Encoding of BMP Images
- [`dds`](#dds) - Decoding of DDS images
- [`farbfeld`](#farbfeld) - Decoding of farbfeld images
- [`gif`](#gif) -  Decoding of GIF Images
- [`hdr`](#hdr) - Decoding of Radiance HDR Images
- [`ico`](#ico) - Decoding and Encoding of ICO files
- [`jpeg`](#jpeg) - Decoding and Encoding of JPEG Images
- [`openexr`](#openexr) - Decoding of OpenEXR (.exr) Images
- [`png`](#png) - Decoding and Encoding of PNG Images
- [`pnm`](#pnm) - Decoding of netpbm image formats (pbm, pgm, ppm and pam).
- [`qoi`](#qoi) - Decoding and encoding of QOI images
- [`tga`](#tga) - Decoding of TGA Images
- [`tiff`](#tiff) - Decoding and Encoding of TIFF Images
- [`webp`](#webp) - Decoding and Encoding of WebP Images

---

## Module: avif

Encoding of AVIF images.



## Module: bmp

Decoding and Encoding of BMP Images

A decoder and encoder for BMP (Windows Bitmap) images

# Related Links
* <https://msdn.microsoft.com/en-us/library/windows/desktop/dd183375%28v=vs.85%29.aspx>
* <https://en.wikipedia.org/wiki/BMP_file_format>



## Module: dds

Decoding of DDS images

DDS (DirectDraw Surface) is a container format for storing DXT (S3TC) compressed images.

# Related Links
* <https://docs.microsoft.com/en-us/windows/win32/direct3ddds/dx-graphics-dds-pguide> - Description of the DDS format.



## Module: farbfeld

Decoding of farbfeld images

farbfeld is a lossless image format which is easy to parse, pipe and compress.

It has the following format:

| Bytes  | Description                                             |
|--------|---------------------------------------------------------|
| 8      | "farbfeld" magic value                                  |
| 4      | 32-Bit BE unsigned integer (width)                      |
| 4      | 32-Bit BE unsigned integer (height)                     |
| [2222] | 4⋅16-Bit BE unsigned integers [RGBA] / pixel, row-major |

The RGB-data should be sRGB for best interoperability and not alpha-premultiplied.

# Related Links
* <https://tools.suckless.org/farbfeld/> - the farbfeld specification



## Module: gif

 Decoding of GIF Images

 GIF (Graphics Interchange Format) is an image format that supports lossless compression.

 # Related Links
 * <http://www.w3.org/Graphics/GIF/spec-gif89a.txt> - The GIF Specification

# Examples
```rust,no_run
use image::codecs::gif::{GifDecoder, GifEncoder};
use image::{ImageDecoder, AnimationDecoder};
use std::fs::File;
use std::io::BufReader;
# fn main() -> std::io::Result<()> {
// Decode a gif into frames
let file_in = BufReader::new(File::open("foo.gif")?);
let mut decoder = GifDecoder::new(file_in).unwrap();
let frames = decoder.into_frames();
let frames = frames.collect_frames().expect("error decoding gif");

// Encode frames into a gif and save to a file
let mut file_out = File::open("out.gif")?;
let mut encoder = GifEncoder::new(file_out);
encoder.encode_frames(frames.into_iter());
# Ok(())
# }
```



## Module: hdr

Decoding of Radiance HDR Images

A decoder for Radiance HDR images

# Related Links

* <http://radsite.lbl.gov/radiance/refer/filefmts.pdf>
* <http://www.graphics.cornell.edu/~bjw/rgbe/rgbe.c>



## Module: ico

Decoding and Encoding of ICO files

A decoder and encoder for ICO (Windows Icon) image container files.

# Related Links
* <https://msdn.microsoft.com/en-us/library/ms997538.aspx>
* <https://en.wikipedia.org/wiki/ICO_%28file_format%29>



## Module: jpeg

Decoding and Encoding of JPEG Images

JPEG (Joint Photographic Experts Group) is an image format that supports lossy compression.
This module implements the Baseline JPEG standard.

# Related Links
* <http://www.w3.org/Graphics/JPEG/itu-t81.pdf> - The JPEG specification



## Module: openexr

Decoding of OpenEXR (.exr) Images

OpenEXR is an image format that is widely used, especially in VFX,
because it supports lossless and lossy compression for float data.

This decoder only supports RGB and RGBA images.
If an image does not contain alpha information,
it is defaulted to `1.0` (no transparency).

# Related Links
* <https://www.openexr.com/documentation.html> - The OpenEXR reference.


Current limitations (July 2021):
    - only pixel type `Rgba32F` and `Rgba16F` are supported
    - only non-deep rgb/rgba files supported, no conversion from/to YCbCr or similar
    - only the first non-deep rgb layer is used
    - only the largest mip map level is used
    - pixels outside display window are lost
    - meta data is lost
    - dwaa/dwab compressed images not supported yet by the exr library
    - (chroma) subsampling not supported yet by the exr library



## Module: png

Decoding and Encoding of PNG Images

PNG (Portable Network Graphics) is an image format that supports lossless compression.

# Related Links
* <http://www.w3.org/TR/PNG/> - The PNG Specification



## Module: pnm

Decoding of netpbm image formats (pbm, pgm, ppm and pam).

The formats pbm, pgm and ppm are fully supported. Only the official subformats
(`BLACKANDWHITE`, `GRAYSCALE`, `RGB`, `BLACKANDWHITE_ALPHA`, `GRAYSCALE_ALPHA`,
and `RGB_ALPHA`) of pam are supported; custom tuple types have no clear
interpretation as an image and will be rejected.



## Module: qoi

Decoding and encoding of QOI images



## Module: tga

Decoding of TGA Images

# Related Links
<http://googlesites.inequation.org/tgautilities>



## Module: tiff

Decoding and Encoding of TIFF Images

TIFF (Tagged Image File Format) is a versatile image format that supports
lossless and lossy compression.

# Related Links
* <http://partners.adobe.com/public/developer/tiff/index.html> - The TIFF specification



## Module: webp

Decoding and Encoding of WebP Images



