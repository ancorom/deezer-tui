**tokio > sync > rwlock**

# Module: sync::rwlock

## Contents

**Structs**

- [`RwLock`](#rwlock) - An asynchronous reader-writer lock.

---

## tokio::sync::rwlock::RwLock

*Struct*

An asynchronous reader-writer lock.

This type of lock allows a number of readers or at most one writer at any
point in time. The write portion of this lock typically allows modification
of the underlying data (exclusive access) and the read portion of this lock
typically allows for read-only access (shared access).

In comparison, a [`Mutex`] does not distinguish between readers or writers
that acquire the lock, therefore causing any tasks waiting for the lock to
become available to yield. An `RwLock` will allow any number of readers to
acquire the lock as long as a writer is not holding the lock.

The priority policy of Tokio's read-write lock is _fair_ (or
[_write-preferring_]), in order to ensure that readers cannot starve
writers. Fairness is ensured using a first-in, first-out queue for the tasks
awaiting the lock; a read lock will not be given out until all write lock
requests that were queued before it have been acquired and released. This is
in contrast to the Rust standard library's `std::sync::RwLock`, where the
priority policy is dependent on the operating system's implementation.

The type parameter `T` represents the data that this lock protects. It is
required that `T` satisfies [`Send`] to be shared across threads. The RAII guards
returned from the locking methods implement [`Deref`](trait@std::ops::Deref)
(and [`DerefMut`](trait@std::ops::DerefMut)
for the `write` methods) to allow access to the content of the lock.

# Examples

```
use tokio::sync::RwLock;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let lock = RwLock::new(5);

// many reader locks can be held at once
{
    let r1 = lock.read().await;
    let r2 = lock.read().await;
    assert_eq!(*r1, 5);
    assert_eq!(*r2, 5);
} // read locks are dropped at this point

// only one write lock may be held, however
{
    let mut w = lock.write().await;
    *w += 1;
    assert_eq!(*w, 6);
} // write lock is dropped here
# }
```

[`Mutex`]: struct@super::Mutex
[`RwLock`]: struct@RwLock
[`RwLockReadGuard`]: struct@RwLockReadGuard
[`RwLockWriteGuard`]: struct@RwLockWriteGuard
[`Send`]: trait@std::marker::Send
[_write-preferring_]: https://en.wikipedia.org/wiki/Readers%E2%80%93writer_lock#Priority_policies

**Generic Parameters:**
- T

**Methods:**

- `fn new(value: T) -> RwLock<T>` - Creates a new instance of an `RwLock<T>` which is unlocked.
- `fn with_max_readers(value: T, max_reads: u32) -> RwLock<T>` - Creates a new instance of an `RwLock<T>` which is unlocked
- `fn const_new(value: T) -> RwLock<T>` - Creates a new instance of an `RwLock<T>` which is unlocked.
- `fn const_with_max_readers(value: T, max_reads: u32) -> RwLock<T>` - Creates a new instance of an `RwLock<T>` which is unlocked
- `fn read(self: &Self) -> RwLockReadGuard<T>` - Locks this `RwLock` with shared read access, causing the current task
- `fn blocking_read(self: &Self) -> RwLockReadGuard<T>` - Blockingly locks this `RwLock` with shared read access.
- `fn read_owned(self: Arc<Self>) -> OwnedRwLockReadGuard<T>` - Locks this `RwLock` with shared read access, causing the current task
- `fn try_read(self: &Self) -> Result<RwLockReadGuard<T>, TryLockError>` - Attempts to acquire this `RwLock` with shared read access.
- `fn try_read_owned(self: Arc<Self>) -> Result<OwnedRwLockReadGuard<T>, TryLockError>` - Attempts to acquire this `RwLock` with shared read access.
- `fn write(self: &Self) -> RwLockWriteGuard<T>` - Locks this `RwLock` with exclusive write access, causing the current
- `fn blocking_write(self: &Self) -> RwLockWriteGuard<T>` - Blockingly locks this `RwLock` with exclusive write access.
- `fn write_owned(self: Arc<Self>) -> OwnedRwLockWriteGuard<T>` - Locks this `RwLock` with exclusive write access, causing the current
- `fn try_write(self: &Self) -> Result<RwLockWriteGuard<T>, TryLockError>` - Attempts to acquire this `RwLock` with exclusive write access.
- `fn try_write_owned(self: Arc<Self>) -> Result<OwnedRwLockWriteGuard<T>, TryLockError>` - Attempts to acquire this `RwLock` with exclusive write access.
- `fn get_mut(self: & mut Self) -> & mut T` - Returns a mutable reference to the underlying data.
- `fn into_inner(self: Self) -> T` - Consumes the lock, returning the underlying data.

**Traits:** Send, Sync

**Trait Implementations:**

- **From**
  - `fn from(s: T) -> Self`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut std::fmt::Formatter) -> std::fmt::Result`



