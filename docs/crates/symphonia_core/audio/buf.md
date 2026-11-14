**symphonia_core > audio > buf**

# Module: audio::buf

## Contents

**Structs**

- [`AudioBuffer`](#audiobuffer) - A dynamically sized container for multi-channel planar audio for a known sample format.

---

## symphonia_core::audio::buf::AudioBuffer

*Struct*

A dynamically sized container for multi-channel planar audio for a known sample format.

An `AudioBuffer` is characterized by a total capacity, and an audio specification describing the
audio channels and sample rate. The capacity of an `AudioBuffer` is the fixed number of audio
frames the buffer may store without growing.

Note, `AudioBuffer` does not automatically grow its capacity. All increases in capacity must
be manually requested.

`AudioBuffer` stores each audio channel in separate audio planes. An audio plane is a
continguous vector of audio samples. The samples across all audio planes at any one particular
index form an audio frame. The order of audio planes, and thus the order of samples in an
audio frame, is a canonical order based on the channels mask.

## Audio Plane Ordering

The order and indexing of the audio planes in an audio buffer is based on the channel
representation specified in the audio specification.

  * If [`Channels::Positioned`](super::Channels::Positioned), the audio buffer contains one
    audio plane for each bit set in the specified [`Position`](super::Position) bitmask. The
    order of audio planes is based on the position bits set to `1` from the least-significant
    bit (LSB) to most-significant bit (MSB) in the mask. This order is known as the canonical
    channel order. The audio buffer may be indexed by a position mask containing exactly one
    position, or a canonical index.
  * If [`Channels::Discrete`](super::Channels::Discrete), the total number of discrete channels
    (`n`) is specified. The audio buffer contains one audio plane for discrete channel indicies
    `0..n`. The buffer may be indexed by a discrete channel index, `i`, in the range `0..n`.
  * If [`Channels::Ambisonic`](super::Channels::Ambisonic), the Ambisonic order (`n`) is
    specified. The audio buffer contains `m = (1 + n) ^ 2` audio planes, one for each Ambisonic
    component. The order of audio planes is based on the Ambisonic Channel Number (ACN). The
    audio buffer may be indexed by the ACN of the desired channel, `k`, in the range `0..m`.
  * If [`Channels::Custom`](super::Channels::Custom), a list of `n`
    [`ChannelLabel`](super::ChannelLabel)s is specified. Each channel label describes the
    disposition of the channel. The audio buffer contains `n` audio planes in the same order
    as specified in the list. The audio buffer may be indexed by an index, `i`, in the range
    `0..n`.

## Realtime Safety

Unless explictly noted, `AudioBuffer` never allocates after instantiation.

**Generic Parameters:**
- S

**Methods:**

- `fn new(spec: AudioSpec, capacity: usize) -> Self` - Instantiate a new `AudioBuffer` using the specified signal specification and an
- `fn is_unused(self: &Self) -> bool` - Returns `true` if the `AudioBuffer` is unused.
- `fn capacity(self: &Self) -> usize` - Gets the total capacity of the buffer. The capacity is the maximum number of audio frames
- `fn clear(self: & mut Self)` - Clears all audio frames.
- `fn grow_capacity(self: & mut Self, new_capacity: usize)` - Grows the capacity of the buffer if the new capacity is larger than the current capacity.
- `fn slice<R>(self: &Self, range: R) -> AudioSlice<S>` - Get an immutable slice of the buffer over `range`.
- `fn slice_mut<R>(self: & mut Self, range: R) -> AudioSliceMut<S>` - Get a mutable slice of the buffer over `range`.
- `fn append<Sin, Src>(self: & mut Self, src: &Src)` - Append the frames from the audio buffer `src` to this buffer.
- `fn resize(self: & mut Self, new_len: usize, frame: &[S])` - Resizes the buffer such that the number of frames is `new_len`. New frames are copies of
- `fn resize_with_sample(self: & mut Self, new_len: usize, sample: S)` - Resizes the buffer such that the number of frames is `new_len`. New frames repeat `sample`
- `fn resize_with_silence(self: & mut Self, new_len: usize)` - Resizes the buffer such that the number of frames is `new_len`. New frames are silent.
- `fn resize_uninit(self: & mut Self, new_len: usize)` - Resizes the buffer such that the number of frames is `new_len`. New frames are left
- `fn render(self: & mut Self, num_frames: Option<usize>, frame: &[S]) -> usize` - Renders a number of repeated frames.
- `fn render_with<F>(self: & mut Self, num_frames: Option<usize>, f: F) -> Result<usize>` - Renders a number of frames using the provided render function.
- `fn render_silence(self: & mut Self, num_frames: Option<usize>) -> usize` - Renders a number of silent frames.
- `fn render_uninit(self: & mut Self, num_frames: Option<usize>) -> usize` - Renders an uninitialized number of frames.
- `fn shift(self: & mut Self, shift: usize)` - Shifts the contents of the buffer back by the number of frames specified. The leading frames
- `fn truncate(self: & mut Self, num_frames: usize)` - Truncates the buffer to the number of frames specified. If the number of frames in the
- `fn trim(self: & mut Self, start: usize, end: usize)` - Trims frames from the start and end of the buffer.

**Trait Implementations:**

- **TryFrom**
  - `fn try_from(value: GenericAudioBuffer) -> Result<Self, <Self as >::Error>`
- **AudioBytes**
  - `fn copy_bytes_interleaved_as<Sout, Dst>(self: &Self, dst: Dst)`
  - `fn copy_bytes_planar_as<Sout, Dst>(self: &Self, dst: & mut [Dst])`
  - `fn copy_bytes_interleaved<Dst>(self: &Self, dst: Dst)`
  - `fn copy_bytes_planar<Dst>(self: &Self, dst: & mut [Dst])`
- **TryFrom**
  - `fn try_from(value: GenericAudioBuffer) -> Result<Self, <Self as >::Error>`
- **AsGenericAudioBufferRef**
  - `fn as_generic_audio_buffer_ref(self: &Self) -> GenericAudioBufferRef`
- **AsGenericAudioBufferRef**
  - `fn as_generic_audio_buffer_ref(self: &Self) -> GenericAudioBufferRef`
- **TryFrom**
  - `fn try_from(value: GenericAudioBuffer) -> Result<Self, <Self as >::Error>`
- **Index**
  - `fn index(self: &Self, index: Position) -> &<Self as >::Output`
- **AsGenericAudioBufferRef**
  - `fn as_generic_audio_buffer_ref(self: &Self) -> GenericAudioBufferRef`
- **TryFrom**
  - `fn try_from(value: GenericAudioBuffer) -> Result<Self, <Self as >::Error>`
- **AudioMut**
  - `fn plane_mut(self: & mut Self, idx: usize) -> Option<& mut [S]>`
  - `fn plane_pair_mut(self: & mut Self, idx0: usize, idx1: usize) -> Option<(& mut [S], & mut [S])>`
  - `fn iter_planes_mut(self: & mut Self) -> AudioPlanesMut<S>`
  - `fn copy_from_slice_interleaved<Sin, Src>(self: & mut Self, src: &Src)`
- **Index**
  - `fn index(self: &Self, index: usize) -> &<Self as >::Output`
- **AsGenericAudioBufferRef**
  - `fn as_generic_audio_buffer_ref(self: &Self) -> GenericAudioBufferRef`
- **TryFrom**
  - `fn try_from(value: GenericAudioBuffer) -> Result<Self, <Self as >::Error>`
- **AsGenericAudioBufferRef**
  - `fn as_generic_audio_buffer_ref(self: &Self) -> GenericAudioBufferRef`
- **Audio**
  - `fn spec(self: &Self) -> &AudioSpec`
  - `fn num_planes(self: &Self) -> usize`
  - `fn is_empty(self: &Self) -> bool`
  - `fn frames(self: &Self) -> usize`
  - `fn plane(self: &Self, idx: usize) -> Option<&[S]>`
  - `fn plane_pair(self: &Self, idx0: usize, idx1: usize) -> Option<(&[S], &[S])>`
  - `fn iter_planes(self: &Self) -> AudioPlanes<S>`
  - `fn iter_interleaved(self: &Self) -> Interleaved<S>`
  - `fn copy_to_slice_interleaved<Sout, Dst>(self: &Self, dst: Dst)`
- **TryFrom**
  - `fn try_from(value: GenericAudioBuffer) -> Result<Self, <Self as >::Error>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **IndexMut**
  - `fn index_mut(self: & mut Self, index: Position) -> & mut <Self as >::Output`
- **AsGenericAudioBufferRef**
  - `fn as_generic_audio_buffer_ref(self: &Self) -> GenericAudioBufferRef`
- **TryFrom**
  - `fn try_from(value: GenericAudioBuffer) -> Result<Self, <Self as >::Error>`
- **IndexMut**
  - `fn index_mut(self: & mut Self, index: usize) -> & mut <Self as >::Output`
- **AsGenericAudioBufferRef**
  - `fn as_generic_audio_buffer_ref(self: &Self) -> GenericAudioBufferRef`
- **AsGenericAudioBufferRef**
  - `fn as_generic_audio_buffer_ref(self: &Self) -> GenericAudioBufferRef`
- **AudioBufferBytes**
  - `fn max_byte_len_per_plane_as<Sout>(self: &Self) -> usize`
  - `fn max_byte_len_per_plane(self: &Self) -> usize`
- **TryFrom**
  - `fn try_from(value: GenericAudioBuffer) -> Result<Self, <Self as >::Error>`
- **AsGenericAudioBufferRef**
  - `fn as_generic_audio_buffer_ref(self: &Self) -> GenericAudioBufferRef`
- **Default**
  - `fn default() -> Self`
- **TryFrom**
  - `fn try_from(value: GenericAudioBuffer) -> Result<Self, <Self as >::Error>`
- **TryFrom**
  - `fn try_from(value: GenericAudioBuffer) -> Result<Self, <Self as >::Error>`
- **AsGenericAudioBufferRef**
  - `fn as_generic_audio_buffer_ref(self: &Self) -> GenericAudioBufferRef`



