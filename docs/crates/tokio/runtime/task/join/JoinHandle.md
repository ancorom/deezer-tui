**tokio > runtime > task > join > JoinHandle**

# Module: runtime::task::join::JoinHandle

## Contents

**Functions**

- [`abort`](#abort) - Abort the task associated with the handle.
- [`id`](#id) - Returns a [task ID] that uniquely identifies this task relative to other
- [`is_finished`](#is_finished) - Checks if the task associated with this `JoinHandle` has finished.

---

## tokio::runtime::task::join::JoinHandle::abort

*Function*

Abort the task associated with the handle.

Awaiting a cancelled task might complete as usual if the task was
already completed at the time it was cancelled, but most likely it
will fail with a [cancelled] `JoinError`.

Be aware that tasks spawned using [`spawn_blocking`] cannot be aborted
because they are not async. If you call `abort` on a `spawn_blocking`
task, then this *will not have any effect*, and the task will continue
running normally. The exception is if the task has not started running
yet; in that case, calling `abort` may prevent the task from starting.

See also [the module level docs] for more information on cancellation.

```rust
use tokio::time;

# #[tokio::main(flavor = "current_thread", start_paused = true)]
# async fn main() {
let mut handles = Vec::new();

handles.push(tokio::spawn(async {
   time::sleep(time::Duration::from_secs(10)).await;
   true
}));

handles.push(tokio::spawn(async {
   time::sleep(time::Duration::from_secs(10)).await;
   false
}));

for handle in &handles {
    handle.abort();
}

for handle in handles {
    assert!(handle.await.unwrap_err().is_cancelled());
}
# }
```

[cancelled]: method@super::error::JoinError::is_cancelled
[the module level docs]: crate::task#cancellation
[`spawn_blocking`]: crate::task::spawn_blocking

```rust
fn abort(self: &Self)
```



## tokio::runtime::task::join::JoinHandle::id

*Function*

Returns a [task ID] that uniquely identifies this task relative to other
currently spawned tasks.

[task ID]: crate::task::Id

```rust
fn id(self: &Self) -> super::Id
```



## tokio::runtime::task::join::JoinHandle::is_finished

*Function*

Checks if the task associated with this `JoinHandle` has finished.

Please note that this method can return `false` even if [`abort`] has been
called on the task. This is because the cancellation process may take
some time, and this method does not return `true` until it has
completed.

```rust
use tokio::time;

# #[tokio::main(flavor = "current_thread", start_paused = true)]
# async fn main() {
let handle1 = tokio::spawn(async {
    // do some stuff here
});
let handle2 = tokio::spawn(async {
    // do some other stuff here
    time::sleep(time::Duration::from_secs(10)).await;
});
// Wait for the task to finish
handle2.abort();
time::sleep(time::Duration::from_secs(1)).await;
assert!(handle1.is_finished());
assert!(handle2.is_finished());
# }
```
[`abort`]: method@JoinHandle::abort

```rust
fn is_finished(self: &Self) -> bool
```



