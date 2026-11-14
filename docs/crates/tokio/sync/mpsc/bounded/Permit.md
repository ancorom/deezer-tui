**tokio > sync > mpsc > bounded > Permit**

# Module: sync::mpsc::bounded::Permit

## Contents

**Functions**

- [`send`](#send) - Sends a value using the reserved capacity.

---

## tokio::sync::mpsc::bounded::Permit::send

*Function*

Sends a value using the reserved capacity.

Capacity for the message has already been reserved. The message is sent
to the receiver and the permit is consumed. The operation will succeed
even if the receiver half has been closed. See [`Receiver::close`] for
more details on performing a clean shutdown.

[`Receiver::close`]: Receiver::close

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

// Send a message on the permit
permit.send(456);

// The value sent on the permit is received
assert_eq!(rx.recv().await.unwrap(), 456);
# }
```

```rust
fn send(self: Self, value: T)
```



