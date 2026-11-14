**image > images > buffer_par**

# Module: images::buffer_par

## Contents

**Structs**

- [`EnumeratePixelsMutPar`](#enumeratepixelsmutpar) - Parallel iterator over mutable pixel refs and their coordinates.
- [`EnumeratePixelsPar`](#enumeratepixelspar) - Parallel iterator over pixel refs and their coordinates.
- [`PixelsMutPar`](#pixelsmutpar) - Parallel iterator over mutable pixel refs.
- [`PixelsPar`](#pixelspar) - Parallel iterator over pixel refs.

---

## image::images::buffer_par::EnumeratePixelsMutPar

*Struct*

Parallel iterator over mutable pixel refs and their coordinates.

**Generic Parameters:**
- 'a
- P

**Trait Implementations:**

- **IndexedParallelIterator**
  - `fn drive<C>(self: Self, consumer: C) -> <C as >::Result`
  - `fn len(self: &Self) -> usize`
  - `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output`
- **ParallelIterator**
  - `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result`
  - `fn opt_len(self: &Self) -> Option<usize>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## image::images::buffer_par::EnumeratePixelsPar

*Struct*

Parallel iterator over pixel refs and their coordinates.

**Generic Parameters:**
- 'a
- P

**Trait Implementations:**

- **IndexedParallelIterator**
  - `fn drive<C>(self: Self, consumer: C) -> <C as >::Result`
  - `fn len(self: &Self) -> usize`
  - `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output`
- **ParallelIterator**
  - `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result`
  - `fn opt_len(self: &Self) -> Option<usize>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## image::images::buffer_par::PixelsMutPar

*Struct*

Parallel iterator over mutable pixel refs.

**Generic Parameters:**
- 'a
- P

**Trait Implementations:**

- **IndexedParallelIterator**
  - `fn drive<C>(self: Self, consumer: C) -> <C as >::Result`
  - `fn len(self: &Self) -> usize`
  - `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output`
- **ParallelIterator**
  - `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result`
  - `fn opt_len(self: &Self) -> Option<usize>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## image::images::buffer_par::PixelsPar

*Struct*

Parallel iterator over pixel refs.

**Generic Parameters:**
- 'a
- P

**Trait Implementations:**

- **IndexedParallelIterator**
  - `fn drive<C>(self: Self, consumer: C) -> <C as >::Result`
  - `fn len(self: &Self) -> usize`
  - `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **ParallelIterator**
  - `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result`
  - `fn opt_len(self: &Self) -> Option<usize>`



