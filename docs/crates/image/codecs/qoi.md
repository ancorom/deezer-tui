**image > codecs > qoi**

# Module: codecs::qoi

## Contents

**Structs**

- [`QoiDecoder`](#qoidecoder) - QOI decoder
- [`QoiEncoder`](#qoiencoder) - QOI encoder

---

## image::codecs::qoi::QoiDecoder

*Struct*

QOI decoder

**Generic Parameters:**
- R

**Methods:**

- `fn new(reader: R) -> ImageResult<Self>` - Creates a new decoder that decodes from the stream ```reader```

**Trait Implementations:**

- **ImageDecoder**
  - `fn dimensions(self: &Self) -> (u32, u32)`
  - `fn color_type(self: &Self) -> ColorType`
  - `fn read_image(self: Self, buf: & mut [u8]) -> ImageResult<()>`
  - `fn read_image_boxed(self: Box<Self>, buf: & mut [u8]) -> ImageResult<()>`



## image::codecs::qoi::QoiEncoder

*Struct*

QOI encoder

**Generic Parameters:**
- W

**Methods:**

- `fn new(writer: W) -> Self` - Creates a new encoder that writes its output to ```writer```

**Trait Implementations:**

- **ImageEncoder**
  - `fn write_image(self: Self, buf: &[u8], width: u32, height: u32, color_type: ExtendedColorType) -> ImageResult<()>`



