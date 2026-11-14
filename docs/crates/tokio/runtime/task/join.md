**tokio > runtime > task > join**

# Module: runtime::task::join

## Contents

**Structs**

- [`JoinHandle`](#joinhandle) - An owned permission to join on a task (await its termination).

---

## tokio::runtime::task::join::JoinHandle

*Struct*

An owned permission to join on a task (await its termination).

This can be thought of as the equivalent of [`std::thread::JoinHandle`]
for a Tokio task rather than a thread. Note that the background task
associated with this `JoinHandle` started running immediately when you
called spawn, even if you have not yet awaited the `JoinHandle`.

A `JoinHandle` *detaches* the associated task when it is dropped, which
means that there is no longer any handle to the task, and no way to `join`
on it.

This `struct` is created by the [`task::spawn`] and [`task::spawn_blocking`]
functions.

It is guaranteed that the destructor of the spawned task has finished
before task completion is observed via `JoinHandle` `await`,
[`JoinHandle::is_finished`] or [`AbortHandle::is_finished`].

# Cancel safety

Awaiting a `&mut JoinHandle<T>` is cancel safe. If it is used as a
branch in `tokio::select!` and another branch completes first, then it
is guaranteed that the output of the task is not lost.

If a `JoinHandle` is dropped, then the task continues running in the
background and its return value is lost.

# Examples

Creation from [`task::spawn`]:

```
use tokio::task;

# async fn doc() {
let join_handle: task::JoinHandle<_> = task::spawn(async {
    // some work here
});
# }
```

Creation from [`task::spawn_blocking`]:

```
use tokio::task;

# async fn doc() {
let join_handle: task::JoinHandle<_> = task::spawn_blocking(|| {
    // some blocking work here
});
# }
```

The generic parameter `T` in `JoinHandle<T>` is the return type of the spawned task.
If the return value is an `i32`, the join handle has type `JoinHandle<i32>`:

```
use tokio::task;

# async fn doc() {
let join_handle: task::JoinHandle<i32> = task::spawn(async {
    5 + 3
});
# }

```

If the task does not have a return value, the join handle has type `JoinHandle<()>`:

```
use tokio::task;

# async fn doc() {
let join_handle: task::JoinHandle<()> = task::spawn(async {
    println!("I return nothing.");
});
# }
```

Note that `handle.await` doesn't give you the return type directly. It is wrapped in a
`Result` because panics in the spawned task are caught by Tokio. The `?` operator has
to be double chained to extract the returned value:

```
use tokio::task;
use std::io;

# #[tokio::main(flavor = "current_thread")]
# async fn main() -> io::Result<()> {
let join_handle: task::JoinHandle<Result<i32, io::Error>> = tokio::spawn(async {
    Ok(5 + 3)
});

let result = join_handle.await??;
assert_eq!(result, 8);
Ok(())
# }
```

If the task panics, the error is a [`JoinError`] that contains the panic:

```
# #[cfg(not(target_family = "wasm"))]
# {
use tokio::task;
use std::io;
use std::panic;

#[tokio::main]
async fn main() -> io::Result<()> {
    let join_handle: task::JoinHandle<Result<i32, io::Error>> = tokio::spawn(async {
        panic!("boom");
    });

    let err = join_handle.await.unwrap_err();
    assert!(err.is_panic());
    Ok(())
}
# }
```
Child being detached and outliving its parent:

```no_run
use tokio::task;
use tokio::time;
use std::time::Duration;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let original_task = task::spawn(async {
    let _detached_task = task::spawn(async {
        // Here we sleep to make sure that the first task returns before.
        time::sleep(Duration::from_millis(10)).await;
        // This will be called, even though the JoinHandle is dropped.
        println!("♫ Still alive ♫");
    });
});

original_task.await.expect("The task being joined has panicked");
println!("Original task is joined.");

// We make sure that the new task has time to run, before the main
// task returns.

time::sleep(Duration::from_millis(1000)).await;
# }
```

[`task::spawn`]: crate::task::spawn()
[`task::spawn_blocking`]: crate::task::spawn_blocking
[`std::thread::JoinHandle`]: std::thread::JoinHandle
[`JoinError`]: crate::task::JoinError

**Generic Parameters:**
- T

**Methods:**

- `fn abort(self: &Self)` - Abort the task associated with the handle.
- `fn is_finished(self: &Self) -> bool` - Checks if the task associated with this `JoinHandle` has finished.
- `fn abort_handle(self: &Self) -> AbortHandle` - Returns a new `AbortHandle` that can be used to remotely abort this task.
- `fn id(self: &Self) -> super::Id` - Returns a [task ID] that uniquely identifies this task relative to other

**Traits:** Send, Sync

**Trait Implementations:**

- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`
- **Drop**
  - `fn drop(self: & mut Self)`
- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`



