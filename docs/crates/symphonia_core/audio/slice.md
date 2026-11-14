**symphonia_core > audio > slice**

# Module: audio::slice

## Contents

**Structs**

- [`AudioSlice`](#audioslice) - An immutable slice of planar audio.
- [`AudioSliceMut`](#audioslicemut) - A mutable slice of planar audio.

---

## symphonia_core::audio::slice::AudioSlice

*Struct*

An immutable slice of planar audio.

**Generic Parameters:**
- 'a
- S

**Methods:**

- `fn slice<R>(self: &'a Self, range: R) -> AudioSlice<'a, S>` - Get an immutable sub-slice of this slice over `range`.

**Trait Implementations:**

- **AudioBytes**
  - `fn copy_bytes_interleaved_as<Sout, Dst>(self: &Self, dst: Dst)`
  - `fn copy_bytes_planar_as<Sout, Dst>(self: &Self, dst: & mut [Dst])`
  - `fn copy_bytes_interleaved<Dst>(self: &Self, dst: Dst)`
  - `fn copy_bytes_planar<Dst>(self: &Self, dst: & mut [Dst])`
- **Index**
  - `fn index(self: &Self, index: usize) -> &<Self as >::Output`
- **Index**
  - `fn index(self: &Self, index: Position) -> &<Self as >::Output`
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
- **Clone**
  - `fn clone(self: &Self) -> Self`



## symphonia_core::audio::slice::AudioSliceMut

*Struct*

A mutable slice of planar audio.

**Generic Parameters:**
- 'a
- S

**Methods:**

- `fn slice<R>(self: &'a Self, range: R) -> AudioSlice<'a, S>` - Get an immutable sub-slice of this slice over `range`.
- `fn slice_mut<R>(self: &'a  mut Self, range: R) -> AudioSliceMut<'a, S>` - Get a mutable sub-slice of this slice over `range`.

**Trait Implementations:**

- **IndexMut**
  - `fn index_mut(self: & mut Self, index: usize) -> & mut <Self as >::Output`
- **AudioMut**
  - `fn plane_mut(self: & mut Self, idx: usize) -> Option<& mut [S]>`
  - `fn plane_pair_mut(self: & mut Self, idx0: usize, idx1: usize) -> Option<(& mut [S], & mut [S])>`
  - `fn iter_planes_mut(self: & mut Self) -> AudioPlanesMut<S>`
  - `fn copy_from_slice_interleaved<Sin, Src>(self: & mut Self, src: &Src)`
- **Index**
  - `fn index(self: &Self, index: Position) -> &<Self as >::Output`
- **AudioBytes**
  - `fn copy_bytes_interleaved_as<Sout, Dst>(self: &Self, dst: Dst)`
  - `fn copy_bytes_planar_as<Sout, Dst>(self: &Self, dst: & mut [Dst])`
  - `fn copy_bytes_interleaved<Dst>(self: &Self, dst: Dst)`
  - `fn copy_bytes_planar<Dst>(self: &Self, dst: & mut [Dst])`
- **Index**
  - `fn index(self: &Self, index: usize) -> &<Self as >::Output`
- **IndexMut**
  - `fn index_mut(self: & mut Self, index: Position) -> & mut <Self as >::Output`
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



