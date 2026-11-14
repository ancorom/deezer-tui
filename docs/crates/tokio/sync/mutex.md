**tokio > sync > mutex**

# Module: sync::mutex

## Contents

**Structs**

- [`MappedMutexGuard`](#mappedmutexguard) - A handle to a held `Mutex` that has had a function applied to it via [`MutexGuard::map`].
- [`Mutex`](#mutex) - An asynchronous `Mutex`-like type.
- [`MutexGuard`](#mutexguard) - A handle to a held `Mutex`. The guard can be held across any `.await` point
- [`OwnedMappedMutexGuard`](#ownedmappedmutexguard) - A owned handle to a held `Mutex` that has had a function applied to it via
- [`OwnedMutexGuard`](#ownedmutexguard) - An owned handle to a held `Mutex`.
- [`TryLockError`](#trylockerror) - Error returned from the [`Mutex::try_lock`], [`RwLock::try_read`] and

---

## tokio::sync::mutex::MappedMutexGuard

*Struct*

A handle to a held `Mutex` that has had a function applied to it via [`MutexGuard::map`].

This can be used to hold a subfield of the protected data.

[`MutexGuard::map`]: method@MutexGuard::map

**Generic Parameters:**
- 'a
- T

**Methods:**

- `fn map<U, F>(this: Self, f: F) -> MappedMutexGuard<'a, U>` - Makes a new [`MappedMutexGuard`] for a component of the locked data.
- `fn try_map<U, F>(this: Self, f: F) -> Result<MappedMutexGuard<'a, U>, Self>` - Attempts to make a new [`MappedMutexGuard`] for a component of the locked data. The

**Traits:** Send, Sync

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Drop**
  - `fn drop(self: & mut Self)`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`



## tokio::sync::mutex::Mutex

*Struct*

An asynchronous `Mutex`-like type.

This type acts similarly to [`std::sync::Mutex`], with two major
differences: [`lock`] is an async method so does not block, and the lock
guard is designed to be held across `.await` points.

Tokio's Mutex operates on a guaranteed FIFO basis.
This means that the order in which tasks call the [`lock`] method is
the exact order in which they will acquire the lock.

# Which kind of mutex should you use?

Contrary to popular belief, it is ok and often preferred to use the ordinary
[`Mutex`][std] from the standard library in asynchronous code.

The feature that the async mutex offers over the blocking mutex is the
ability to keep it locked across an `.await` point. This makes the async
mutex more expensive than the blocking mutex, so the blocking mutex should
be preferred in the cases where it can be used. The primary use case for the
async mutex is to provide shared mutable access to IO resources such as a
database connection. If the value behind the mutex is just data, it's
usually appropriate to use a blocking mutex such as the one in the standard
library or [`parking_lot`].

Note that, although the compiler will not prevent the std `Mutex` from holding
its guard across `.await` points in situations where the task is not movable
between threads, this virtually never leads to correct concurrent code in
practice as it can easily lead to deadlocks.

A common pattern is to wrap the `Arc<Mutex<...>>` in a struct that provides
non-async methods for performing operations on the data within, and only
lock the mutex inside these methods. The [mini-redis] example provides an
illustration of this pattern.

Additionally, when you _do_ want shared access to an IO resource, it is
often better to spawn a task to manage the IO resource, and to use message
passing to communicate with that task.

[std]: std::sync::Mutex
[`parking_lot`]: https://docs.rs/parking_lot
[mini-redis]: https://github.com/tokio-rs/mini-redis/blob/master/src/db.rs

# Examples:

```rust,no_run
use tokio::sync::Mutex;
use std::sync::Arc;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let data1 = Arc::new(Mutex::new(0));
let data2 = Arc::clone(&data1);

tokio::spawn(async move {
    let mut lock = data2.lock().await;
    *lock += 1;
});

let mut lock = data1.lock().await;
*lock += 1;
# }
```


```rust,no_run
use tokio::sync::Mutex;
use std::sync::Arc;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let count = Arc::new(Mutex::new(0));

for i in 0..5 {
    let my_count = Arc::clone(&count);
    tokio::spawn(async move {
        for j in 0..10 {
            let mut lock = my_count.lock().await;
            *lock += 1;
            println!("{} {} {}", i, j, lock);
        }
    });
}

loop {
    if *count.lock().await >= 50 {
        break;
    }
}
println!("Count hit 50.");
# }
```
There are a few things of note here to pay attention to in this example.
1. The mutex is wrapped in an [`Arc`] to allow it to be shared across
   threads.
2. Each spawned task obtains a lock and releases it on every iteration.
3. Mutation of the data protected by the Mutex is done by de-referencing
   the obtained lock as seen on lines 13 and 20.

Tokio's Mutex works in a simple FIFO (first in, first out) style where all
calls to [`lock`] complete in the order they were performed. In that way the
Mutex is "fair" and predictable in how it distributes the locks to inner
data. Locks are released and reacquired after every iteration, so basically,
each thread goes to the back of the line after it increments the value once.
Note that there's some unpredictability to the timing between when the
threads are started, but once they are going they alternate predictably.
Finally, since there is only a single valid lock at any given time, there is
no possibility of a race condition when mutating the inner value.

Note that in contrast to [`std::sync::Mutex`], this implementation does not
poison the mutex when a thread holding the [`MutexGuard`] panics. In such a
case, the mutex will be unlocked. If the panic is caught, this might leave
the data protected by the mutex in an inconsistent state.

[`Mutex`]: struct@Mutex
[`MutexGuard`]: struct@MutexGuard
[`Arc`]: struct@std::sync::Arc
[`std::sync::Mutex`]: struct@std::sync::Mutex
[`Send`]: trait@std::marker::Send
[`lock`]: method@Mutex::lock

**Generic Parameters:**
- T

**Methods:**

- `fn new(t: T) -> Self` - Creates a new lock in an unlocked state ready for use.
- `fn const_new(t: T) -> Self` - Creates a new lock in an unlocked state ready for use.
- `fn lock(self: &Self) -> MutexGuard<T>` - Locks this mutex, causing the current task to yield until the lock has
- `fn blocking_lock(self: &Self) -> MutexGuard<T>` - Blockingly locks this `Mutex`. When the lock has been acquired, function returns a
- `fn blocking_lock_owned(self: Arc<Self>) -> OwnedMutexGuard<T>` - Blockingly locks this `Mutex`. When the lock has been acquired, function returns an
- `fn lock_owned(self: Arc<Self>) -> OwnedMutexGuard<T>` - Locks this mutex, causing the current task to yield until the lock has
- `fn try_lock(self: &Self) -> Result<MutexGuard<T>, TryLockError>` - Attempts to acquire the lock, and returns [`TryLockError`] if the
- `fn get_mut(self: & mut Self) -> & mut T` - Returns a mutable reference to the underlying data.
- `fn try_lock_owned(self: Arc<Self>) -> Result<OwnedMutexGuard<T>, TryLockError>` - Attempts to acquire the lock, and returns [`TryLockError`] if the lock
- `fn into_inner(self: Self) -> T` - Consumes the mutex, returning the underlying data.

**Traits:** Sync, Send

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut std::fmt::Formatter) -> std::fmt::Result`
- **From**
  - `fn from(s: T) -> Self`
- **Default**
  - `fn default() -> Self`



## tokio::sync::mutex::MutexGuard

*Struct*

A handle to a held `Mutex`. The guard can be held across any `.await` point
as it is [`Send`].

As long as you have this guard, you have exclusive access to the underlying
`T`. The guard internally borrows the `Mutex`, so the mutex will not be
dropped while a guard exists.

The lock is automatically released whenever the guard is dropped, at which
point `lock` will succeed yet again.

**Generic Parameters:**
- 'a
- T

**Methods:**

- `fn map<U, F>(this: Self, f: F) -> MappedMutexGuard<'a, U>` - Makes a new [`MappedMutexGuard`] for a component of the locked data.
- `fn try_map<U, F>(this: Self, f: F) -> Result<MappedMutexGuard<'a, U>, Self>` - Attempts to make a new [`MappedMutexGuard`] for a component of the locked data. The
- `fn mutex(this: &Self) -> &'a Mutex<T>` - Returns a reference to the original `Mutex`.

**Traits:** Sync

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Drop**
  - `fn drop(self: & mut Self)`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`



## tokio::sync::mutex::OwnedMappedMutexGuard

*Struct*

A owned handle to a held `Mutex` that has had a function applied to it via
[`OwnedMutexGuard::map`].

This can be used to hold a subfield of the protected data.

[`OwnedMutexGuard::map`]: method@OwnedMutexGuard::map

**Generic Parameters:**
- T
- U

**Methods:**

- `fn map<S, F>(this: Self, f: F) -> OwnedMappedMutexGuard<T, S>` - Makes a new [`OwnedMappedMutexGuard`] for a component of the locked data.
- `fn try_map<S, F>(this: Self, f: F) -> Result<OwnedMappedMutexGuard<T, S>, Self>` - Attempts to make a new [`OwnedMappedMutexGuard`] for a component of the locked data. The

**Traits:** Send, Sync

**Trait Implementations:**

- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Drop**
  - `fn drop(self: & mut Self)`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## tokio::sync::mutex::OwnedMutexGuard

*Struct*

An owned handle to a held `Mutex`.

This guard is only available from a `Mutex` that is wrapped in an [`Arc`]. It
is identical to `MutexGuard`, except that rather than borrowing the `Mutex`,
it clones the `Arc`, incrementing the reference count. This means that
unlike `MutexGuard`, it will have the `'static` lifetime.

As long as you have this guard, you have exclusive access to the underlying
`T`. The guard internally keeps a reference-counted pointer to the original
`Mutex`, so even if the lock goes away, the guard remains valid.

The lock is automatically released whenever the guard is dropped, at which
point `lock` will succeed yet again.

[`Arc`]: std::sync::Arc

**Generic Parameters:**
- T

**Methods:**

- `fn map<U, F>(this: Self, f: F) -> OwnedMappedMutexGuard<T, U>` - Makes a new [`OwnedMappedMutexGuard`] for a component of the locked data.
- `fn try_map<U, F>(this: Self, f: F) -> Result<OwnedMappedMutexGuard<T, U>, Self>` - Attempts to make a new [`OwnedMappedMutexGuard`] for a component of the locked data. The
- `fn mutex(this: &Self) -> &Arc<Mutex<T>>` - Returns a reference to the original `Arc<Mutex>`.

**Traits:** Sync

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Drop**
  - `fn drop(self: & mut Self)`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`



## tokio::sync::mutex::TryLockError

*Struct*

Error returned from the [`Mutex::try_lock`], [`RwLock::try_read`] and
[`RwLock::try_write`] functions.

`Mutex::try_lock` operation will only fail if the mutex is already locked.

`RwLock::try_read` operation will only fail if the lock is currently held
by an exclusive writer.

`RwLock::try_write` operation will only fail if the lock is currently held
by any reader or by an exclusive writer.

[`Mutex::try_lock`]: Mutex::try_lock
[`RwLock::try_read`]: fn@super::RwLock::try_read
[`RwLock::try_write`]: fn@super::RwLock::try_write

**Tuple Struct**: `()`

**Traits:** Error

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



