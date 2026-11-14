**tokio > sync > mpsc > unbounded > UnboundedReceiver**

# Module: sync::mpsc::unbounded::UnboundedReceiver

## Contents

**Functions**

- [`blocking_recv`](#blocking_recv) - Blocking receive to call outside of asynchronous contexts.
- [`close`](#close) - Closes the receiving half of a channel, without dropping it.
- [`poll_recv`](#poll_recv) - Polls to receive the next message on this channel.
- [`recv`](#recv) - Receives the next value for this receiver.
- [`recv_many`](#recv_many) - Receives the next values for this receiver and extends `buffer`.

---

## tokio::sync::mpsc::unbounded::UnboundedReceiver::blocking_recv

*Function*

Blocking receive to call outside of asynchronous contexts.

# Panics

This function panics if called within an asynchronous execution
context.

# Examples

```
# #[cfg(not(target_family = "wasm"))]
# {
use std::thread;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::unbounded_channel::<u8>();

    let sync_code = thread::spawn(move || {
        assert_eq!(Some(10), rx.blocking_recv());
    });

    let _ = tx.send(10);
    sync_code.join().unwrap();
}
# }
```

```rust
fn blocking_recv(self: & mut Self) -> Option<T>
```



## tokio::sync::mpsc::unbounded::UnboundedReceiver::close

*Function*

Closes the receiving half of a channel, without dropping it.

This prevents any further messages from being sent on the channel while
still enabling the receiver to drain messages that are buffered.

To guarantee that no messages are dropped, after calling `close()`,
`recv()` must be called until `None` is returned.

```rust
fn close(self: & mut Self)
```



## tokio::sync::mpsc::unbounded::UnboundedReceiver::poll_recv

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



## tokio::sync::mpsc::unbounded::UnboundedReceiver::recv

*Function*

Receives the next value for this receiver.

This method returns `None` if the channel has been closed and there are
no remaining messages in the channel's buffer. This indicates that no
further values can ever be received from this `Receiver`. The channel is
closed when all senders have been dropped, or when [`close`] is called.

If there are no messages in the channel's buffer, but the channel has
not yet been closed, this method will sleep until a message is sent or
the channel is closed.

# Cancel safety

This method is cancel safe. If `recv` is used as a branch in
[`tokio::select!`](crate::select) and another branch completes first,
it is guaranteed that no messages were received on this
channel.

[`close`]: Self::close

# Examples

```
use tokio::sync::mpsc;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, mut rx) = mpsc::unbounded_channel();

tokio::spawn(async move {
    tx.send("hello").unwrap();
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
let (tx, mut rx) = mpsc::unbounded_channel();

tx.send("hello").unwrap();
tx.send("world").unwrap();

assert_eq!(Some("hello"), rx.recv().await);
assert_eq!(Some("world"), rx.recv().await);
# }
```

```rust
fn recv(self: & mut Self) -> Option<T>
```



## tokio::sync::mpsc::unbounded::UnboundedReceiver::recv_many

*Function*

Receives the next values for this receiver and extends `buffer`.

This method extends `buffer` by no more than a fixed number of values
as specified by `limit`. If `limit` is zero, the function returns
immediately with `0`. The return value is the number of values added to
`buffer`.

For `limit > 0`, if there are no messages in the channel's queue,
but the channel has not yet been closed, this method will sleep
until a message is sent or the channel is closed.

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

# Examples

```
use tokio::sync::mpsc;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let mut buffer: Vec<&str> = Vec::with_capacity(2);
let limit = 2;
let (tx, mut rx) = mpsc::unbounded_channel();
let tx2 = tx.clone();
tx2.send("first").unwrap();
tx2.send("second").unwrap();
tx2.send("third").unwrap();

// Call `recv_many` to receive up to `limit` (2) values.
assert_eq!(2, rx.recv_many(&mut buffer, limit).await);
assert_eq!(vec!["first", "second"], buffer);

// If the buffer is full, the next call to `recv_many`
// reserves additional capacity.
assert_eq!(1, rx.recv_many(&mut buffer, limit).await);

tokio::spawn(async move {
    tx.send("fourth").unwrap();
});

// 'tx' is dropped, but `recv_many`
// is guaranteed not to return 0 as the channel
// is not yet closed.
assert_eq!(1, rx.recv_many(&mut buffer, limit).await);
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



