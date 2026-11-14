**tokio > sync > mpsc > bounded**

# Module: sync::mpsc::bounded

## Contents

**Structs**

- [`OwnedPermit`](#ownedpermit) - Owned permit to send one value into the channel.
- [`Permit`](#permit) - Permits to send one value into the channel.
- [`PermitIterator`](#permititerator) - An [`Iterator`] of [`Permit`] that can be used to hold `n` slots in the channel.
- [`Receiver`](#receiver) - Receives values from the associated `Sender`.
- [`Sender`](#sender) - Sends values to the associated `Receiver`.
- [`WeakSender`](#weaksender) - A sender that does not prevent the channel from being closed.

**Functions**

- [`channel`](#channel) - Creates a bounded mpsc channel for communicating between asynchronous tasks

---

## tokio::sync::mpsc::bounded::OwnedPermit

*Struct*

Owned permit to send one value into the channel.

This is identical to the [`Permit`] type, except that it moves the sender
rather than borrowing it.

`OwnedPermit` values are returned by [`Sender::reserve_owned()`] and
[`Sender::try_reserve_owned()`] and are used to guarantee channel capacity
before generating a message to send.

[`Permit`]: Permit
[`Sender::reserve_owned()`]: Sender::reserve_owned
[`Sender::try_reserve_owned()`]: Sender::try_reserve_owned

**Generic Parameters:**
- T

**Methods:**

- `fn send(self: Self, value: T) -> Sender<T>` - Sends a value using the reserved capacity.
- `fn release(self: Self) -> Sender<T>` - Releases the reserved capacity *without* sending a message, returning the
- `fn same_channel(self: &Self, other: &Self) -> bool` - Returns `true` if permits belong to the same channel.
- `fn same_channel_as_sender(self: &Self, sender: &Sender<T>) -> bool` - Returns `true` if this permit belongs to the same channel as the given [`Sender`].

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`
- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`



## tokio::sync::mpsc::bounded::Permit

*Struct*

Permits to send one value into the channel.

`Permit` values are returned by [`Sender::reserve()`] and [`Sender::try_reserve()`]
and are used to guarantee channel capacity before generating a message to send.

[`Sender::reserve()`]: Sender::reserve
[`Sender::try_reserve()`]: Sender::try_reserve

**Generic Parameters:**
- 'a
- T

**Methods:**

- `fn send(self: Self, value: T)` - Sends a value using the reserved capacity.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **Drop**
  - `fn drop(self: & mut Self)`



## tokio::sync::mpsc::bounded::PermitIterator

*Struct*

An [`Iterator`] of [`Permit`] that can be used to hold `n` slots in the channel.

`PermitIterator` values are returned by [`Sender::reserve_many()`] and [`Sender::try_reserve_many()`]
and are used to guarantee channel capacity before generating `n` messages to send.

[`Sender::reserve_many()`]: Sender::reserve_many
[`Sender::try_reserve_many()`]: Sender::try_reserve_many

**Generic Parameters:**
- 'a
- T

**Traits:** ExactSizeIterator, FusedIterator

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`
- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



## tokio::sync::mpsc::bounded::Receiver

*Struct*

Receives values from the associated `Sender`.

Instances are created by the [`channel`] function.

This receiver can be turned into a `Stream` using [`ReceiverStream`].

[`ReceiverStream`]: https://docs.rs/tokio-stream/0.1/tokio_stream/wrappers/struct.ReceiverStream.html

**Generic Parameters:**
- T

**Methods:**

- `fn recv(self: & mut Self) -> Option<T>` - Receives the next value for this receiver.
- `fn recv_many(self: & mut Self, buffer: & mut Vec<T>, limit: usize) -> usize` - Receives the next values for this receiver and extends `buffer`.
- `fn try_recv(self: & mut Self) -> Result<T, TryRecvError>` - Tries to receive the next value for this receiver.
- `fn blocking_recv(self: & mut Self) -> Option<T>` - Blocking receive to call outside of asynchronous contexts.
- `fn blocking_recv_many(self: & mut Self, buffer: & mut Vec<T>, limit: usize) -> usize` - Variant of [`Self::recv_many`] for blocking contexts.
- `fn close(self: & mut Self)` - Closes the receiving half of a channel without dropping it.
- `fn is_closed(self: &Self) -> bool` - Checks if a channel is closed.
- `fn is_empty(self: &Self) -> bool` - Checks if a channel is empty.
- `fn len(self: &Self) -> usize` - Returns the number of messages in the channel.
- `fn capacity(self: &Self) -> usize` - Returns the current capacity of the channel.
- `fn max_capacity(self: &Self) -> usize` - Returns the maximum buffer capacity of the channel.
- `fn poll_recv(self: & mut Self, cx: & mut Context) -> Poll<Option<T>>` - Polls to receive the next message on this channel.
- `fn poll_recv_many(self: & mut Self, cx: & mut Context, buffer: & mut Vec<T>, limit: usize) -> Poll<usize>` - Polls to receive multiple messages on this channel, extending the provided buffer.
- `fn sender_strong_count(self: &Self) -> usize` - Returns the number of [`Sender`] handles.
- `fn sender_weak_count(self: &Self) -> usize` - Returns the number of [`WeakSender`] handles.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`



## tokio::sync::mpsc::bounded::Sender

*Struct*

Sends values to the associated `Receiver`.

Instances are created by the [`channel`] function.

To convert the `Sender` into a `Sink` or use it in a poll function, you can
use the [`PollSender`] utility.

[`PollSender`]: https://docs.rs/tokio-util/latest/tokio_util/sync/struct.PollSender.html

**Generic Parameters:**
- T

**Methods:**

- `fn send(self: &Self, value: T) -> Result<(), SendError<T>>` - Sends a value, waiting until there is capacity.
- `fn closed(self: &Self)` - Completes when the receiver has dropped.
- `fn try_send(self: &Self, message: T) -> Result<(), TrySendError<T>>` - Attempts to immediately send a message on this `Sender`.
- `fn send_timeout(self: &Self, value: T, timeout: Duration) -> Result<(), SendTimeoutError<T>>` - Sends a value, waiting until there is capacity, but only for a limited time.
- `fn blocking_send(self: &Self, value: T) -> Result<(), SendError<T>>` - Blocking send to call outside of asynchronous contexts.
- `fn is_closed(self: &Self) -> bool` - Checks if the channel has been closed. This happens when the
- `fn reserve(self: &Self) -> Result<Permit<T>, SendError<()>>` - Waits for channel capacity. Once capacity to send one message is
- `fn reserve_many(self: &Self, n: usize) -> Result<PermitIterator<T>, SendError<()>>` - Waits for channel capacity. Once capacity to send `n` messages is
- `fn reserve_owned(self: Self) -> Result<OwnedPermit<T>, SendError<()>>` - Waits for channel capacity, moving the `Sender` and returning an owned
- `fn try_reserve(self: &Self) -> Result<Permit<T>, TrySendError<()>>` - Tries to acquire a slot in the channel without waiting for the slot to become
- `fn try_reserve_many(self: &Self, n: usize) -> Result<PermitIterator<T>, TrySendError<()>>` - Tries to acquire `n` slots in the channel without waiting for the slot to become
- `fn try_reserve_owned(self: Self) -> Result<OwnedPermit<T>, TrySendError<Self>>` - Tries to acquire a slot in the channel without waiting for the slot to become
- `fn same_channel(self: &Self, other: &Self) -> bool` - Returns `true` if senders belong to the same channel.
- `fn capacity(self: &Self) -> usize` - Returns the current capacity of the channel.
- `fn downgrade(self: &Self) -> WeakSender<T>` - Converts the `Sender` to a [`WeakSender`] that does not count
- `fn max_capacity(self: &Self) -> usize` - Returns the maximum buffer capacity of the channel.
- `fn strong_count(self: &Self) -> usize` - Returns the number of [`Sender`] handles.
- `fn weak_count(self: &Self) -> usize` - Returns the number of [`WeakSender`] handles.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`



## tokio::sync::mpsc::bounded::WeakSender

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
use tokio::sync::mpsc::channel;

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

- `fn upgrade(self: &Self) -> Option<Sender<T>>` - Tries to convert a `WeakSender` into a [`Sender`]. This will return `Some`
- `fn strong_count(self: &Self) -> usize` - Returns the number of [`Sender`] handles.
- `fn weak_count(self: &Self) -> usize` - Returns the number of [`WeakSender`] handles.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Drop**
  - `fn drop(self: & mut Self)`



## tokio::sync::mpsc::bounded::channel

*Function*

Creates a bounded mpsc channel for communicating between asynchronous tasks
with backpressure.

The channel will buffer up to the provided number of messages.  Once the
buffer is full, attempts to send new messages will wait until a message is
received from the channel. The provided buffer capacity must be at least 1.

All data sent on `Sender` will become available on `Receiver` in the same
order as it was sent.

The `Sender` can be cloned to `send` to the same channel from multiple code
locations. Only one `Receiver` is supported.

If the `Receiver` is disconnected while trying to `send`, the `send` method
will return a `SendError`. Similarly, if `Sender` is disconnected while
trying to `recv`, the `recv` method will return `None`.

# Panics

Panics if the buffer capacity is 0, or too large. Currently the maximum
capacity is [`Semaphore::MAX_PERMITS`].

[`Semaphore::MAX_PERMITS`]: crate::sync::Semaphore::MAX_PERMITS

# Examples

```rust
use tokio::sync::mpsc;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, mut rx) = mpsc::channel(100);

tokio::spawn(async move {
    for i in 0..10 {
        if let Err(_) = tx.send(i).await {
            println!("receiver dropped");
            return;
        }
    }
});

while let Some(i) = rx.recv().await {
     println!("got = {}", i);
}
# }
```

```rust
fn channel<T>(buffer: usize) -> (Sender<T>, Receiver<T>)
```



