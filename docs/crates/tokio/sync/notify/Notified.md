**tokio > sync > notify > Notified**

# Module: sync::notify::Notified

## Contents

**Functions**

- [`enable`](#enable) - Adds this future to the list of futures that are ready to receive

---

## tokio::sync::notify::Notified::enable

*Function*

Adds this future to the list of futures that are ready to receive
wakeups from calls to [`notify_one`].

Polling the future also adds it to the list, so this method should only
be used if you want to add the future to the list before the first call
to `poll`. (In fact, this method is equivalent to calling `poll` except
that no `Waker` is registered.)

This has no effect on notifications sent using [`notify_waiters`], which
are received as long as they happen after the creation of the `Notified`
regardless of whether `enable` or `poll` has been called.

This method returns true if the `Notified` is ready. This happens in the
following situations:

 1. The `notify_waiters` method was called between the creation of the
    `Notified` and the call to this method.
 2. This is the first call to `enable` or `poll` on this future, and the
    `Notify` was holding a permit from a previous call to `notify_one`.
    The call consumes the permit in that case.
 3. The future has previously been enabled or polled, and it has since
    then been marked ready by either consuming a permit from the
    `Notify`, or by a call to `notify_one` or `notify_waiters` that
    removed it from the list of futures ready to receive wakeups.

If this method returns true, any future calls to poll on the same future
will immediately return `Poll::Ready`.

# Examples

Unbound multi-producer multi-consumer (mpmc) channel.

The call to `enable` is important because otherwise if you have two
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

[`notify_one`]: Notify::notify_one()
[`notify_waiters`]: Notify::notify_waiters()

```rust
fn enable(self: Pin<& mut Self>) -> bool
```



