**tokio > sync > rwlock > RwLock**

# Module: sync::rwlock::RwLock

## Contents

**Functions**

- [`new`](#new) - Creates a new instance of an `RwLock<T>` which is unlocked.
- [`read`](#read) - Locks this `RwLock` with shared read access, causing the current task
- [`read_owned`](#read_owned) - Locks this `RwLock` with shared read access, causing the current task
- [`try_read`](#try_read) - Attempts to acquire this `RwLock` with shared read access.
- [`try_write`](#try_write) - Attempts to acquire this `RwLock` with exclusive write access.
- [`write`](#write) - Locks this `RwLock` with exclusive write access, causing the current
- [`write_owned`](#write_owned) - Locks this `RwLock` with exclusive write access, causing the current

---

## tokio::sync::rwlock::RwLock::new

*Function*

Creates a new instance of an `RwLock<T>` which is unlocked.

# Examples

```
use tokio::sync::RwLock;

let lock = RwLock::new(5);
```

```rust
fn new(value: T) -> RwLock<T>
```



## tokio::sync::rwlock::RwLock::read

*Function*

Locks this `RwLock` with shared read access, causing the current task
to yield until the lock has been acquired.

The calling task will yield until there are no writers which hold the
lock. There may be other readers inside the lock when the task resumes.

Note that under the priority policy of [`RwLock`], read locks are not
granted until prior write locks, to prevent starvation. Therefore
deadlock may occur if a read lock is held by the current task, a write
lock attempt is made, and then a subsequent read lock attempt is made
by the current task.

Returns an RAII guard which will drop this read access of the `RwLock`
when dropped.

# Cancel safety

This method uses a queue to fairly distribute locks in the order they
were requested. Cancelling a call to `read` makes you lose your place in
the queue.

# Examples

```
use std::sync::Arc;
use tokio::sync::RwLock;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let lock = Arc::new(RwLock::new(1));
let c_lock = lock.clone();

let n = lock.read().await;
assert_eq!(*n, 1);

tokio::spawn(async move {
    // While main has an active read lock, we acquire one too.
    let r = c_lock.read().await;
    assert_eq!(*r, 1);
}).await.expect("The spawned task has panicked");

// Drop the guard after the spawned task finishes.
drop(n);
# }
```

```rust
fn read(self: &Self) -> RwLockReadGuard<T>
```



## tokio::sync::rwlock::RwLock::read_owned

*Function*

Locks this `RwLock` with shared read access, causing the current task
to yield until the lock has been acquired.

The calling task will yield until there are no writers which hold the
lock. There may be other readers inside the lock when the task resumes.

This method is identical to [`RwLock::read`], except that the returned
guard references the `RwLock` with an [`Arc`] rather than by borrowing
it. Therefore, the `RwLock` must be wrapped in an `Arc` to call this
method, and the guard will live for the `'static` lifetime, as it keeps
the `RwLock` alive by holding an `Arc`.

Note that under the priority policy of [`RwLock`], read locks are not
granted until prior write locks, to prevent starvation. Therefore
deadlock may occur if a read lock is held by the current task, a write
lock attempt is made, and then a subsequent read lock attempt is made
by the current task.

Returns an RAII guard which will drop this read access of the `RwLock`
when dropped.

# Cancel safety

This method uses a queue to fairly distribute locks in the order they
were requested. Cancelling a call to `read_owned` makes you lose your
place in the queue.

# Examples

```
use std::sync::Arc;
use tokio::sync::RwLock;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let lock = Arc::new(RwLock::new(1));
let c_lock = lock.clone();

let n = lock.read_owned().await;
assert_eq!(*n, 1);

tokio::spawn(async move {
    // While main has an active read lock, we acquire one too.
    let r = c_lock.read_owned().await;
    assert_eq!(*r, 1);
}).await.expect("The spawned task has panicked");

// Drop the guard after the spawned task finishes.
drop(n);
# }
```

```rust
fn read_owned(self: Arc<Self>) -> OwnedRwLockReadGuard<T>
```



## tokio::sync::rwlock::RwLock::try_read

*Function*

Attempts to acquire this `RwLock` with shared read access.

If the access couldn't be acquired immediately, returns [`TryLockError`].
Otherwise, an RAII guard is returned which will release read access
when dropped.

[`TryLockError`]: TryLockError

# Examples

```
use std::sync::Arc;
use tokio::sync::RwLock;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let lock = Arc::new(RwLock::new(1));
let c_lock = lock.clone();

let v = lock.try_read().unwrap();
assert_eq!(*v, 1);

tokio::spawn(async move {
    // While main has an active read lock, we acquire one too.
    let n = c_lock.read().await;
    assert_eq!(*n, 1);
}).await.expect("The spawned task has panicked");

// Drop the guard when spawned task finishes.
drop(v);
# }
```

```rust
fn try_read(self: &Self) -> Result<RwLockReadGuard<T>, TryLockError>
```



## tokio::sync::rwlock::RwLock::try_write

*Function*

Attempts to acquire this `RwLock` with exclusive write access.

If the access couldn't be acquired immediately, returns [`TryLockError`].
Otherwise, an RAII guard is returned which will release write access
when dropped.

[`TryLockError`]: TryLockError

# Examples

```
use tokio::sync::RwLock;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let rw = RwLock::new(1);

let v = rw.read().await;
assert_eq!(*v, 1);

assert!(rw.try_write().is_err());
# }
```

```rust
fn try_write(self: &Self) -> Result<RwLockWriteGuard<T>, TryLockError>
```



## tokio::sync::rwlock::RwLock::write

*Function*

Locks this `RwLock` with exclusive write access, causing the current
task to yield until the lock has been acquired.

The calling task will yield while other writers or readers currently
have access to the lock.

Returns an RAII guard which will drop the write access of this `RwLock`
when dropped.

# Cancel safety

This method uses a queue to fairly distribute locks in the order they
were requested. Cancelling a call to `write` makes you lose your place
in the queue.

# Examples

```
use tokio::sync::RwLock;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let lock = RwLock::new(1);

let mut n = lock.write().await;
*n = 2;
# }
```

```rust
fn write(self: &Self) -> RwLockWriteGuard<T>
```



## tokio::sync::rwlock::RwLock::write_owned

*Function*

Locks this `RwLock` with exclusive write access, causing the current
task to yield until the lock has been acquired.

The calling task will yield while other writers or readers currently
have access to the lock.

This method is identical to [`RwLock::write`], except that the returned
guard references the `RwLock` with an [`Arc`] rather than by borrowing
it. Therefore, the `RwLock` must be wrapped in an `Arc` to call this
method, and the guard will live for the `'static` lifetime, as it keeps
the `RwLock` alive by holding an `Arc`.

Returns an RAII guard which will drop the write access of this `RwLock`
when dropped.

# Cancel safety

This method uses a queue to fairly distribute locks in the order they
were requested. Cancelling a call to `write_owned` makes you lose your
place in the queue.

# Examples

```
use std::sync::Arc;
use tokio::sync::RwLock;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let lock = Arc::new(RwLock::new(1));

let mut n = lock.write_owned().await;
*n = 2;
# }
```

```rust
fn write_owned(self: Arc<Self>) -> OwnedRwLockWriteGuard<T>
```



