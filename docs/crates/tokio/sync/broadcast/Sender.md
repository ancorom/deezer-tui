**tokio > sync > broadcast > Sender**

# Module: sync::broadcast::Sender

## Contents

**Functions**

- [`send`](#send) - Attempts to send a value to all active [`Receiver`] handles, returning
- [`subscribe`](#subscribe) - Creates a new [`Receiver`] handle that will receive values sent **after**

---

## tokio::sync::broadcast::Sender::send

*Function*

Attempts to send a value to all active [`Receiver`] handles, returning
it back if it could not be sent.

A successful send occurs when there is at least one active [`Receiver`]
handle. An unsuccessful send would be one where all associated
[`Receiver`] handles have already been dropped.

# Return

On success, the number of subscribed [`Receiver`] handles is returned.
This does not mean that this number of receivers will see the message as
a receiver may drop or lag ([see lagging](self#lagging)) before receiving
the message.

# Note

A return value of `Ok` **does not** mean that the sent value will be
observed by all or any of the active [`Receiver`] handles. [`Receiver`]
handles may be dropped before receiving the sent message.

A return value of `Err` **does not** mean that future calls to `send`
will fail. New [`Receiver`] handles may be created by calling
[`subscribe`].

[`Receiver`]: crate::sync::broadcast::Receiver
[`subscribe`]: crate::sync::broadcast::Sender::subscribe

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

```rust
fn send(self: &Self, value: T) -> Result<usize, SendError<T>>
```



## tokio::sync::broadcast::Sender::subscribe

*Function*

Creates a new [`Receiver`] handle that will receive values sent **after**
this call to `subscribe`.

# Examples

```
use tokio::sync::broadcast;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, _rx) = broadcast::channel(16);

// Will not be seen
tx.send(10).unwrap();

let mut rx = tx.subscribe();

tx.send(20).unwrap();

let value = rx.recv().await.unwrap();
assert_eq!(20, value);
# }
```

```rust
fn subscribe(self: &Self) -> Receiver<T>
```



