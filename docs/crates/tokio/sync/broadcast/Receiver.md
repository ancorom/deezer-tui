**tokio > sync > broadcast > Receiver**

# Module: sync::broadcast::Receiver

## Contents

**Functions**

- [`len`](#len) - Returns the number of messages that were sent into the channel and that
- [`recv`](#recv) - Receives the next value for this receiver.
- [`try_recv`](#try_recv) - Attempts to return a pending value on this receiver without awaiting.

---

## tokio::sync::broadcast::Receiver::len

*Function*

Returns the number of messages that were sent into the channel and that
this [`Receiver`] has yet to receive.

This count includes messages that have already been overwritten in the
ring buffer and are no longer readable. If `len` is **greater than** the
channel's effective capacity (the provided capacity rounded up to the
next power of two), the next call to [`recv`] returns
`Err(RecvError::Lagged)` and the next call to [`try_recv`] returns
`Err(TryRecvError::Lagged)`. For example, with `channel(10)` the buffer
length is 16, so lagging begins once `len` is larger than 16.

After a successful receive (including after handling `Lagged` and then
reading retained messages), `len` decreases accordingly.

[`Receiver`]: crate::sync::broadcast::Receiver
[`recv`]: crate::sync::broadcast::Receiver::recv
[`try_recv`]: crate::sync::broadcast::Receiver::try_recv

# Examples

```
use tokio::sync::broadcast;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, mut rx1) = broadcast::channel(16);

tx.send(10).unwrap();
tx.send(20).unwrap();

assert_eq!(rx1.len(), 2);
assert_eq!(rx1.recv().await.unwrap(), 10);
assert_eq!(rx1.len(), 1);
assert_eq!(rx1.recv().await.unwrap(), 20);
assert_eq!(rx1.len(), 0);
# }
```

```rust
fn len(self: &Self) -> usize
```



## tokio::sync::broadcast::Receiver::recv

*Function*

Receives the next value for this receiver.

Each [`Receiver`] handle will receive a clone of all values sent
**after** it has subscribed.

`Err(RecvError::Closed)` is returned when all `Sender` halves have
dropped, indicating that no further values can be sent on the channel.

If the [`Receiver`] handle falls behind, once the channel is full, newly
sent values overwrite old values in the ring buffer. The next call to
[`recv`] then returns `Err(RecvError::Lagged(n))`, where `n` is the
number of overwritten messages the receiver missed. The receiver stays
subscribed; its internal cursor is advanced to the oldest value still
held by the channel. A subsequent call to [`recv`] returns that value,
unless further sends overwrite it before the receiver reads it. See
[lagging](self#lagging) for details.

# Cancel safety

This method is cancel safe. If `recv` is used as a branch in
[`tokio::select!`](crate::select) and another branch
completes first, it is guaranteed that no messages were received on this
channel.

[`Receiver`]: crate::sync::broadcast::Receiver
[`recv`]: crate::sync::broadcast::Receiver::recv

# Examples

```
use tokio::sync::broadcast;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, mut rx1) = broadcast::channel(16);
let mut rx2 = tx.subscribe();

tokio::spawn(async move {
    assert_eq!(rx1.recv().await.unwrap(), 10);
    assert_eq!(rx1.recv().await.unwrap(), 20);
});

tokio::spawn(async move {
    assert_eq!(rx2.recv().await.unwrap(), 10);
    assert_eq!(rx2.recv().await.unwrap(), 20);
});

tx.send(10).unwrap();
tx.send(20).unwrap();
# }
```

Handling lag

```
use tokio::sync::broadcast;
use tokio::sync::broadcast::error::RecvError;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, mut rx) = broadcast::channel(2);

tx.send(10).unwrap();
tx.send(20).unwrap();
tx.send(30).unwrap();

// One message was overwritten before this receiver could read it.
assert!(matches!(rx.recv().await, Err(RecvError::Lagged(1))));

// Resume from the oldest retained message, or abort the task instead.
assert_eq!(20, rx.recv().await.unwrap());
assert_eq!(30, rx.recv().await.unwrap());
# }
```

```rust
fn recv(self: & mut Self) -> Result<T, RecvError>
```



## tokio::sync::broadcast::Receiver::try_recv

*Function*

Attempts to return a pending value on this receiver without awaiting.

This is useful for a flavor of "optimistic check" before deciding to
await on a receiver.

Compared with [`recv`], this function has three failure cases instead of two
(one for closed, one for an empty buffer, one for a lagging receiver).

`Err(TryRecvError::Closed)` is returned when all `Sender` halves have
dropped, indicating that no further values can be sent on the channel.

If the [`Receiver`] handle falls behind, once the channel is full, newly
sent values overwrite old values in the ring buffer. The next call to
[`try_recv`] then returns `Err(TryRecvError::Lagged(n))`, where `n` is
the number of overwritten messages the receiver missed. The receiver
stays subscribed; its internal cursor is advanced to the oldest value
still held by the channel. A subsequent call to [`try_recv`] returns
that value, unless further sends overwrite it before the receiver reads
it. If there are no values to receive, `Err(TryRecvError::Empty)` is
returned. See [lagging](self#lagging) for details.

[`recv`]: crate::sync::broadcast::Receiver::recv
[`try_recv`]: crate::sync::broadcast::Receiver::try_recv
[`Receiver`]: crate::sync::broadcast::Receiver

# Examples

```
use tokio::sync::broadcast;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, mut rx) = broadcast::channel(16);

assert!(rx.try_recv().is_err());

tx.send(10).unwrap();

let value = rx.try_recv().unwrap();
assert_eq!(10, value);
# }
```

```rust
fn try_recv(self: & mut Self) -> Result<T, TryRecvError>
```



