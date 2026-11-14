**tokio > task > coop**

# Module: task::coop

## Contents

**Structs**

- [`Coop`](#coop) - Future wrapper to ensure cooperative scheduling created by [`cooperative`].
- [`RestoreOnPending`](#restoreonpending) - Value returned by the [`poll_proceed`] method.

**Functions**

- [`cooperative`](#cooperative) - Creates a wrapper future that makes the inner future cooperate with the Tokio scheduler.
- [`has_budget_remaining`](#has_budget_remaining) -  Returns `true` if there is still budget left on the task.
- [`poll_proceed`](#poll_proceed) - Decrements the task budget and returns [`Poll::Pending`] if the budget is depleted.

---

## tokio::task::coop::Coop

*Struct*

Future wrapper to ensure cooperative scheduling created by [`cooperative`].

**Generic Parameters:**
- F

**Trait Implementations:**

- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## tokio::task::coop::RestoreOnPending

*Struct*

Value returned by the [`poll_proceed`] method.

**Tuple Struct**: `()`

**Methods:**

- `fn made_progress(self: &Self)` - Signals that the task that obtained this `RestoreOnPending` was able to make

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Drop**
  - `fn drop(self: & mut Self)`



## tokio::task::coop::cooperative

*Function*

Creates a wrapper future that makes the inner future cooperate with the Tokio scheduler.

When polled, the wrapper will first call [`poll_proceed`] to consume task budget, and
immediately yield if the budget has been depleted. If budget was available, the inner future
is polled. The budget consumption will be made final using [`RestoreOnPending::made_progress`]
if the inner future resolves to its final value.

# Examples

When you call `recv` on the `Receiver` of a [`tokio::sync::mpsc`](crate::sync::mpsc)
channel, task budget will automatically be consumed when the next value is returned.
This makes tasks that use Tokio mpsc channels automatically cooperative.

If you're using [`futures::channel::mpsc`](https://docs.rs/futures/latest/futures/channel/mpsc/index.html)
instead, automatic task budget consumption will not happen. This example shows how can use
`cooperative` to make `futures::channel::mpsc` channels cooperate with the scheduler in the
same way Tokio channels do.

```
use tokio::task::coop::cooperative;
use futures::channel::mpsc::Receiver;
use futures::stream::StreamExt;

async fn receive_next<T>(receiver: &mut Receiver<T>) -> Option<T> {
    // Use `StreamExt::next` to obtain a `Future` that resolves to the next value
    let recv_future = receiver.next();
    // Wrap it a cooperative wrapper
    let coop_future = cooperative(recv_future);
    // And await
    coop_future.await
}

```rust
fn cooperative<F>(fut: F) -> Coop<F>
```



## tokio::task::coop::has_budget_remaining

*Function*

 Returns `true` if there is still budget left on the task.

 # Examples

 This example defines a `Timeout` future that requires a given `future` to complete before the
 specified duration elapses. If it does, its result is returned; otherwise, an error is returned
 and the future is canceled.

 Note that the future could exhaust the budget before we evaluate the timeout. Using `has_budget_remaining`,
 we can detect this scenario and ensure the timeout is always checked.

 ```
 # use std::future::Future;
 # use std::pin::{pin, Pin};
 # use std::task::{ready, Context, Poll};
 # use tokio::task::coop;
 # use tokio::time::Sleep;
 pub struct Timeout<T> {
     future: T,
     delay: Pin<Box<Sleep>>,
 }

 impl<T> Future for Timeout<T>
 where
     T: Future + Unpin,
 {
     type Output = Result<T::Output, ()>;

     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
         let this = Pin::into_inner(self);
         let future = Pin::new(&mut this.future);
         let delay = Pin::new(&mut this.delay);

         // check if the future is ready
         let had_budget_before = coop::has_budget_remaining();
         if let Poll::Ready(v) = future.poll(cx) {
             return Poll::Ready(Ok(v));
         }
         let has_budget_now = coop::has_budget_remaining();

         // evaluate the timeout
         if let (true, false) = (had_budget_before, has_budget_now) {
             // it is the underlying future that exhausted the budget
             ready!(pin!(coop::unconstrained(delay)).poll(cx));
         } else {
             ready!(delay.poll(cx));
         }
         return Poll::Ready(Err(()));
     }
 }
```

```rust
fn has_budget_remaining() -> bool
```



## tokio::task::coop::poll_proceed

*Function*

Decrements the task budget and returns [`Poll::Pending`] if the budget is depleted.
This indicates that the task should yield to the scheduler. Otherwise, returns
[`RestoreOnPending`] which can be used to commit the budget consumption.

The returned [`RestoreOnPending`] will revert the budget to its former
value when dropped unless [`RestoreOnPending::made_progress`]
is called. It is the caller's responsibility to do so when it _was_ able to
make progress after the call to [`poll_proceed`].
Restoring the budget automatically ensures the task can try to make progress in some other
way.

Note that [`RestoreOnPending`] restores the budget **as it was before [`poll_proceed`]**.
Therefore, if the budget is _further_ adjusted between when [`poll_proceed`] returns and
[`RestoreOnPending`] is dropped, those adjustments are erased unless the caller indicates
that progress was made.

# Examples

This example wraps the `futures::channel::mpsc::UnboundedReceiver` to
cooperate with the Tokio scheduler. Each time a value is received, task budget
is consumed. If no budget is available, the task yields to the scheduler.

```
use std::pin::Pin;
use std::task::{ready, Context, Poll};
use tokio::task::coop;
use futures::stream::{Stream, StreamExt};
use futures::channel::mpsc::UnboundedReceiver;

struct CoopUnboundedReceiver<T> {
   receiver: UnboundedReceiver<T>,
}

impl<T> Stream for CoopUnboundedReceiver<T> {
    type Item = T;
    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<Option<T>> {
        let coop = ready!(coop::poll_proceed(cx));
        match self.receiver.poll_next_unpin(cx) {
            Poll::Ready(v) => {
                // We received a value, so consume budget.
                coop.made_progress();
                Poll::Ready(v)
            }
            Poll::Pending => Poll::Pending,
       }
    }
}
```

```rust
fn poll_proceed(cx: & mut std::task::Context) -> std::task::Poll<RestoreOnPending>
```



