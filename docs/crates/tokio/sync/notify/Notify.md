**tokio > sync > notify > Notify**

# Module: sync::notify::Notify

## Contents

**Functions**

- [`new`](#new) - Create a new `Notify`, initialized without a permit.
- [`notified`](#notified) - Wait for a notification.
- [`notified_owned`](#notified_owned) - Wait for a notification with an owned `Future`.
- [`notify_one`](#notify_one) - Notifies the first waiting task.
- [`notify_waiters`](#notify_waiters) - Notifies all waiting tasks.

---

## tokio::sync::notify::Notify::new

*Function*

Create a new `Notify`, initialized without a permit.

# Examples

```
use tokio::sync::Notify;

let notify = Notify::new();
```

```rust
fn new() -> Notify
```



## tokio::sync::notify::Notify::notified

*Function*

Wait for a notification.

Equivalent to:

```ignore
async fn notified(&self);
```

Each `Notify` value holds a single permit. If a permit is available from
an earlier call to [`notify_one()`], then `notified().await` will complete
immediately, consuming that permit. Otherwise, `notified().await` waits
for a permit to be made available by the next call to `notify_one()`.

The `Notified` future is not guaranteed to receive wakeups from calls to
`notify_one()` if it has not yet been polled. See the documentation for
[`Notified::enable()`] for more details.

The `Notified` future is guaranteed to receive wakeups from
`notify_waiters()` as soon as it has been created, even if it has not
yet been polled.

[`notify_one()`]: Notify::notify_one
[`Notified::enable()`]: Notified::enable

# Cancel safety

This method uses a queue to fairly distribute notifications in the order
they were requested. Cancelling a call to `notified` makes you lose your
place in the queue.

# Examples

```
use tokio::sync::Notify;
use std::sync::Arc;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let notify = Arc::new(Notify::new());
let notify2 = notify.clone();

tokio::spawn(async move {
    notify2.notified().await;
    println!("received notification");
});

println!("sending notification");
notify.notify_one();
# }
```

```rust
fn notified(self: &Self) -> Notified
```



## tokio::sync::notify::Notify::notified_owned

*Function*

Wait for a notification with an owned `Future`.

Unlike [`Self::notified`] which returns a future tied to the `Notify`'s
lifetime, `notified_owned` creates a self-contained future that owns its
notification state, making it safe to move between threads.

See [`Self::notified`] for more details.

# Cancel safety

This method uses a queue to fairly distribute notifications in the order
they were requested. Cancelling a call to `notified_owned` makes you lose your
place in the queue.

# Examples

```
use std::sync::Arc;
use tokio::sync::Notify;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let notify = Arc::new(Notify::new());

for _ in 0..10 {
    let notified = notify.clone().notified_owned();
    tokio::spawn(async move {
        notified.await;
        println!("received notification");
    });
}

println!("sending notification");
notify.notify_waiters();
# }
```

```rust
fn notified_owned(self: Arc<Self>) -> OwnedNotified
```



## tokio::sync::notify::Notify::notify_one

*Function*

Notifies the first waiting task.

If a task is currently waiting, that task is notified. Otherwise, a
permit is stored in this `Notify` value and the **next** call to
[`notified().await`] will complete immediately consuming the permit made
available by this call to `notify_one()`.

At most one permit may be stored by `Notify`. Many sequential calls to
`notify_one` will result in a single permit being stored. The next call to
`notified().await` will complete immediately, but the one after that
will wait.

[`notified().await`]: Notify::notified()

# Examples

```
use tokio::sync::Notify;
use std::sync::Arc;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let notify = Arc::new(Notify::new());
let notify2 = notify.clone();

tokio::spawn(async move {
    notify2.notified().await;
    println!("received notification");
});

println!("sending notification");
notify.notify_one();
# }
```

```rust
fn notify_one(self: &Self)
```



## tokio::sync::notify::Notify::notify_waiters

*Function*

Notifies all waiting tasks.

If a task is currently waiting, that task is notified. Unlike with
`notify_one()`, no permit is stored to be used by the next call to
`notified().await`. The purpose of this method is to notify all
already registered waiters. Registering for notification is done by
acquiring an instance of the `Notified` future via calling `notified()`.

# Examples

```
use tokio::sync::Notify;
use std::sync::Arc;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let notify = Arc::new(Notify::new());
let notify2 = notify.clone();

let notified1 = notify.notified();
let notified2 = notify.notified();

let handle = tokio::spawn(async move {
    println!("sending notifications");
    notify2.notify_waiters();
});

notified1.await;
notified2.await;
println!("received notifications");
# }
```

```rust
fn notify_waiters(self: &Self)
```



