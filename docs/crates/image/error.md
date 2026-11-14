**image > error**

# Module: error

## Contents

**Structs**

- [`DecodingError`](#decodingerror) - An error was encountered while decoding an image.
- [`EncodingError`](#encodingerror) - An error was encountered while encoding an image.
- [`LimitError`](#limiterror) - Completing the operation would have required more resources than allowed.
- [`ParameterError`](#parametererror) - An error was encountered in inputs arguments.
- [`TryFromExtendedColorError`](#tryfromextendedcolorerror) - Converting [`ExtendedColorType`] to [`ColorType`][`crate::ColorType`] failed.
- [`UnsupportedError`](#unsupportederror) - The implementation for an operation was not provided.

**Enums**

- [`ImageError`](#imageerror) - The generic error type for image operations.
- [`ImageFormatHint`](#imageformathint) - A best effort representation for image formats.
- [`LimitErrorKind`](#limiterrorkind) - Indicates the limit that prevented an operation from completing.
- [`ParameterErrorKind`](#parametererrorkind) - Details how a parameter is malformed.
- [`UnsupportedErrorKind`](#unsupportederrorkind) - Details what feature is not supported.

**Type Aliases**

- [`ImageResult`](#imageresult) - Result of an image decoding/encoding process

---

## image::error::DecodingError

*Struct*

An error was encountered while decoding an image.

This is used as an opaque representation for the [`ImageError::Decoding`] variant. See its
documentation for more information.

[`ImageError::Decoding`]: enum.ImageError.html#variant.Decoding

**Methods:**

- `fn new<impl Into<Box<dyn Error + Send + Sync>>>(format: ImageFormatHint, err: impl Trait) -> Self` - Create a `DecodingError` that stems from an arbitrary error of an underlying decoder.
- `fn from_format_hint(format: ImageFormatHint) -> Self` - Create a `DecodingError` for an image format.
- `fn format_hint(self: &Self) -> ImageFormatHint` - Returns the image format associated with this error.

**Trait Implementations:**

- **Error**
  - `fn source(self: &Self) -> Option<&dyn Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> Result<(), fmt::Error>`



## image::error::EncodingError

*Struct*

An error was encountered while encoding an image.

This is used as an opaque representation for the [`ImageError::Encoding`] variant. See its
documentation for more information.

[`ImageError::Encoding`]: enum.ImageError.html#variant.Encoding

**Methods:**

- `fn new<impl Into<Box<dyn Error + Send + Sync>>>(format: ImageFormatHint, err: impl Trait) -> Self` - Create an `EncodingError` that stems from an arbitrary error of an underlying encoder.
- `fn from_format_hint(format: ImageFormatHint) -> Self` - Create an `EncodingError` for an image format.
- `fn format_hint(self: &Self) -> ImageFormatHint` - Return the image format associated with this error.

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> Result<(), fmt::Error>`
- **Error**
  - `fn source(self: &Self) -> Option<&dyn Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## image::error::ImageError

*Enum*

The generic error type for image operations.

This high level enum allows, by variant matching, a rough separation of concerns between
underlying IO, the caller, format specifications, and the `image` implementation.

**Variants:**
- `Decoding(DecodingError)` - An error was encountered while decoding.
- `Encoding(EncodingError)` - An error was encountered while encoding.
- `Parameter(ParameterError)` - An error was encountered in input arguments.
- `Limits(LimitError)` - Completing the operation would have required more resources than allowed.
- `Unsupported(UnsupportedError)` - An operation can not be completed by the chosen abstraction.
- `IoError(io::Error)` - An error occurred while interacting with the environment.

**Trait Implementations:**

- **Error**
  - `fn source(self: &Self) -> Option<&dyn Error>`
- **From**
  - `fn from(err: TryFromExtendedColorError) -> ImageError`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(error: Error) -> ImageError`
- **From**
  - `fn from(err: io::Error) -> ImageError`
- **From**
  - `fn from(_: TryReserveError) -> ImageError`
- **Display**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> Result<(), fmt::Error>`



## image::error::ImageFormatHint

*Enum*

A best effort representation for image formats.

**Variants:**
- `Exact(crate::ImageFormat)` - The format is known exactly.
- `Name(String)` - The format can be identified by a name.
- `PathExtension(std::path::PathBuf)` - A common path extension for the format is known.
- `Unknown` - The format is not known or could not be determined.

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **From**
  - `fn from(format: ImageFormat) -> Self`
- **From**
  - `fn from(path: &std::path::Path) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> Result<(), fmt::Error>`



## image::error::ImageResult

*Type Alias*: `Result<T, ImageError>`

Result of an image decoding/encoding process



## image::error::LimitError

*Struct*

Completing the operation would have required more resources than allowed.

This is used as an opaque representation for the [`ImageError::Limits`] variant. See its
documentation for more information.

[`ImageError::Limits`]: enum.ImageError.html#variant.Limits

**Methods:**

- `fn from_kind(kind: LimitErrorKind) -> Self` - Construct a generic `LimitError` directly from a corresponding kind.
- `fn kind(self: &Self) -> LimitErrorKind` - Returns the corresponding `LimitErrorKind` of the error.

**Traits:** Error

**Trait Implementations:**

- **From**
  - `fn from(kind: LimitErrorKind) -> Self`
- **Display**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> Result<(), fmt::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## image::error::LimitErrorKind

*Enum*

Indicates the limit that prevented an operation from completing.

Note that this enumeration is not exhaustive and may in the future be extended to provide more
detailed information or to incorporate other resources types.

**Variants:**
- `DimensionError` - The resulting image exceed dimension limits in either direction.
- `InsufficientMemory` - The operation would have performed an allocation larger than allowed.
- `Unsupported{ limits: crate::Limits, supported: crate::LimitSupport }` - The specified strict limits are not supported for this operation

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## image::error::ParameterError

*Struct*

An error was encountered in inputs arguments.

This is used as an opaque representation for the [`ImageError::Parameter`] variant. See its
documentation for more information.

[`ImageError::Parameter`]: enum.ImageError.html#variant.Parameter

**Methods:**

- `fn from_kind(kind: ParameterErrorKind) -> Self` - Construct a `ParameterError` directly from a corresponding kind.
- `fn kind(self: &Self) -> ParameterErrorKind` - Returns the corresponding `ParameterErrorKind` of the error.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> Result<(), fmt::Error>`
- **Error**
  - `fn source(self: &Self) -> Option<&dyn Error>`



## image::error::ParameterErrorKind

*Enum*

Details how a parameter is malformed.

**Variants:**
- `DimensionMismatch` - The dimensions passed are wrong.
- `FailedAlready` - Repeated an operation for which error that could not be cloned was emitted already.
- `RgbCicpRequired(crate::metadata::Cicp)` - The cicp is required to be RGB-like but had other matrix transforms or narrow range.
- `Generic(String)` - A string describing the parameter.
- `NoMoreData` - The end of the image has been reached.
- `CicpMismatch{ expected: crate::metadata::Cicp, found: crate::metadata::Cicp }` - An operation expected a concrete color space but another was found.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## image::error::TryFromExtendedColorError

*Struct*

Converting [`ExtendedColorType`] to [`ColorType`][`crate::ColorType`] failed.

This type is convertible to [`ImageError`] as [`ImageError::Unsupported`].

**Traits:** Error

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## image::error::UnsupportedError

*Struct*

The implementation for an operation was not provided.

See the variant [`Unsupported`] for more documentation.

[`Unsupported`]: enum.ImageError.html#variant.Unsupported

**Methods:**

- `fn from_format_and_kind(format: ImageFormatHint, kind: UnsupportedErrorKind) -> Self` - Create an `UnsupportedError` for an image with details on the unsupported feature.
- `fn kind(self: &Self) -> UnsupportedErrorKind` - Returns the corresponding `UnsupportedErrorKind` of the error.
- `fn format_hint(self: &Self) -> ImageFormatHint` - Returns the image format associated with this error.

**Traits:** Error

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> Result<(), fmt::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(hint: ImageFormatHint) -> Self`



## image::error::UnsupportedErrorKind

*Enum*

Details what feature is not supported.

**Variants:**
- `Color(crate::color::ExtendedColorType)` - The required color type can not be handled.
- `ColorLayout(crate::color::ExtendedColorType)` - Dealing with an intricate layout is not implemented for an algorithm.
- `ColorspaceCicp(crate::metadata::Cicp)` - The colors or transfer function of the CICP are not supported.
- `Format(ImageFormatHint)` - An image format is not supported.
- `GenericFeature(String)` - Some feature specified by string.

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



