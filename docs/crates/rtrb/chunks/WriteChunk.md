**rtrb > chunks > WriteChunk**

# Module: chunks::WriteChunk

## Contents

**Functions**

- [`as_mut_slices`](#as_mut_slices) - Returns two slices for writing to the requested slots.
- [`commit`](#commit) - Makes the first `n` slots of the chunk available for reading.
- [`commit_all`](#commit_all) - Makes the whole chunk available for reading.

---

## rtrb::chunks::WriteChunk::as_mut_slices

*Function*

Returns two slices for writing to the requested slots.

All slots are initially filled with their [`Default`] value.

The first slice can only be empty if `0` slots have been requested.
If the first slice contains all requested slots, the second one is empty.

After writing to the slots, they are *not* automatically made available
to be read by the [`Consumer`].
This has to be explicitly done by calling [`commit()`](WriteChunk::commit)
or [`commit_all()`](WriteChunk::commit_all).
If items are written but *not* committed afterwards,
they will *not* become available for reading and
they will eventually be dropped (if `T` implements [`Drop`]).

```rust
fn as_mut_slices(self: & mut Self) -> (& mut [T], & mut [T])
```



## rtrb::chunks::WriteChunk::commit

*Function*

Makes the first `n` slots of the chunk available for reading.

The rest of the chunk is dropped.

# Panics

Panics in debug mode if `n` is greater than the number of slots in the chunk.
In release mode, `n` is capped to the number of slots and no panic happens.

```rust
fn commit(self: Self, n: usize)
```



## rtrb::chunks::WriteChunk::commit_all

*Function*

Makes the whole chunk available for reading.

```rust
fn commit_all(self: Self)
```



