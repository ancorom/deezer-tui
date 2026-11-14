**image > hooks**

# Module: hooks

## Contents

**Structs**

- [`GenericReader`](#genericreader) - A wrapper around a type-erased trait object that implements `Read` and `Seek`.

**Functions**

- [`decoding_hook_registered`](#decoding_hook_registered) - Returns whether a decoding hook has been registered for the given format.
- [`register_decoding_hook`](#register_decoding_hook) - Register a new decoding hook or returns false if one already exists for the given format.
- [`register_format_detection_hook`](#register_format_detection_hook) - Registers a format detection hook.

**Type Aliases**

- [`DecodingHook`](#decodinghook) - A function to produce an `ImageDecoder` for a given image format.

---

## image::hooks::DecodingHook

*Type Alias*: `Box<dyn Fn>`

A function to produce an `ImageDecoder` for a given image format.



## image::hooks::GenericReader

*Struct*

A wrapper around a type-erased trait object that implements `Read` and `Seek`.

**Generic Parameters:**
- 'a

**Tuple Struct**: `()`

**Trait Implementations:**

- **Seek**
  - `fn seek(self: & mut Self, pos: std::io::SeekFrom) -> std::io::Result<u64>`
  - `fn rewind(self: & mut Self) -> std::io::Result<()>`
  - `fn stream_position(self: & mut Self) -> std::io::Result<u64>`
- **BufRead**
  - `fn fill_buf(self: & mut Self) -> std::io::Result<&[u8]>`
  - `fn consume(self: & mut Self, amt: usize)`
  - `fn read_until(self: & mut Self, byte: u8, buf: & mut Vec<u8>) -> std::io::Result<usize>`
  - `fn read_line(self: & mut Self, buf: & mut String) -> std::io::Result<usize>`
- **Read**
  - `fn read(self: & mut Self, buf: & mut [u8]) -> std::io::Result<usize>`
  - `fn read_vectored(self: & mut Self, bufs: & mut [std::io::IoSliceMut]) -> std::io::Result<usize>`
  - `fn read_to_end(self: & mut Self, buf: & mut Vec<u8>) -> std::io::Result<usize>`
  - `fn read_to_string(self: & mut Self, buf: & mut String) -> std::io::Result<usize>`
  - `fn read_exact(self: & mut Self, buf: & mut [u8]) -> std::io::Result<()>`



## image::hooks::decoding_hook_registered

*Function*

Returns whether a decoding hook has been registered for the given format.

```rust
fn decoding_hook_registered(extension: &std::OsStr) -> bool
```



## image::hooks::register_decoding_hook

*Function*

Register a new decoding hook or returns false if one already exists for the given format.

```rust
fn register_decoding_hook(extension: std::OsString, hook: DecodingHook) -> bool
```



## image::hooks::register_format_detection_hook

*Function*

Registers a format detection hook.

The signature field holds the magic bytes from the start of the file that must be matched to
detect the format. The mask field is optional and can be used to specify which bytes in the
signature should be ignored during the detection.

# Examples

## Using the mask to ignore some bytes

```
# use image::hooks::register_format_detection_hook;
// WebP signature is 'riff' followed by 4 bytes of length and then by 'webp'.
// This requires a mask to ignore the length.
register_format_detection_hook("webp".into(),
     &[b'r', b'i', b'f', b'f', 0, 0, 0, 0, b'w', b'e', b'b', b'p'],
Some(&[0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff]),
);
```

## Multiple signatures

```
# use image::hooks::register_format_detection_hook;
// JPEG XL has two different signatures: https://en.wikipedia.org/wiki/JPEG_XL
// This function should be called twice to register them both.
register_format_detection_hook("jxl".into(), &[0xff, 0x0a], None);
register_format_detection_hook("jxl".into(),
     &[0x00, 0x00, 0x00, 0x0c, 0x4a, 0x58, 0x4c, 0x20, 0x0d, 0x0a, 0x87, 0x0a], None,
);
```


```rust
fn register_format_detection_hook(extension: std::OsString, signature: &'static [u8], mask: Option<&'static [u8]>)
```



