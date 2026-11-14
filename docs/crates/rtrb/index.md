# rtrb

A realtime-safe single-producer single-consumer (SPSC) ring buffer.

A [`RingBuffer`] consists of two parts:
a [`Producer`] for writing into the ring buffer and
a [`Consumer`] for reading from the ring buffer.

A fixed-capacity buffer is allocated on construction.
After that, no more memory is allocated (unless the type `T` does that internally).
Reading from and writing into the ring buffer is *lock-free* and *wait-free*.
All reading and writing functions return immediately.
Attempts to write to a full buffer return an error;
values inside the buffer are *not* overwritten.
Attempts to read from an empty buffer return an error as well.
Only a single thread can write into the ring buffer and a single thread
(typically a different one) can read from the ring buffer.
If the queue is empty, there is no way for the reading thread to wait
for new data, other than trying repeatedly until reading succeeds.
Similarly, if the queue is full, there is no way for the writing thread
to wait for newly available space to write to, other than trying repeatedly.

# Examples

Moving single elements into and out of a queue with
[`Producer::push()`] and [`Consumer::pop()`], respectively:

```
use rtrb::{RingBuffer, PushError, PopError};

let (mut producer, mut consumer) = RingBuffer::new(2);

assert_eq!(producer.push(10), Ok(()));
assert_eq!(producer.push(20), Ok(()));
assert_eq!(producer.push(30), Err(PushError::Full(30)));

std::thread::spawn(move || {
    assert_eq!(consumer.pop(), Ok(10));
    assert_eq!(consumer.pop(), Ok(20));
    assert_eq!(consumer.pop(), Err(PopError::Empty));
}).join().unwrap();
```

See the documentation of the [`chunks#examples`] module
for examples that write/read multiple items at once.
See also:

  * [`Producer::write_chunk()`]
  * [`Producer::write_chunk_uninit()`]
  * [`Producer::push_partial_slice()`] (if `T: Copy`)
  * [`Producer::push_entire_slice()`] (if `T: Copy`)

  * [`Consumer::read_chunk()`]
  * [`Consumer::pop_partial_slice()`] (if `T: Copy`)
  * [`Consumer::pop_partial_slice_uninit()`] (if `T: Copy`)
  * [`Consumer::pop_entire_slice()`] (if `T: Copy`)
  * [`Consumer::pop_entire_slice_uninit()`] (if `T: Copy`)

## Modules

### [`rtrb`](rtrb.md)

*1 module, 1 trait, 3 enums, 3 structs*

### [`Consumer`](Consumer.md)

*10 functions*

### [`Producer`](Producer.md)

*8 functions*

### [`RingBuffer`](RingBuffer.md)

*2 functions*

### [`chunks`](chunks.md)

*1 enum, 4 structs*

### [`chunks::ReadChunk`](chunks/ReadChunk.md)

*3 functions*

### [`chunks::WriteChunk`](chunks/WriteChunk.md)

*3 functions*

### [`chunks::WriteChunkUninit`](chunks/WriteChunkUninit.md)

*4 functions*



## Project usage (deezer-tui) — Phase 2 ring buffers
- Capacities: audio ring = 65536 interleaved-stereo f32 (`src/audio/ring_buffers.rs::AUDIO_RING_BUFFER_CAPACITY`); vis ring = 16384 mono f32 where `mono=(L+R)/2`.
- Creation: `(producer, consumer) = RingBuffer::<f32>::new(capacity)`.
- Producer: `push(x)` (single), `push_partial_slice(&s)` (slice, Copy), `is_full()` (single-slot probe).
- Consumer: `pop()` -> `Err(PopError::Empty)` when empty (non-blocking, RT-safe); read in a loop in the RT callback and fill silence on `PopError::Empty`.
- Source: `src/audio/ring_buffers.rs` (creation), `src/audio/decoder.rs` (writes), `src/audio/output.rs` (reads).

### Verified API signatures (rtrb 0.4)
- `Producer::push_partial_slice(&mut self, data: &[T]) -> (Vec<T>, &[T])`: pushes up to as much as fits, returns `(pushed, remaining)` where `remaining` is `&[T]`.
- Non-blocking write loop pattern (RT / decoder thread):
  ```rust
  let mut remaining = src;
  loop {
      if remaining.is_empty() { break; }
      if dst.is_full() {
          std::thread::yield_now();
          continue;
      }
      let (pushed, rest) = dst.push_partial_slice(remaining);
      remaining = rest; // NOT &rest — rest is already &[T] (clippy::needless_borrow otherwise)
  }
  ```
- `Producer` and `Consumer` are `Send` but **not `Sync`**; move each to its owning thread (producer → decoder thread, consumer → CPAL RT callback / UI thread).
- `into_producer()` is NOT a thing on `RingBuffer`; it is a custom method on the wrapper (`AudioRing`/`VisRing`) that returns the **owned** `(Producer, Consumer)`.