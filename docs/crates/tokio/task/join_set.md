**tokio > task > join_set**

# Module: task::join_set

## Contents

**Structs**

- [`JoinSet`](#joinset) - A collection of tasks spawned on a Tokio runtime.

---

## tokio::task::join_set::JoinSet

*Struct*

A collection of tasks spawned on a Tokio runtime.

A `JoinSet` can be used to await the completion of some or all of the tasks
in the set. The set is not ordered, and the tasks will be returned in the
order they complete.

All of the tasks must have the same return type `T`.

When the `JoinSet` is dropped, all tasks in the `JoinSet` are immediately aborted.

# Examples

Spawn multiple tasks and wait for them.

```
use tokio::task::JoinSet;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let mut set = JoinSet::new();

for i in 0..10 {
    set.spawn(async move { i });
}

let mut seen = [false; 10];
while let Some(res) = set.join_next().await {
    let idx = res.unwrap();
    seen[idx] = true;
}

for i in 0..10 {
    assert!(seen[i]);
}
# }
```

# Task ID guarantees

While a task is tracked in a `JoinSet`, that task's ID is unique relative
to all other running tasks in Tokio. For this purpose, tracking a task in a
`JoinSet` is equivalent to holding a [`JoinHandle`] to it. See the [task ID]
documentation for more info.

[`JoinHandle`]: crate::task::JoinHandle
[task ID]: crate::task::Id

**Generic Parameters:**
- T

**Methods:**

- `fn new() -> Self` - Create a new `JoinSet`.
- `fn len(self: &Self) -> usize` - Returns the number of tasks currently in the `JoinSet`.
- `fn is_empty(self: &Self) -> bool` - Returns whether the `JoinSet` is empty.
- `fn spawn<F>(self: & mut Self, task: F) -> AbortHandle` - Spawn the provided task on the `JoinSet`, returning an [`AbortHandle`]
- `fn spawn_on<F>(self: & mut Self, task: F, handle: &Handle) -> AbortHandle` - Spawn the provided task on the provided runtime and store it in this
- `fn spawn_local<F>(self: & mut Self, task: F) -> AbortHandle` - Spawn the provided task on the current [`LocalSet`] or [`LocalRuntime`]
- `fn spawn_local_on<F>(self: & mut Self, task: F, local_set: &LocalSet) -> AbortHandle` - Spawn the provided task on the provided [`LocalSet`] and store it in
- `fn spawn_blocking<F>(self: & mut Self, f: F) -> AbortHandle` - Spawn the blocking code on the blocking threadpool and store
- `fn spawn_blocking_on<F>(self: & mut Self, f: F, handle: &Handle) -> AbortHandle` - Spawn the blocking code on the blocking threadpool of the
- `fn join_next(self: & mut Self) -> Option<Result<T, JoinError>>` - Waits until one of the tasks in the set completes and returns its output.
- `fn join_next_with_id(self: & mut Self) -> Option<Result<(Id, T), JoinError>>` - Waits until one of the tasks in the set completes and returns its
- `fn try_join_next(self: & mut Self) -> Option<Result<T, JoinError>>` - Tries to join one of the tasks in the set that has completed and return its output.
- `fn try_join_next_with_id(self: & mut Self) -> Option<Result<(Id, T), JoinError>>` - Tries to join one of the tasks in the set that has completed and return its output,
- `fn shutdown(self: & mut Self)` - Aborts all tasks and waits for them to finish shutting down.
- `fn join_all(self: Self) -> Vec<T>` - Awaits the completion of all tasks in this `JoinSet`, returning a vector of their results.
- `fn abort_all(self: & mut Self)` - Aborts all tasks on this `JoinSet`.
- `fn detach_all(self: & mut Self)` - Removes all tasks from this `JoinSet` without aborting them.
- `fn poll_join_next(self: & mut Self, cx: & mut Context) -> Poll<Option<Result<T, JoinError>>>` - Polls for one of the tasks in the set to complete.
- `fn poll_join_next_with_id(self: & mut Self, cx: & mut Context) -> Poll<Option<Result<(Id, T), JoinError>>>` - Polls for one of the tasks in the set to complete.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Default**
  - `fn default() -> Self`
- **FromIterator**
  - `fn from_iter<I>(iter: I) -> Self`
- **Extend**
  - `fn extend<I>(self: & mut Self, iter: I)`
- **Drop**
  - `fn drop(self: & mut Self)`



