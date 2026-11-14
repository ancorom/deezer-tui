**tokio > task**

# Module: task

## Contents

**Modules**

- [`coop`](#coop) - Utilities for improved cooperative scheduling.
- [`futures`](#futures) - Task-related futures.

---

## Module: coop

Utilities for improved cooperative scheduling.

### Cooperative scheduling

A single call to [`poll`] on a top-level task may potentially do a lot of
work before it returns `Poll::Pending`. If a task runs for a long period of
time without yielding back to the executor, it can starve other tasks
waiting on that executor to execute them, or drive underlying resources.
Since Rust does not have a runtime, it is difficult to forcibly preempt a
long-running task. Instead, this module provides an opt-in mechanism for
futures to collaborate with the executor to avoid starvation.

Consider a future like this one:

```
# use tokio_stream::{Stream, StreamExt};
async fn drop_all<I: Stream + Unpin>(mut input: I) {
    while let Some(_) = input.next().await {}
}
```

It may look harmless, but consider what happens under heavy load if the
input stream is _always_ ready. If we spawn `drop_all`, the task will never
yield, and will starve other tasks and resources on the same executor.

To account for this, Tokio has explicit yield points in a number of library
functions, which force tasks to return to the executor periodically.


#### unconstrained

If necessary, [`task::unconstrained`] lets you opt a future out of Tokio's cooperative
scheduling. When a future is wrapped with `unconstrained`, it will never be forced to yield to
Tokio. For example:

```
# #[tokio::main(flavor = "current_thread")]
# async fn main() {
use tokio::{task, sync::mpsc};

let fut = async {
    let (tx, mut rx) = mpsc::unbounded_channel();

    for i in 0..1000 {
        let _ = tx.send(());
        // This will always be ready. If coop was in effect, this code would be forced to yield
        // periodically. However, if left unconstrained, then this code will never yield.
        rx.recv().await;
    }
};

task::coop::unconstrained(fut).await;
# }
```
[`poll`]: method@std::future::Future::poll
[`task::unconstrained`]: crate::task::unconstrained()



## Module: futures

Task-related futures.



