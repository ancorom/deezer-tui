**tokio > runtime > task > abort**

# Module: runtime::task::abort

## Contents

**Structs**

- [`AbortHandle`](#aborthandle) - An owned permission to abort a spawned task, without awaiting its completion.

---

## tokio::runtime::task::abort::AbortHandle

*Struct*

An owned permission to abort a spawned task, without awaiting its completion.

Unlike a [`JoinHandle`], an `AbortHandle` does *not* represent the
permission to await the task's completion, only to terminate it.

The task may be aborted by calling the [`AbortHandle::abort`] method.
Dropping an `AbortHandle` releases the permission to terminate the task
--- it does *not* abort the task.

Be aware that tasks spawned using [`spawn_blocking`] cannot be aborted
because they are not async. If you call `abort` on a `spawn_blocking` task,
then this *will not have any effect*, and the task will continue running
normally. The exception is if the task has not started running yet; in that
case, calling `abort` may prevent the task from starting.

[`JoinHandle`]: crate::task::JoinHandle
[`spawn_blocking`]: crate::task::spawn_blocking

**Methods:**

- `fn abort(self: &Self)` - Abort the task associated with the handle.
- `fn is_finished(self: &Self) -> bool` - Checks if the task associated with this `AbortHandle` has finished.
- `fn id(self: &Self) -> super::Id` - Returns a [task ID] that uniquely identifies this task relative to other

**Traits:** Sync, Send

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self` - Returns a cloned `AbortHandle` that can be used to remotely abort this task.
- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **Drop**
  - `fn drop(self: & mut Self)`



