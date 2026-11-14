**tokio > sync > mutex > Mutex**

# Module: sync::mutex::Mutex

## Contents

**Functions**

- [`blocking_lock`](#blocking_lock) - Blockingly locks this `Mutex`. When the lock has been acquired, function returns a
- [`lock`](#lock) - Locks this mutex, causing the current task to yield until the lock has
- [`new`](#new) - Creates a new lock in an unlocked state ready for use.
- [`try_lock`](#try_lock) - Attempts to acquire the lock, and returns [`TryLockError`] if the

---

## tokio::sync::mutex::Mutex::blocking_lock

*Function*

Blockingly locks this `Mutex`. When the lock has been acquired, function returns a
[`MutexGuard`].

This method is intended for use cases where you
need to use this mutex in asynchronous code as well as in synchronous code.

# Panics

This function panics if called within an asynchronous execution context.

  - If you find yourself in an asynchronous execution context and needing
    to call some (synchronous) function which performs one of these
    `blocking_` operations, then consider wrapping that call inside
    [`spawn_blocking()`][crate::runtime::Handle::spawn_blocking]
    (or [`block_in_place()`][crate::task::block_in_place]).

# Examples

```
# #[cfg(not(target_family = "wasm"))]
# {
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let mutex =  Arc::new(Mutex::new(1));
    let lock = mutex.lock().await;

    let mutex1 = Arc::clone(&mutex);
    let blocking_task = tokio::task::spawn_blocking(move || {
        // This shall block until the `lock` is released.
        let mut n = mutex1.blocking_lock();
        *n = 2;
    });

    assert_eq!(*lock, 1);
    // Release the lock.
    drop(lock);

    // Await the completion of the blocking task.
    blocking_task.await.unwrap();

    // Assert uncontended.
    let n = mutex.try_lock().unwrap();
    assert_eq!(*n, 2);
}
# }
```

```rust
fn blocking_lock(self: &Self) -> MutexGuard<T>
```



## tokio::sync::mutex::Mutex::lock

*Function*

Locks this mutex, causing the current task to yield until the lock has
been acquired.  When the lock has been acquired, function returns a
[`MutexGuard`].

If the mutex is available to be acquired immediately, then this call
will typically not yield to the runtime. However, this is not guaranteed
under all circumstances.

# Cancel safety

This method uses a queue to fairly distribute locks in the order they
were requested. Cancelling a call to `lock` makes you lose your place in
the queue.

# Examples

```
use tokio::sync::Mutex;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let mutex = Mutex::new(1);

let mut n = mutex.lock().await;
*n = 2;
# }
```

```rust
fn lock(self: &Self) -> MutexGuard<T>
```



## tokio::sync::mutex::Mutex::new

*Function*

Creates a new lock in an unlocked state ready for use.

# Examples

```
use tokio::sync::Mutex;

let lock = Mutex::new(5);
```

```rust
fn new(t: T) -> Self
```



## tokio::sync::mutex::Mutex::try_lock

*Function*

Attempts to acquire the lock, and returns [`TryLockError`] if the
lock is currently held somewhere else.

[`TryLockError`]: TryLockError
# Examples

```
use tokio::sync::Mutex;
# async fn dox() -> Result<(), tokio::sync::TryLockError> {

let mutex = Mutex::new(1);

let n = mutex.try_lock()?;
assert_eq!(*n, 1);
# Ok(())
# }
```

```rust
fn try_lock(self: &Self) -> Result<MutexGuard<T>, TryLockError>
```



