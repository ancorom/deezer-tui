**rtrb > chunks**

# Module: chunks

## Contents

**Structs**

- [`ReadChunk`](#readchunk) - Structure for reading from multiple slots in one go.
- [`ReadChunkIntoIter`](#readchunkintoiter) - An iterator that moves out of a [`ReadChunk`].
- [`WriteChunk`](#writechunk) - Structure for writing into multiple ([`Default`]-initialized) slots in one go.
- [`WriteChunkUninit`](#writechunkuninit) - Structure for writing into multiple (uninitialized) slots in one go.

**Enums**

- [`ChunkError`](#chunkerror) - Error type for [`Consumer::read_chunk()`], [`Consumer::pop_entire_slice()`],

---

## rtrb::chunks::ChunkError

*Enum*

Error type for [`Consumer::read_chunk()`], [`Consumer::pop_entire_slice()`],
[`Consumer::pop_entire_slice_uninit()`], [`Producer::write_chunk()`],
[`Producer::write_chunk_uninit()`] and [`Producer::push_entire_slice()`].

**Variants:**
- `TooFewSlots(usize)` - Fewer than the requested number of slots were available.

**Traits:** Error, Eq, Copy

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rtrb::chunks::ReadChunk

*Struct*

Structure for reading from multiple slots in one go.

This is returned from [`Consumer::read_chunk()`].

**Generic Parameters:**
- 'a
- T

**Methods:**

- `fn as_slices(self: &Self) -> (&[T], &[T])` - Returns two slices for reading from the requested slots.
- `fn as_mut_slices(self: & mut Self) -> (& mut [T], & mut [T])` - Returns two mutable slices for reading from the requested slots.
- `fn commit(self: Self, n: usize)` - Drops the first `n` slots of the chunk, making the space available for writing again.
- `fn commit_all(self: Self)` - Drops all slots of the chunk, making the space available for writing again.
- `fn len(self: &Self) -> usize` - Returns the number of slots in the chunk.
- `fn is_empty(self: &Self) -> bool` - Returns `true` if the chunk contains no slots.

**Traits:** Eq, Send

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter` - Turns a [`ReadChunk`] into an iterator.



## rtrb::chunks::ReadChunkIntoIter

*Struct*

An iterator that moves out of a [`ReadChunk`].

This `struct` is created by the [`into_iter()`](ReadChunk::into_iter) method
on [`ReadChunk`] (provided by the [`IntoIterator`] trait).

When this `struct` is dropped, the iterated slots are made available for writing again.
Non-iterated items remain in the ring buffer.
If this `struct` is *not* dropped for some reason (e.g. via [`std::mem::forget()`]),
all non-iterated items are leaked.

**Generic Parameters:**
- 'a
- T

**Methods:**

- `fn iterated(self: &Self) -> usize` - Returns the number of items consumed from this iterator.

**Traits:** FusedIterator, ExactSizeIterator

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
  - `fn fold<B, F>(self: Self, init: B, f: F) -> B`
  - `fn count(self: Self) -> usize`
- **Drop**
  - `fn drop(self: & mut Self)` - Makes all iterated slots available for writing again.
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rtrb::chunks::WriteChunk

*Struct*

Structure for writing into multiple ([`Default`]-initialized) slots in one go.

This is returned from [`Producer::write_chunk()`].

To obtain uninitialized slots, use [`Producer::write_chunk_uninit()`] instead,
which also allows moving items from an iterator into the ring buffer
by means of [`WriteChunkUninit::fill_from_iter()`].

**Generic Parameters:**
- 'a
- T

**Tuple Struct**: `()`

**Methods:**

- `fn as_mut_slices(self: & mut Self) -> (& mut [T], & mut [T])` - Returns two slices for writing to the requested slots.
- `fn commit(self: Self, n: usize)` - Makes the first `n` slots of the chunk available for reading.
- `fn commit_all(self: Self)` - Makes the whole chunk available for reading.
- `fn len(self: &Self) -> usize` - Returns the number of slots in the chunk.
- `fn is_empty(self: &Self) -> bool` - Returns `true` if the chunk contains no slots.

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(chunk: WriteChunkUninit<'a, T>) -> Self` - Fills all slots with the [`Default`] value.
- **Drop**
  - `fn drop(self: & mut Self)`



## rtrb::chunks::WriteChunkUninit

*Struct*

Structure for writing into multiple (uninitialized) slots in one go.

This is returned from [`Producer::write_chunk_uninit()`].

**Generic Parameters:**
- 'a
- T

**Methods:**

- `fn as_mut_slices(self: & mut Self) -> (& mut [MaybeUninit<T>], & mut [MaybeUninit<T>])` - Returns two slices for writing to the requested slots.
- `fn commit(self: Self, n: usize)` - Makes the first `n` slots of the chunk available for reading.
- `fn commit_all(self: Self)` - Makes the whole chunk available for reading.
- `fn fill_from_iter<I>(self: Self, iter: I) -> usize` - Moves items from an iterator into the (uninitialized) slots of the chunk.
- `fn len(self: &Self) -> usize` - Returns the number of slots in the chunk.
- `fn is_empty(self: &Self) -> bool` - Returns `true` if the chunk contains no slots.

**Traits:** Send, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



