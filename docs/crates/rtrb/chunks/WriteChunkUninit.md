**rtrb > chunks > WriteChunkUninit**

# Module: chunks::WriteChunkUninit

## Contents

**Functions**

- [`as_mut_slices`](#as_mut_slices) - Returns two slices for writing to the requested slots.
- [`commit`](#commit) - Makes the first `n` slots of the chunk available for reading.
- [`commit_all`](#commit_all) - Makes the whole chunk available for reading.
- [`fill_from_iter`](#fill_from_iter) - Moves items from an iterator into the (uninitialized) slots of the chunk.

---

## rtrb::chunks::WriteChunkUninit::as_mut_slices

*Function*

Returns two slices for writing to the requested slots.

The first slice can only be empty if `0` slots have been requested.
If the first slice contains all requested slots, the second one is empty.

The extension trait [`CopyToUninit`] can be used to safely copy data into those slices.

After writing to the slots, they are *not* automatically made available
to be read by the [`Consumer`].
This has to be explicitly done by calling [`commit()`](WriteChunkUninit::commit)
or [`commit_all()`](WriteChunkUninit::commit_all).
If items are written but *not* committed afterwards,
they will *not* become available for reading and
they will be leaked (which is only relevant if `T` implements [`Drop`]).

```rust
fn as_mut_slices(self: & mut Self) -> (& mut [MaybeUninit<T>], & mut [MaybeUninit<T>])
```



## rtrb::chunks::WriteChunkUninit::commit

*Function*

Makes the first `n` slots of the chunk available for reading.

# Panics

Panics in debug mode if `n` is greater than the number of slots in the chunk.
In release mode, `n` is capped to the number of slots and no panic happens.

# Safety

The caller must make sure that the first `n` elements have been initialized.

```rust
fn commit(self: Self, n: usize)
```



## rtrb::chunks::WriteChunkUninit::commit_all

*Function*

Makes the whole chunk available for reading.

# Safety

The caller must make sure that all elements have been initialized.

```rust
fn commit_all(self: Self)
```



## rtrb::chunks::WriteChunkUninit::fill_from_iter

*Function*

Moves items from an iterator into the (uninitialized) slots of the chunk.

The number of moved items is returned.

All moved items are automatically made availabe to be read by the [`Consumer`].

# Examples

If the iterator contains too few items, only a part of the chunk
is made available for reading:

```
use rtrb::{RingBuffer, PopError};

let (mut p, mut c) = RingBuffer::new(4);

if let Ok(chunk) = p.write_chunk_uninit(3) {
    assert_eq!(chunk.fill_from_iter([10, 20]), 2);
} else {
    unreachable!();
}
assert_eq!(p.slots(), 2);
assert_eq!(c.pop(), Ok(10));
assert_eq!(c.pop(), Ok(20));
assert_eq!(c.pop(), Err(PopError::Empty));
```

If the chunk size is too small, some items may remain in the iterator.
To be able to keep using the iterator after the call,
`&mut` (or [`Iterator::by_ref()`]) can be used.

```
use rtrb::{RingBuffer, PopError};

let (mut p, mut c) = RingBuffer::new(4);

let mut it = vec![10, 20, 30].into_iter();
if let Ok(chunk) = p.write_chunk_uninit(2) {
    assert_eq!(chunk.fill_from_iter(&mut it), 2);
} else {
    unreachable!();
}
assert_eq!(c.pop(), Ok(10));
assert_eq!(c.pop(), Ok(20));
assert_eq!(c.pop(), Err(PopError::Empty));
assert_eq!(it.next(), Some(30));
```

```rust
fn fill_from_iter<I>(self: Self, iter: I) -> usize
```



