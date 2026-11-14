**tokio > sync > mpsc > bounded > OwnedPermit**

# Module: sync::mpsc::bounded::OwnedPermit

## Contents

**Functions**

- [`send`](#send) - Sends a value using the reserved capacity.

---

## tokio::sync::mpsc::bounded::OwnedPermit::send

*Function*

Sends a value using the reserved capacity.

Capacity for the message has already been reserved. The message is sent
to the receiver and the permit is consumed. The operation will succeed
even if the receiver half has been closed. See [`Receiver::close`] for
more details on performing a clean shutdown.

Unlike [`Permit::send`], this method returns the [`Sender`] from which
the `OwnedPermit` was reserved.

[`Receiver::close`]: Receiver::close

# Examples

```
use tokio::sync::mpsc;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, mut rx) = mpsc::channel(1);

// Reserve capacity
let permit = tx.reserve_owned().await.unwrap();

// Send a message on the permit, returning the sender.
let tx = permit.send(456);

// The value sent on the permit is received
assert_eq!(rx.recv().await.unwrap(), 456);

// We may now reuse `tx` to send another message.
tx.send(789).await.unwrap();
# }
```

```rust
fn send(self: Self, value: T) -> Sender<T>
```



