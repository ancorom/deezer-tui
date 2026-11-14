**tokio > runtime > task > abort > AbortHandle**

# Module: runtime::task::abort::AbortHandle

## Contents

**Functions**

- [`abort`](#abort) - Abort the task associated with the handle.
- [`is_finished`](#is_finished) - Checks if the task associated with this `AbortHandle` has finished.

---

## tokio::runtime::task::abort::AbortHandle::abort

*Function*

Abort the task associated with the handle.

Awaiting a cancelled task might complete as usual if the task was
already completed at the time it was cancelled, but most likely it
will fail with a [cancelled] `JoinError`.

If the task was already cancelled, such as by [`JoinHandle::abort`],
this method will do nothing.

Be aware that tasks spawned using [`spawn_blocking`] cannot be aborted
because they are not async. If you call `abort` on a `spawn_blocking`
task, then this *will not have any effect*, and the task will continue
running normally. The exception is if the task has not started running
yet; in that case, calling `abort` may prevent the task from starting.

See also [the module level docs] for more information on cancellation.

[cancelled]: method@super::error::JoinError::is_cancelled
[`JoinHandle::abort`]: method@super::JoinHandle::abort
[the module level docs]: crate::task#cancellation
[`spawn_blocking`]: crate::task::spawn_blocking

```rust
fn abort(self: &Self)
```



## tokio::runtime::task::abort::AbortHandle::is_finished

*Function*

Checks if the task associated with this `AbortHandle` has finished.

Please note that this method can return `false` even if `abort` has been
called on the task. This is because the cancellation process may take
some time, and this method does not return `true` until it has
completed.

```rust
fn is_finished(self: &Self) -> bool
```



