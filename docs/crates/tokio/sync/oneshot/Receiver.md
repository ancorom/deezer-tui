**tokio > sync > oneshot > Receiver**

# Module: sync::oneshot::Receiver

## Contents

**Functions**

- [`close`](#close) - Prevents the associated [`Sender`] handle from sending a value.
- [`is_terminated`](#is_terminated) - Checks if this receiver is terminated.
- [`try_recv`](#try_recv) - Attempts to receive a value.

---

## tokio::sync::oneshot::Receiver::close

*Function*

Prevents the associated [`Sender`] handle from sending a value.

Any `send` operation which happens after calling `close` is guaranteed
to fail. After calling `close`, [`try_recv`] should be called to
receive a value if one was sent **before** the call to `close`
completed.

This function is useful to perform a graceful shutdown and ensure that a
value will not be sent into the channel and never received.

`close` is no-op if a message is already received or the channel
is already closed.

[`Sender`]: Sender
[`try_recv`]: Receiver::try_recv

# Examples

Prevent a value from being sent

```
use tokio::sync::oneshot;
use tokio::sync::oneshot::error::TryRecvError;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, mut rx) = oneshot::channel();

assert!(!tx.is_closed());

rx.close();

assert!(tx.is_closed());
assert!(tx.send("never received").is_err());

match rx.try_recv() {
    Err(TryRecvError::Closed) => {}
    _ => unreachable!(),
}
# }
```

Receive a value sent **before** calling `close`

```
use tokio::sync::oneshot;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, mut rx) = oneshot::channel();

assert!(tx.send("will receive").is_ok());

rx.close();

let msg = rx.try_recv().unwrap();
assert_eq!(msg, "will receive");
# }
```

```rust
fn close(self: & mut Self)
```



## tokio::sync::oneshot::Receiver::is_terminated

*Function*

Checks if this receiver is terminated.

This function returns true if this receiver has already yielded a [`Poll::Ready`] result.
If so, this receiver should no longer be polled.

# Examples

Sending a value and polling it.

```
use tokio::sync::oneshot;

use std::task::Poll;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, mut rx) = oneshot::channel();

// A receiver is not terminated when it is initialized.
assert!(!rx.is_terminated());

// A receiver is not terminated it is polled and is still pending.
let poll = futures::poll!(&mut rx);
assert_eq!(poll, Poll::Pending);
assert!(!rx.is_terminated());

// A receiver is not terminated if a value has been sent, but not yet read.
tx.send(0).unwrap();
assert!(!rx.is_terminated());

// A receiver *is* terminated after it has been polled and yielded a value.
assert_eq!((&mut rx).await, Ok(0));
assert!(rx.is_terminated());
# }
```

Dropping the sender.

```
use tokio::sync::oneshot;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, mut rx) = oneshot::channel::<()>();

// A receiver is not immediately terminated when the sender is dropped.
drop(tx);
assert!(!rx.is_terminated());

// A receiver *is* terminated after it has been polled and yielded an error.
let _ = (&mut rx).await.unwrap_err();
assert!(rx.is_terminated());
# }
```

```rust
fn is_terminated(self: &Self) -> bool
```



## tokio::sync::oneshot::Receiver::try_recv

*Function*

Attempts to receive a value.

If a pending value exists in the channel, it is returned. If no value
has been sent, the current task **will not** be registered for
future notification.

This function is useful to call from outside the context of an
asynchronous task.

Note that unlike the `poll` method, the `try_recv` method cannot fail
spuriously. Any send or close event that happens before this call to
`try_recv` will be correctly returned to the caller.

# Return

- `Ok(T)` if a value is pending in the channel.
- `Err(TryRecvError::Empty)` if no value has been sent yet.
- `Err(TryRecvError::Closed)` if the sender has dropped without sending
  a value, or if the message has already been received.

# Examples

`try_recv` before a value is sent, then after.

```
use tokio::sync::oneshot;
use tokio::sync::oneshot::error::TryRecvError;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, mut rx) = oneshot::channel();

match rx.try_recv() {
    // The channel is currently empty
    Err(TryRecvError::Empty) => {}
    _ => unreachable!(),
}

// Send a value
tx.send("hello").unwrap();

match rx.try_recv() {
     Ok(value) => assert_eq!(value, "hello"),
     _ => unreachable!(),
}
# }
```

`try_recv` when the sender dropped before sending a value

```
use tokio::sync::oneshot;
use tokio::sync::oneshot::error::TryRecvError;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, mut rx) = oneshot::channel::<()>();

drop(tx);

match rx.try_recv() {
    // The channel will never receive a value.
    Err(TryRecvError::Closed) => {}
    _ => unreachable!(),
}
# }
```

```rust
fn try_recv(self: & mut Self) -> Result<T, TryRecvError>
```



