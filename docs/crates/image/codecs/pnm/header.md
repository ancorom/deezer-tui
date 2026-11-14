**image > codecs > pnm > header**

# Module: codecs::pnm::header

## Contents

**Structs**

- [`ArbitraryHeader`](#arbitraryheader) - Header produced by a `pam` file ("Portable Arbitrary Map")
- [`BitmapHeader`](#bitmapheader) - Header produced by a `pbm` file ("Portable Bit Map")
- [`GraymapHeader`](#graymapheader) - Header produced by a `pgm` file ("Portable Gray Map")
- [`PixmapHeader`](#pixmapheader) - Header produced by a `ppm` file ("Portable Pixel Map")
- [`PnmHeader`](#pnmheader) - Stores the complete header data of a file.

**Enums**

- [`ArbitraryTuplType`](#arbitrarytupltype) - Standardized tuple type specifiers in the header of a `pam`.
- [`PnmSubtype`](#pnmsubtype) - Denotes the category of the magic number
- [`SampleEncoding`](#sampleencoding) - The kind of encoding used to store sample values

---

## image::codecs::pnm::header::ArbitraryHeader

*Struct*

Header produced by a `pam` file ("Portable Arbitrary Map")

**Fields:**
- `height: u32` - Height of the image file
- `width: u32` - Width of the image file
- `depth: u32` - Number of color channels
- `maxval: u32` - Maximum sample value within the image
- `tupltype: Option<ArbitraryTuplType>` - Color interpretation of image pixels

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## image::codecs::pnm::header::ArbitraryTuplType

*Enum*

Standardized tuple type specifiers in the header of a `pam`.

**Variants:**
- `BlackAndWhite` - Pixels are either black (0) or white (1)
- `BlackAndWhiteAlpha` - Pixels are either black (0) or white (1) and a second alpha channel
- `Grayscale` - Pixels represent the amount of white
- `GrayscaleAlpha` - Grayscale with an additional alpha channel
- `RGB` - Three channels: Red, Green, Blue
- `RGBAlpha` - Four channels: Red, Green, Blue, Alpha
- `Custom(String)` - An image format which is not standardized

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## image::codecs::pnm::header::BitmapHeader

*Struct*

Header produced by a `pbm` file ("Portable Bit Map")

**Fields:**
- `encoding: SampleEncoding` - Binary or Ascii encoded file
- `height: u32` - Height of the image file
- `width: u32` - Width of the image file

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## image::codecs::pnm::header::GraymapHeader

*Struct*

Header produced by a `pgm` file ("Portable Gray Map")

**Fields:**
- `encoding: SampleEncoding` - Binary or Ascii encoded file
- `height: u32` - Height of the image file
- `width: u32` - Width of the image file
- `maxwhite: u32` - Maximum sample value within the image

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## image::codecs::pnm::header::PixmapHeader

*Struct*

Header produced by a `ppm` file ("Portable Pixel Map")

**Fields:**
- `encoding: SampleEncoding` - Binary or Ascii encoded file
- `height: u32` - Height of the image file
- `width: u32` - Width of the image file
- `maxval: u32` - Maximum sample value within the image

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## image::codecs::pnm::header::PnmHeader

*Struct*

Stores the complete header data of a file.

Internally, provides mechanisms for lossless reencoding. After reading a file with the decoder
it is possible to recover the header and construct an encoder. Using the encoder on the just
loaded image should result in a byte copy of the original file (for single image pnms without
additional trailing data).

**Methods:**

- `fn subtype(self: &Self) -> PnmSubtype` - Retrieve the format subtype from which the header was created.
- `fn width(self: &Self) -> u32` - The width of the image this header is for.
- `fn height(self: &Self) -> u32` - The height of the image this header is for.
- `fn maximal_sample(self: &Self) -> u32` - The biggest value a sample can have. In other words, the colour resolution.
- `fn as_bitmap(self: &Self) -> Option<&BitmapHeader>` - Retrieve the underlying bitmap header if any
- `fn as_graymap(self: &Self) -> Option<&GraymapHeader>` - Retrieve the underlying graymap header if any
- `fn as_pixmap(self: &Self) -> Option<&PixmapHeader>` - Retrieve the underlying pixmap header if any
- `fn as_arbitrary(self: &Self) -> Option<&ArbitraryHeader>` - Retrieve the underlying arbitrary header if any
- `fn write(self: &Self, writer: & mut dyn io::Write) -> io::Result<()>` - Write the header back into a binary stream

**Trait Implementations:**

- **From**
  - `fn from(header: GraymapHeader) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **From**
  - `fn from(header: PixmapHeader) -> Self`
- **From**
  - `fn from(header: BitmapHeader) -> Self`
- **From**
  - `fn from(header: ArbitraryHeader) -> Self`



## image::codecs::pnm::header::PnmSubtype

*Enum*

Denotes the category of the magic number

**Variants:**
- `Bitmap(SampleEncoding)` - Magic numbers P1 and P4
- `Graymap(SampleEncoding)` - Magic numbers P2 and P5
- `Pixmap(SampleEncoding)` - Magic numbers P3 and P6
- `ArbitraryMap` - Magic number P7

**Methods:**

- `fn magic_constant(self: Self) -> &'static [u8; 2]` - Get the two magic constant bytes corresponding to this format subtype.
- `fn sample_encoding(self: Self) -> SampleEncoding` - Whether samples are stored as binary or as decimal ascii

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## image::codecs::pnm::header::SampleEncoding

*Enum*

The kind of encoding used to store sample values

**Variants:**
- `Binary` - Samples are unsigned binary integers in big endian
- `Ascii` - Samples are encoded as decimal ascii strings separated by whitespace

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



