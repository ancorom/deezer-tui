**tokio > sync > mpsc > bounded > Sender**

# Module: sync::mpsc::bounded::Sender

## Contents

**Functions**

- [`blocking_send`](#blocking_send) - Blocking send to call outside of asynchronous contexts.
- [`capacity`](#capacity) - Returns the current capacity of the channel.
- [`max_capacity`](#max_capacity) - Returns the maximum buffer capacity of the channel.
- [`reserve`](#reserve) - Waits for channel capacity. Once capacity to send one message is
- [`reserve_many`](#reserve_many) - Waits for channel capacity. Once capacity to send `n` messages is
- [`reserve_owned`](#reserve_owned) - Waits for channel capacity, moving the `Sender` and returning an owned
- [`send`](#send) - Sends a value, waiting until there is capacity.
- [`send_timeout`](#send_timeout) - Sends a value, waiting until there is capacity, but only for a limited time.
- [`try_reserve`](#try_reserve) - Tries to acquire a slot in the channel without waiting for the slot to become
- [`try_reserve_many`](#try_reserve_many) - Tries to acquire `n` slots in the channel without waiting for the slot to become
- [`try_reserve_owned`](#try_reserve_owned) - Tries to acquire a slot in the channel without waiting for the slot to become
- [`try_send`](#try_send) - Attempts to immediately send a message on this `Sender`.

---

## tokio::sync::mpsc::bounded::Sender::blocking_send

*Function*

Blocking send to call outside of asynchronous contexts.

This method is intended for use cases where you are sending from
synchronous code to asynchronous code, and will work even if the
receiver is not using [`blocking_recv`] to receive the message.

[`blocking_recv`]: fn@crate::sync::mpsc::Receiver::blocking_recv

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
    let (tx, mut rx) = mpsc::channel::<u8>(1);

    let sync_code = thread::spawn(move || {
        tx.blocking_send(10).unwrap();
    });

    Runtime::new().unwrap().block_on(async move {
        assert_eq!(Some(10), rx.recv().await);
    });
    sync_code.join().unwrap()
}
# }
```

```rust
fn blocking_send(self: &Self, value: T) -> Result<(), SendError<T>>
```



## tokio::sync::mpsc::bounded::Sender::capacity

*Function*

Returns the current capacity of the channel.

The capacity goes down when sending a value by calling [`send`] or by reserving capacity
with [`reserve`]. The capacity goes up when values are received by the [`Receiver`].
This is distinct from [`max_capacity`], which always returns buffer capacity initially
specified when calling [`channel`]

# Examples

```
use tokio::sync::mpsc;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, mut rx) = mpsc::channel::<()>(5);

assert_eq!(tx.capacity(), 5);

// Making a reservation drops the capacity by one.
let permit = tx.reserve().await.unwrap();
assert_eq!(tx.capacity(), 4);

// Sending and receiving a value increases the capacity by one.
permit.send(());
rx.recv().await.unwrap();
assert_eq!(tx.capacity(), 5);
# }
```

[`send`]: Sender::send
[`reserve`]: Sender::reserve
[`channel`]: channel
[`max_capacity`]: Sender::max_capacity

```rust
fn capacity(self: &Self) -> usize
```



## tokio::sync::mpsc::bounded::Sender::max_capacity

*Function*

Returns the maximum buffer capacity of the channel.

The maximum capacity is the buffer capacity initially specified when calling
[`channel`]. This is distinct from [`capacity`], which returns the *current*
available buffer capacity: as messages are sent and received, the
value returned by [`capacity`] will go up or down, whereas the value
returned by [`max_capacity`] will remain constant.

# Examples

```
use tokio::sync::mpsc;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, _rx) = mpsc::channel::<()>(5);

// both max capacity and capacity are the same at first
assert_eq!(tx.max_capacity(), 5);
assert_eq!(tx.capacity(), 5);

// Making a reservation doesn't change the max capacity.
let permit = tx.reserve().await.unwrap();
assert_eq!(tx.max_capacity(), 5);
// but drops the capacity by one
assert_eq!(tx.capacity(), 4);
# }
```

[`channel`]: channel
[`max_capacity`]: Sender::max_capacity
[`capacity`]: Sender::capacity

```rust
fn max_capacity(self: &Self) -> usize
```



## tokio::sync::mpsc::bounded::Sender::reserve

*Function*

Waits for channel capacity. Once capacity to send one message is
available, it is reserved for the caller.

If the channel is full, the function waits for the number of unreceived
messages to become less than the channel capacity. Capacity to send one
message is reserved for the caller. A [`Permit`] is returned to track
the reserved capacity. The [`send`] function on [`Permit`] consumes the
reserved capacity.

Dropping [`Permit`] without sending a message releases the capacity back
to the channel.

[`Permit`]: Permit
[`send`]: Permit::send

# Cancel safety

This channel uses a queue to ensure that calls to `send` and `reserve`
complete in the order they were requested.  Cancelling a call to
`reserve` makes you lose your place in the queue.

# Examples

```
use tokio::sync::mpsc;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, mut rx) = mpsc::channel(1);

// Reserve capacity
let permit = tx.reserve().await.unwrap();

// Trying to send directly on the `tx` will fail due to no
// available capacity.
assert!(tx.try_send(123).is_err());

// Sending on the permit succeeds
permit.send(456);

// The value sent on the permit is received
assert_eq!(rx.recv().await.unwrap(), 456);
# }
```

```rust
fn reserve(self: &Self) -> Result<Permit<T>, SendError<()>>
```



## tokio::sync::mpsc::bounded::Sender::reserve_many

*Function*

Waits for channel capacity. Once capacity to send `n` messages is
available, it is reserved for the caller.

If the channel is full or if there are fewer than `n` permits available, the function waits
for the number of unreceived messages to become `n` less than the channel capacity.
Capacity to send `n` message is then reserved for the caller.

A [`PermitIterator`] is returned to track the reserved capacity.
You can call this [`Iterator`] until it is exhausted to
get a [`Permit`] and then call [`Permit::send`]. This function is similar to
[`try_reserve_many`] except it waits for the slots to become available.

If the channel is closed, the function returns a [`SendError`].

Dropping [`PermitIterator`] without consuming it entirely releases the remaining
permits back to the channel.

[`PermitIterator`]: PermitIterator
[`Permit`]: Permit
[`send`]: Permit::send
[`try_reserve_many`]: Sender::try_reserve_many

# Cancel safety

This channel uses a queue to ensure that calls to `send` and `reserve_many`
complete in the order they were requested. Cancelling a call to
`reserve_many` makes you lose your place in the queue.

# Examples

```
use tokio::sync::mpsc;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, mut rx) = mpsc::channel(2);

// Reserve capacity
let mut permit = tx.reserve_many(2).await.unwrap();

// Trying to send directly on the `tx` will fail due to no
// available capacity.
assert!(tx.try_send(123).is_err());

// Sending with the permit iterator succeeds
permit.next().unwrap().send(456);
permit.next().unwrap().send(457);

// The iterator should now be exhausted
assert!(permit.next().is_none());

// The value sent on the permit is received
assert_eq!(rx.recv().await.unwrap(), 456);
assert_eq!(rx.recv().await.unwrap(), 457);
# }
```

```rust
fn reserve_many(self: &Self, n: usize) -> Result<PermitIterator<T>, SendError<()>>
```



## tokio::sync::mpsc::bounded::Sender::reserve_owned

*Function*

Waits for channel capacity, moving the `Sender` and returning an owned
permit. Once capacity to send one message is available, it is reserved
for the caller.

This moves the sender _by value_, and returns an owned permit that can
be used to send a message into the channel. Unlike [`Sender::reserve`],
this method may be used in cases where the permit must be valid for the
`'static` lifetime. `Sender`s may be cloned cheaply (`Sender::clone` is
essentially a reference count increment, comparable to [`Arc::clone`]),
so when multiple [`OwnedPermit`]s are needed or the `Sender` cannot be
moved, it can be cloned prior to calling `reserve_owned`.

If the channel is full, the function waits for the number of unreceived
messages to become less than the channel capacity. Capacity to send one
message is reserved for the caller. An [`OwnedPermit`] is returned to
track the reserved capacity. The [`send`] function on [`OwnedPermit`]
consumes the reserved capacity.

Dropping the [`OwnedPermit`] without sending a message releases the
capacity back to the channel.

# Cancel safety

This channel uses a queue to ensure that calls to `send` and `reserve`
complete in the order they were requested.  Cancelling a call to
`reserve_owned` makes you lose your place in the queue.

# Examples
Sending a message using an [`OwnedPermit`]:
```
use tokio::sync::mpsc;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, mut rx) = mpsc::channel(1);

// Reserve capacity, moving the sender.
let permit = tx.reserve_owned().await.unwrap();

// Send a message, consuming the permit and returning
// the moved sender.
let tx = permit.send(123);

// The value sent on the permit is received.
assert_eq!(rx.recv().await.unwrap(), 123);

// The sender can now be used again.
tx.send(456).await.unwrap();
# }
```

When multiple [`OwnedPermit`]s are needed, or the sender cannot be moved
by value, it can be inexpensively cloned before calling `reserve_owned`:

```
use tokio::sync::mpsc;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, mut rx) = mpsc::channel(1);

// Clone the sender and reserve capacity.
let permit = tx.clone().reserve_owned().await.unwrap();

// Trying to send directly on the `tx` will fail due to no
// available capacity.
assert!(tx.try_send(123).is_err());

// Sending on the permit succeeds.
permit.send(456);

// The value sent on the permit is received
assert_eq!(rx.recv().await.unwrap(), 456);
# }
```

[`Sender::reserve`]: Sender::reserve
[`OwnedPermit`]: OwnedPermit
[`send`]: OwnedPermit::send
[`Arc::clone`]: std::sync::Arc::clone

```rust
fn reserve_owned(self: Self) -> Result<OwnedPermit<T>, SendError<()>>
```



## tokio::sync::mpsc::bounded::Sender::send

*Function*

Sends a value, waiting until there is capacity.

A successful send occurs when it is determined that the other end of the
channel has not hung up already. An unsuccessful send would be one where
the corresponding receiver has already been closed. Note that a return
value of `Err` means that the data will never be received, but a return
value of `Ok` does not mean that the data will be received. It is
possible for the corresponding receiver to hang up immediately after
this function returns `Ok`.

# Errors

If the receive half of the channel is closed, either due to [`close`]
being called or the [`Receiver`] handle dropping, the function returns
an error. The error includes the value passed to `send`.

[`close`]: Receiver::close
[`Receiver`]: Receiver

# Cancel safety

If `send` is used as a branch in [`tokio::select!`](crate::select) and
another branch completes first, then it is guaranteed
that the message was not sent. **However, in that case, the message
is dropped and will be lost.**

To avoid losing messages, use [`reserve`](Self::reserve) to reserve
capacity, then use the returned [`Permit`] to send the message.

This channel uses a queue to ensure that calls to `send` and `reserve`
complete in the order they were requested.  Cancelling a call to
`send` makes you lose your place in the queue.

# Examples

In the following example, each call to `send` will block until the
previously sent value was received.

```rust
use tokio::sync::mpsc;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, mut rx) = mpsc::channel(1);

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
fn send(self: &Self, value: T) -> Result<(), SendError<T>>
```



## tokio::sync::mpsc::bounded::Sender::send_timeout

*Function*

Sends a value, waiting until there is capacity, but only for a limited time.

Shares the same success and error conditions as [`send`], adding one more
condition for an unsuccessful send, which is when the provided timeout has
elapsed, and there is no capacity available.

[`send`]: Sender::send

# Errors

If the receive half of the channel is closed, either due to [`close`]
being called or the [`Receiver`] having been dropped,
the function returns an error. The error includes the value passed to `send`.

[`close`]: Receiver::close
[`Receiver`]: Receiver

# Panics

This function panics if it is called outside the context of a Tokio
runtime [with time enabled](crate::runtime::Builder::enable_time).

# Examples

In the following example, each call to `send_timeout` will block until the
previously sent value was received, unless the timeout has elapsed.

```rust
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, mut rx) = mpsc::channel(1);

tokio::spawn(async move {
    for i in 0..10 {
        if let Err(e) = tx.send_timeout(i, Duration::from_millis(100)).await {
            println!("send error: #{:?}", e);
            return;
        }
    }
});

while let Some(i) = rx.recv().await {
    println!("got = {}", i);
    sleep(Duration::from_millis(200)).await;
}
# }
```

```rust
fn send_timeout(self: &Self, value: T, timeout: Duration) -> Result<(), SendTimeoutError<T>>
```



## tokio::sync::mpsc::bounded::Sender::try_reserve

*Function*

Tries to acquire a slot in the channel without waiting for the slot to become
available.

If the channel is full this function will return [`TrySendError`], otherwise
if there is a slot available it will return a [`Permit`] that will then allow you
to [`send`] on the channel with a guaranteed slot. This function is similar to
[`reserve`] except it does not await for the slot to become available.

Dropping [`Permit`] without sending a message releases the capacity back
to the channel.

[`Permit`]: Permit
[`send`]: Permit::send
[`reserve`]: Sender::reserve

# Examples

```
use tokio::sync::mpsc;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, mut rx) = mpsc::channel(1);

// Reserve capacity
let permit = tx.try_reserve().unwrap();

// Trying to send directly on the `tx` will fail due to no
// available capacity.
assert!(tx.try_send(123).is_err());

// Trying to reserve an additional slot on the `tx` will
// fail because there is no capacity.
assert!(tx.try_reserve().is_err());

// Sending on the permit succeeds
permit.send(456);

// The value sent on the permit is received
assert_eq!(rx.recv().await.unwrap(), 456);

# }
```

```rust
fn try_reserve(self: &Self) -> Result<Permit<T>, TrySendError<()>>
```



## tokio::sync::mpsc::bounded::Sender::try_reserve_many

*Function*

Tries to acquire `n` slots in the channel without waiting for the slot to become
available.

A [`PermitIterator`] is returned to track the reserved capacity.
You can call this [`Iterator`] until it is exhausted to
get a [`Permit`] and then call [`Permit::send`]. This function is similar to
[`reserve_many`] except it does not await for the slots to become available.

If there are fewer than `n` permits available on the channel, then
this function will return a [`TrySendError::Full`]. If the channel is closed
this function will return a [`TrySendError::Closed`].

Dropping [`PermitIterator`] without consuming it entirely releases the remaining
permits back to the channel.

[`PermitIterator`]: PermitIterator
[`send`]: Permit::send
[`reserve_many`]: Sender::reserve_many

# Examples

```
use tokio::sync::mpsc;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, mut rx) = mpsc::channel(2);

// Reserve capacity
let mut permit = tx.try_reserve_many(2).unwrap();

// Trying to send directly on the `tx` will fail due to no
// available capacity.
assert!(tx.try_send(123).is_err());

// Trying to reserve an additional slot on the `tx` will
// fail because there is no capacity.
assert!(tx.try_reserve().is_err());

// Sending with the permit iterator succeeds
permit.next().unwrap().send(456);
permit.next().unwrap().send(457);

// The iterator should now be exhausted
assert!(permit.next().is_none());

// The value sent on the permit is received
assert_eq!(rx.recv().await.unwrap(), 456);
assert_eq!(rx.recv().await.unwrap(), 457);

// Trying to call try_reserve_many with 0 will return an empty iterator
let mut permit = tx.try_reserve_many(0).unwrap();
assert!(permit.next().is_none());

// Trying to call try_reserve_many with a number greater than the channel
// capacity will return an error
let permit = tx.try_reserve_many(3);
assert!(permit.is_err());

// Trying to call try_reserve_many on a closed channel will return an error
drop(rx);
let permit = tx.try_reserve_many(1);
assert!(permit.is_err());

let permit = tx.try_reserve_many(0);
assert!(permit.is_err());
# }
```

```rust
fn try_reserve_many(self: &Self, n: usize) -> Result<PermitIterator<T>, TrySendError<()>>
```



## tokio::sync::mpsc::bounded::Sender::try_reserve_owned

*Function*

Tries to acquire a slot in the channel without waiting for the slot to become
available, returning an owned permit.

This moves the sender _by value_, and returns an owned permit that can
be used to send a message into the channel. Unlike [`Sender::try_reserve`],
this method may be used in cases where the permit must be valid for the
`'static` lifetime.  `Sender`s may be cloned cheaply (`Sender::clone` is
essentially a reference count increment, comparable to [`Arc::clone`]),
so when multiple [`OwnedPermit`]s are needed or the `Sender` cannot be
moved, it can be cloned prior to calling `try_reserve_owned`.

If the channel is full this function will return a [`TrySendError`].
Since the sender is taken by value, the `TrySendError` returned in this
case contains the sender, so that it may be used again. Otherwise, if
there is a slot available, this method will return an [`OwnedPermit`]
that can then be used to [`send`] on the channel with a guaranteed slot.
This function is similar to  [`reserve_owned`] except it does not await
for the slot to become available.

Dropping the [`OwnedPermit`] without sending a message releases the capacity back
to the channel.

[`OwnedPermit`]: OwnedPermit
[`send`]: OwnedPermit::send
[`reserve_owned`]: Sender::reserve_owned
[`Arc::clone`]: std::sync::Arc::clone

# Examples

```
use tokio::sync::mpsc;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, mut rx) = mpsc::channel(1);

// Reserve capacity
let permit = tx.clone().try_reserve_owned().unwrap();

// Trying to send directly on the `tx` will fail due to no
// available capacity.
assert!(tx.try_send(123).is_err());

// Trying to reserve an additional slot on the `tx` will
// fail because there is no capacity.
assert!(tx.try_reserve().is_err());

// Sending on the permit succeeds
permit.send(456);

// The value sent on the permit is received
assert_eq!(rx.recv().await.unwrap(), 456);

# }
```

```rust
fn try_reserve_owned(self: Self) -> Result<OwnedPermit<T>, TrySendError<Self>>
```



## tokio::sync::mpsc::bounded::Sender::try_send

*Function*

Attempts to immediately send a message on this `Sender`.

This method differs from [`send`] by returning immediately if the channel's
buffer is full or no receiver is waiting to acquire some data. Compared
with [`send`], this function has two failure cases instead of one (one for
disconnection, one for a full buffer).

# Errors

If the channel capacity has been reached, i.e., the channel has `n`
buffered values where `n` is the argument passed to [`channel`], then an
error is returned.

If the receive half of the channel is closed, either due to [`close`]
being called or the [`Receiver`] handle dropping, the function returns
an error. The error includes the value passed to `send`.

[`send`]: Sender::send
[`channel`]: channel
[`close`]: Receiver::close

# Examples

```
use tokio::sync::mpsc;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
// Create a channel with buffer size 1
let (tx1, mut rx) = mpsc::channel(1);
let tx2 = tx1.clone();

tokio::spawn(async move {
    tx1.send(1).await.unwrap();
    tx1.send(2).await.unwrap();
    // task waits until the receiver receives a value.
});

tokio::spawn(async move {
    // This will return an error and send
    // no message if the buffer is full
    let _ = tx2.try_send(3);
});

let mut msg;
msg = rx.recv().await.unwrap();
println!("message {} received", msg);

msg = rx.recv().await.unwrap();
println!("message {} received", msg);

// Third message may have never been sent
match rx.recv().await {
    Some(msg) => println!("message {} received", msg),
    None => println!("the third message was never sent"),
}
# }
```

```rust
fn try_send(self: &Self, message: T) -> Result<(), TrySendError<T>>
```



