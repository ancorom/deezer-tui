**tokio > runtime > handle > Handle**

# Module: runtime::handle::Handle

## Contents

**Functions**

- [`block_on`](#block_on) - Runs a future to completion on this `Handle`'s associated `Runtime`.
- [`current`](#current) - Returns a `Handle` view over the currently running `Runtime`.
- [`enter`](#enter) - Enters the runtime context. This allows you to construct types that must
- [`runtime_flavor`](#runtime_flavor) - Returns the flavor of the current `Runtime`.
- [`spawn_blocking`](#spawn_blocking) - Runs the provided function on an executor dedicated to blocking

---

## tokio::runtime::handle::Handle::block_on

*Function*

Runs a future to completion on this `Handle`'s associated `Runtime`.

This runs the given future on the current thread, blocking until it is
complete, and yielding its resolved result. Any tasks or timers which
the future spawns internally will be executed on the runtime.

When this is used on a `current_thread` runtime, only the
[`Runtime::block_on`] method can drive the IO and timer drivers, but the
`Handle::block_on` method cannot drive them. This means that, when using
this method on a `current_thread` runtime, anything that relies on IO or
timers will not work unless there is another thread currently calling
[`Runtime::block_on`] on the same runtime.

# If the runtime has been shut down

If the `Handle`'s associated `Runtime` has been shut down (through
[`Runtime::shutdown_background`], [`Runtime::shutdown_timeout`], or by
dropping it) and `Handle::block_on` is used it might return an error or
panic. Specifically IO resources will return an error and timers will
panic. Runtime independent futures will run as normal.

# Panics

This function will panic if any of the following conditions are met:
- The provided future panics.
- It is called from within an asynchronous context, such as inside
  [`Runtime::block_on`], `Handle::block_on`, or from a function annotated
  with [`tokio::main`].
- A timer future is executed on a runtime that has been shut down.

# Examples

```
# #[cfg(not(target_family = "wasm"))]
# {
use tokio::runtime::Runtime;

// Create the runtime
let rt  = Runtime::new().unwrap();

// Get a handle from this runtime
let handle = rt.handle();

// Execute the future, blocking the current thread until completion
handle.block_on(async {
    println!("hello");
});
# }
```

Or using `Handle::current`:

```
# #[cfg(not(target_family = "wasm"))]
# {
use tokio::runtime::Handle;

#[tokio::main]
async fn main () {
    let handle = Handle::current();
    std::thread::spawn(move || {
        // Using Handle::block_on to run async code in the new thread.
        handle.block_on(async {
            println!("hello");
        });
    });
}
# }
```

`Handle::block_on` may be combined with [`task::block_in_place`] to
re-enter the async context of a multi-thread scheduler runtime:
```
# #[cfg(not(target_family = "wasm"))]
# {
use tokio::task;
use tokio::runtime::Handle;

# async fn docs() {
task::block_in_place(move || {
    Handle::current().block_on(async move {
        // do something async
    });
});
# }
# }
```

[`JoinError`]: struct@crate::task::JoinError
[`JoinHandle`]: struct@crate::task::JoinHandle
[`Runtime::block_on`]: fn@crate::runtime::Runtime::block_on
[`Runtime::shutdown_background`]: fn@crate::runtime::Runtime::shutdown_background
[`Runtime::shutdown_timeout`]: fn@crate::runtime::Runtime::shutdown_timeout
[`spawn_blocking`]: crate::task::spawn_blocking
[`tokio::fs`]: crate::fs
[`tokio::net`]: crate::net
[`tokio::time`]: crate::time
[`tokio::main`]: ../attr.main.html
[`task::block_in_place`]: crate::task::block_in_place

```rust
fn block_on<F>(self: &Self, future: F) -> <F as >::Output
```



## tokio::runtime::handle::Handle::current

*Function*

Returns a `Handle` view over the currently running `Runtime`.

# Panics

This will panic if called outside the context of a Tokio runtime. That means that you must
call this on one of the threads **being run by the runtime**, or from a thread with an active
`EnterGuard`. Calling this from within a thread created by `std::thread::spawn` (for example)
will cause a panic unless that thread has an active `EnterGuard`.

# Examples

This can be used to obtain the handle of the surrounding runtime from an async
block or function running on that runtime.

```
# #[cfg(not(target_family = "wasm"))]
# {
# use std::thread;
# use tokio::runtime::Runtime;
# fn dox() {
# let rt = Runtime::new().unwrap();
# rt.spawn(async {
use tokio::runtime::Handle;

// Inside an async block or function.
let handle = Handle::current();
handle.spawn(async {
    println!("now running in the existing Runtime");
});

# let handle =
thread::spawn(move || {
    // Notice that the handle is created outside of this thread and then moved in
    handle.spawn(async { /* ... */ });
    // This next line would cause a panic because we haven't entered the runtime
    // and created an EnterGuard
    // let handle2 = Handle::current(); // panic
    // So we create a guard here with Handle::enter();
    let _guard = handle.enter();
    // Now we can call Handle::current();
    let handle2 = Handle::current();
});
# handle.join().unwrap();
# });
# }
# }
```

```rust
fn current() -> Self
```



## tokio::runtime::handle::Handle::enter

*Function*

Enters the runtime context. This allows you to construct types that must
have an executor available on creation such as [`Sleep`] or
[`TcpStream`]. It will also allow you to call methods such as
[`tokio::spawn`] and [`Handle::current`] without panicking.

# Panics

When calling `Handle::enter` multiple times, the returned guards
**must** be dropped in the reverse order that they were acquired.
Failure to do so will result in a panic and possible memory leaks.

# Examples

```
# #[cfg(not(target_family = "wasm"))]
# {
use tokio::runtime::Runtime;

let rt = Runtime::new().unwrap();

let _guard = rt.enter();
tokio::spawn(async {
    println!("Hello world!");
});
# }
```

Do **not** do the following, this shows a scenario that will result in a
panic and possible memory leak.

```should_panic,ignore-wasm
use tokio::runtime::Runtime;

let rt1 = Runtime::new().unwrap();
let rt2 = Runtime::new().unwrap();

let enter1 = rt1.enter();
let enter2 = rt2.enter();

drop(enter1);
drop(enter2);
```

[`Sleep`]: struct@crate::time::Sleep
[`TcpStream`]: struct@crate::net::TcpStream
[`tokio::spawn`]: fn@crate::spawn

```rust
fn enter(self: &Self) -> EnterGuard
```



## tokio::runtime::handle::Handle::runtime_flavor

*Function*

Returns the flavor of the current `Runtime`.

# Examples

```
use tokio::runtime::{Handle, RuntimeFlavor};

#[tokio::main(flavor = "current_thread")]
async fn main() {
  assert_eq!(RuntimeFlavor::CurrentThread, Handle::current().runtime_flavor());
}
```

```
# #[cfg(not(target_family = "wasm"))]
# {
use tokio::runtime::{Handle, RuntimeFlavor};

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
  assert_eq!(RuntimeFlavor::MultiThread, Handle::current().runtime_flavor());
}
# }
```

```rust
fn runtime_flavor(self: &Self) -> RuntimeFlavor
```



## tokio::runtime::handle::Handle::spawn_blocking

*Function*

Runs the provided function on an executor dedicated to blocking
operations.

# Examples

```
# #[cfg(not(target_family = "wasm"))]
# {
use tokio::runtime::Runtime;

# fn dox() {
// Create the runtime
let rt = Runtime::new().unwrap();
// Get a handle from this runtime
let handle = rt.handle();

// Spawn a blocking function onto the runtime using the handle
handle.spawn_blocking(|| {
    println!("now running on a worker thread");
});
# }
# }
```

```rust
fn spawn_blocking<F, R>(self: &Self, func: F) -> JoinHandle<R>
```



