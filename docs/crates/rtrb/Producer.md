**rtrb > Producer**

# Module: Producer

## Contents

**Functions**

- [`is_abandoned`](#is_abandoned) - Returns `true` if the corresponding [`Consumer`] has been destroyed.
- [`is_full`](#is_full) - Returns `true` if there are currently no slots available for writing.
- [`push`](#push) - Attempts to push an element into the queue.
- [`push_entire_slice`](#push_entire_slice) - Copies all items from the given `slice` into the ring buffer.
- [`push_partial_slice`](#push_partial_slice) - Copies as many items as possible from the given `slice` into the ring buffer.
- [`slots`](#slots) - Returns the number of slots available for writing.
- [`write_chunk`](#write_chunk) - Returns `n` slots (initially containing their [`Default`] value) for writing.
- [`write_chunk_uninit`](#write_chunk_uninit) - Returns `n` (uninitialized) slots for writing.

---

## rtrb::Producer::is_abandoned

*Function*

Returns `true` if the corresponding [`Consumer`] has been destroyed.

Note that since Rust version 1.74.0 and before `rtrb` version 0.4,
this was not synchronizing with the consumer thread anymore,
see [issue #114](https://github.com/mgeier/rtrb/issues/114).
In `rtrb` version 0.4, the synchronizing behavior has been restored.

# Examples

```
use rtrb::RingBuffer;

let (mut p, c) = RingBuffer::new(7);
assert!(!p.is_abandoned());
assert_eq!(p.push(10), Ok(()));
drop(c);
// The items that are still in the ring buffer are not accessible anymore.
assert!(p.is_abandoned());
// Even though it's futile, items can still be written:
assert_eq!(p.push(11), Ok(()));
```

Since the consumer can be concurrently dropped on another thread,
the producer might become abandoned at any time:

```
# use rtrb::RingBuffer;
# let (p, c) = RingBuffer::<i32>::new(1);
if !p.is_abandoned() {
    // Right now, the consumer might still be alive, but it might as well not be
    // if another thread has just dropped it.
}
```

However, if it already is abandoned, it will stay that way:

```
# use rtrb::RingBuffer;
# let (p, c) = RingBuffer::<i32>::new(1);
if p.is_abandoned() {
    // The consumer does definitely not exist anymore.
}
```

```rust
fn is_abandoned(self: &Self) -> bool
```



## rtrb::Producer::is_full

*Function*

Returns `true` if there are currently no slots available for writing.

A full ring buffer might cease to be full at any time
if the corresponding [`Consumer`] is consuming items in another thread.

# Examples

```
use rtrb::RingBuffer;

let (p, c) = RingBuffer::<f32>::new(1);

assert!(!p.is_full());
```

Since items can be concurrently consumed on another thread, the ring buffer
might not be full for long:

```
# use rtrb::RingBuffer;
# let (p, c) = RingBuffer::<f32>::new(1);
if p.is_full() {
    // The buffer might be full, but it might as well not be
    // if an item was just consumed on another thread.
}
```

However, if it's not full, another thread cannot change that:

```
# use rtrb::RingBuffer;
# let (p, c) = RingBuffer::<f32>::new(1);
if !p.is_full() {
    // At least one slot is guaranteed to be available for writing.
}
```

```rust
fn is_full(self: &Self) -> bool
```



## rtrb::Producer::push

*Function*

Attempts to push an element into the queue.

The element is *moved* into the ring buffer and its slot
is made available to be read by the [`Consumer`].

# Errors

If the queue is full, the element is returned back as an error.

# Examples

```
use rtrb::{RingBuffer, PushError};

let (mut p, c) = RingBuffer::new(1);

assert_eq!(p.push(10), Ok(()));
assert_eq!(p.push(20), Err(PushError::Full(20)));
```

```rust
fn push(self: & mut Self, value: T) -> Result<(), PushError<T>>
```



## rtrb::Producer::push_entire_slice

*Function*

Copies all items from the given `slice` into the ring buffer.

The written slots are automatically made available to be read by the [`Consumer`].

To copy only into the available slots, [`Producer::push_partial_slice()`] can be used.

# Errors

If not enough free space is available in the ring buffer,
a [`ChunkError`] with the available slots is returned.

```rust
fn push_entire_slice(self: & mut Self, slice: &[T]) -> Result<(), ChunkError>
```



## rtrb::Producer::push_partial_slice

*Function*

Copies as many items as possible from the given `slice` into the ring buffer.

The written slots are automatically made available to be read by the [`Consumer`].

Returns two sub-slices of `slice`:
- The part that has been copied into the ring buffer (possibly empty).
- The unused remainder (possibly empty).

To copy an entire slice (and fail otherwise), [`Producer::push_entire_slice()`] can be used.

# Examples

```
use rtrb::Producer;

fn push_at_least_one_element<'a>(
    p: &mut Producer<i32>,
    s: &'a [i32],
) -> Result<&'a [i32], &'a [i32]> {
    match p.push_partial_slice(s) {
        ([], remainder) => Err(remainder),
        (_, remainder) => Ok(remainder),
    }
}

fn block_while_pushing_entire_slice(p: &mut Producer<i32>, mut s: &[i32]) {
    while let (_, remainder @ [_, ..]) = p.push_partial_slice(s) {
        std::thread::yield_now();
        s = remainder;
    }
}
```

For more examples, see the documentation of the [`chunks`](crate::chunks#examples) module.

```rust
fn push_partial_slice<'a>(self: & mut Self, slice: &'a [T]) -> (&'a [T], &'a [T])
```



## rtrb::Producer::slots

*Function*

Returns the number of slots available for writing.

Since items can be concurrently consumed on another thread, the actual number
of available slots may increase at any time (up to the [`RingBuffer::capacity()`]).

To check for a single available slot,
using [`Producer::is_full()`] is often quicker
(because it might not have to check an atomic variable).

# Examples

```
use rtrb::RingBuffer;

let (p, c) = RingBuffer::<f32>::new(1024);

assert_eq!(p.slots(), 1024);
```

```rust
fn slots(self: &Self) -> usize
```



## rtrb::Producer::write_chunk

*Function*

Returns `n` slots (initially containing their [`Default`] value) for writing.

[`WriteChunk::as_mut_slices()`] provides mutable access to the slots.
After writing to those slots, they explicitly have to be made available
to be read by the [`Consumer`] by calling [`WriteChunk::commit()`]
or [`WriteChunk::commit_all()`].

For an alternative that does not require the trait bound [`Default`],
see [`Producer::write_chunk_uninit()`].

If items are supposed to be moved from an iterator into the ring buffer,
[`Producer::write_chunk_uninit()`] followed by [`WriteChunkUninit::fill_from_iter()`]
can be used.

# Errors

If not enough slots are available, an error
(containing the number of available slots) is returned.
Use [`Producer::slots()`] to obtain the number of available slots beforehand.

# Examples

See the documentation of the [`chunks`](crate::chunks#examples) module.

```rust
fn write_chunk(self: & mut Self, n: usize) -> Result<WriteChunk<T>, ChunkError>
```



## rtrb::Producer::write_chunk_uninit

*Function*

Returns `n` (uninitialized) slots for writing.

[`WriteChunkUninit::as_mut_slices()`] provides mutable access
to the uninitialized slots.
After writing to those slots, they explicitly have to be made available
to be read by the [`Consumer`] by calling [`WriteChunkUninit::commit()`]
or [`WriteChunkUninit::commit_all()`].

Alternatively, [`WriteChunkUninit::fill_from_iter()`] can be used
to move items from an iterator into the available slots.
All moved items are automatically made available to be read by the [`Consumer`].

# Errors

If not enough slots are available, an error
(containing the number of available slots) is returned.
Use [`Producer::slots()`] to obtain the number of available slots beforehand.

# Safety

This function itself is safe, as is [`WriteChunkUninit::fill_from_iter()`].
However, when using [`WriteChunkUninit::as_mut_slices()`],
the user has to make sure that the relevant slots have been initialized
before calling [`WriteChunkUninit::commit()`] or [`WriteChunkUninit::commit_all()`].

For a safe alternative that provides mutable slices of [`Default`]-initialized slots,
see [`Producer::write_chunk()`].

# Examples

See the documentation of the [`chunks`](crate::chunks#examples) module.

```rust
fn write_chunk_uninit(self: & mut Self, n: usize) -> Result<WriteChunkUninit<T>, ChunkError>
```



