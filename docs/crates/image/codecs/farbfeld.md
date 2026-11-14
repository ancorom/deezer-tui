**image > codecs > farbfeld**

# Module: codecs::farbfeld

## Contents

**Structs**

- [`FarbfeldDecoder`](#farbfelddecoder) - farbfeld decoder
- [`FarbfeldEncoder`](#farbfeldencoder) - farbfeld encoder
- [`FarbfeldReader`](#farbfeldreader) - farbfeld Reader

---

## image::codecs::farbfeld::FarbfeldDecoder

*Struct*

farbfeld decoder

**Generic Parameters:**
- R

**Methods:**

- `fn new(buffered_read: R) -> ImageResult<FarbfeldDecoder<R>>` - Creates a new decoder that decodes from the stream ```r```

**Trait Implementations:**

- **ImageDecoder**
  - `fn dimensions(self: &Self) -> (u32, u32)`
  - `fn color_type(self: &Self) -> ColorType`
  - `fn read_image(self: Self, buf: & mut [u8]) -> ImageResult<()>`
  - `fn read_image_boxed(self: Box<Self>, buf: & mut [u8]) -> ImageResult<()>`
- **ImageDecoderRect**
  - `fn read_rect(self: & mut Self, x: u32, y: u32, width: u32, height: u32, buf: & mut [u8], row_pitch: usize) -> ImageResult<()>`



## image::codecs::farbfeld::FarbfeldEncoder

*Struct*

farbfeld encoder

**Generic Parameters:**
- W

**Methods:**

- `fn new(buffered_writer: W) -> FarbfeldEncoder<W>` - Create a new encoder that writes its output to ```w```. The writer should be buffered.
- `fn encode(self: Self, data: &[u8], width: u32, height: u32) -> ImageResult<()>` - Encodes the image `data` (native endian) that has dimensions `width` and `height`.

**Trait Implementations:**

- **ImageEncoder**
  - `fn write_image(self: Self, buf: &[u8], width: u32, height: u32, color_type: ExtendedColorType) -> ImageResult<()>`



## image::codecs::farbfeld::FarbfeldReader

*Struct*

farbfeld Reader

**Generic Parameters:**
- R

**Trait Implementations:**

- **Read**
  - `fn read(self: & mut Self, buf: & mut [u8]) -> io::Result<usize>`
- **Seek**
  - `fn seek(self: & mut Self, pos: SeekFrom) -> io::Result<u64>`



