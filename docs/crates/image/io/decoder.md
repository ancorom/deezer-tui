**image > io > decoder**

# Module: io::decoder

## Contents

**Traits**

- [`AnimationDecoder`](#animationdecoder) - `AnimationDecoder` trait
- [`ImageDecoder`](#imagedecoder) - The trait that all decoders implement
- [`ImageDecoderRect`](#imagedecoderrect) - Specialized image decoding not be supported by all formats

---

## image::io::decoder::AnimationDecoder

*Trait*

`AnimationDecoder` trait

**Methods:**

- `into_frames`: Consume the decoder producing a series of frames.
- `loop_count`: Loop count of the animated image.



## image::io::decoder::ImageDecoder

*Trait*

The trait that all decoders implement

**Methods:**

- `dimensions`: Returns a tuple containing the width and height of the image
- `color_type`: Returns the color type of the image data produced by this decoder
- `original_color_type`: Returns the color type of the image file before decoding
- `icc_profile`: Returns the ICC color profile embedded in the image, or `Ok(None)` if the image does not have one.
- `exif_metadata`: Returns the raw [Exif](https://en.wikipedia.org/wiki/Exif) chunk, if it is present.
- `xmp_metadata`: Returns the raw [XMP](https://en.wikipedia.org/wiki/Extensible_Metadata_Platform) chunk, if it is present.
- `iptc_metadata`: Returns the raw [IPTC](https://en.wikipedia.org/wiki/IPTC_Information_Interchange_Model) chunk, if it is present.
- `orientation`: Returns the orientation of the image.
- `total_bytes`: Returns the total number of bytes in the decoded image.
- `read_image`: Returns all the bytes in the image.
- `set_limits`: Set the decoder to have the specified limits. See [`Limits`] for the different kinds of
- `read_image_boxed`: Use `read_image` instead; this method is an implementation detail needed so the trait can



## image::io::decoder::ImageDecoderRect

*Trait*

Specialized image decoding not be supported by all formats

**Methods:**

- `read_rect`: Decode a rectangular section of the image.



