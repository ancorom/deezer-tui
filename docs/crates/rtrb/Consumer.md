**rtrb > Consumer**

# Module: Consumer

## Contents

**Functions**

- [`is_abandoned`](#is_abandoned) - Returns `true` if the corresponding [`Producer`] has been destroyed.
- [`is_empty`](#is_empty) - Returns `true` if there are currently no slots available for reading.
- [`peek`](#peek) - Attempts to read an element from the queue without removing it.
- [`pop`](#pop) - Attempts to pop an element from the queue.
- [`pop_entire_slice`](#pop_entire_slice) - Copies as many items from the ring buffer as to fill the given `slice`.
- [`pop_entire_slice_uninit`](#pop_entire_slice_uninit) - Copies as many items from the ring buffer as to fill the given uninitialized `slice`.
- [`pop_partial_slice`](#pop_partial_slice) - Copies as many items as possible from the ring buffer to the given `slice`.
- [`pop_partial_slice_uninit`](#pop_partial_slice_uninit) - Copies as many items as possible from the ring buffer to the given uninitialized `slice`.
- [`read_chunk`](#read_chunk) - Returns `n` slots for reading.
- [`slots`](#slots) - Returns the number of slots available for reading.

---

## rtrb::Consumer::is_abandoned

*Function*

Returns `true` if the corresponding [`Producer`] has been destroyed.

Note that since Rust version 1.74.0 and before `rtrb` version 0.4,
this was not synchronizing with the consumer thread anymore,
see [issue #114](https://github.com/mgeier/rtrb/issues/114).
In `rtrb` version 0.4, the synchronizing behavior has been restored.

# Examples

```
use rtrb::RingBuffer;

let (mut p, mut c) = RingBuffer::new(7);
assert!(!c.is_abandoned());
assert_eq!(p.push(10), Ok(()));
drop(p);
assert!(c.is_abandoned());
// The items that are left in the ring buffer can still be consumed:
assert_eq!(c.pop(), Ok(10));
```

Since the producer can be concurrently dropped on another thread,
the consumer might become abandoned at any time:

```
# use rtrb::RingBuffer;
# let (p, c) = RingBuffer::<i32>::new(1);
if !c.is_abandoned() {
    // Right now, the producer might still be alive, but it might as well not be
    // if another thread has just dropped it.
}
```

However, if it already is abandoned, it will stay that way:

```
# use rtrb::RingBuffer;
# let (p, c) = RingBuffer::<i32>::new(1);
if c.is_abandoned() {
    // The producer does definitely not exist anymore.
}
```

```rust
fn is_abandoned(self: &Self) -> bool
```



## rtrb::Consumer::is_empty

*Function*

Returns `true` if there are currently no slots available for reading.

An empty ring buffer might cease to be empty at any time
if the corresponding [`Producer`] is producing items in another thread.

# Examples

```
use rtrb::RingBuffer;

let (p, c) = RingBuffer::<f32>::new(1);

assert!(c.is_empty());
```

Since items can be concurrently produced on another thread, the ring buffer
might not be empty for long:

```
# use rtrb::RingBuffer;
# let (p, c) = RingBuffer::<f32>::new(1);
if c.is_empty() {
    // The buffer might be empty, but it might as well not be
    // if an item was just produced on another thread.
}
```

However, if it's not empty, another thread cannot change that:

```
# use rtrb::RingBuffer;
# let (p, c) = RingBuffer::<f32>::new(1);
if !c.is_empty() {
    // At least one slot is guaranteed to be available for reading.
}
```

```rust
fn is_empty(self: &Self) -> bool
```



## rtrb::Consumer::peek

*Function*

Attempts to read an element from the queue without removing it.

# Errors

If the queue is empty, an error is returned.

# Examples

```
use rtrb::{PeekError, RingBuffer};

let (mut p, c) = RingBuffer::new(1);

assert_eq!(c.peek(), Err(PeekError::Empty));
assert_eq!(p.push(10), Ok(()));
assert_eq!(c.peek(), Ok(&10));
assert_eq!(c.peek(), Ok(&10));
```

Note that `peek()` takes a shared reference to `self`,
which means that other methods that take `&self` can be called
while the returned reference is still in use.
However, calling methods that take `&mut self`
(like [`Consumer::pop()`] and [`Consumer::read_chunk()`]) leads to a compiler error:

```compile_fail
use rtrb::RingBuffer;

let (mut p, mut c) = RingBuffer::new(8);
p.push(10).unwrap();
let shared_ref = c.peek().unwrap();
let value = c.pop().unwrap();
assert_eq!(shared_ref, &10);
```

```rust
fn peek(self: &Self) -> Result<&T, PeekError>
```



## rtrb::Consumer::pop

*Function*

Attempts to pop an element from the queue.

The element is *moved* out of the ring buffer and its slot
is made available to be filled by the [`Producer`] again.

# Errors

If the queue is empty, an error is returned.

# Examples

```
use rtrb::{PopError, RingBuffer};

let (mut p, mut c) = RingBuffer::new(1);

assert_eq!(p.push(10), Ok(()));
assert_eq!(c.pop(), Ok(10));
assert_eq!(c.pop(), Err(PopError::Empty));
```

To obtain an [`Option<T>`](Option), use [`.ok()`](Result::ok) on the result.

```
# use rtrb::RingBuffer;
# let (mut p, mut c) = RingBuffer::new(1);
assert_eq!(p.push(20), Ok(()));
assert_eq!(c.pop().ok(), Some(20));
```

```rust
fn pop(self: & mut Self) -> Result<T, PopError>
```



## rtrb::Consumer::pop_entire_slice

*Function*

Copies as many items from the ring buffer as to fill the given `slice`.

The copied slots are automatically made available to be written again by the [`Producer`].

# Errors

If not enough data is available in the ring buffer, no items are copied and
a [`ChunkError`] with the available items is returned.

To copy only the available slots, [`Consumer::pop_partial_slice()`] can be used.
To copy into an uninitialized slice, [`Consumer::pop_entire_slice_uninit()`] can be used.

```rust
fn pop_entire_slice(self: & mut Self, slice: & mut [T]) -> Result<(), ChunkError>
```



## rtrb::Consumer::pop_entire_slice_uninit

*Function*

Copies as many items from the ring buffer as to fill the given uninitialized `slice`.

The copied slots are automatically made available to be written again by the [`Producer`].

Returns the given slice, but now initialized.

# Errors

If not enough data is available in the ring buffer, no items are copied and
a [`ChunkError`] with the available items is returned.

To copy only the available slots, [`Consumer::pop_partial_slice_uninit()`] can be used.
To copy into an initialized slice, [`Consumer::pop_entire_slice()`] can be used.

```rust
fn pop_entire_slice_uninit<'a>(self: & mut Self, slice: &'a  mut [MaybeUninit<T>]) -> Result<&'a  mut [T], ChunkError>
```



## rtrb::Consumer::pop_partial_slice

*Function*

Copies as many items as possible from the ring buffer to the given `slice`.

The copied slots are automatically made available to be written again by the [`Producer`].

Returns two sub-slices of `slice`:
- The part that has been filled with data from the ring buffer (possibly empty).
- The unused remainder (possibly empty).

To copy an entire slice (and fail otherwise), [`Consumer::pop_entire_slice()`] can be used.
To copy into an uninitialized slice, [`Consumer::pop_partial_slice_uninit()`] can be used.

# Examples

```
use rtrb::Consumer;

fn pop_at_least_one_element<'a>(
    c: &mut Consumer<i32>,
    s: &'a mut [i32],
) -> Result<&'a mut [i32], &'a mut [i32]> {
    match c.pop_partial_slice(s) {
        ([], remainder) => Err(remainder),
        (_, remainder) => Ok(remainder),
    }
}

fn block_while_popping_entire_slice(c: &mut Consumer<i32>, mut s: &mut [i32]) {
    while let (_, remainder @ [_, ..]) = c.pop_partial_slice(s) {
        std::thread::yield_now();
        s = remainder;
    }
}
```

For more examples, see the documentation of the [`chunks`](crate::chunks#examples) module.

```rust
fn pop_partial_slice<'a>(self: & mut Self, slice: &'a  mut [T]) -> (&'a  mut [T], &'a  mut [T])
```



## rtrb::Consumer::pop_partial_slice_uninit

*Function*

Copies as many items as possible from the ring buffer to the given uninitialized `slice`.

The copied slots are automatically made available to be written again by the [`Producer`].

Returns two sub-slices of `slice`:
- The part that has been filled with data from the ring buffer (possibly empty).
- The unused remainder (possibly empty).

The first of the returned slices has been initialized,
while the second one remains uninitialized.

To copy an entire slice (and fail otherwise),
[`Consumer::pop_entire_slice_uninit()`] can be used.
To copy into an initialized slice, [`Consumer::pop_partial_slice()`] can be used.

# Examples

```
use std::mem::MaybeUninit;

use rtrb::Consumer;

fn pop_at_least_one_element_uninit<'a>(
    c: &mut Consumer<i32>,
    s: &'a mut [MaybeUninit<i32>],
) -> Result<&'a mut [MaybeUninit<i32>], &'a mut [MaybeUninit<i32>]> {
    match c.pop_partial_slice_uninit(s) {
        ([], remainder) => Err(remainder),
        (_, remainder) => Ok(remainder),
    }
}

fn block_while_popping_entire_slice_uninit(
    c: &mut Consumer<i32>,
    mut s: &mut [MaybeUninit<i32>],
) {
    while let (_, remainder @ [_, ..]) = c.pop_partial_slice_uninit(s) {
        std::thread::yield_now();
        s = remainder;
    }
}
```

Typically, we might get an uninitialized buffer via FFI,
but in this example we are using the uninitialized part of a [`Vec`]:

```
use std::mem::MaybeUninit;

use rtrb::RingBuffer;

let (mut producer, mut consumer) = RingBuffer::new(4);
let (_, remainder) = producer.push_partial_slice(&[1, 2, 3]);
assert!(remainder.is_empty());
let mut buffer = Vec::with_capacity(5);
let buffer_uninit = buffer.spare_capacity_mut();
let (popped, remainder) = consumer.pop_partial_slice_uninit(buffer_uninit);
assert_eq!(popped, [1, 2, 3]);
// The returned slices are mutable ...
popped[0] = -42;
// ... but the second one is still uninitialized:
remainder[0] = MaybeUninit::new(99);
// All this happened in the uninitialized part of the buffer,
// which is still "officially" empty:
assert!(buffer.is_empty());
// SAFETY: The first 4 elements have been initialized.
unsafe {
    buffer.set_len(4);
}
assert_eq!(buffer, [-42, 2, 3, 99]);
```

```rust
fn pop_partial_slice_uninit<'a>(self: & mut Self, slice: &'a  mut [MaybeUninit<T>]) -> (&'a  mut [T], &'a  mut [MaybeUninit<T>])
```



## rtrb::Consumer::read_chunk

*Function*

Returns `n` slots for reading.

[`ReadChunk::as_slices()`] provides immutable access to the slots.
After reading from those slots, they explicitly have to be made available
to be written again by the [`Producer`] by calling [`ReadChunk::commit()`]
or [`ReadChunk::commit_all()`].

Alternatively, items can be moved out of the [`ReadChunk`] using iteration
because it implements [`IntoIterator`]
([`ReadChunk::into_iter()`] can be used to explicitly turn it into an [`Iterator`]).
All moved items are automatically made available to be written again by the [`Producer`].

# Errors

If not enough slots are available, an error
(containing the number of available slots) is returned.
Use [`Consumer::slots()`] to obtain the number of available slots beforehand.

# Examples

See the documentation of the [`chunks`](crate::chunks#examples) module.

```rust
fn read_chunk(self: & mut Self, n: usize) -> Result<ReadChunk<T>, ChunkError>
```



## rtrb::Consumer::slots

*Function*

Returns the number of slots available for reading.

Since items can be concurrently produced on another thread, the actual number
of available slots may increase at any time (up to the [`RingBuffer::capacity()`]).

To check for a single available slot,
using [`Consumer::is_empty()`] is often quicker
(because it might not have to check an atomic variable).

# Examples

```
use rtrb::RingBuffer;

let (p, c) = RingBuffer::<f32>::new(1024);

assert_eq!(c.slots(), 0);
```

```rust
fn slots(self: &Self) -> usize
```



