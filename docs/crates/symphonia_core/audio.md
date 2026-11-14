**symphonia_core > audio**

# Module: audio

## Contents

**Modules**

- [`conv`](#conv) - The `conv` module provides methods to convert samples between different sample types (formats).
- [`sample`](#sample) - The `sample` module defines the audio sample trait and implements any non-primitive sample data

**Structs**

- [`AudioPlanes`](#audioplanes) - Iterator over immutable audio plane slices.
- [`AudioPlanesMut`](#audioplanesmut) - Iterator over mutable audio plane slices.
- [`AudioSpec`](#audiospec) - A specification defining the core characteristics of some audio.
- [`Interleaved`](#interleaved) - Iterator over interleaved samples.

**Traits**

- [`Audio`](#audio) - Trait for accessing immutable planar audio.
- [`AudioBufferBytes`](#audiobufferbytes) - Trait for querying the maximum capacity in bytes for dynamically sized audio storage.
- [`AudioBytes`](#audiobytes) - Trait for marshalling planar audio to byte buffers.
- [`AudioMut`](#audiomut) - Trait for manipulating mutable planar audio.

---

## symphonia_core::audio::Audio

*Trait*

Trait for accessing immutable planar audio.

**Methods:**

- `spec`: Get the audio specification.
- `num_planes`: Get the total number of audio planes.
- `is_empty`: Returns `true` if there are no audio frames.
- `frames`: Gets the number of audio frames in the buffer.
- `plane`: Try to get an immutable slice to the audio plane at the canonical buffer index `idx`.
- `plane_by_position`: Try to get an immutable slice to the audio plane at position `pos`.
- `plane_pair`: Try to get immutable slices to a pair of audio planes at the canonical buffer indicies
- `plane_pair_by_position`: Try to get an immutable slice to a pair of audio planes at positions `pos0` and `pos1`.
- `iter_planes`: Iterate over immutable slices to all planes.
- `iter_interleaved`: Iterate over all samples in an interleaved order.
- `samples_interleaved`: Get the total number of samples contained in all audio planes.
- `samples_planar`: Get the total number of samples contained in each audio plane.
- `copy_to_slice_interleaved`: Copy all audio frames to a slice of samples in interleaved order.
- `copy_to_slice_planar`: Copy all audio planes to discrete slices.
- `copy_to_vec_interleaved`: Copy all audio frames to a vector of samples in interleaved order.
- `copy_to_vecs_planar`: Copy all audio planes to discrete vectors.



## symphonia_core::audio::AudioBufferBytes

*Trait*

Trait for querying the maximum capacity in bytes for dynamically sized audio storage.

**Methods:**

- `max_byte_len_as`: Get the maximum possible length in bytes of all samples after converting to a different
- `max_byte_len_per_plane_as`: Get the maximum possible length in bytes of all samples in a single plane after converting
- `max_byte_len`: Get the maximum possible length in bytes of all samples.
- `max_byte_len_per_plane`: Get the maximum possible length in bytes of all samples in a single plane.



## symphonia_core::audio::AudioBytes

*Trait*

Trait for marshalling planar audio to byte buffers.

**Methods:**

- `byte_len_as`: Get the length in bytes of all samples if converted to a new sample format.
- `byte_len_per_plane_as`: Get the length in bytes of all samples in a single plane if converted to a new sample
- `byte_len_per_frame_as`: Get the length of bytes of a single interleaved audio frame if converted to a new sample
- `copy_bytes_interleaved_as`: Copy interleaved audio to the destination byte slice after converting to a different sample
- `copy_bytes_planar_as`: Copy planar audio as bytes to a destination slice per plane after converting to a different
- `byte_len`: Get the length in bytes of all samples.
- `byte_len_per_plane`: Get the length in bytes of all samples in a single plane.
- `byte_len_per_frame`: Get the length of bytes of a single interleaved audio frame.
- `copy_bytes_interleaved`: Copy interleaved audio to the destination byte slice.
- `copy_bytes_planar`: Copy planar audio as bytes to a destination slice per plane.
- `copy_bytes_to_vec_interleaved`: Copy interleaved audio to the destination byte vector.
- `copy_bytes_to_vec_interleaved_as`: Copy interleaved audio to the destination byte vector after converting to a different sample
- `copy_bytes_to_vecs_planar`: Copy audio planes as bytes to discrete byte vectors.
- `copy_bytes_to_vecs_planar_as`: Copy audio planes as bytes to discrete byte vectors after converting to a different sample



## symphonia_core::audio::AudioMut

*Trait*

Trait for manipulating mutable planar audio.

**Methods:**

- `plane_mut`: Try to get a mutable slice to the audio plane at the canonical buffer index `idx`.
- `plane_by_position_mut`: Try to get a mutable slice to the audio plane at position `pos`.
- `plane_pair_mut`: Try to get mutable slices to a pair of audio planes at the canonical buffer indicies `idx0`
- `plane_pair_by_position_mut`: Try to get mutable slices to a pair of audio planes at positions `pos0` and `pos1`.
- `iter_planes_mut`: Iterate over mutable slices to all planes.
- `apply`: Applies a transformation function over the samples of all planes.
- `copy_from`: Copy audio from a source.
- `copy_from_slice_planar`: Copy audio from a slice of slices.
- `copy_from_slice_interleaved`: Copy audio from an interleaved slice of samples.



## symphonia_core::audio::AudioPlanes

*Struct*

Iterator over immutable audio plane slices.

**Generic Parameters:**
- 'a
- S

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## symphonia_core::audio::AudioPlanesMut

*Struct*

Iterator over mutable audio plane slices.

**Generic Parameters:**
- 'a
- S

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## symphonia_core::audio::AudioSpec

*Struct*

A specification defining the core characteristics of some audio.

**Methods:**

- `fn new(rate: u32, channels: Channels) -> Self` - Create an audio specification from a sample rate in Hertz (Hz) and set of channels.
- `fn rate(self: &Self) -> u32` - Get the sample rate in Hz.
- `fn channels(self: &Self) -> &Channels` - Get the channels.

**Traits:** Eq

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## symphonia_core::audio::Interleaved

*Struct*

Iterator over interleaved samples.

**Generic Parameters:**
- 'a
- S

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## Module: conv

The `conv` module provides methods to convert samples between different sample types (formats).



## Module: sample

The `sample` module defines the audio sample trait and implements any non-primitive sample data
types.



