**image > codecs > gif**

# Module: codecs::gif

## Contents

**Structs**

- [`GifDecoder`](#gifdecoder) - GIF decoder
- [`GifEncoder`](#gifencoder) - GIF encoder.
- [`GifReader`](#gifreader) - Wrapper struct around a `Cursor<Vec<u8>>`

**Enums**

- [`Repeat`](#repeat) - Number of repetitions for a GIF animation

---

## image::codecs::gif::GifDecoder

*Struct*

GIF decoder

**Generic Parameters:**
- R

**Methods:**

- `fn new(r: R) -> ImageResult<GifDecoder<R>>` - Creates a new decoder that decodes the input steam `r`

**Trait Implementations:**

- **ImageDecoder**
  - `fn dimensions(self: &Self) -> (u32, u32)`
  - `fn color_type(self: &Self) -> ColorType`
  - `fn set_limits(self: & mut Self, limits: Limits) -> ImageResult<()>`
  - `fn read_image(self: Self, buf: & mut [u8]) -> ImageResult<()>`
  - `fn icc_profile(self: & mut Self) -> ImageResult<Option<Vec<u8>>>`
  - `fn xmp_metadata(self: & mut Self) -> ImageResult<Option<Vec<u8>>>`
  - `fn read_image_boxed(self: Box<Self>, buf: & mut [u8]) -> ImageResult<()>`
- **AnimationDecoder**
  - `fn loop_count(self: &Self) -> LoopCount`
  - `fn into_frames(self: Self) -> animation::Frames<'a>`



## image::codecs::gif::GifEncoder

*Struct*

GIF encoder.

**Generic Parameters:**
- W

**Methods:**

- `fn new(w: W) -> GifEncoder<W>` - Creates a new GIF encoder with a speed of 1. This prioritizes quality over performance at any cost.
- `fn new_with_speed(w: W, speed: i32) -> GifEncoder<W>` - Create a new GIF encoder, and has the speed parameter `speed`. See
- `fn set_repeat(self: & mut Self, repeat: Repeat) -> ImageResult<()>` - Set the repeat behaviour of the encoded GIF
- `fn encode(self: & mut Self, data: &[u8], width: u32, height: u32, color: ExtendedColorType) -> ImageResult<()>` - Encode a single image.
- `fn encode_frame(self: & mut Self, img_frame: animation::Frame) -> ImageResult<()>` - Encode one frame of animation.
- `fn encode_frames<F>(self: & mut Self, frames: F) -> ImageResult<()>` - Encodes Frames.
- `fn try_encode_frames<F>(self: & mut Self, frames: F) -> ImageResult<()>` - Try to encode a collection of `ImageResult<animation::Frame>` objects.

**Trait Implementations:**

- **ImageEncoder**
  - `fn write_image(self: Self, buf: &[u8], width: u32, height: u32, color_type: ExtendedColorType) -> ImageResult<()>`



## image::codecs::gif::GifReader

*Struct*

Wrapper struct around a `Cursor<Vec<u8>>`

**Generic Parameters:**
- R

**Tuple Struct**: `()`

**Trait Implementations:**

- **Read**
  - `fn read(self: & mut Self, buf: & mut [u8]) -> io::Result<usize>`
  - `fn read_to_end(self: & mut Self, buf: & mut Vec<u8>) -> io::Result<usize>`



## image::codecs::gif::Repeat

*Enum*

Number of repetitions for a GIF animation

**Variants:**
- `Finite(u16)` - Finite number of repetitions
- `Infinite` - Looping GIF

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



