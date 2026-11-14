**tokio > sync > semaphore > Semaphore**

# Module: sync::semaphore::Semaphore

## Contents

**Functions**

- [`acquire`](#acquire) - Acquires a permit from the semaphore.
- [`acquire_many`](#acquire_many) - Acquires `n` permits from the semaphore.
- [`acquire_many_owned`](#acquire_many_owned) - Acquires `n` permits from the semaphore.
- [`acquire_owned`](#acquire_owned) - Acquires a permit from the semaphore.
- [`add_permits`](#add_permits) - Adds `n` new permits to the semaphore.
- [`available_permits`](#available_permits) - Returns the current number of available permits.
- [`close`](#close) - Closes the semaphore.
- [`forget_permits`](#forget_permits) - Decrease a semaphore's permits by a maximum of `n`.
- [`is_closed`](#is_closed) - Returns true if the semaphore is closed
- [`new`](#new) - Creates a new semaphore with the initial number of permits.
- [`try_acquire`](#try_acquire) - Tries to acquire a permit from the semaphore.
- [`try_acquire_many`](#try_acquire_many) - Tries to acquire `n` permits from the semaphore.

---

## tokio::sync::semaphore::Semaphore::acquire

*Function*

Acquires a permit from the semaphore.

If the semaphore has been closed, this returns an [`AcquireError`].
Otherwise, this returns a [`SemaphorePermit`] representing the
acquired permit.

# Cancel safety

This method uses a queue to fairly distribute permits in the order they
were requested. Cancelling a call to `acquire` makes you lose your place
in the queue.

# Examples

```
use tokio::sync::Semaphore;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let semaphore = Semaphore::new(2);

let permit_1 = semaphore.acquire().await.unwrap();
assert_eq!(semaphore.available_permits(), 1);

let permit_2 = semaphore.acquire().await.unwrap();
assert_eq!(semaphore.available_permits(), 0);

drop(permit_1);
assert_eq!(semaphore.available_permits(), 1);
# }
```

[`AcquireError`]: crate::sync::AcquireError
[`SemaphorePermit`]: crate::sync::SemaphorePermit

```rust
fn acquire(self: &Self) -> Result<SemaphorePermit, AcquireError>
```



## tokio::sync::semaphore::Semaphore::acquire_many

*Function*

Acquires `n` permits from the semaphore.

If the semaphore has been closed, this returns an [`AcquireError`].
Otherwise, this returns a [`SemaphorePermit`] representing the
acquired permits.

# Cancel safety

This method uses a queue to fairly distribute permits in the order they
were requested. Cancelling a call to `acquire_many` makes you lose your
place in the queue.

# Examples

```
use tokio::sync::Semaphore;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let semaphore = Semaphore::new(5);

let permit = semaphore.acquire_many(3).await.unwrap();
assert_eq!(semaphore.available_permits(), 2);
# }
```

[`AcquireError`]: crate::sync::AcquireError
[`SemaphorePermit`]: crate::sync::SemaphorePermit

```rust
fn acquire_many(self: &Self, n: u32) -> Result<SemaphorePermit, AcquireError>
```



## tokio::sync::semaphore::Semaphore::acquire_many_owned

*Function*

Acquires `n` permits from the semaphore.

The semaphore must be wrapped in an [`Arc`] to call this method.
If the semaphore has been closed, this returns an [`AcquireError`].
Otherwise, this returns a [`OwnedSemaphorePermit`] representing the
acquired permit.

# Cancel safety

This method uses a queue to fairly distribute permits in the order they
were requested. Cancelling a call to `acquire_many_owned` makes you lose
your place in the queue.

# Examples

```
use std::sync::Arc;
use tokio::sync::Semaphore;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let semaphore = Arc::new(Semaphore::new(10));
let mut join_handles = Vec::new();

for _ in 0..5 {
    let permit = semaphore.clone().acquire_many_owned(2).await.unwrap();
    join_handles.push(tokio::spawn(async move {
        // perform task...
        // explicitly own `permit` in the task
        drop(permit);
    }));
}

for handle in join_handles {
    handle.await.unwrap();
}
# }
```

[`Arc`]: std::sync::Arc
[`AcquireError`]: crate::sync::AcquireError
[`OwnedSemaphorePermit`]: crate::sync::OwnedSemaphorePermit

```rust
fn acquire_many_owned(self: Arc<Self>, n: u32) -> Result<OwnedSemaphorePermit, AcquireError>
```



## tokio::sync::semaphore::Semaphore::acquire_owned

*Function*

Acquires a permit from the semaphore.

The semaphore must be wrapped in an [`Arc`] to call this method.
If the semaphore has been closed, this returns an [`AcquireError`].
Otherwise, this returns a [`OwnedSemaphorePermit`] representing the
acquired permit.

# Cancel safety

This method uses a queue to fairly distribute permits in the order they
were requested. Cancelling a call to `acquire_owned` makes you lose your
place in the queue.

# Examples

```
use std::sync::Arc;
use tokio::sync::Semaphore;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let semaphore = Arc::new(Semaphore::new(3));
let mut join_handles = Vec::new();

for _ in 0..5 {
    let permit = semaphore.clone().acquire_owned().await.unwrap();
    join_handles.push(tokio::spawn(async move {
        // perform task...
        // explicitly own `permit` in the task
        drop(permit);
    }));
}

for handle in join_handles {
    handle.await.unwrap();
}
# }
```

[`Arc`]: std::sync::Arc
[`AcquireError`]: crate::sync::AcquireError
[`OwnedSemaphorePermit`]: crate::sync::OwnedSemaphorePermit

```rust
fn acquire_owned(self: Arc<Self>) -> Result<OwnedSemaphorePermit, AcquireError>
```



## tokio::sync::semaphore::Semaphore::add_permits

*Function*

Adds `n` new permits to the semaphore.

The maximum number of permits is [`Semaphore::MAX_PERMITS`], and this function will panic if the limit is exceeded.

```rust
fn add_permits(self: &Self, n: usize)
```



## tokio::sync::semaphore::Semaphore::available_permits

*Function*

Returns the current number of available permits.

```rust
fn available_permits(self: &Self) -> usize
```



## tokio::sync::semaphore::Semaphore::close

*Function*

Closes the semaphore.

This prevents the semaphore from issuing new permits and notifies all pending waiters.

# Examples

```
use tokio::sync::Semaphore;
use std::sync::Arc;
use tokio::sync::TryAcquireError;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let semaphore = Arc::new(Semaphore::new(1));
let semaphore2 = semaphore.clone();

tokio::spawn(async move {
    let permit = semaphore.acquire_many(2).await;
    assert!(permit.is_err());
    println!("waiter received error");
});

println!("closing semaphore");
semaphore2.close();

// Cannot obtain more permits
assert_eq!(semaphore2.try_acquire().err(), Some(TryAcquireError::Closed))
# }
```

```rust
fn close(self: &Self)
```



## tokio::sync::semaphore::Semaphore::forget_permits

*Function*

Decrease a semaphore's permits by a maximum of `n`.

If there are insufficient permits and it's not possible to reduce by `n`,
return the number of permits that were actually reduced.

```rust
fn forget_permits(self: &Self, n: usize) -> usize
```



## tokio::sync::semaphore::Semaphore::is_closed

*Function*

Returns true if the semaphore is closed

```rust
fn is_closed(self: &Self) -> bool
```



## tokio::sync::semaphore::Semaphore::new

*Function*

Creates a new semaphore with the initial number of permits.

Panics if `permits` exceeds [`Semaphore::MAX_PERMITS`].

```rust
fn new(permits: usize) -> Self
```



## tokio::sync::semaphore::Semaphore::try_acquire

*Function*

Tries to acquire a permit from the semaphore.

If the semaphore has been closed, this returns a [`TryAcquireError::Closed`]
and a [`TryAcquireError::NoPermits`] if there are no permits left. Otherwise,
this returns a [`SemaphorePermit`] representing the acquired permits.

# Examples

```
use tokio::sync::{Semaphore, TryAcquireError};

# fn main() {
let semaphore = Semaphore::new(2);

let permit_1 = semaphore.try_acquire().unwrap();
assert_eq!(semaphore.available_permits(), 1);

let permit_2 = semaphore.try_acquire().unwrap();
assert_eq!(semaphore.available_permits(), 0);

let permit_3 = semaphore.try_acquire();
assert_eq!(permit_3.err(), Some(TryAcquireError::NoPermits));
# }
```

[`TryAcquireError::Closed`]: crate::sync::TryAcquireError::Closed
[`TryAcquireError::NoPermits`]: crate::sync::TryAcquireError::NoPermits
[`SemaphorePermit`]: crate::sync::SemaphorePermit

```rust
fn try_acquire(self: &Self) -> Result<SemaphorePermit, TryAcquireError>
```



## tokio::sync::semaphore::Semaphore::try_acquire_many

*Function*

Tries to acquire `n` permits from the semaphore.

If the semaphore has been closed, this returns a [`TryAcquireError::Closed`]
and a [`TryAcquireError::NoPermits`] if there are not enough permits left.
Otherwise, this returns a [`SemaphorePermit`] representing the acquired permits.

# Examples

```
use tokio::sync::{Semaphore, TryAcquireError};

# fn main() {
let semaphore = Semaphore::new(4);

let permit_1 = semaphore.try_acquire_many(3).unwrap();
assert_eq!(semaphore.available_permits(), 1);

let permit_2 = semaphore.try_acquire_many(2);
assert_eq!(permit_2.err(), Some(TryAcquireError::NoPermits));
# }
```

[`TryAcquireError::Closed`]: crate::sync::TryAcquireError::Closed
[`TryAcquireError::NoPermits`]: crate::sync::TryAcquireError::NoPermits
[`SemaphorePermit`]: crate::sync::SemaphorePermit

```rust
fn try_acquire_many(self: &Self, n: u32) -> Result<SemaphorePermit, TryAcquireError>
```



