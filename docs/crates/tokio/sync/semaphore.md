**tokio > sync > semaphore**

# Module: sync::semaphore

## Contents

**Structs**

- [`OwnedSemaphorePermit`](#ownedsemaphorepermit) - An owned permit from the semaphore.
- [`Semaphore`](#semaphore) - Counting semaphore performing asynchronous permit acquisition.
- [`SemaphorePermit`](#semaphorepermit) - A permit from the semaphore.

---

## tokio::sync::semaphore::OwnedSemaphorePermit

*Struct*

An owned permit from the semaphore.

This type is created by the [`acquire_owned`] method.

[`acquire_owned`]: crate::sync::Semaphore::acquire_owned()

**Methods:**

- `fn forget(self: Self)` - Forgets the permit **without** releasing it back to the semaphore.
- `fn merge(self: & mut Self, other: Self)` - Merge two [`OwnedSemaphorePermit`] instances together, consuming `other`
- `fn split(self: & mut Self, n: usize) -> Option<Self>` - Splits `n` permits from `self` and returns a new [`OwnedSemaphorePermit`] instance that holds `n` permits.
- `fn semaphore(self: &Self) -> &Arc<Semaphore>` - Returns the [`Semaphore`] from which this permit was acquired.
- `fn num_permits(self: &Self) -> usize` - Returns the number of permits held by `self`.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Drop**
  - `fn drop(self: & mut Self)`



## tokio::sync::semaphore::Semaphore

*Struct*

Counting semaphore performing asynchronous permit acquisition.

A semaphore maintains a set of permits. Permits are used to synchronize
access to a shared resource. A semaphore differs from a mutex in that it
can allow more than one concurrent caller to access the shared resource at a
time.

When `acquire` is called and the semaphore has remaining permits, the
function immediately returns a permit. However, if no remaining permits are
available, `acquire` (asynchronously) waits until an outstanding permit is
dropped. At this point, the freed permit is assigned to the caller.

This `Semaphore` is fair, which means that permits are given out in the order
they were requested. This fairness is also applied when `acquire_many` gets
involved, so if a call to `acquire_many` at the front of the queue requests
more permits than currently available, this can prevent a call to `acquire`
from completing, even if the semaphore has enough permits to complete the call
to `acquire`.

To use the `Semaphore` in a poll function, you can use the [`PollSemaphore`]
utility.

# Memory ordering

If a task writes some data and then releases a permit, any task that later
acquires a permit is guaranteed to see that data. This makes it safe to use
a semaphore to hand data off between tasks through shared state.

Stated more precisely in terms of atomic memory orderings: acquiring a
permit (via [`acquire`], [`acquire_many`], [`try_acquire`],
[`try_acquire_many`], or their `_owned` variants), releasing permits (by
dropping a [`SemaphorePermit`] or [`OwnedSemaphorePermit`], or by calling
[`add_permits`] or [`forget_permits`]), and closing the semaphore (via
[`close`]) are all `AcqRel` operations. They are totally ordered, and each
one synchronizes-with all such operations that precede it, giving the same
guarantees as `AcqRel` operations on a single atomic.

A failed acquisition attempt (including [`TryAcquireError::NoPermits`] and
[`TryAcquireError::Closed`]), along with the [`available_permits`] and
[`is_closed`] methods, behave like an `Acquire` load.

[`acquire`]: Semaphore::acquire
[`acquire_many`]: Semaphore::acquire_many
[`try_acquire`]: Semaphore::try_acquire
[`try_acquire_many`]: Semaphore::try_acquire_many
[`add_permits`]: Semaphore::add_permits
[`forget_permits`]: Semaphore::forget_permits
[`close`]: Semaphore::close
[`available_permits`]: Semaphore::available_permits
[`is_closed`]: Semaphore::is_closed

# Examples

Basic usage:

```
use tokio::sync::{Semaphore, TryAcquireError};

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let semaphore = Semaphore::new(3);

let a_permit = semaphore.acquire().await.unwrap();
let two_permits = semaphore.acquire_many(2).await.unwrap();

assert_eq!(semaphore.available_permits(), 0);

let permit_attempt = semaphore.try_acquire();
assert_eq!(permit_attempt.err(), Some(TryAcquireError::NoPermits));
# }
```

## Limit the number of simultaneously opened files in your program

Most operating systems have limits on the number of open file
handles. Even in systems without explicit limits, resource constraints
implicitly set an upper bound on the number of open files. If your
program attempts to open a large number of files and exceeds this
limit, it will result in an error.

This example uses a Semaphore with 100 permits. By acquiring a permit from
the Semaphore before accessing a file, you ensure that your program opens
no more than 100 files at a time. When trying to open the 101st
file, the program will wait until a permit becomes available before
proceeding to open another file.
```
# #[cfg(not(target_family = "wasm"))]
# {
use std::io::Result;
use tokio::fs::File;
use tokio::sync::Semaphore;
use tokio::io::AsyncWriteExt;

static PERMITS: Semaphore = Semaphore::const_new(100);

async fn write_to_file(message: &[u8]) -> Result<()> {
    let _permit = PERMITS.acquire().await.unwrap();
    let mut buffer = File::create("example.txt").await?;
    buffer.write_all(message).await?;
    Ok(()) // Permit goes out of scope here, and is available again for acquisition
}
# }
```

## Limit the number of outgoing requests being sent at the same time

In some scenarios, it might be required to limit the number of outgoing
requests being sent in parallel. This could be due to limits of a consumed
API or the network resources of the system the application is running on.

This example uses an `Arc<Semaphore>` with 10 permits. Each task spawned is
given a reference to the semaphore by cloning the `Arc<Semaphore>`. Before
a task sends a request, it must acquire a permit from the semaphore by
calling [`Semaphore::acquire`]. This ensures that at most 10 requests are
sent in parallel at any given time. After a task has sent a request, it
drops the permit to allow other tasks to send requests.

```
use std::sync::Arc;
use tokio::sync::Semaphore;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
// Define maximum number of parallel requests.
let semaphore = Arc::new(Semaphore::new(5));
// Spawn many tasks that will send requests.
let mut jhs = Vec::new();
for task_id in 0..50 {
    let semaphore = semaphore.clone();
    let jh = tokio::spawn(async move {
        // Acquire permit before sending request.
        let _permit = semaphore.acquire().await.unwrap();
        // Send the request.
        let response = send_request(task_id).await;
        // Drop the permit after the request has been sent.
        drop(_permit);
        // Handle response.
        // ...

        response
    });
    jhs.push(jh);
}
// Collect responses from tasks.
let mut responses = Vec::new();
for jh in jhs {
    let response = jh.await.unwrap();
    responses.push(response);
}
// Process responses.
// ...
# }
# async fn send_request(task_id: usize) {
#     // Send request.
# }
```

## Limit the number of incoming requests being handled at the same time

Similar to limiting the number of simultaneously opened files, network handles
are a limited resource. Allowing an unbounded amount of requests to be processed
could result in a denial-of-service, among many other issues.

This example uses an `Arc<Semaphore>` instead of a global variable.
To limit the number of requests that can be processed at the time,
we acquire a permit for each task before spawning it. Once acquired,
a new task is spawned; and once finished, the permit is dropped inside
of the task to allow others to spawn. Permits must be acquired via
[`Semaphore::acquire_owned`] to be movable across the task boundary.
(Since our semaphore is not a global variable — if it was, then `acquire` would be enough.)

```no_run
# #[cfg(not(target_family = "wasm"))]
# {
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let semaphore = Arc::new(Semaphore::new(3));
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        // Acquire permit before accepting the next socket.
        //
        // We use `acquire_owned` so that we can move `permit` into
        // other tasks.
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            // Do work using the socket.
            handle_connection(&mut socket).await;
            // Drop socket while the permit is still live.
            drop(socket);
            // Drop the permit, so more tasks can be created.
            drop(permit);
        });
    }
}
# async fn handle_connection(_socket: &mut tokio::net::TcpStream) {
#   // Do work
# }
# }
```

## Prevent tests from running in parallel

By default, Rust runs tests in the same file in parallel. However, in some
cases, running two tests in parallel may lead to problems. For example, this
can happen when tests use the same database.

Consider the following scenario:
1. `test_insert`: Inserts a key-value pair into the database, then retrieves
   the value using the same key to verify the insertion.
2. `test_update`: Inserts a key, then updates the key to a new value and
   verifies that the value has been accurately updated.
3. `test_others`: A third test that doesn't modify the database state. It
   can run in parallel with the other tests.

In this example, `test_insert` and `test_update` need to run in sequence to
work, but it doesn't matter which test runs first. We can leverage a
semaphore with a single permit to address this challenge.

```
# use tokio::sync::Mutex;
# use std::collections::BTreeMap;
# struct Database {
#   map: Mutex<BTreeMap<String, i32>>,
# }
# impl Database {
#    pub const fn setup() -> Database {
#        Database {
#            map: Mutex::const_new(BTreeMap::new()),
#        }
#    }
#    pub async fn insert(&self, key: &str, value: i32) {
#        self.map.lock().await.insert(key.to_string(), value);
#    }
#    pub async fn update(&self, key: &str, value: i32) {
#        self.map.lock().await
#            .entry(key.to_string())
#            .and_modify(|origin| *origin = value);
#    }
#    pub async fn delete(&self, key: &str) {
#        self.map.lock().await.remove(key);
#    }
#    pub async fn get(&self, key: &str) -> i32 {
#        *self.map.lock().await.get(key).unwrap()
#    }
# }
use tokio::sync::Semaphore;

// Initialize a static semaphore with only one permit, which is used to
// prevent test_insert and test_update from running in parallel.
static PERMIT: Semaphore = Semaphore::const_new(1);

// Initialize the database that will be used by the subsequent tests.
static DB: Database = Database::setup();

#[tokio::test]
# async fn fake_test_insert() {}
async fn test_insert() {
    // Acquire permit before proceeding. Since the semaphore has only one permit,
    // the test will wait if the permit is already acquired by other tests.
    let permit = PERMIT.acquire().await.unwrap();

    // Do the actual test stuff with database

    // Insert a key-value pair to database
    let (key, value) = ("name", 0);
    DB.insert(key, value).await;

    // Verify that the value has been inserted correctly.
    assert_eq!(DB.get(key).await, value);

    // Undo the insertion, so the database is empty at the end of the test.
    DB.delete(key).await;

    // Drop permit. This allows the other test to start running.
    drop(permit);
}

#[tokio::test]
# async fn fake_test_update() {}
async fn test_update() {
    // Acquire permit before proceeding. Since the semaphore has only one permit,
    // the test will wait if the permit is already acquired by other tests.
    let permit = PERMIT.acquire().await.unwrap();

    // Do the same insert.
    let (key, value) = ("name", 0);
    DB.insert(key, value).await;

    // Update the existing value with a new one.
    let new_value = 1;
    DB.update(key, new_value).await;

    // Verify that the value has been updated correctly.
    assert_eq!(DB.get(key).await, new_value);

    // Undo any modificattion.
    DB.delete(key).await;

    // Drop permit. This allows the other test to start running.
    drop(permit);
}

#[tokio::test]
# async fn fake_test_others() {}
async fn test_others() {
    // This test can run in parallel with test_insert and test_update,
    // so it does not use PERMIT.
}
# #[tokio::main(flavor = "current_thread")]
# async fn main() {
#   test_insert().await;
#   test_update().await;
#   test_others().await;
# }
```

## Rate limiting using a token bucket

This example showcases the [`add_permits`] and [`SemaphorePermit::forget`] methods.

Many applications and systems have constraints on the rate at which certain
operations should occur. Exceeding this rate can result in suboptimal
performance or even errors.

This example implements rate limiting using a [token bucket]. A token bucket is a form of rate
limiting that doesn't kick in immediately, to allow for short bursts of incoming requests that
arrive at the same time.

With a token bucket, each incoming request consumes a token, and the tokens are refilled at a
certain rate that defines the rate limit. When a burst of requests arrives, tokens are
immediately given out until the bucket is empty. Once the bucket is empty, requests will have to
wait for new tokens to be added.

Unlike the example that limits how many requests can be handled at the same time, we do not add
tokens back when we finish handling a request. Instead, tokens are added only by a timer task.

Note that this implementation is suboptimal when the duration is small, because it consumes a
lot of cpu constantly looping and sleeping.

[token bucket]: https://en.wikipedia.org/wiki/Token_bucket
[`add_permits`]: crate::sync::Semaphore::add_permits
[`SemaphorePermit::forget`]: crate::sync::SemaphorePermit::forget
```
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::{interval, Duration};

struct TokenBucket {
    sem: Arc<Semaphore>,
    jh: tokio::task::JoinHandle<()>,
}

impl TokenBucket {
    fn new(duration: Duration, capacity: usize) -> Self {
        let sem = Arc::new(Semaphore::new(capacity));

        // refills the tokens at the end of each interval
        let jh = tokio::spawn({
            let sem = sem.clone();
            let mut interval = interval(duration);
            interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

            async move {
                loop {
                    interval.tick().await;

                    if sem.available_permits() < capacity {
                        sem.add_permits(1);
                    }
                }
            }
        });

        Self { jh, sem }
    }

    async fn acquire(&self) {
        // This can return an error if the semaphore is closed, but we
        // never close it, so this error can never happen.
        let permit = self.sem.acquire().await.unwrap();
        // To avoid releasing the permit back to the semaphore, we use
        // the `SemaphorePermit::forget` method.
        permit.forget();
    }
}

impl Drop for TokenBucket {
    fn drop(&mut self) {
        // Kill the background task so it stops taking up resources when we
        // don't need it anymore.
        self.jh.abort();
    }
}

# #[tokio::main(flavor = "current_thread")]
# async fn _hidden() {}
# #[tokio::main(flavor = "current_thread", start_paused = true)]
# async fn main() {
let capacity = 5;
let update_interval = Duration::from_secs_f32(1.0 / capacity as f32);
let bucket = TokenBucket::new(update_interval, capacity);

for _ in 0..5 {
    bucket.acquire().await;

    // do the operation
}
# }
```

[`PollSemaphore`]: https://docs.rs/tokio-util/latest/tokio_util/sync/struct.PollSemaphore.html
[`Semaphore::acquire_owned`]: crate::sync::Semaphore::acquire_owned

**Methods:**

- `fn new(permits: usize) -> Self` - Creates a new semaphore with the initial number of permits.
- `fn const_new(permits: usize) -> Self` - Creates a new semaphore with the initial number of permits.
- `fn available_permits(self: &Self) -> usize` - Returns the current number of available permits.
- `fn add_permits(self: &Self, n: usize)` - Adds `n` new permits to the semaphore.
- `fn forget_permits(self: &Self, n: usize) -> usize` - Decrease a semaphore's permits by a maximum of `n`.
- `fn acquire(self: &Self) -> Result<SemaphorePermit, AcquireError>` - Acquires a permit from the semaphore.
- `fn acquire_many(self: &Self, n: u32) -> Result<SemaphorePermit, AcquireError>` - Acquires `n` permits from the semaphore.
- `fn try_acquire(self: &Self) -> Result<SemaphorePermit, TryAcquireError>` - Tries to acquire a permit from the semaphore.
- `fn try_acquire_many(self: &Self, n: u32) -> Result<SemaphorePermit, TryAcquireError>` - Tries to acquire `n` permits from the semaphore.
- `fn blocking_acquire(self: &Self) -> Result<SemaphorePermit, AcquireError>` - Acquires a permit from the semaphore, blocking the current thread until
- `fn blocking_acquire_many(self: &Self, n: u32) -> Result<SemaphorePermit, AcquireError>` - Acquires `n` permits from the semaphore, blocking the current thread
- `fn acquire_owned(self: Arc<Self>) -> Result<OwnedSemaphorePermit, AcquireError>` - Acquires a permit from the semaphore.
- `fn acquire_many_owned(self: Arc<Self>, n: u32) -> Result<OwnedSemaphorePermit, AcquireError>` - Acquires `n` permits from the semaphore.
- `fn try_acquire_owned(self: Arc<Self>) -> Result<OwnedSemaphorePermit, TryAcquireError>` - Tries to acquire a permit from the semaphore.
- `fn try_acquire_many_owned(self: Arc<Self>, n: u32) -> Result<OwnedSemaphorePermit, TryAcquireError>` - Tries to acquire `n` permits from the semaphore.
- `fn blocking_acquire_owned(self: Arc<Self>) -> Result<OwnedSemaphorePermit, AcquireError>` - Acquires a permit from the semaphore, blocking the current thread until
- `fn blocking_acquire_many_owned(self: Arc<Self>, n: u32) -> Result<OwnedSemaphorePermit, AcquireError>` - Acquires `n` permits from the semaphore, blocking the current thread
- `fn close(self: &Self)` - Closes the semaphore.
- `fn is_closed(self: &Self) -> bool` - Returns true if the semaphore is closed

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## tokio::sync::semaphore::SemaphorePermit

*Struct*

A permit from the semaphore.

This type is created by the [`acquire`] method.

[`acquire`]: crate::sync::Semaphore::acquire()

**Generic Parameters:**
- 'a

**Methods:**

- `fn forget(self: Self)` - Forgets the permit **without** releasing it back to the semaphore.
- `fn merge(self: & mut Self, other: Self)` - Merge two [`SemaphorePermit`] instances together, consuming `other`
- `fn split(self: & mut Self, n: usize) -> Option<Self>` - Splits `n` permits from `self` and returns a new [`SemaphorePermit`] instance that holds `n` permits.
- `fn semaphore(self: &Self) -> &Semaphore` - Returns the [`Semaphore`] from which this permit was acquired.
- `fn num_permits(self: &Self) -> usize` - Returns the number of permits held by `self`.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Drop**
  - `fn drop(self: & mut Self)`



