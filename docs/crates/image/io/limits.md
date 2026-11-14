**image > io > limits**

# Module: io::limits

## Contents

**Structs**

- [`LimitSupport`](#limitsupport) - Set of supported strict limits for a decoder.
- [`Limits`](#limits) - Resource limits for decoding.

---

## image::io::limits::LimitSupport

*Struct*

Set of supported strict limits for a decoder.

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Default**
  - `fn default() -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## image::io::limits::Limits

*Struct*

Resource limits for decoding.

Limits can be either *strict* or *non-strict*. Non-strict limits are best-effort
limits where the library does not guarantee that limit will not be exceeded. Do note
that it is still considered a bug if a non-strict limit is exceeded.
Some of the underlying decoders do not support such limits, so one cannot
rely on these limits being supported. For strict limits, the library makes a stronger
guarantee that the limit will not be exceeded. Exceeding a strict limit is considered
a critical bug. If a decoder cannot guarantee that it will uphold a strict limit, it
*must* fail with [`error::LimitErrorKind::Unsupported`].

The only currently supported strict limits are the `max_image_width` and `max_image_height`
limits, but more will be added in the future. [`LimitSupport`] will default to support
being false, and decoders should enable support for the limits they support in
[`ImageDecoder::set_limits`].

The limit check should only ever fail if a limit will be exceeded or an unsupported strict
limit is used.

[`LimitSupport`]: ./struct.LimitSupport.html
[`ImageDecoder::set_limits`]: ../trait.ImageDecoder.html#method.set_limits

**Fields:**
- `max_image_width: Option<u32>` - The maximum allowed image width. This limit is strict. The default is no limit.
- `max_image_height: Option<u32>` - The maximum allowed image height. This limit is strict. The default is no limit.
- `max_alloc: Option<u64>` - The maximum allowed sum of allocations allocated by the decoder at any one time excluding

**Methods:**

- `fn no_limits() -> Limits` - Disable all limits.
- `fn check_support(self: &Self, _supported: &LimitSupport) -> ImageResult<()>` - This function checks that all currently set strict limits are supported.
- `fn check_dimensions(self: &Self, width: u32, height: u32) -> ImageResult<()>` - This function checks the `max_image_width` and `max_image_height` limits given
- `fn reserve(self: & mut Self, amount: u64) -> ImageResult<()>` - This function checks that the current limit allows for reserving the set amount
- `fn reserve_usize(self: & mut Self, amount: usize) -> ImageResult<()>` - This function acts identically to [`reserve`], but takes a `usize` for convenience.
- `fn reserve_buffer(self: & mut Self, width: u32, height: u32, color_type: ColorType) -> ImageResult<()>` - This function acts identically to [`reserve`], but accepts the width, height and color type
- `fn free(self: & mut Self, amount: u64)` - This function increases the `max_alloc` limit with amount. Should only be used
- `fn free_usize(self: & mut Self, amount: usize)` - This function acts identically to [`free`], but takes a `usize` for convenience.

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Default**
  - `fn default() -> Limits`



