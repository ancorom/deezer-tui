**tokio > sync > mpsc > unbounded**

# Module: sync::mpsc::unbounded

## Contents

**Structs**

- [`UnboundedReceiver`](#unboundedreceiver) - Receive values from the associated `UnboundedSender`.
- [`UnboundedSender`](#unboundedsender) - Send values to the associated `UnboundedReceiver`.
- [`WeakUnboundedSender`](#weakunboundedsender) - An unbounded sender that does not prevent the channel from being closed.

**Functions**

- [`unbounded_channel`](#unbounded_channel) - Creates an unbounded mpsc channel for communicating between asynchronous

---

## tokio::sync::mpsc::unbounded::UnboundedReceiver

*Struct*

Receive values from the associated `UnboundedSender`.

Instances are created by the [`unbounded_channel`] function.

This receiver can be turned into a `Stream` using [`UnboundedReceiverStream`].

[`UnboundedReceiverStream`]: https://docs.rs/tokio-stream/0.1/tokio_stream/wrappers/struct.UnboundedReceiverStream.html

**Generic Parameters:**
- T

**Methods:**

- `fn recv(self: & mut Self) -> Option<T>` - Receives the next value for this receiver.
- `fn recv_many(self: & mut Self, buffer: & mut Vec<T>, limit: usize) -> usize` - Receives the next values for this receiver and extends `buffer`.
- `fn try_recv(self: & mut Self) -> Result<T, TryRecvError>` - Tries to receive the next value for this receiver.
- `fn blocking_recv(self: & mut Self) -> Option<T>` - Blocking receive to call outside of asynchronous contexts.
- `fn blocking_recv_many(self: & mut Self, buffer: & mut Vec<T>, limit: usize) -> usize` - Variant of [`Self::recv_many`] for blocking contexts.
- `fn close(self: & mut Self)` - Closes the receiving half of a channel, without dropping it.
- `fn is_closed(self: &Self) -> bool` - Checks if a channel is closed.
- `fn is_empty(self: &Self) -> bool` - Checks if a channel is empty.
- `fn len(self: &Self) -> usize` - Returns the number of messages in the channel.
- `fn poll_recv(self: & mut Self, cx: & mut Context) -> Poll<Option<T>>` - Polls to receive the next message on this channel.
- `fn poll_recv_many(self: & mut Self, cx: & mut Context, buffer: & mut Vec<T>, limit: usize) -> Poll<usize>` - Polls to receive multiple messages on this channel, extending the provided buffer.
- `fn sender_strong_count(self: &Self) -> usize` - Returns the number of [`UnboundedSender`] handles.
- `fn sender_weak_count(self: &Self) -> usize` - Returns the number of [`WeakUnboundedSender`] handles.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`



## tokio::sync::mpsc::unbounded::UnboundedSender

*Struct*

Send values to the associated `UnboundedReceiver`.

Instances are created by the [`unbounded_channel`] function.

**Generic Parameters:**
- T

**Methods:**

- `fn send(self: &Self, message: T) -> Result<(), SendError<T>>` - Attempts to send a message on this `UnboundedSender` without blocking.
- `fn closed(self: &Self)` - Completes when the receiver has dropped.
- `fn is_closed(self: &Self) -> bool` - Checks if the channel has been closed. This happens when the
- `fn same_channel(self: &Self, other: &Self) -> bool` - Returns `true` if senders belong to the same channel.
- `fn downgrade(self: &Self) -> WeakUnboundedSender<T>` - Converts the `UnboundedSender` to a [`WeakUnboundedSender`] that does not count
- `fn strong_count(self: &Self) -> usize` - Returns the number of [`UnboundedSender`] handles.
- `fn weak_count(self: &Self) -> usize` - Returns the number of [`WeakUnboundedSender`] handles.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`



## tokio::sync::mpsc::unbounded::WeakUnboundedSender

*Struct*

An unbounded sender that does not prevent the channel from being closed.

If all [`UnboundedSender`] instances of a channel were dropped and only
`WeakUnboundedSender` instances remain, the channel is closed.

In order to send messages, the `WeakUnboundedSender` needs to be upgraded using
[`WeakUnboundedSender::upgrade`], which returns `Option<UnboundedSender>`. It returns `None`
if all `UnboundedSender`s have been dropped, and otherwise it returns an `UnboundedSender`.

[`UnboundedSender`]: UnboundedSender
[`WeakUnboundedSender::upgrade`]: WeakUnboundedSender::upgrade

# Examples

```
use tokio::sync::mpsc::unbounded_channel;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, _rx) = unbounded_channel::<i32>();
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

- `fn upgrade(self: &Self) -> Option<UnboundedSender<T>>` - Tries to convert a `WeakUnboundedSender` into an [`UnboundedSender`].
- `fn strong_count(self: &Self) -> usize` - Returns the number of [`UnboundedSender`] handles.
- `fn weak_count(self: &Self) -> usize` - Returns the number of [`WeakUnboundedSender`] handles.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Drop**
  - `fn drop(self: & mut Self)`
- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`



## tokio::sync::mpsc::unbounded::unbounded_channel

*Function*

Creates an unbounded mpsc channel for communicating between asynchronous
tasks without backpressure.

A `send` on this channel will always succeed as long as the receive half has
not been closed. If the receiver falls behind, messages will be arbitrarily
buffered.

**Note** that the amount of available system memory is an implicit bound to
the channel. Using an `unbounded` channel has the ability of causing the
process to run out of memory. In this case, the process will be aborted.

```rust
fn unbounded_channel<T>() -> (UnboundedSender<T>, UnboundedReceiver<T>)
```



