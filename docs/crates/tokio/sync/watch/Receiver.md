**tokio > sync > watch > Receiver**

# Module: sync::watch::Receiver

## Contents

**Functions**

- [`borrow`](#borrow) - Returns a reference to the most recently sent value.
- [`borrow_and_update`](#borrow_and_update) - Returns a reference to the most recently sent value and marks that value
- [`changed`](#changed) - Waits for a change notification, then marks the current value as seen.
- [`has_changed`](#has_changed) - Checks if this channel contains a message that this receiver has not yet

---

## tokio::sync::watch::Receiver::borrow

*Function*

Returns a reference to the most recently sent value.

This method does not mark the returned value as seen, so future calls to
[`changed`] may return immediately even if you have already seen the
value with a call to `borrow`.

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

For more information on when to use this method versus
[`borrow_and_update`], see [here](self#borrow_and_update-versus-borrow).

[`changed`]: Receiver::changed
[`borrow_and_update`]: Receiver::borrow_and_update

# Examples

```
use tokio::sync::watch;

let (_, rx) = watch::channel("hello");
assert_eq!(*rx.borrow(), "hello");
```

```rust
fn borrow(self: &Self) -> Ref<T>
```



## tokio::sync::watch::Receiver::borrow_and_update

*Function*

Returns a reference to the most recently sent value and marks that value
as seen.

This method marks the current value as seen. Subsequent calls to [`changed`]
will not return immediately until the [`Sender`] has modified the shared
value again.

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
// Task 1 (on thread A)                |  // Task 2 (on thread B)
let _ref1 = rx1.borrow_and_update();   |
                                       |  // will block
                                       |  let _ = tx.send(());
// may deadlock                        |
let _ref2 = rx2.borrow_and_update();   |
```
</details>

For more information on when to use this method versus [`borrow`], see
[here](self#borrow_and_update-versus-borrow).

[`changed`]: Receiver::changed
[`borrow`]: Receiver::borrow

```rust
fn borrow_and_update(self: & mut Self) -> Ref<T>
```



## tokio::sync::watch::Receiver::changed

*Function*

Waits for a change notification, then marks the current value as seen.

If the current value in the channel has not yet been marked seen when
this method is called, the method marks that value seen and returns
immediately. If the newest value has already been marked seen, then the
method sleeps until a new message is sent by a [`Sender`] connected to
this `Receiver`, or until all [`Sender`]s are dropped.

For more information, see
[*Change notifications*](self#change-notifications) in the module-level documentation.

# Errors

Returns a [`RecvError`](error::RecvError) if the channel has been closed __AND__
the current value is seen.

# Cancel safety

This method is cancel safe. If you use it as a branch in
[`tokio::select!`](crate::select) and another branch
completes first, then it is guaranteed that no values have been marked
seen by this call to `changed`.

[`Sender`]: struct@Sender

# Examples

```
use tokio::sync::watch;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, mut rx) = watch::channel("hello");

tokio::spawn(async move {
    tx.send("goodbye").unwrap();
});

assert!(rx.changed().await.is_ok());
assert_eq!(*rx.borrow_and_update(), "goodbye");

// The `tx` handle has been dropped
assert!(rx.changed().await.is_err());
# }
```

```rust
fn changed(self: & mut Self) -> Result<(), error::RecvError>
```



## tokio::sync::watch::Receiver::has_changed

*Function*

Checks if this channel contains a message that this receiver has not yet
seen. The current value will not be marked as seen.

Although this method is called `has_changed`, it does not check
messages for equality, so this call will return true even if the current
message is equal to the previous message.

# Errors

Returns a [`RecvError`](error::RecvError) if and only if the channel has been closed.

# Examples

## Basic usage

```
use tokio::sync::watch;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, mut rx) = watch::channel("hello");

tx.send("goodbye").unwrap();

assert!(rx.has_changed().unwrap());
assert_eq!(*rx.borrow_and_update(), "goodbye");

// The value has been marked as seen
assert!(!rx.has_changed().unwrap());
# }
```

## Closed channel example

```
use tokio::sync::watch;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, rx) = watch::channel("hello");
tx.send("goodbye").unwrap();

drop(tx);

// The channel is closed
assert!(rx.has_changed().is_err());
# }
```

```rust
fn has_changed(self: &Self) -> Result<bool, error::RecvError>
```



