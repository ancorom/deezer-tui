**image > io > encoder**

# Module: io::encoder

## Contents

**Structs**

- [`MethodSealedToImage`](#methodsealedtoimage) - Nominally public but DO NOT expose this type.

**Traits**

- [`ImageEncoder`](#imageencoder) - The trait all encoders implement

---

## image::io::encoder::ImageEncoder

*Trait*

The trait all encoders implement

**Methods:**

- `write_image`: Writes all the bytes in an image to the encoder.
- `set_icc_profile`: Set the ICC profile to use for the image.
- `set_exif_metadata`: Set the EXIF metadata to use for the image.



## image::io::encoder::MethodSealedToImage

*Struct*

Nominally public but DO NOT expose this type.

To be somewhat sure here's a compile fail test:

```compile_fail
use image::MethodSealedToImage;
```

```compile_fail
use image::io::MethodSealedToImage;
```

The same implementation strategy for a partially public trait is used in the standard library,
for the different effect of forbidding `Error::type_id` overrides thus making them reliable for
their calls through the `dyn` version of the trait.

Read more: <https://predr.ag/blog/definitive-guide-to-sealed-traits-in-rust/>

**Unit Struct**



