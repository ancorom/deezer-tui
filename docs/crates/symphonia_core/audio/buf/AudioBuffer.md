**symphonia_core > audio > buf > AudioBuffer**

# Module: audio::buf::AudioBuffer

## Contents

**Functions**

- [`render_silence`](#render_silence) - Renders a number of silent frames.
- [`render_uninit`](#render_uninit) - Renders an uninitialized number of frames.
- [`resize_uninit`](#resize_uninit) - Resizes the buffer such that the number of frames is `new_len`. New frames are left
- [`resize_with_silence`](#resize_with_silence) - Resizes the buffer such that the number of frames is `new_len`. New frames are silent.

---

## symphonia_core::audio::buf::AudioBuffer::render_silence

*Function*

Renders a number of silent frames.

If `num_frames` is `None`, the remaining capacity will be used.

Returns the number of frames rendered.

# Panics

Panics if `num_frames` is `Some` and would exceed the capacity of the buffer.

```rust
fn render_silence(self: & mut Self, num_frames: Option<usize>) -> usize
```



## symphonia_core::audio::buf::AudioBuffer::render_uninit

*Function*

Renders an uninitialized number of frames.

This is a cheap operation and simply advances the frame counter. The underlying samples are
not modified and should be overwritten.

If `num_frames` is `None`, the remaining capacity will be used.

Returns the number of frames rendered.

# Panics

Panics if `num_frames` is `Some` and would exceed the capacity of the buffer.

```rust
fn render_uninit(self: & mut Self, num_frames: Option<usize>) -> usize
```



## symphonia_core::audio::buf::AudioBuffer::resize_uninit

*Function*

Resizes the buffer such that the number of frames is `new_len`. New frames are left
uninitialized and may contain stale data that should be overwritten.

This function does not modify the maximum capacity of the buffer.

# Panics

Panics if `new_len` exceeds the buffer capacity.

```rust
fn resize_uninit(self: & mut Self, new_len: usize)
```



## symphonia_core::audio::buf::AudioBuffer::resize_with_silence

*Function*

Resizes the buffer such that the number of frames is `new_len`. New frames are silent.

This function does not modify the maximum capacity of the buffer. If `new_len` is larger
than the current number of frames in the buffer, then the buffer will be padded with silent
frames. If `new_len` is smaller than the current number of frames, the buffer will be
truncated.

# Panics

Panics if `new_len` exceeds the buffer capacity.

```rust
fn resize_with_silence(self: & mut Self, new_len: usize)
```



