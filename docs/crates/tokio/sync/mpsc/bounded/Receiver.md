**tokio > sync > mpsc > bounded > Receiver**

# Module: sync::mpsc::bounded::Receiver

## Contents

**Functions**

- [`blocking_recv`](#blocking_recv) - Blocking receive to call outside of asynchronous contexts.
- [`capacity`](#capacity) - Returns the current capacity of the channel.
- [`close`](#close) - Closes the receiving half of a channel without dropping it.
- [`max_capacity`](#max_capacity) - Returns the maximum buffer capacity of the channel.
- [`poll_recv`](#poll_recv) - Polls to receive the next message on this channel.
- [`recv`](#recv) - Receives the next value for this receiver.
- [`recv_many`](#recv_many) - Receives the next values for this receiver and extends `buffer`.
- [`try_recv`](#try_recv) - Tries to receive the next value for this receiver.

---

## tokio::sync::mpsc::bounded::Receiver::blocking_recv

*Function*

Blocking receive to call outside of asynchronous contexts.

This method returns `None` if the channel has been closed and there are
no remaining messages in the channel's buffer. This indicates that no
further values can ever be received from this `Receiver`. The channel is
closed when all senders have been dropped, or when [`close`] is called.

If there are no messages in the channel's buffer, but the channel has
not yet been closed, this method will block until a message is sent or
the channel is closed.

This method is intended for use cases where you are sending from
asynchronous code to synchronous code, and will work even if the sender
is not using [`blocking_send`] to send the message.

Note that if [`close`] is called, but there are still outstanding
[`Permits`] from before it was closed, the channel is not considered
closed by `blocking_recv` until the permits are released.

[`close`]: Self::close
[`Permits`]: struct@crate::sync::mpsc::Permit
[`blocking_send`]: fn@crate::sync::mpsc::Sender::blocking_send

# Panics

This function panics if called within an asynchronous execution
context.

# Examples

```
# #[cfg(not(target_family = "wasm"))]
# {
use std::thread;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;

fn main() {
    let (tx, mut rx) = mpsc::channel::<u8>(10);

    let sync_code = thread::spawn(move || {
        assert_eq!(Some(10), rx.blocking_recv());
    });

    Runtime::new()
        .unwrap()
        .block_on(async move {
            let _ = tx.send(10).await;
        });
    sync_code.join().unwrap()
}
# }
```

```rust
fn blocking_recv(self: & mut Self) -> Option<T>
```



## tokio::sync::mpsc::bounded::Receiver::capacity

*Function*

Returns the current capacity of the channel.

The capacity goes down when the sender sends a value by calling [`Sender::send`] or by reserving
capacity with [`Sender::reserve`]. The capacity goes up when values are received.
This is distinct from [`max_capacity`], which always returns buffer capacity initially
specified when calling [`channel`].

# Examples

```
use tokio::sync::mpsc;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, mut rx) = mpsc::channel::<()>(5);

assert_eq!(rx.capacity(), 5);

// Making a reservation drops the capacity by one.
let permit = tx.reserve().await.unwrap();
assert_eq!(rx.capacity(), 4);
assert_eq!(rx.len(), 0);

// Sending and receiving a value increases the capacity by one.
permit.send(());
assert_eq!(rx.len(), 1);
rx.recv().await.unwrap();
assert_eq!(rx.capacity(), 5);

// Directly sending a message drops the capacity by one.
tx.send(()).await.unwrap();
assert_eq!(rx.capacity(), 4);
assert_eq!(rx.len(), 1);

// Receiving the message increases the capacity by one.
rx.recv().await.unwrap();
assert_eq!(rx.capacity(), 5);
assert_eq!(rx.len(), 0);
# }
```
[`capacity`]: Receiver::capacity
[`max_capacity`]: Receiver::max_capacity

```rust
fn capacity(self: &Self) -> usize
```



## tokio::sync::mpsc::bounded::Receiver::close

*Function*

Closes the receiving half of a channel without dropping it.

This prevents any further messages from being sent on the channel while
still enabling the receiver to drain messages that are buffered. Any
outstanding [`Permit`] values will still be able to send messages.

To guarantee that no messages are dropped, after calling `close()`,
`recv()` must be called until `None` is returned. If there are
outstanding [`Permit`] or [`OwnedPermit`] values, the `recv` method will
not return `None` until those are released.

[`Permit`]: Permit
[`OwnedPermit`]: OwnedPermit

# Examples

```
use tokio::sync::mpsc;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, mut rx) = mpsc::channel(20);

tokio::spawn(async move {
    let mut i = 0;
    while let Ok(permit) = tx.reserve().await {
        permit.send(i);
        i += 1;
    }
});

rx.close();

while let Some(msg) = rx.recv().await {
    println!("got {}", msg);
}

// Channel closed and no messages are lost.
# }
```

```rust
fn close(self: & mut Self)
```



## tokio::sync::mpsc::bounded::Receiver::max_capacity

*Function*

Returns the maximum buffer capacity of the channel.

The maximum capacity is the buffer capacity initially specified when calling
[`channel`]. This is distinct from [`capacity`], which returns the *current*
available buffer capacity: as messages are sent and received, the value
returned by [`capacity`] will go up or down, whereas the value
returned by [`max_capacity`] will remain constant.

# Examples

```
use tokio::sync::mpsc;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, rx) = mpsc::channel::<()>(5);

// both max capacity and capacity are the same at first
assert_eq!(rx.max_capacity(), 5);
assert_eq!(rx.capacity(), 5);

// Making a reservation doesn't change the max capacity.
let permit = tx.reserve().await.unwrap();
assert_eq!(rx.max_capacity(), 5);
// but drops the capacity by one
assert_eq!(rx.capacity(), 4);
# }
```
[`capacity`]: Receiver::capacity
[`max_capacity`]: Receiver::max_capacity

```rust
fn max_capacity(self: &Self) -> usize
```



## tokio::sync::mpsc::bounded::Receiver::poll_recv

*Function*

Polls to receive the next message on this channel.

This method returns:

 * `Poll::Pending` if no messages are available but the channel is not
   closed, or if a spurious failure happens.
 * `Poll::Ready(Some(message))` if a message is available.
 * `Poll::Ready(None)` if the channel has been closed and all messages
   sent before it was closed have been received.

When the method returns `Poll::Pending`, the `Waker` in the provided
`Context` is scheduled to receive a wakeup when a message is sent on any
receiver, or when the channel is closed.  Note that on multiple calls to
`poll_recv` or `poll_recv_many`, only the `Waker` from the `Context`
passed to the most recent call is scheduled to receive a wakeup.

If this method returns `Poll::Pending` due to a spurious failure, then
the `Waker` will be notified when the situation causing the spurious
failure has been resolved. Note that receiving such a wakeup does not
guarantee that the next call will succeed — it could fail with another
spurious failure.

```rust
fn poll_recv(self: & mut Self, cx: & mut Context) -> Poll<Option<T>>
```



## tokio::sync::mpsc::bounded::Receiver::recv

*Function*

Receives the next value for this receiver.

This method returns `None` if the channel has been closed and there are
no remaining messages in the channel's buffer. This indicates that no
further values can ever be received from this `Receiver`. The channel is
closed when all senders have been dropped, or when [`close`] is called.

If there are no messages in the channel's buffer, but the channel has
not yet been closed, this method will sleep until a message is sent or
the channel is closed.  Note that if [`close`] is called, but there are
still outstanding [`Permits`] from before it was closed, the channel is
not considered closed by `recv` until the permits are released.

# Cancel safety

This method is cancel safe. If `recv` is used as a branch in
[`tokio::select!`](crate::select) and another branch completes first,
it is guaranteed that no messages were received on this
channel.

[`close`]: Self::close
[`Permits`]: struct@crate::sync::mpsc::Permit

# Examples

```
use tokio::sync::mpsc;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, mut rx) = mpsc::channel(100);

tokio::spawn(async move {
    tx.send("hello").await.unwrap();
});

assert_eq!(Some("hello"), rx.recv().await);
assert_eq!(None, rx.recv().await);
# }
```

Values are buffered:

```
use tokio::sync::mpsc;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, mut rx) = mpsc::channel(100);

tx.send("hello").await.unwrap();
tx.send("world").await.unwrap();

assert_eq!(Some("hello"), rx.recv().await);
assert_eq!(Some("world"), rx.recv().await);
# }
```

```rust
fn recv(self: & mut Self) -> Option<T>
```



## tokio::sync::mpsc::bounded::Receiver::recv_many

*Function*

Receives the next values for this receiver and extends `buffer`.

This method extends `buffer` by no more than a fixed number of values
as specified by `limit`. If `limit` is zero, the function immediately
returns `0`. The return value is the number of values added to `buffer`.

For `limit > 0`, if there are no messages in the channel's queue, but
the channel has not yet been closed, this method will sleep until a
message is sent or the channel is closed. Note that if [`close`] is
called, but there are still outstanding [`Permits`] from before it was
closed, the channel is not considered closed by `recv_many` until the
permits are released.

For non-zero values of `limit`, this method will never return `0` unless
the channel has been closed and there are no remaining messages in the
channel's queue. This indicates that no further values can ever be
received from this `Receiver`. The channel is closed when all senders
have been dropped, or when [`close`] is called.

The capacity of `buffer` is increased as needed.

# Cancel safety

This method is cancel safe. If `recv_many` is used as a branch in
[`tokio::select!`](crate::select) and another branch completes first,
it is guaranteed that no messages were received on this
channel.

[`close`]: Self::close
[`Permits`]: struct@crate::sync::mpsc::Permit

# Examples

```
use tokio::sync::mpsc;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let mut buffer: Vec<&str> = Vec::with_capacity(2);
let limit = 2;
let (tx, mut rx) = mpsc::channel(100);
let tx2 = tx.clone();
tx2.send("first").await.unwrap();
tx2.send("second").await.unwrap();
tx2.send("third").await.unwrap();

// Call `recv_many` to receive up to `limit` (2) values.
assert_eq!(2, rx.recv_many(&mut buffer, limit).await);
assert_eq!(vec!["first", "second"], buffer);

// If the buffer is full, the next call to `recv_many`
// reserves additional capacity.
assert_eq!(1, rx.recv_many(&mut buffer, 1).await);

tokio::spawn(async move {
    tx.send("fourth").await.unwrap();
});

// 'tx' is dropped, but `recv_many`
// is guaranteed not to return 0 as the channel
// is not yet closed.
assert_eq!(1, rx.recv_many(&mut buffer, 1).await);
assert_eq!(vec!["first", "second", "third", "fourth"], buffer);

// Once the last sender is dropped, the channel is
// closed and `recv_many` returns 0, capacity unchanged.
drop(tx2);
assert_eq!(0, rx.recv_many(&mut buffer, limit).await);
assert_eq!(vec!["first", "second", "third", "fourth"], buffer);
# }
```

```rust
fn recv_many(self: & mut Self, buffer: & mut Vec<T>, limit: usize) -> usize
```



## tokio::sync::mpsc::bounded::Receiver::try_recv

*Function*

Tries to receive the next value for this receiver.

This method returns the [`Empty`] error if the channel is currently
empty, but there are still outstanding [senders] or [permits].

This method returns the [`Disconnected`] error if the channel is
currently empty, and there are no outstanding [senders] or [permits].

Unlike the [`poll_recv`] method, this method will never return an
[`Empty`] error spuriously.

[`Empty`]: crate::sync::mpsc::error::TryRecvError::Empty
[`Disconnected`]: crate::sync::mpsc::error::TryRecvError::Disconnected
[`poll_recv`]: Self::poll_recv
[senders]: crate::sync::mpsc::Sender
[permits]: crate::sync::mpsc::Permit

# Examples

```
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TryRecvError;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, mut rx) = mpsc::channel(100);

tx.send("hello").await.unwrap();

assert_eq!(Ok("hello"), rx.try_recv());
assert_eq!(Err(TryRecvError::Empty), rx.try_recv());

tx.send("hello").await.unwrap();
// Drop the last sender, closing the channel.
drop(tx);

assert_eq!(Ok("hello"), rx.try_recv());
assert_eq!(Err(TryRecvError::Disconnected), rx.try_recv());
# }
```

```rust
fn try_recv(self: & mut Self) -> Result<T, TryRecvError>
```



