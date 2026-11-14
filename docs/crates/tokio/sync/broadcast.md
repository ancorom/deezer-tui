**tokio > sync > broadcast**

# Module: sync::broadcast

## Contents

**Modules**

- [`error`](#error) - Broadcast error types

**Structs**

- [`Receiver`](#receiver) - Receiving-half of the [`broadcast`] channel.
- [`Sender`](#sender) - Sending-half of the [`broadcast`] channel.
- [`WeakSender`](#weaksender) - A sender that does not prevent the channel from being closed.

**Functions**

- [`channel`](#channel) - Create a bounded, multi-producer, multi-consumer channel where each sent

---

## tokio::sync::broadcast::Receiver

*Struct*

Receiving-half of the [`broadcast`] channel.

Must not be used concurrently. Messages may be retrieved using
[`recv`][Receiver::recv].

To turn this receiver into a `Stream`, you can use the [`BroadcastStream`]
wrapper.

[`BroadcastStream`]: https://docs.rs/tokio-stream/0.1/tokio_stream/wrappers/struct.BroadcastStream.html

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

[`broadcast`]: crate::sync::broadcast

**Generic Parameters:**
- T

**Methods:**

- `fn len(self: &Self) -> usize` - Returns the number of messages that were sent into the channel and that
- `fn is_empty(self: &Self) -> bool` - Returns true if there aren't any messages in the channel that the [`Receiver`]
- `fn same_channel(self: &Self, other: &Self) -> bool` - Returns `true` if receivers belong to the same channel.
- `fn sender_strong_count(self: &Self) -> usize` - Returns the number of [`Sender`] handles.
- `fn sender_weak_count(self: &Self) -> usize` - Returns the number of [`WeakSender`] handles.
- `fn is_closed(self: &Self) -> bool` - Checks if a channel is closed.
- `fn resubscribe(self: &Self) -> Self` - Re-subscribes to the channel starting from the current tail element.
- `fn recv(self: & mut Self) -> Result<T, RecvError>` - Receives the next value for this receiver.
- `fn try_recv(self: & mut Self) -> Result<T, TryRecvError>` - Attempts to return a pending value on this receiver without awaiting.
- `fn blocking_recv(self: & mut Self) -> Result<T, RecvError>` - Blocking receive to call outside of asynchronous contexts.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **Drop**
  - `fn drop(self: & mut Self)`



## tokio::sync::broadcast::Sender

*Struct*

Sending-half of the [`broadcast`] channel.

May be used from many threads. Messages can be sent with
[`send`][Sender::send].

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

[`broadcast`]: crate::sync::broadcast

**Generic Parameters:**
- T

**Methods:**

- `fn new(capacity: usize) -> Self` - Creates the sending-half of the [`broadcast`] channel.
- `fn send(self: &Self, value: T) -> Result<usize, SendError<T>>` - Attempts to send a value to all active [`Receiver`] handles, returning
- `fn subscribe(self: &Self) -> Receiver<T>` - Creates a new [`Receiver`] handle that will receive values sent **after**
- `fn downgrade(self: &Self) -> WeakSender<T>` - Converts the `Sender` to a [`WeakSender`] that does not count
- `fn len(self: &Self) -> usize` - Returns the number of queued values.
- `fn is_empty(self: &Self) -> bool` - Returns true if there are no queued values.
- `fn receiver_count(self: &Self) -> usize` - Returns the number of active receivers.
- `fn same_channel(self: &Self, other: &Self) -> bool` - Returns `true` if senders belong to the same channel.
- `fn closed(self: &Self)` - A future which completes when the number of [Receiver]s subscribed to this `Sender` reaches
- `fn strong_count(self: &Self) -> usize` - Returns the number of [`Sender`] handles.
- `fn weak_count(self: &Self) -> usize` - Returns the number of [`WeakSender`] handles.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Sender<T>`
- **Drop**
  - `fn drop(self: & mut Self)`



## tokio::sync::broadcast::WeakSender

*Struct*

A sender that does not prevent the channel from being closed.

If all [`Sender`] instances of a channel were dropped and only `WeakSender`
instances remain, the channel is closed.

In order to send messages, the `WeakSender` needs to be upgraded using
[`WeakSender::upgrade`], which returns `Option<Sender>`. It returns `None`
if all `Sender`s have been dropped, and otherwise it returns a `Sender`.

[`Sender`]: Sender
[`WeakSender::upgrade`]: WeakSender::upgrade

# Examples

```
use tokio::sync::broadcast::channel;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, _rx) = channel::<i32>(15);
let tx_weak = tx.downgrade();

// Upgrading will succeed because `tx` still exists.
assert!(tx_weak.upgrade().is_some());

// If we drop `tx`, then it will fail.
drop(tx);
assert!(tx_weak.clone().upgrade().is_none());
# }
```

**Generic Parameters:**
- T

**Methods:**

- `fn upgrade(self: &Self) -> Option<Sender<T>>` - Tries to convert a `WeakSender` into a [`Sender`].
- `fn strong_count(self: &Self) -> usize` - Returns the number of [`Sender`] handles.
- `fn weak_count(self: &Self) -> usize` - Returns the number of [`WeakSender`] handles.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> WeakSender<T>`
- **Drop**
  - `fn drop(self: & mut Self)`



## tokio::sync::broadcast::channel

*Function*

Create a bounded, multi-producer, multi-consumer channel where each sent
value is broadcasted to all active receivers.

**Note:** The provided `capacity` is rounded **up** to the next power of
two. That rounded size is the number of messages the internal ring buffer
can retain, and is what [lag detection](self#lagging) uses. For example,
`channel(3)` behaves as if the capacity were 4.

All data sent on [`Sender`] will become available on every active
[`Receiver`] in the same order as it was sent.

The `Sender` can be cloned to `send` to the same channel from multiple
points in the process or it can be used concurrently from an `Arc`. New
`Receiver` handles are created by calling [`Sender::subscribe`].

If all [`Receiver`] handles are dropped, the `send` method will return a
[`SendError`]. Similarly, if all [`Sender`] handles are dropped, the [`recv`]
method will return a [`RecvError`].

[`Sender`]: crate::sync::broadcast::Sender
[`Sender::subscribe`]: crate::sync::broadcast::Sender::subscribe
[`Receiver`]: crate::sync::broadcast::Receiver
[`recv`]: crate::sync::broadcast::Receiver::recv
[`SendError`]: crate::sync::broadcast::error::SendError
[`RecvError`]: crate::sync::broadcast::error::RecvError

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

# Panics

This will panic if `capacity` is equal to `0` or exceeds `usize::MAX / 2`.

This pre-allocates space for `capacity` messages. Allocation failure may result in a panic or
[an allocation error](std::alloc::handle_alloc_error).

```rust
fn channel<T>(capacity: usize) -> (Sender<T>, Receiver<T>)
```



## Module: error

Broadcast error types



