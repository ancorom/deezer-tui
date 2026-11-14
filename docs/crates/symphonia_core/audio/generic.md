**symphonia_core > audio > generic**

# Module: audio::generic

## Contents

**Enums**

- [`GenericAudioBuffer`](#genericaudiobuffer) - An owning wrapper for an [`AudioBuffer`] of any standard sample format.
- [`GenericAudioBufferRef`](#genericaudiobufferref) - A non-owning immutable reference wrapper for an [`AudioBuffer`] of any standard sample format.
- [`GenericAudioSlice`](#genericaudioslice) - An owning wrapper for an [`AudioSlice`] of any standard sample format.

**Traits**

- [`AsGenericAudioBufferRef`](#asgenericaudiobufferref) - A trait for generically borrowing an [`AudioBuffer`] by wrapping it in a

---

## symphonia_core::audio::generic::AsGenericAudioBufferRef

*Trait*

A trait for generically borrowing an [`AudioBuffer`] by wrapping it in a
[`GenericAudioBufferRef`].

**Methods:**

- `as_generic_audio_buffer_ref`: Get an immutable reference to the audio buffer as a generic audio buffer reference.



## symphonia_core::audio::generic::GenericAudioBuffer

*Enum*

An owning wrapper for an [`AudioBuffer`] of any standard sample format.

Calls on this wrapper are dispatched to the underlying, wrapped, buffer and are semantically
identical.

**Variants:**
- `U8(super::AudioBuffer<u8>)` - An unsigned 8-bit integer audio buffer.
- `U16(super::AudioBuffer<u16>)` - An unsigned 16-bit integer audio buffer.
- `U24(super::AudioBuffer<crate::audio::u24>)` - An unsigned 24-bit integer audio buffer.
- `U32(super::AudioBuffer<u32>)` - An unsigned 32-bit integer audio buffer.
- `S8(super::AudioBuffer<i8>)` - A signed 8-bit integer audio buffer.
- `S16(super::AudioBuffer<i16>)` - A signed 16-bit integer audio buffer.
- `S24(super::AudioBuffer<crate::audio::i24>)` - A signed 24-bit integer audio buffer.
- `S32(super::AudioBuffer<i32>)` - A signed 32-bit integer audio buffer.
- `F32(super::AudioBuffer<f32>)` - A single precision (32-bit) floating point audio buffer.
- `F64(super::AudioBuffer<f64>)` - A double precision (64-bit) floating point audio buffer.

**Methods:**

- `fn new(format: SampleFormat, spec: AudioSpec, capacity: usize) -> Self`
- `fn spec(self: &Self) -> &AudioSpec` - Get the audio specification.
- `fn num_planes(self: &Self) -> usize` - Get the total number of audio planes.
- `fn is_empty(self: &Self) -> bool` - Returns `true` if there are no audio frames.
- `fn frames(self: &Self) -> usize` - Gets the number of audio frames in the buffer.
- `fn is_unused(self: &Self) -> bool` - Returns `true` if the `AudioBuffer` is unused.
- `fn capacity(self: &Self) -> usize` - Gets the total capacity of the buffer. The capacity is the maximum number of audio frames
- `fn clear(self: & mut Self)` - Clears all audio frames.
- `fn grow_capacity(self: & mut Self, new_capacity: usize)` - Grows the capacity of the buffer if the new capacity is larger than the current capacity.
- `fn resize_with_silence(self: & mut Self, new_len: usize)` - Resizes the buffer such that the number of frames is `new_len`. New frames are silent.
- `fn resize_uninit(self: & mut Self, new_len: usize)` - Resizes the buffer such that the number of frames is `new_len`. New frames are left
- `fn render_silence(self: & mut Self, num_frames: Option<usize>) -> usize` - Renders a number of silent frames.
- `fn render_uninit(self: & mut Self, num_frames: Option<usize>) -> usize` - Renders an uninitialized number of frames.
- `fn shift(self: & mut Self, shift: usize)` - Shifts the contents of the buffer back by the number of frames specified. The leading frames
- `fn slice<R>(self: &Self, range: R) -> GenericAudioSlice` - Get an immutable slice of the buffer over `range`.
- `fn truncate(self: & mut Self, num_frames: usize)` - Truncates the buffer to the number of frames specified. If the number of frames in the
- `fn trim(self: & mut Self, start: usize, end: usize)` - Trims frames from the start and end of the buffer.
- `fn samples_interleaved(self: &Self) -> usize` - Get the total number of samples contained in all audio planes.
- `fn samples_planar(self: &Self) -> usize` - Get the total number of samples contained in each audio plane.
- `fn copy_to<Sout, Dst>(self: &Self, dst: & mut Dst)` - Copy audio to a mutable audio slice while doing any necessary sample format conversions.
- `fn copy_to_slice_interleaved<Sout, Dst>(self: &Self, dst: Dst)` - Copy all audio frames to a slice of samples in interleaved order.
- `fn copy_to_slice_planar<Sout, Dst>(self: &Self, dst: & mut [Dst])` - Copy all audio planes to discrete slices.
- `fn copy_to_vec_interleaved<Sout>(self: &Self, dst: & mut Vec<Sout>)` - Copy all audio frames to a vector of samples in interleaved order.
- `fn copy_to_vecs_planar<Sout>(self: &Self, dst: & mut Vec<Vec<Sout>>)` - Copy all audio planes to discrete vectors.
- `fn copy_bytes_interleaved_as<Sout, Dst>(self: &Self, dst: Dst)` - Copy interleaved audio to the destination byte slice after converting to a different sample
- `fn copy_bytes_planar_as<Sout, Dst>(self: &Self, dst: & mut [Dst])` - Copy planar audio as bytes to a destination slice per plane after converting to a different
- `fn copy_bytes_interleaved<Dst>(self: &Self, dst: Dst)` - Copy interleaved audio to the destination byte slice.
- `fn copy_bytes_planar<Dst>(self: &Self, dst: & mut [Dst])` - Copy planar audio as bytes to a destination slice per plane.
- `fn copy_bytes_to_vec_interleaved(self: &Self, dst: & mut Vec<u8>)` - Copy interleaved audio to the destination byte vector.
- `fn copy_bytes_to_vec_interleaved_as<Sout>(self: &Self, dst: & mut Vec<u8>)` - Copy interleaved audio to the destination byte vector after converting to a different sample
- `fn copy_bytes_to_vecs_planar(self: &Self, dst: & mut Vec<Vec<u8>>)` - Copy audio planes as bytes to discrete byte vectors.
- `fn copy_bytes_to_vecs_planar_as<Sout>(self: &Self, dst: & mut Vec<Vec<u8>>)` - Copy audio planes as bytes to discrete byte vectors after converting to a different sample
- `fn byte_len_as<Sout>(self: &Self) -> usize` - Get the length in bytes of all samples if converted to a new sample format.
- `fn byte_len_per_plane_as<Sout>(self: &Self) -> usize` - Get the length in bytes of all samples in a single plane if converted to a new sample
- `fn byte_len_per_frame_as<Sout>(self: &Self) -> usize` - Get the length of bytes of a single interleaved audio frame if converted to a new sample
- `fn byte_len(self: &Self) -> usize` - Get the length in bytes of all samples.
- `fn byte_len_per_plane(self: &Self) -> usize` - Get the length in bytes of all samples in a single plane.
- `fn byte_len_per_frame(self: &Self) -> usize` - Get the length of bytes of a single interleaved audio frame.

**Trait Implementations:**

- **From**
  - `fn from(value: AudioBuffer<u16>) -> Self`
- **From**
  - `fn from(value: AudioBuffer<i32>) -> Self`
- **AsGenericAudioBufferRef**
  - `fn as_generic_audio_buffer_ref(self: &Self) -> GenericAudioBufferRef`
- **From**
  - `fn from(value: AudioBuffer<i24>) -> Self`
- **From**
  - `fn from(value: AudioBuffer<i16>) -> Self`
- **From**
  - `fn from(value: AudioBuffer<u8>) -> Self`
- **From**
  - `fn from(value: AudioBuffer<i8>) -> Self`
- **From**
  - `fn from(value: AudioBuffer<u32>) -> Self`
- **From**
  - `fn from(value: AudioBuffer<f64>) -> Self`
- **From**
  - `fn from(value: AudioBuffer<u24>) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **From**
  - `fn from(value: AudioBuffer<f32>) -> Self`



## symphonia_core::audio::generic::GenericAudioBufferRef

*Enum*

A non-owning immutable reference wrapper for an [`AudioBuffer`] of any standard sample format.

Calls on this wrapper are dispatched to the underlying, wrapped, buffer and are semantically
identical.

**Generic Parameters:**
- 'a

**Variants:**
- `U8(&'a super::AudioBuffer<u8>)` - An immutable unsigned 8-bit integer audio buffer reference.
- `U16(&'a super::AudioBuffer<u16>)` - An immutable unsigned 16-bit integer audio buffer reference.
- `U24(&'a super::AudioBuffer<crate::audio::u24>)` - An immutable unsigned 24-bit integer audio buffer reference.
- `U32(&'a super::AudioBuffer<u32>)` - An immutable unsigned 32-bit integer audio buffer reference.
- `S8(&'a super::AudioBuffer<i8>)` - An immutable signed 8-bit integer audio buffer reference.
- `S16(&'a super::AudioBuffer<i16>)` - An immutable signed 16-bit integer audio buffer reference.
- `S24(&'a super::AudioBuffer<crate::audio::i24>)` - An immutable signed 24-bit integer audio buffer reference.
- `S32(&'a super::AudioBuffer<i32>)` - An immutable signed 32-bit integer audio buffer reference.
- `F32(&'a super::AudioBuffer<f32>)` - An immutable single precision (32-bit) floating point audio buffer reference.
- `F64(&'a super::AudioBuffer<f64>)` - An immutable double precision (64-bit) floating point audio buffer reference.

**Methods:**

- `fn spec(self: &Self) -> &AudioSpec` - Get the audio specification.
- `fn num_planes(self: &Self) -> usize` - Get the total number of audio planes.
- `fn is_empty(self: &Self) -> bool` - Returns `true` if there are no audio frames.
- `fn frames(self: &Self) -> usize` - Gets the number of audio frames in the buffer.
- `fn is_unused(self: &Self) -> bool` - Returns `true` if the referenced `AudioBuffer` is unused.
- `fn capacity(self: &Self) -> usize` - Gets the total capacity of the buffer. The capacity is the maximum number of audio frames
- `fn samples_interleaved(self: &Self) -> usize` - Get the total number of samples contained in all audio planes.
- `fn samples_planar(self: &Self) -> usize` - Get the total number of samples contained in each audio plane.
- `fn slice<R>(self: &Self, range: R) -> GenericAudioSlice` - Get an immutable slice of the referenced buffer over `range`.
- `fn copy_to<Sout, Dst>(self: &Self, dst: & mut Dst)` - Copy audio to a mutable audio slice while doing any necessary sample format conversions.
- `fn copy_to_slice_interleaved<Sout, Dst>(self: &Self, dst: Dst)` - Copy all audio frames to a slice of samples in interleaved order.
- `fn copy_to_slice_planar<Sout, Dst>(self: &Self, dst: & mut [Dst])` - Copy all audio planes to discrete slices.
- `fn copy_to_vec_interleaved<Sout>(self: &Self, dst: & mut Vec<Sout>)` - Copy all audio frames to a vector of samples in interleaved order.
- `fn copy_to_vecs_planar<Sout>(self: &Self, dst: & mut Vec<Vec<Sout>>)` - Copy all audio planes to discrete vectors.
- `fn copy_bytes_interleaved_as<Sout, Dst>(self: &Self, dst: Dst)` - Copy interleaved audio to the destination byte slice after converting to a different sample
- `fn copy_bytes_planar_as<Sout, Dst>(self: &Self, dst: & mut [Dst])` - Copy planar audio as bytes to a destination slice per plane after converting to a different
- `fn copy_bytes_interleaved<Dst>(self: &Self, dst: Dst)` - Copy interleaved audio to the destination byte slice.
- `fn copy_bytes_planar<Dst>(self: &Self, dst: & mut [Dst])` - Copy planar audio as bytes to a destination slice per plane.
- `fn copy_bytes_to_vec_interleaved(self: &Self, dst: & mut Vec<u8>)` - Copy interleaved audio to the destination byte vector.
- `fn copy_bytes_to_vec_interleaved_as<Sout>(self: &Self, dst: & mut Vec<u8>)` - Copy interleaved audio to the destination byte vector after converting to a different sample
- `fn copy_bytes_to_vecs_planar(self: &Self, dst: & mut Vec<Vec<u8>>)` - Copy audio planes as bytes to discrete byte vectors.
- `fn copy_bytes_to_vecs_planar_as<Sout>(self: &Self, dst: & mut Vec<Vec<u8>>)` - Copy audio planes as bytes to discrete byte vectors after converting to a different sample
- `fn byte_len_as<Sout>(self: &Self) -> usize` - Get the length in bytes of all samples if converted to a new sample format.
- `fn byte_len_per_plane_as<Sout>(self: &Self) -> usize` - Get the length in bytes of all samples in a single plane if converted to a new sample
- `fn byte_len_per_frame_as<Sout>(self: &Self) -> usize` - Get the length of bytes of a single interleaved audio frame if converted to a new sample
- `fn byte_len(self: &Self) -> usize` - Get the length in bytes of all samples.
- `fn byte_len_per_plane(self: &Self) -> usize` - Get the length in bytes of all samples in a single plane.
- `fn byte_len_per_frame(self: &Self) -> usize` - Get the length of bytes of a single interleaved audio frame.

**Trait Implementations:**

- **From**
  - `fn from(value: &'a AudioBuffer<i8>) -> Self`
- **From**
  - `fn from(value: &'a AudioBuffer<u32>) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **From**
  - `fn from(value: &'a AudioBuffer<f64>) -> Self`
- **From**
  - `fn from(value: &'a AudioBuffer<u24>) -> Self`
- **From**
  - `fn from(value: &'a AudioBuffer<f32>) -> Self`
- **From**
  - `fn from(value: &'a AudioBuffer<u16>) -> Self`
- **From**
  - `fn from(value: &'a AudioBuffer<i32>) -> Self`
- **From**
  - `fn from(value: &'a AudioBuffer<u8>) -> Self`
- **From**
  - `fn from(value: &'a AudioBuffer<i24>) -> Self`
- **From**
  - `fn from(value: &'a AudioBuffer<i16>) -> Self`



## symphonia_core::audio::generic::GenericAudioSlice

*Enum*

An owning wrapper for an [`AudioSlice`] of any standard sample format.

Calls on this wrapper are dispatched to the underlying, wrapped, slice and are semantically
identical.

**Generic Parameters:**
- 'a

**Variants:**
- `U8(super::AudioSlice<'a, u8>)` - An unsigned 8-bit integer audio slice.
- `U16(super::AudioSlice<'a, u16>)` - An unsigned 16-bit integer audio slice.
- `U24(super::AudioSlice<'a, crate::audio::u24>)` - An unsigned 24-bit integer audio slice.
- `U32(super::AudioSlice<'a, u32>)` - An unsigned 32-bit integer audio slice.
- `S8(super::AudioSlice<'a, i8>)` - A signed 8-bit integer audio slice.
- `S16(super::AudioSlice<'a, i16>)` - A signed 16-bit integer audio slice.
- `S24(super::AudioSlice<'a, crate::audio::i24>)` - A signed 24-bit integer audio slice.
- `S32(super::AudioSlice<'a, i32>)` - A signed 32-bit integer audio slice.
- `F32(super::AudioSlice<'a, f32>)` - A single precision (32-bit) floating point audio slice.
- `F64(super::AudioSlice<'a, f64>)` - A double precision (64-bit) floating point audio slice.

**Methods:**

- `fn spec(self: &Self) -> &AudioSpec` - Get the audio specification.
- `fn num_planes(self: &Self) -> usize` - Get the total number of audio planes.
- `fn is_empty(self: &Self) -> bool` - Returns `true` if there are no audio frames.
- `fn frames(self: &Self) -> usize` - Gets the number of audio frames in the slice.
- `fn samples_interleaved(self: &Self) -> usize` - Get the total number of samples contained in all audio planes.
- `fn samples_planar(self: &Self) -> usize` - Get the total number of samples contained in each audio plane.
- `fn slice<R>(self: &'a Self, range: R) -> GenericAudioSlice<'a>` - Get an immutable sub-slice of this slice over `range`.
- `fn copy_to<Sout, Dst>(self: &Self, dst: & mut Dst)` - Copy audio to a mutable audio slice while doing any necessary sample format conversions.
- `fn copy_to_slice_interleaved<Sout, Dst>(self: &Self, dst: Dst)` - Copy all audio frames to a slice of samples in interleaved order.
- `fn copy_to_slice_planar<Sout, Dst>(self: &Self, dst: & mut [Dst])` - Copy all audio planes to discrete slices.
- `fn copy_to_vec_interleaved<Sout>(self: &Self, dst: & mut Vec<Sout>)` - Copy all audio frames to a vector of samples in interleaved order.
- `fn copy_to_vecs_planar<Sout>(self: &Self, dst: & mut Vec<Vec<Sout>>)` - Copy all audio planes to discrete vectors.
- `fn copy_bytes_interleaved_as<Sout, Dst>(self: &Self, dst: Dst)` - Copy interleaved audio to the destination byte slice after converting to a different sample
- `fn copy_bytes_planar_as<Sout, Dst>(self: &Self, dst: & mut [Dst])` - Copy planar audio as bytes to a destination slice per plane after converting to a different
- `fn copy_bytes_interleaved<Dst>(self: &Self, dst: Dst)` - Copy interleaved audio to the destination byte slice.
- `fn copy_bytes_planar<Dst>(self: &Self, dst: & mut [Dst])` - Copy planar audio as bytes to a destination slice per plane.
- `fn copy_bytes_to_vec_interleaved(self: &Self, dst: & mut Vec<u8>)` - Copy interleaved audio to the destination byte vector.
- `fn copy_bytes_to_vec_interleaved_as<Sout>(self: &Self, dst: & mut Vec<u8>)` - Copy interleaved audio to the destination byte vector after converting to a different sample
- `fn copy_bytes_to_vecs_planar(self: &Self, dst: & mut Vec<Vec<u8>>)` - Copy audio planes as bytes to discrete byte vectors.
- `fn copy_bytes_to_vecs_planar_as<Sout>(self: &Self, dst: & mut Vec<Vec<u8>>)` - Copy audio planes as bytes to discrete byte vectors after converting to a different sample
- `fn byte_len_as<Sout>(self: &Self) -> usize` - Get the length in bytes of all samples if converted to a new sample format.
- `fn byte_len_per_plane_as<Sout>(self: &Self) -> usize` - Get the length in bytes of all samples in a single plane if converted to a new sample
- `fn byte_len_per_frame_as<Sout>(self: &Self) -> usize` - Get the length of bytes of a single interleaved audio frame if converted to a new sample
- `fn byte_len(self: &Self) -> usize` - Get the length in bytes of all samples.
- `fn byte_len_per_plane(self: &Self) -> usize` - Get the length in bytes of all samples in a single plane.
- `fn byte_len_per_frame(self: &Self) -> usize` - Get the length of bytes of a single interleaved audio frame.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`



