**tokio > sync > watch > Sender**

# Module: sync::watch::Sender

## Contents

**Functions**

- [`closed`](#closed) - Completes when all receivers have dropped.
- [`is_closed`](#is_closed) - Checks if the channel has been closed. This happens when all receivers
- [`send_if_modified`](#send_if_modified) - Modifies the watched value **conditionally** in-place,
- [`send_modify`](#send_modify) - Modifies the watched value **unconditionally** in-place,
- [`send_replace`](#send_replace) - Sends a new value via the channel, notifying all receivers and returning
- [`subscribe`](#subscribe) - Creates a new [`Receiver`] connected to this `Sender`.

---

## tokio::sync::watch::Sender::closed

*Function*

Completes when all receivers have dropped.

This allows the producer to get notified when interest in the produced
values is canceled and immediately stop doing work. Once a channel is
closed, the only way to reopen it is to call [`Sender::subscribe`] to
get a new receiver.

If the channel becomes closed for a brief amount of time (e.g., the last
receiver is dropped and then `subscribe` is called), then this call to
`closed` might return, but it is also possible that it does not "notice"
that the channel was closed for a brief amount of time.

# Cancel safety

This method is cancel safe.

# Examples

```
use tokio::sync::watch;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, rx) = watch::channel("hello");

tokio::spawn(async move {
    // use `rx`
    drop(rx);
});

// Waits for `rx` to drop
tx.closed().await;
println!("the `rx` handles dropped")
# }
```

```rust
fn closed(self: &Self)
```



## tokio::sync::watch::Sender::is_closed

*Function*

Checks if the channel has been closed. This happens when all receivers
have dropped.

# Examples

```
let (tx, rx) = tokio::sync::watch::channel(());
assert!(!tx.is_closed());

drop(rx);
assert!(tx.is_closed());
```

```rust
fn is_closed(self: &Self) -> bool
```



## tokio::sync::watch::Sender::send_if_modified

*Function*

Modifies the watched value **conditionally** in-place,
notifying all receivers only if modified.

This can be useful for modifying the watched value, without
having to allocate a new instance. Additionally, this
method permits sending values even when there are no receivers.

The `modify` closure must return `true` if the value has actually
been modified during the mutable borrow. It should only return `false`
if the value is guaranteed to be unmodified despite the mutable
borrow.

Receivers are only notified if the closure returned `true`. If the
closure has modified the value but returned `false` this results
in a *silent modification*, i.e. the modified value will be visible
in subsequent calls to `borrow`, but receivers will not receive
a change notification.

Returns the result of the closure, i.e. `true` if the value has
been modified and `false` otherwise.

# Panics

This function panics when the invocation of the `modify` closure panics.
No receivers are notified when panicking. All changes of the watched
value applied by the closure before panicking will be visible in
subsequent calls to `borrow`.

# Examples

```
use tokio::sync::watch;

struct State {
    counter: usize,
}
let (state_tx, mut state_rx) = watch::channel(State { counter: 1 });
let inc_counter_if_odd = |state: &mut State| {
    if state.counter % 2 == 1 {
        state.counter += 1;
        return true;
    }
    false
};

assert_eq!(state_rx.borrow().counter, 1);

assert!(!state_rx.has_changed().unwrap());
assert!(state_tx.send_if_modified(inc_counter_if_odd));
assert!(state_rx.has_changed().unwrap());
assert_eq!(state_rx.borrow_and_update().counter, 2);

assert!(!state_rx.has_changed().unwrap());
assert!(!state_tx.send_if_modified(inc_counter_if_odd));
assert!(!state_rx.has_changed().unwrap());
assert_eq!(state_rx.borrow_and_update().counter, 2);
```

```rust
fn send_if_modified<F>(self: &Self, modify: F) -> bool
```



## tokio::sync::watch::Sender::send_modify

*Function*

Modifies the watched value **unconditionally** in-place,
notifying all receivers.

This can be useful for modifying the watched value, without
having to allocate a new instance. Additionally, this
method permits sending values even when there are no receivers.

Prefer to use the more versatile function [`Self::send_if_modified()`]
if the value is only modified conditionally during the mutable borrow
to prevent unneeded change notifications for unmodified values.

# Panics

This function panics when the invocation of the `modify` closure panics.
No receivers are notified when panicking. All changes of the watched
value applied by the closure before panicking will be visible in
subsequent calls to `borrow`.

# Examples

```
use tokio::sync::watch;

struct State {
    counter: usize,
}
let (state_tx, state_rx) = watch::channel(State { counter: 0 });
state_tx.send_modify(|state| state.counter += 1);
assert_eq!(state_rx.borrow().counter, 1);
```

```rust
fn send_modify<F>(self: &Self, modify: F)
```



## tokio::sync::watch::Sender::send_replace

*Function*

Sends a new value via the channel, notifying all receivers and returning
the previous value in the channel.

This can be useful for reusing the buffers inside a watched value.
Additionally, this method permits sending values even when there are no
receivers.

# Examples

```
use tokio::sync::watch;

let (tx, _rx) = watch::channel(1);
assert_eq!(tx.send_replace(2), 1);
assert_eq!(tx.send_replace(3), 2);
```

```rust
fn send_replace(self: &Self, value: T) -> T
```



## tokio::sync::watch::Sender::subscribe

*Function*

Creates a new [`Receiver`] connected to this `Sender`.

All messages sent before this call to `subscribe` are initially marked
as seen by the new `Receiver`.

This method can be called even if there are no other receivers. In this
case, the channel is reopened.

# Examples

The new channel will receive messages sent on this `Sender`.

```
use tokio::sync::watch;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, _rx) = watch::channel(0u64);

tx.send(5).unwrap();

let rx = tx.subscribe();
assert_eq!(5, *rx.borrow());

tx.send(10).unwrap();
assert_eq!(10, *rx.borrow());
# }
```

The most recent message is considered seen by the channel, so this test
is guaranteed to pass.

```
use tokio::sync::watch;
use tokio::time::Duration;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, _rx) = watch::channel(0u64);
tx.send(5).unwrap();
let mut rx = tx.subscribe();

tokio::spawn(async move {
    // by spawning and sleeping, the message is sent after `main`
    // hits the call to `changed`.
    # if false {
    tokio::time::sleep(Duration::from_millis(10)).await;
    # }
    tx.send(100).unwrap();
});

rx.changed().await.unwrap();
assert_eq!(100, *rx.borrow());
# }
```

```rust
fn subscribe(self: &Self) -> Receiver<T>
```



