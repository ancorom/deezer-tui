**image**

# Module: image

## Contents

**Modules**

- [`buffer`](#buffer) - Iterators and other auxiliary structure for the `ImageBuffer` type.
- [`codecs`](#codecs) - Encoding and decoding for various image file formats.
- [`error`](#error) - Contains detailed error representation.
- [`hooks`](#hooks) - This module provides a way to register decoding hooks for image formats not directly supported
- [`imageops`](#imageops) - Image Processing Functions
- [`io`](#io) - Deprecated io module the original io module has been renamed to `image_reader`.
- [`math`](#math) - Mathematical helper functions and types.
- [`metadata`](#metadata) - Types describing image metadata

---

## Module: buffer

Iterators and other auxiliary structure for the `ImageBuffer` type.



## Module: codecs

Encoding and decoding for various image file formats.

# Supported formats

| Feature | Format   | Notes
| ------- | -------- | -----
| `avif`  | AVIF     | Decoding requires the `avif-native` feature, uses the libdav1d C library.
| `bmp`   | BMP      |
| `dds`   | DDS      | Only decoding is supported.
| `exr`   | OpenEXR  |
| `ff`    | Farbfeld |
| `gif`   | GIF      |
| `hdr`   | HDR      |
| `ico`   | ICO      |
| `jpeg`  | JPEG     |
| `png`   | PNG      |
| `pnm`   | PNM      |
| `qoi`   | QOI      |
| `tga`   | TGA      |
| `tiff`  | TIFF     |
| `webp`  | WebP     | Only lossless encoding is currently supported.

## A note on format specific features

One of the main goals of `image` is stability, in runtime but also for programmers. This
ensures that performance as well as safety fixes reach a majority of its user base with little
effort. Re-exporting all details of its dependencies would run counter to this goal as it
linked _all_ major version bumps between them and `image`. As such, we are wary of exposing too
many details, or configuration options, that are not shared between different image formats.

Nevertheless, the advantage of precise control is hard to ignore. We will thus consider
_wrappers_, not direct re-exports, in either of the following cases:

1. A standard specifies that configuration _x_ is required for decoders/encoders and there
   exists an essentially canonical way to control it.
2. At least two different implementations agree on some (sub-)set of features in practice.
3. A technical argument including measurements of the performance, space benefits, or otherwise
   objectively quantified benefits can be made, and the added interface is unlikely to require
   breaking changes.

Features that fulfill two or more criteria are preferred.

Re-exports of dependencies that reach version `1` will be discussed when it happens.



## Module: error

Contains detailed error representation.

See the main [`ImageError`] which contains a variant for each specialized error type. The
subtypes used in each variant are opaque by design. They can be roughly inspected through their
respective `kind` methods which work similar to `std::io::Error::kind`.

The error interface makes it possible to inspect the error of an underlying decoder or encoder,
through the `Error::source` method. Note that this is not part of the stable interface and you
may not rely on a particular error value for a particular operation. This means mainly that
`image` does not promise to remain on a particular version of its underlying decoders but if
you ensure to use the same version of the dependency (or at least of the error type) through
external means then you could inspect the error type in slightly more detail.

[`ImageError`]: enum.ImageError.html



## Module: hooks

This module provides a way to register decoding hooks for image formats not directly supported
by this crate.



## Module: imageops

Image Processing Functions



## Module: io

Deprecated io module the original io module has been renamed to `image_reader`.
This is going to be internal.
Input and output of images.



## Module: math

Mathematical helper functions and types.



## Module: metadata

Types describing image metadata



