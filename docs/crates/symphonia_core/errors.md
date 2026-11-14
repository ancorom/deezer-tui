**symphonia_core > errors**

# Module: errors

## Contents

**Enums**

- [`Error`](#error) - `Error` provides an enumeration of all possible errors reported by Symphonia.
- [`SeekErrorKind`](#seekerrorkind) - `SeekErrorKind` is a list of generic reasons why a seek may fail.

**Functions**

- [`decode_error`](#decode_error) - Convenience function to create a decode error.
- [`limit_error`](#limit_error) - Convenience function to create a limit error.
- [`reset_error`](#reset_error) - Convenience function to create a reset required error.
- [`seek_error`](#seek_error) - Convenience function to create a seek error.
- [`unsupported_error`](#unsupported_error) - Convenience function to create an unsupport feature error.

**Type Aliases**

- [`Result`](#result)

---

## symphonia_core::errors::Error

*Enum*

`Error` provides an enumeration of all possible errors reported by Symphonia.

**Variants:**
- `IoError(std::io::Error)` - An IO error occured while reading, writing, or seeking the stream.
- `DecodeError(&'static str)` - The stream contained malformed data and could not be decoded or demuxed.
- `SeekError(SeekErrorKind)` - The stream could not be seeked.
- `Unsupported(&'static str)` - An unsupported container or codec feature was encounted.
- `LimitError(&'static str)` - A default or user-defined limit was reached while decoding or demuxing the stream. Limits
- `ResetRequired` - The demuxer or decoder needs to be reset before continuing.

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Error**
  - `fn cause(self: &Self) -> Option<&dyn error::Error>`
- **From**
  - `fn from(err: io::Error) -> Error`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## symphonia_core::errors::Result

*Type Alias*: `result::Result<T, Error>`



## symphonia_core::errors::SeekErrorKind

*Enum*

`SeekErrorKind` is a list of generic reasons why a seek may fail.

**Variants:**
- `Unseekable` - The stream is not seekable at all.
- `ForwardOnly` - The stream can only be seeked forward.
- `OutOfRange` - The timestamp to seek to is out of range.
- `InvalidTrack` - The track ID provided is invalid.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## symphonia_core::errors::decode_error

*Function*

Convenience function to create a decode error.

```rust
fn decode_error<T>(desc: &'static str) -> Result<T>
```



## symphonia_core::errors::limit_error

*Function*

Convenience function to create a limit error.

```rust
fn limit_error<T>(constraint: &'static str) -> Result<T>
```



## symphonia_core::errors::reset_error

*Function*

Convenience function to create a reset required error.

```rust
fn reset_error<T>() -> Result<T>
```



## symphonia_core::errors::seek_error

*Function*

Convenience function to create a seek error.

```rust
fn seek_error<T>(kind: SeekErrorKind) -> Result<T>
```



## symphonia_core::errors::unsupported_error

*Function*

Convenience function to create an unsupport feature error.

```rust
fn unsupported_error<T>(feature: &'static str) -> Result<T>
```



