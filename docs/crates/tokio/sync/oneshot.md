**tokio > sync > oneshot**

# Module: sync::oneshot

## Contents

**Modules**

- [`error`](#error) - `Oneshot` error types.

**Structs**

- [`Receiver`](#receiver) - Receives a value from the associated [`Sender`].
- [`Sender`](#sender) - Sends a value to the associated [`Receiver`].

**Functions**

- [`channel`](#channel) - Creates a new one-shot channel for sending single values across asynchronous

---

## tokio::sync::oneshot::Receiver

*Struct*

Receives a value from the associated [`Sender`].

A pair of both a [`Sender`] and a [`Receiver`]  are created by the
[`channel`](fn@channel) function.

This channel has no `recv` method because the receiver itself implements the
[`Future`] trait. To receive a `Result<T, `[`error::RecvError`]`>`, `.await` the `Receiver` object directly.

The `poll` method on the `Future` trait is allowed to spuriously return
`Poll::Pending` even if the message has been sent. If such a spurious
failure happens, then the caller will be woken when the spurious failure has
been resolved so that the caller can attempt to receive the message again.
Note that receiving such a wakeup does not guarantee that the next call will
succeed — it could fail with another spurious failure. (A spurious failure
does not mean that the message is lost. It is just delayed.)

[`Future`]: trait@std::future::Future

# Cancel safety

Awaiting a `&mut Receiver<T>` is cancel safe. If it is used as a branch in
[`tokio::select!`](crate::select) and another branch completes first, it is
guaranteed that no message was received on this
channel.

# Examples

```
use tokio::sync::oneshot;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, rx) = oneshot::channel();

tokio::spawn(async move {
    if let Err(_) = tx.send(3) {
        println!("the receiver dropped");
    }
});

match rx.await {
    Ok(v) => println!("got = {:?}", v),
    Err(_) => println!("the sender dropped"),
}
# }
```

If the sender is dropped without sending, the receiver will fail with
[`error::RecvError`]:

```
use tokio::sync::oneshot;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, rx) = oneshot::channel::<u32>();

tokio::spawn(async move {
    drop(tx);
});

match rx.await {
    Ok(_) => panic!("This doesn't happen"),
    Err(_) => println!("the sender dropped"),
}
# }
```

To use a `Receiver` in a `tokio::select!` loop, add `&mut` in front of the
channel.

```
use tokio::sync::oneshot;
use tokio::time::{interval, sleep, Duration};

# #[tokio::main(flavor = "current_thread")]
# async fn _doc() {}
# #[tokio::main(flavor = "current_thread", start_paused = true)]
# async fn main() {
let (send, mut recv) = oneshot::channel();
let mut interval = interval(Duration::from_millis(100));

# let handle =
tokio::spawn(async move {
    sleep(Duration::from_secs(1)).await;
    send.send("shut down").unwrap();
});

loop {
    tokio::select! {
        _ = interval.tick() => println!("Another 100ms"),
        msg = &mut recv => {
            println!("Got message: {}", msg.unwrap());
            break;
        }
    }
}
# handle.await.unwrap();
# }
```

**Generic Parameters:**
- T

**Methods:**

- `fn close(self: & mut Self)` - Prevents the associated [`Sender`] handle from sending a value.
- `fn is_terminated(self: &Self) -> bool` - Checks if this receiver is terminated.
- `fn is_empty(self: &Self) -> bool` - Checks if a channel is empty.
- `fn try_recv(self: & mut Self) -> Result<T, TryRecvError>` - Attempts to receive a value.
- `fn blocking_recv(self: Self) -> Result<T, RecvError>` - Blocking receive to call outside of asynchronous contexts.

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## tokio::sync::oneshot::Sender

*Struct*

Sends a value to the associated [`Receiver`].

A pair of both a [`Sender`] and a [`Receiver`]  are created by the
[`channel`](fn@channel) function.

# Examples

```
use tokio::sync::oneshot;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, rx) = oneshot::channel();

tokio::spawn(async move {
    if let Err(_) = tx.send(3) {
        println!("the receiver dropped");
    }
});

match rx.await {
    Ok(v) => println!("got = {:?}", v),
    Err(_) => println!("the sender dropped"),
}
# }
```

If the sender is dropped without sending, the receiver will fail with
[`error::RecvError`]:

```
use tokio::sync::oneshot;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, rx) = oneshot::channel::<u32>();

tokio::spawn(async move {
    drop(tx);
});

match rx.await {
    Ok(_) => panic!("This doesn't happen"),
    Err(_) => println!("the sender dropped"),
}
# }
```

To use a `Sender` from a destructor, put it in an [`Option`] and call
[`Option::take`].

```
use tokio::sync::oneshot;

struct SendOnDrop {
    sender: Option<oneshot::Sender<&'static str>>,
}
impl Drop for SendOnDrop {
    fn drop(&mut self) {
        if let Some(sender) = self.sender.take() {
            // Using `let _ =` to ignore send errors.
            let _ = sender.send("I got dropped!");
        }
    }
}

# #[tokio::main(flavor = "current_thread")]
# async fn _doc() {}
# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (send, recv) = oneshot::channel();

let send_on_drop = SendOnDrop { sender: Some(send) };
drop(send_on_drop);

assert_eq!(recv.await, Ok("I got dropped!"));
# }
```

[`Option`]: std::option::Option
[`Option::take`]: std::option::Option::take

**Generic Parameters:**
- T

**Methods:**

- `fn send(self: Self, t: T) -> Result<(), T>` - Attempts to send a value on this channel, returning it back if it could
- `fn closed(self: & mut Self)` - Waits for the associated [`Receiver`] handle to close.
- `fn is_closed(self: &Self) -> bool` - Returns `true` if the associated [`Receiver`] handle has been dropped.
- `fn poll_closed(self: & mut Self, cx: & mut Context) -> Poll<()>` - Checks whether the `oneshot` channel has been closed, and if not, schedules the

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Drop**
  - `fn drop(self: & mut Self)`



## tokio::sync::oneshot::channel

*Function*

Creates a new one-shot channel for sending single values across asynchronous
tasks.

The function returns separate "send" and "receive" handles. The `Sender`
handle is used by the producer to send the value. The `Receiver` handle is
used by the consumer to receive the value.

Each handle can be used on separate tasks.

# Examples

```
use tokio::sync::oneshot;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, rx) = oneshot::channel();

tokio::spawn(async move {
    if let Err(_) = tx.send(3) {
        println!("the receiver dropped");
    }
});

match rx.await {
    Ok(v) => println!("got = {:?}", v),
    Err(_) => println!("the sender dropped"),
}
# }
```

```rust
fn channel<T>() -> (Sender<T>, Receiver<T>)
```



## Module: error

`Oneshot` error types.



