**rtrb > chunks > ReadChunk**

# Module: chunks::ReadChunk

## Contents

**Functions**

- [`as_slices`](#as_slices) - Returns two slices for reading from the requested slots.
- [`commit`](#commit) - Drops the first `n` slots of the chunk, making the space available for writing again.
- [`commit_all`](#commit_all) - Drops all slots of the chunk, making the space available for writing again.

---

## rtrb::chunks::ReadChunk::as_slices

*Function*

Returns two slices for reading from the requested slots.

The first slice can only be empty if `0` slots have been requested.
If the first slice contains all requested slots, the second one is empty.

The provided slots are *not* automatically made available
to be written again by the [`Producer`].
This has to be explicitly done by calling [`commit()`](ReadChunk::commit)
or [`commit_all()`](ReadChunk::commit_all).
Note that this runs the destructor of the committed items (if `T` implements [`Drop`]).
You can "peek" at the contained values by simply not calling any of the "commit" methods.

```rust
fn as_slices(self: &Self) -> (&[T], &[T])
```



## rtrb::chunks::ReadChunk::commit

*Function*

Drops the first `n` slots of the chunk, making the space available for writing again.

# Panics

Panics in debug mode if `n` is greater than the number of slots in the chunk.
In release mode, `n` is capped to the number of slots and no panic happens.

# Examples

The following example shows that items are dropped when "committed"
(which is only relevant if `T` implements [`Drop`]).

```
use rtrb::RingBuffer;

// Static variable to count all drop() invocations
static mut DROP_COUNT: i32 = 0;
#[derive(Debug)]
struct Thing;
impl Drop for Thing {
    fn drop(&mut self) { unsafe { DROP_COUNT += 1; } }
}

// Scope to limit lifetime of ring buffer
{
    let (mut p, mut c) = RingBuffer::new(2);

    assert!(p.push(Thing).is_ok()); // 1
    assert!(p.push(Thing).is_ok()); // 2
    if let Ok(thing) = c.pop() {
        // "thing" has been *moved* out of the queue but not yet dropped
        assert_eq!(unsafe { DROP_COUNT }, 0);
    } else {
        unreachable!();
    }
    // First Thing has been dropped when "thing" went out of scope:
    assert_eq!(unsafe { DROP_COUNT }, 1);
    assert!(p.push(Thing).is_ok()); // 3

    if let Ok(chunk) = c.read_chunk(2) {
        assert_eq!(chunk.len(), 2);
        assert_eq!(unsafe { DROP_COUNT }, 1);
        chunk.commit(1); // Drops only one of the two Things
        assert_eq!(unsafe { DROP_COUNT }, 2);
    } else {
        unreachable!();
    }
    // The last Thing is still in the queue ...
    assert_eq!(unsafe { DROP_COUNT }, 2);
}
// ... and it is dropped when the ring buffer goes out of scope:
assert_eq!(unsafe { DROP_COUNT }, 3);
```

```rust
fn commit(self: Self, n: usize)
```



## rtrb::chunks::ReadChunk::commit_all

*Function*

Drops all slots of the chunk, making the space available for writing again.

```rust
fn commit_all(self: Self)
```



