**image > metadata > cicp**

# Module: metadata::cicp

## Contents

**Structs**

- [`Cicp`](#cicp) - Reference: <https://www.itu.int/rec/T-REC-H.273-202407-I/en> (V4)
- [`CicpTransform`](#cicptransform) - Apply to colors of the input color space to get output color values.

**Enums**

- [`CicpColorPrimaries`](#cicpcolorprimaries) - Defines the exact color of red, green, blue primary colors.
- [`CicpMatrixCoefficients`](#cicpmatrixcoefficients)
- [`CicpTransferCharacteristics`](#cicptransfercharacteristics) - The transfer characteristics, expressing relation between encoded values and linear color
- [`CicpVideoFullRangeFlag`](#cicpvideofullrangeflag) - The used encoded value range.

---

## image::metadata::cicp::Cicp

*Struct*

Reference: <https://www.itu.int/rec/T-REC-H.273-202407-I/en> (V4)

**Fields:**
- `primaries: CicpColorPrimaries` - Defines the exact color of red, green, blue primary colors.
- `transfer: CicpTransferCharacteristics` - The electro-optical transfer function (EOTF) that maps color components to linear values.
- `matrix: CicpMatrixCoefficients` - A matrix between linear values and primary color representation.
- `full_range: CicpVideoFullRangeFlag` - Whether the color components use all bits of the encoded values, or have headroom.

**Methods:**


**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## image::metadata::cicp::CicpColorPrimaries

*Enum*

Defines the exact color of red, green, blue primary colors.

Each set defines the CIE 1931 XYZ (2°) color space coordinates of the primary colors and an
illuminant/whitepoint under which those colors are viewed.

Refer to Rec H.273 Table 2.

**Variants:**
- `SRgb` - ITU-R BT.709-6
- `Unspecified` - Explicitly, the color space is not determined.
- `RgbM` - ITU-R BT.470-6 System M
- `RgbB` - ITU-R BT.470-6 System B, G
- `Bt601` - SMPTE 170M
- `Rgb240m` - SMPTE 240M
- `GenericFilm` - Generic film (colour filters using Illuminant C)
- `Rgb2020` - Rec. ITU-R BT.2020-2
- `Xyz` - SMPTE ST 428-1
- `SmpteRp431` - SMPTE RP 431-2 (aka. DCI P3)
- `SmpteRp432` - SMPTE EG 432-1, DCI P3 variant with the D65 whitepoint (matching sRGB and BT.2020)
- `Industry22` - Corresponds to value 22 but

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## image::metadata::cicp::CicpMatrixCoefficients

*Enum*


Refer to Rec H.273 Table 4.

**Variants:**
- `Identity` - The identity matrix.
- `Bt709` - Rec. ITU-R BT.709-6
- `Unspecified` - Explicitly, the matrix coefficients are not determined.
- `UsFCC` - United States Federal Communications Commission (2003) Title 47 Code of Federal Regulations 73.682 (a) (20)
- `Bt470BG` - Rec. ITU-R BT.470-6 System B, G (historical)
- `Smpte170m` - (functionally the same as the value 5)
- `Smpte240m` - SMPTE ST 240
- `YCgCo` - YCgCo
- `Bt2020NonConstant` - Rec. ITU-R BT.2020-2 (non-constant luminance)
- `Bt2020Constant` - Rec. ITU-R BT.2020-2 (constant luminance)
- `Smpte2085` - SMPTE ST 2085
- `ChromaticityDerivedNonConstant` - Chromaticity-derived non-constant luminance system
- `ChromaticityDerivedConstant` - Chromaticity-derived constant luminance system
- `Bt2100` - Rec. ITU-R BT.2100-2 ICTCp
- `IptPqC2` - Colour representation developed in SMPTE as IPT-PQ-C2.
- `YCgCoRe` - YCgCo with added bit-depth (2-bit).
- `YCgCoRo` - YCgCo with added bit-depth (1-bit).

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## image::metadata::cicp::CicpTransferCharacteristics

*Enum*

The transfer characteristics, expressing relation between encoded values and linear color
values.

Refer to Rec H.273 Table 3.

**Variants:**
- `Bt709` - Rec. ITU-R BT.709-6
- `Unspecified` - Explicitly, the transfer characteristics are not determined.
- `Bt470M` - Rec. ITU-R BT.470-6 System M (historical)
- `Bt470BG` - Rec. ITU-R BT.470-6 System B, G (historical)
- `Bt601` - Rec. ITU-R BT.601-7 525 or 625
- `Smpte240m` - SMPTE ST 240
- `Linear` - Linear transfer characteristics
- `Log100` - Logarithmic transfer characteristic (100:1 range)
- `LogSqrt` - Logarithmic transfer characteristic (100 * Sqrt( 10 ) : 1 range)
- `Iec61966_2_4` - IEC 61966-2-4
- `Bt1361` - Rec. ITU-R BT.1361-0 extended colour gamut system (historical)
- `SRgb` - IEC 61966-2-1 sRGB (with MatrixCoefficients equal to 0)
- `Bt2020_10bit` - Rec. ITU-R BT.2020-2 (10-bit system)
- `Bt2020_12bit` - Rec. ITU-R BT.2020-2 (12-bit system)
- `Smpte2084` - SMPTE ST 2084 for 10-, 12-, 14- and 16-bit systems
- `Smpte428` - SMPTE ST 428-1
- `Bt2100Hlg` - ARIB STD-B67

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## image::metadata::cicp::CicpTransform

*Struct*

Apply to colors of the input color space to get output color values.

We do not support all possible Cicp color spaces, but when we support one then all builtin
`Pixel` types can be converted with their respective components. This value is used to signify
that some particular combination is supported.

**Methods:**

- `fn new(from: Cicp, into: Cicp) -> Option<Self>` - Construct a transform between two color spaces.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`



## image::metadata::cicp::CicpVideoFullRangeFlag

*Enum*

The used encoded value range.

**Variants:**
- `NarrowRange` - The color components are encoded in a limited range, e.g., 16-235 for 8-bit.
- `FullRange` - The color components are encoded in the full range, e.g., 0-255 for 8-bit.

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



