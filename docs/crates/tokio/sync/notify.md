**tokio > sync > notify**

# Module: sync::notify

## Contents

**Structs**

- [`Notified`](#notified) - Future returned from [`Notify::notified()`].
- [`Notify`](#notify) - Notifies a single task to wake up.
- [`OwnedNotified`](#ownednotified) - Future returned from [`Notify::notified_owned()`].

---

## tokio::sync::notify::Notified

*Struct*

Future returned from [`Notify::notified()`].

This future is fused, so once it has completed, any future calls to poll
will immediately return `Poll::Ready`.

**Generic Parameters:**
- 'a

**Methods:**

- `fn enable(self: Pin<& mut Self>) -> bool` - Adds this future to the list of futures that are ready to receive

**Traits:** Sync, Send

**Trait Implementations:**

- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<()>`
- **Drop**
  - `fn drop(self: & mut Self)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## tokio::sync::notify::Notify

*Struct*

Notifies a single task to wake up.

`Notify` provides a basic mechanism to notify a single task of an event.
`Notify` itself does not carry any data. Instead, it is to be used to signal
another task to perform an operation.

A `Notify` can be thought of as a [`Semaphore`] starting with 0 permits. The
[`notified().await`] method waits for a permit to become available, and
[`notify_one()`] sets a permit **if there currently are no available
permits**.

The synchronization details of `Notify` are similar to
[`thread::park`][park] and [`Thread::unpark`][unpark] from std. A [`Notify`]
value contains a single permit. [`notified().await`] waits for the permit to
be made available, consumes the permit, and resumes.  [`notify_one()`] sets
the permit, waking a pending task if there is one.

If `notify_one()` is called **before** `notified().await`, then the next
call to `notified().await` will complete immediately, consuming the permit.
Any subsequent calls to `notified().await` will wait for a new permit.

If `notify_one()` is called **multiple** times before `notified().await`,
only a **single** permit is stored. The next call to `notified().await` will
complete immediately, but the one after will wait for a new permit.

# Examples

Basic usage.

```
use tokio::sync::Notify;
use std::sync::Arc;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let notify = Arc::new(Notify::new());
let notify2 = notify.clone();

let handle = tokio::spawn(async move {
    notify2.notified().await;
    println!("received notification");
});

println!("sending notification");
notify.notify_one();

// Wait for task to receive notification.
handle.await.unwrap();
# }
```

Unbound multi-producer single-consumer (mpsc) channel.

No wakeups can be lost when using this channel because the call to
`notify_one()` will store a permit in the `Notify`, which the following call
to `notified()` will consume.

```
use tokio::sync::Notify;

use std::collections::VecDeque;
use std::sync::Mutex;

struct Channel<T> {
    values: Mutex<VecDeque<T>>,
    notify: Notify,
}

impl<T> Channel<T> {
    pub fn send(&self, value: T) {
        self.values.lock().unwrap()
            .push_back(value);

        // Notify the consumer a value is available
        self.notify.notify_one();
    }

    // This is a single-consumer channel, so several concurrent calls to
    // `recv` are not allowed.
    pub async fn recv(&self) -> T {
        loop {
            // Drain values
            if let Some(value) = self.values.lock().unwrap().pop_front() {
                return value;
            }

            // Wait for values to be available
            self.notify.notified().await;
        }
    }
}
```

Unbound multi-producer multi-consumer (mpmc) channel.

The call to [`enable`] is important because otherwise if you have two
calls to `recv` and two calls to `send` in parallel, the following could
happen:

 1. Both calls to `try_recv` return `None`.
 2. Both new elements are added to the vector.
 3. The `notify_one` method is called twice, adding only a single
    permit to the `Notify`.
 4. Both calls to `recv` reach the `Notified` future. One of them
    consumes the permit, and the other sleeps forever.

By adding the `Notified` futures to the list by calling `enable` before
`try_recv`, the `notify_one` calls in step three would remove the
futures from the list and mark them notified instead of adding a permit
to the `Notify`. This ensures that both futures are woken.

Notice that this failure can only happen if there are two concurrent calls
to `recv`. This is why the mpsc example above does not require a call to
`enable`.

```
use tokio::sync::Notify;

use std::collections::VecDeque;
use std::sync::Mutex;

struct Channel<T> {
    messages: Mutex<VecDeque<T>>,
    notify_on_sent: Notify,
}

impl<T> Channel<T> {
    pub fn send(&self, msg: T) {
        let mut locked_queue = self.messages.lock().unwrap();
        locked_queue.push_back(msg);
        drop(locked_queue);

        // Send a notification to one of the calls currently
        // waiting in a call to `recv`.
        self.notify_on_sent.notify_one();
    }

    pub fn try_recv(&self) -> Option<T> {
        let mut locked_queue = self.messages.lock().unwrap();
        locked_queue.pop_front()
    }

    pub async fn recv(&self) -> T {
        let future = self.notify_on_sent.notified();
        tokio::pin!(future);

        loop {
            // Make sure that no wakeup is lost if we get
            // `None` from `try_recv`.
            future.as_mut().enable();

            if let Some(msg) = self.try_recv() {
                return msg;
            }

            // Wait for a call to `notify_one`.
            //
            // This uses `.as_mut()` to avoid consuming the future,
            // which lets us call `Pin::set` below.
            future.as_mut().await;

            // Reset the future in case another call to
            // `try_recv` got the message before us.
            future.set(self.notify_on_sent.notified());
        }
    }
}
```

[park]: std::thread::park
[unpark]: std::thread::Thread::unpark
[`notified().await`]: Notify::notified()
[`notify_one()`]: Notify::notify_one()
[`enable`]: Notified::enable()
[`Semaphore`]: crate::sync::Semaphore

**Methods:**

- `fn new() -> Notify` - Create a new `Notify`, initialized without a permit.
- `fn const_new() -> Notify` - Create a new `Notify`, initialized without a permit.
- `fn notified(self: &Self) -> Notified` - Wait for a notification.
- `fn notified_owned(self: Arc<Self>) -> OwnedNotified` - Wait for a notification with an owned `Future`.
- `fn notify_one(self: &Self)` - Notifies the first waiting task.
- `fn notify_last(self: &Self)` - Notifies the last waiting task.
- `fn notify_waiters(self: &Self)` - Notifies all waiting tasks.

**Trait Implementations:**

- **Default**
  - `fn default() -> Notify`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## tokio::sync::notify::OwnedNotified

*Struct*

Future returned from [`Notify::notified_owned()`].

This future is fused, so once it has completed, any future calls to poll
will immediately return `Poll::Ready`.

**Methods:**

- `fn enable(self: Pin<& mut Self>) -> bool` - Adds this future to the list of futures that are ready to receive

**Traits:** Sync

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Drop**
  - `fn drop(self: & mut Self)`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<()>`



