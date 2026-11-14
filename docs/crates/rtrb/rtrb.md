**rtrb**

# Module: rtrb

## Contents

**Modules**

- [`chunks`](#chunks) - Writing and reading multiple items at once into and from a [`RingBuffer`].

**Structs**

- [`Consumer`](#consumer) - The consumer side of a [`RingBuffer`].
- [`Producer`](#producer) - The producer side of a [`RingBuffer`].
- [`RingBuffer`](#ringbuffer) - A bounded single-producer single-consumer (SPSC) queue.

**Enums**

- [`PeekError`](#peekerror) - Error type for [`Consumer::peek()`].
- [`PopError`](#poperror) - Error type for [`Consumer::pop()`].
- [`PushError`](#pusherror) - Error type for [`Producer::push()`].

**Traits**

- [`CopyToUninit`](#copytouninit) - Extension trait used to provide a [`copy_to_uninit()`](CopyToUninit::copy_to_uninit)

---

## rtrb::Consumer

*Struct*

The consumer side of a [`RingBuffer`].

Can be moved between threads,
but references from different threads are not allowed
(i.e. it is [`Send`] but not [`Sync`]).

Can only be created with [`RingBuffer::new()`]
(together with its counterpart, the [`Producer`]).

Individual elements can be moved out of the ring buffer with [`Consumer::pop()`],
multiple elements at once can be read with [`Consumer::read_chunk()`],
[`Consumer::pop_partial_slice()`] and [`Consumer::pop_partial_slice_uninit()`].

The number of slots currently available for reading can be obtained with
[`Consumer::slots()`].

When the `Consumer` is dropped, [`Producer::is_abandoned()`] will return `true`.
This can be used as a crude way to communicate to the sending thread
that no more data will be consumed.
When the `Consumer` is dropped after the [`Producer`] has already been dropped,
[`RingBuffer::drop()`] will be called, freeing the allocated memory.

**Generic Parameters:**
- T

**Methods:**

- `fn pop(self: & mut Self) -> Result<T, PopError>` - Attempts to pop an element from the queue.
- `fn peek(self: &Self) -> Result<&T, PeekError>` - Attempts to read an element from the queue without removing it.
- `fn slots(self: &Self) -> usize` - Returns the number of slots available for reading.
- `fn cached_slots(self: &Self) -> usize` - Returns the number of cached slots.
- `fn is_empty(self: &Self) -> bool` - Returns `true` if there are currently no slots available for reading.
- `fn is_abandoned(self: &Self) -> bool` - Returns `true` if the corresponding [`Producer`] has been destroyed.
- `fn buffer(self: &Self) -> &RingBuffer<T>` - Returns a read-only reference to the ring buffer.
- `fn pop_partial_slice<'a>(self: & mut Self, slice: &'a  mut [T]) -> (&'a  mut [T], &'a  mut [T])` - Copies as many items as possible from the ring buffer to the given `slice`.
- `fn pop_partial_slice_uninit<'a>(self: & mut Self, slice: &'a  mut [MaybeUninit<T>]) -> (&'a  mut [T], &'a  mut [MaybeUninit<T>])` - Copies as many items as possible from the ring buffer to the given uninitialized `slice`.
- `fn pop_entire_slice(self: & mut Self, slice: & mut [T]) -> Result<(), ChunkError>` - Copies as many items from the ring buffer as to fill the given `slice`.
- `fn pop_entire_slice_uninit<'a>(self: & mut Self, slice: &'a  mut [MaybeUninit<T>]) -> Result<&'a  mut [T], ChunkError>` - Copies as many items from the ring buffer as to fill the given uninitialized `slice`.
- `fn read_chunk(self: & mut Self, n: usize) -> Result<ReadChunk<T>, ChunkError>` - Returns `n` slots for reading.

**Traits:** Send, Eq

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Read**
  - `fn read(self: & mut Self, buf: & mut [u8]) -> std::io::Result<usize>`



## rtrb::CopyToUninit

*Trait*

Extension trait used to provide a [`copy_to_uninit()`](CopyToUninit::copy_to_uninit)
method on built-in slices.

This can be used to safely copy data to the slices returned from
[`WriteChunkUninit::as_mut_slices()`].

To use this, the trait has to be brought into scope, e.g. with:

```
use rtrb::CopyToUninit;
```

**Methods:**

- `copy_to_uninit`: Copies contents to a possibly uninitialized slice.



## rtrb::PeekError

*Enum*

Error type for [`Consumer::peek()`].

**Variants:**
- `Empty` - The queue was empty.

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



## rtrb::PopError

*Enum*

Error type for [`Consumer::pop()`].

**Variants:**
- `Empty` - The queue was empty.

**Traits:** Copy, Error, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## rtrb::Producer

*Struct*

The producer side of a [`RingBuffer`].

Can be moved between threads,
but references from different threads are not allowed
(i.e. it is [`Send`] but not [`Sync`]).

Can only be created with [`RingBuffer::new()`]
(together with its counterpart, the [`Consumer`]).

Individual elements can be moved into the ring buffer with [`Producer::push()`],
multiple elements at once can be written with [`Producer::write_chunk()`],
[`Producer::write_chunk_uninit()`] and [`Producer::push_partial_slice()`].

The number of free slots currently available for writing can be obtained with
[`Producer::slots()`].

When the `Producer` is dropped, [`Consumer::is_abandoned()`] will return `true`.
This can be used as a crude way to communicate to the receiving thread
that no more data will be produced.
When the `Producer` is dropped after the [`Consumer`] has already been dropped,
[`RingBuffer::drop()`] will be called, freeing the allocated memory.

**Generic Parameters:**
- T

**Methods:**

- `fn push(self: & mut Self, value: T) -> Result<(), PushError<T>>` - Attempts to push an element into the queue.
- `fn slots(self: &Self) -> usize` - Returns the number of slots available for writing.
- `fn cached_slots(self: &Self) -> usize` - Returns the number of cached slots.
- `fn is_full(self: &Self) -> bool` - Returns `true` if there are currently no slots available for writing.
- `fn is_abandoned(self: &Self) -> bool` - Returns `true` if the corresponding [`Consumer`] has been destroyed.
- `fn buffer(self: &Self) -> &RingBuffer<T>` - Returns a read-only reference to the ring buffer.
- `fn push_partial_slice<'a>(self: & mut Self, slice: &'a [T]) -> (&'a [T], &'a [T])` - Copies as many items as possible from the given `slice` into the ring buffer.
- `fn push_entire_slice(self: & mut Self, slice: &[T]) -> Result<(), ChunkError>` - Copies all items from the given `slice` into the ring buffer.
- `fn write_chunk(self: & mut Self, n: usize) -> Result<WriteChunk<T>, ChunkError>` - Returns `n` slots (initially containing their [`Default`] value) for writing.
- `fn write_chunk_uninit(self: & mut Self, n: usize) -> Result<WriteChunkUninit<T>, ChunkError>` - Returns `n` (uninitialized) slots for writing.

**Traits:** Send, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Write**
  - `fn write(self: & mut Self, buf: &[u8]) -> std::io::Result<usize>`
  - `fn flush(self: & mut Self) -> std::io::Result<()>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## rtrb::PushError

*Enum*

Error type for [`Producer::push()`].

**Generic Parameters:**
- T

**Variants:**
- `Full(T)` - The queue was full.

**Traits:** Error, Eq, Copy

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## rtrb::RingBuffer

*Struct*

A bounded single-producer single-consumer (SPSC) queue.

Elements can be written with a [`Producer`] and read with a [`Consumer`],
both of which can be obtained with [`RingBuffer::new()`].

*See also the [crate-level documentation](crate).*

**Generic Parameters:**
- T

**Methods:**

- `fn new(capacity: usize) -> (Producer<T>, Consumer<T>)` - Creates a `RingBuffer` with the given `capacity` and returns [`Producer`] and [`Consumer`].
- `fn capacity(self: &Self) -> usize` - Returns the capacity of the queue.

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool` - This method tests for `self` and `other` values to be equal, and is used by `==`.
- **Drop**
  - `fn drop(self: & mut Self)` - Drops all non-empty slots.
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## Module: chunks

Writing and reading multiple items at once into and from a [`RingBuffer`].

Multiple items at once can be moved from an iterator into the ring buffer by using
[`Producer::write_chunk_uninit()`] followed by [`WriteChunkUninit::fill_from_iter()`].
Alternatively, mutable access to the (uninitialized) slots of the chunk can be obtained with
[`WriteChunkUninit::as_mut_slices()`], which requires writing some `unsafe` code.
To avoid that, [`Producer::write_chunk()`] can be used,
which initializes all slots with their [`Default`] value
and provides mutable access by means of [`WriteChunk::as_mut_slices()`].

Multiple items at once can be moved out of the ring buffer by using
[`Consumer::read_chunk()`] and iterating over the returned [`ReadChunk`]
(or by explicitly calling [`ReadChunk::into_iter()`]).
Immutable access to the slots of the chunk can be obtained with [`ReadChunk::as_slices()`].

If the item type `T` implements [`Copy`], the convenience functions
[`Producer::push_partial_slice()`], [`Producer::push_entire_slice()`],
[`Consumer::pop_partial_slice()`], [`Consumer::pop_entire_slice()`],
[`Consumer::pop_partial_slice_uninit()`]
and [`Consumer::pop_entire_slice_uninit()`] can be used.

# Examples

The following examples use a single thread for simplicity, but in a real application,
`producer` and `consumer` would of course live on different threads:

If the trait bound `T: Copy` is satisfied,
the `push_*_slice()` and `pop_*_slice()` methods can be used.

```
use rtrb::RingBuffer;

let (mut producer, mut consumer) = RingBuffer::new(4);

let source = vec![1, 2, 3, 4, 5, 6];
let (pushed, remainder) = producer.push_partial_slice(&source);
assert_eq!(pushed, [1, 2, 3, 4]);
assert_eq!(remainder, [5, 6]);

let mut destination = vec![0; 3];
consumer.pop_entire_slice(&mut destination).unwrap();
assert_eq!(destination, [1, 2, 3]);

let (popped, remainder) = consumer.pop_partial_slice(&mut destination);
assert_eq!(popped, [4]);
assert_eq!(remainder, [2, 3]);
// The returned slices are mutable sub-slices into `destination`.
remainder[0] = 99;
assert_eq!(destination, [4, 99, 3]);
```

If this convenience interface is too limited (or if `T` is not `Copy`)
the more fundamental methods [`Producer::write_chunk()`],
[`Producer::write_chunk_uninit()`] and [`Consumer::read_chunk()`] can be used.

```
use rtrb::RingBuffer;

let (mut producer, mut consumer) = RingBuffer::new(5);

if let Ok(chunk) = producer.write_chunk_uninit(4) {
    chunk.fill_from_iter([10, 11, 12]);
    // Note that we requested 4 slots but we've only written to 3 of them!
} else {
    unreachable!();
}

assert_eq!(producer.slots(), 2);
assert_eq!(consumer.slots(), 3);

if let Ok(chunk) = consumer.read_chunk(2) {
    assert_eq!(chunk.into_iter().collect::<Vec<_>>(), [10, 11]);
} else {
    unreachable!();
}

// One element is still in the queue:
assert_eq!(consumer.peek(), Ok(&12));

let data = vec![20, 21, 22, 23];
// NB: write_chunk_uninit() could be used for possibly better performance:
if let Ok(mut chunk) = producer.write_chunk(4) {
    let (one, two) = chunk.as_mut_slices();
    let mid = one.len();
    one.copy_from_slice(&data[..mid]);
    two.copy_from_slice(&data[mid..]);
    chunk.commit_all();
} else {
    unreachable!();
}

assert!(producer.is_full());
assert_eq!(consumer.slots(), 5);

let mut v = Vec::<i32>::with_capacity(5);
if let Ok(chunk) = consumer.read_chunk(5) {
    let (one, two) = chunk.as_slices();
    v.extend(one);
    v.extend(two);
    chunk.commit_all();
} else {
    unreachable!();
}
assert_eq!(v, [12, 20, 21, 22, 23]);
assert!(consumer.is_empty());
```

The iterator API can be used to move items from one ring buffer to another:

```
use rtrb::{Consumer, Producer};

fn move_items<T>(src: &mut Consumer<T>, dst: &mut Producer<T>) -> usize {
    let n = src.slots().min(dst.slots());
    dst.write_chunk_uninit(n).unwrap().fill_from_iter(src.read_chunk(n).unwrap())
}
```

Write as many slots as possible, given an iterator
(and return the number of written slots):

```
use rtrb::{Producer, chunks::ChunkError::TooFewSlots};

fn push_from_iter<T, I>(queue: &mut Producer<T>, iter: I) -> usize
where
    T: Default,
    I: IntoIterator<Item = T>,
{
    let iter = iter.into_iter();
    let n = match iter.size_hint() {
        (_, None) => queue.slots(),
        (_, Some(n)) => n,
    };
    let chunk = match queue.write_chunk_uninit(n) {
        Ok(chunk) => chunk,
        // Remaining slots are returned, this will always succeed:
        Err(TooFewSlots(n)) => queue.write_chunk_uninit(n).unwrap(),
    };
    chunk.fill_from_iter(iter)
}
```



