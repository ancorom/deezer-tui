**rtrb > RingBuffer**

# Module: RingBuffer

## Contents

**Functions**

- [`capacity`](#capacity) - Returns the capacity of the queue.
- [`new`](#new) - Creates a `RingBuffer` with the given `capacity` and returns [`Producer`] and [`Consumer`].

---

## rtrb::RingBuffer::capacity

*Function*

Returns the capacity of the queue.

# Examples

```
use rtrb::RingBuffer;

let (producer, consumer) = RingBuffer::<f32>::new(100);
assert_eq!(producer.buffer().capacity(), 100);
assert_eq!(consumer.buffer().capacity(), 100);
// Both producer and consumer of course refer to the same ring buffer:
assert_eq!(producer.buffer(), consumer.buffer());
```

```rust
fn capacity(self: &Self) -> usize
```



## rtrb::RingBuffer::new

*Function*

Creates a `RingBuffer` with the given `capacity` and returns [`Producer`] and [`Consumer`].

# Panics

Panics if `capacity * size_of::<T>()` exceeds `isize::MAX` bytes or,
when `T` is a zero-sized type, if `capacity` is larger than `usize::MAX / 2`.

# Examples

```
use rtrb::RingBuffer;

let (producer, consumer) = RingBuffer::<f32>::new(100);
```

Specifying an explicit type with the [turbofish](https://turbo.fish/)
is is only necessary if it cannot be deduced by the compiler.

```
use rtrb::RingBuffer;

let (mut producer, consumer) = RingBuffer::new(100);
assert_eq!(producer.push(0.0f32), Ok(()));
```

```rust
fn new(capacity: usize) -> (Producer<T>, Consumer<T>)
```



