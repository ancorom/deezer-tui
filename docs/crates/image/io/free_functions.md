**image > io > free_functions**

# Module: io::free_functions

## Contents

**Functions**

- [`guess_format`](#guess_format) - Guess image format from memory block
- [`load`](#load) - Create a new image from a Reader.
- [`save_buffer`](#save_buffer) - Saves the supplied buffer to a file at the path specified.
- [`save_buffer_with_format`](#save_buffer_with_format) - Saves the supplied buffer to a file given the path and desired format.

---

## image::io::free_functions::guess_format

*Function*

Guess image format from memory block

Makes an educated guess about the image format based on the Magic Bytes at the beginning.
TGA is not supported by this function.
This is not to be trusted on the validity of the whole memory block

```rust
fn guess_format(buffer: &[u8]) -> crate::error::ImageResult<crate::ImageFormat>
```



## image::io::free_functions::load

*Function*

Create a new image from a Reader.

Assumes the reader is already buffered. For optimal performance,
consider wrapping the reader with a `BufReader::new()`.

Try [`ImageReader`] for more advanced uses.

```rust
fn load<R>(r: R, format: crate::ImageFormat) -> crate::error::ImageResult<crate::DynamicImage>
```



## image::io::free_functions::save_buffer

*Function*

Saves the supplied buffer to a file at the path specified.

The image format is derived from the file extension. The buffer is assumed to have the correct
format according to the specified color type. This will lead to corrupted files if the buffer
contains malformed data.

```rust
fn save_buffer<impl AsRef<Path>, impl Into<ExtendedColorType>>(path: impl Trait, buf: &[u8], width: u32, height: u32, color: impl Trait) -> crate::error::ImageResult<()>
```



## image::io::free_functions::save_buffer_with_format

*Function*

Saves the supplied buffer to a file given the path and desired format.

The buffer is assumed to have the correct format according to the specified color type. This
will lead to corrupted files if the buffer contains malformed data.

```rust
fn save_buffer_with_format<impl AsRef<Path>, impl Into<ExtendedColorType>>(path: impl Trait, buf: &[u8], width: u32, height: u32, color: impl Trait, format: crate::ImageFormat) -> crate::error::ImageResult<()>
```



