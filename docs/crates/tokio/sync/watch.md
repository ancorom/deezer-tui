**tokio > sync > watch**

# Module: sync::watch

## Contents

**Modules**

- [`error`](#error) - Watch error types.

**Structs**

- [`Receiver`](#receiver) - Receives values from the associated [`Sender`](struct@Sender).
- [`Ref`](#ref) - Returns a reference to the inner value.
- [`Sender`](#sender) - Sends values to the associated [`Receiver`](struct@Receiver).

**Functions**

- [`channel`](#channel) - Creates a new watch channel, returning the "send" and "receive" handles.

---

## tokio::sync::watch::Receiver

*Struct*

Receives values from the associated [`Sender`](struct@Sender).

Instances are created by the [`channel`](fn@channel) function.

To turn this receiver into a `Stream`, you can use the [`WatchStream`]
wrapper.

[`WatchStream`]: https://docs.rs/tokio-stream/0.1/tokio_stream/wrappers/struct.WatchStream.html

**Generic Parameters:**
- T

**Methods:**

- `fn borrow(self: &Self) -> Ref<T>` - Returns a reference to the most recently sent value.
- `fn borrow_and_update(self: & mut Self) -> Ref<T>` - Returns a reference to the most recently sent value and marks that value
- `fn has_changed(self: &Self) -> Result<bool, error::RecvError>` - Checks if this channel contains a message that this receiver has not yet
- `fn is_closed(self: &Self) -> bool` - Checks if the channel has been closed. This happens when all [`Sender`]s
- `fn mark_changed(self: & mut Self)` - Marks the state as changed.
- `fn mark_unchanged(self: & mut Self)` - Marks the state as unchanged.
- `fn changed(self: & mut Self) -> Result<(), error::RecvError>` - Waits for a change notification, then marks the current value as seen.
- `fn wait_for<impl FnMut(&T) -> bool>(self: & mut Self, f: impl Trait) -> Result<Ref<T>, error::RecvError>` - Waits for a value that satisfies the provided condition.
- `fn same_channel(self: &Self, other: &Self) -> bool` - Returns `true` if receivers belong to the same channel.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Drop**
  - `fn drop(self: & mut Self)`



## tokio::sync::watch::Ref

*Struct*

Returns a reference to the inner value.

Outstanding borrows hold a read lock on the inner value. This means that
long-lived borrows could cause the producer half to block. It is recommended
to keep the borrow as short-lived as possible. Additionally, if you are
running in an environment that allows `!Send` futures, you must ensure that
the returned `Ref` type is never held alive across an `.await` point,
otherwise, it can lead to a deadlock.

The priority policy of the lock is dependent on the underlying lock
implementation, and this type does not guarantee that any particular policy
will be used. In particular, a producer which is waiting to acquire the lock
in `send` might or might not block concurrent calls to `borrow`, e.g.:

<details><summary>Potential deadlock example</summary>

```text
// Task 1 (on thread A)    |  // Task 2 (on thread B)
let _ref1 = rx.borrow();   |
                           |  // will block
                           |  let _ = tx.send(());
// may deadlock            |
let _ref2 = rx.borrow();   |
```
</details>

**Generic Parameters:**
- 'a
- T

**Methods:**

- `fn has_changed(self: &Self) -> bool` - Indicates if the borrowed value is considered as _changed_ since the last

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Deref**
  - `fn deref(self: &Self) -> &T`



## tokio::sync::watch::Sender

*Struct*

Sends values to the associated [`Receiver`](struct@Receiver).

Instances are created by the [`channel`](fn@channel) function.

**Generic Parameters:**
- T

**Methods:**

- `fn new(init: T) -> Self` - Creates the sending-half of the [`watch`] channel.
- `fn send(self: &Self, value: T) -> Result<(), error::SendError<T>>` - Sends a new value via the channel, notifying all receivers.
- `fn send_modify<F>(self: &Self, modify: F)` - Modifies the watched value **unconditionally** in-place,
- `fn send_if_modified<F>(self: &Self, modify: F) -> bool` - Modifies the watched value **conditionally** in-place,
- `fn send_replace(self: &Self, value: T) -> T` - Sends a new value via the channel, notifying all receivers and returning
- `fn borrow(self: &Self) -> Ref<T>` - Returns a reference to the most recently sent value
- `fn is_closed(self: &Self) -> bool` - Checks if the channel has been closed. This happens when all receivers
- `fn closed(self: &Self)` - Completes when all receivers have dropped.
- `fn subscribe(self: &Self) -> Receiver<T>` - Creates a new [`Receiver`] connected to this `Sender`.
- `fn receiver_count(self: &Self) -> usize` - Returns the number of receivers that currently exist.
- `fn sender_count(self: &Self) -> usize` - Returns the number of senders that currently exist.
- `fn same_channel(self: &Self, other: &Self) -> bool` - Returns `true` if senders belong to the same channel.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Drop**
  - `fn drop(self: & mut Self)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Default**
  - `fn default() -> Self`



## tokio::sync::watch::channel

*Function*

Creates a new watch channel, returning the "send" and "receive" handles.

All values sent by [`Sender`] will become visible to the [`Receiver`] handles.
Only the last value sent is made available to the [`Receiver`] half. All
intermediate values are dropped.

# Examples

The following example prints `hello! world! `.

```
use tokio::sync::watch;
use tokio::time::{Duration, sleep};

# async fn dox() -> Result<(), Box<dyn std::error::Error>> {
let (tx, mut rx) = watch::channel("hello");

tokio::spawn(async move {
    // Use the equivalent of a "do-while" loop so the initial value is
    // processed before awaiting the `changed()` future.
    loop {
        println!("{}! ", *rx.borrow_and_update());
        if rx.changed().await.is_err() {
            break;
        }
    }
});

sleep(Duration::from_millis(100)).await;
tx.send("world")?;
# Ok(())
# }
```

[`Sender`]: struct@Sender
[`Receiver`]: struct@Receiver

```rust
fn channel<T>(init: T) -> (Sender<T>, Receiver<T>)
```



## Module: error

Watch error types.



