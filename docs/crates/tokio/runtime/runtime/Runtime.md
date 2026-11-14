**tokio > runtime > runtime > Runtime**

# Module: runtime::runtime::Runtime

## Contents

**Functions**

- [`block_on`](#block_on) - Runs a future to completion on the Tokio runtime. This is the
- [`enter`](#enter) - Enters the runtime context.
- [`handle`](#handle) - Returns a handle to the runtime's spawner.
- [`metrics`](#metrics) - Returns a view that lets you get information about how the runtime
- [`new`](#new) - Creates a new runtime instance with default configuration values.
- [`shutdown_background`](#shutdown_background) - Shuts down the runtime, without waiting for any spawned work to stop.
- [`shutdown_timeout`](#shutdown_timeout) - Shuts down the runtime, waiting for at most `duration` for all spawned
- [`spawn`](#spawn) - Spawns a future onto the Tokio runtime.
- [`spawn_blocking`](#spawn_blocking) - Runs the provided function on an executor dedicated to blocking operations.

---

## tokio::runtime::runtime::Runtime::block_on

*Function*

Runs a future to completion on the Tokio runtime. This is the
runtime's entry point.

This runs the given future on the current thread, blocking until it is
complete, and yielding its resolved result. Any tasks or timers
which the future spawns internally will be executed on the runtime.

# Non-worker future

Note that the future required by this function does not run as a
worker. The expectation is that other tasks are spawned by the future here.
Awaiting on other futures from the future provided here will not
perform as fast as those spawned as workers.

# Multi thread scheduler

When the multi thread scheduler is used this will allow futures
to run within the io driver and timer context of the overall runtime.

Any spawned tasks will continue running after `block_on` returns.

# Current thread scheduler

When the current thread scheduler is enabled `block_on`
can be called concurrently from multiple threads. The first call
will take ownership of the io and timer drivers. This means
other threads which do not own the drivers will hook into that one.
When the first `block_on` completes, other threads will be able to
"steal" the driver to allow continued execution of their futures.

Any spawned tasks will be suspended after `block_on` returns. Calling
`block_on` again will resume previously spawned tasks.

# Panics

This function panics if the provided future panics, or if called within an
asynchronous execution context.

# Examples

```no_run
# #[cfg(not(target_family = "wasm"))]
# {
use tokio::runtime::Runtime;

// Create the runtime
let rt  = Runtime::new().unwrap();

// Execute the future, blocking the current thread until completion
rt.block_on(async {
    println!("hello");
});
# }
```

```rust
fn block_on<F>(self: &Self, future: F) -> <F as >::Output
```



## tokio::runtime::runtime::Runtime::enter

*Function*

Enters the runtime context.

This allows you to construct types that must have an executor
available on creation such as [`Sleep`] or [`TcpStream`]. It will
also allow you to call methods such as [`tokio::spawn`].

[`Sleep`]: struct@crate::time::Sleep
[`TcpStream`]: struct@crate::net::TcpStream
[`tokio::spawn`]: fn@crate::spawn

# Example

```
# #[cfg(not(target_family = "wasm"))]
# {
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;

fn function_that_spawns(msg: String) -> JoinHandle<()> {
    // Had we not used `rt.enter` below, this would panic.
    tokio::spawn(async move {
        println!("{}", msg);
    })
}

fn main() {
    let rt = Runtime::new().unwrap();

    let s = "Hello World!".to_string();

    // By entering the context, we tie `tokio::spawn` to this executor.
    let _guard = rt.enter();
    let handle = function_that_spawns(s);

    // Wait for the task before we end the test.
    rt.block_on(handle).unwrap();
}
# }
```

```rust
fn enter(self: &Self) -> EnterGuard
```



## tokio::runtime::runtime::Runtime::handle

*Function*

Returns a handle to the runtime's spawner.

The returned handle can be used to spawn tasks that run on this runtime, and can
be cloned to allow moving the `Handle` to other threads.

Calling [`Handle::block_on`] on a handle to a `current_thread` runtime is error-prone.
Refer to the documentation of [`Handle::block_on`] for more.

# Examples

```
# #[cfg(not(target_family = "wasm"))]
# {
use tokio::runtime::Runtime;

let rt = Runtime::new()
    .unwrap();

let handle = rt.handle();

// Use the handle...
# }
```

```rust
fn handle(self: &Self) -> &Handle
```



## tokio::runtime::runtime::Runtime::metrics

*Function*

Returns a view that lets you get information about how the runtime
is performing.

```rust
fn metrics(self: &Self) -> crate::runtime::RuntimeMetrics
```



## tokio::runtime::runtime::Runtime::new

*Function*

Creates a new runtime instance with default configuration values.

This results in the multi threaded scheduler, I/O driver, and time driver being
initialized.

Most applications will not need to call this function directly. Instead,
they will use the  [`#[tokio::main]` attribute][main]. When a more complex
configuration is necessary, the [runtime builder] may be used.

See [module level][mod] documentation for more details.

# Examples

Creating a new `Runtime` with default configuration values.

```
use tokio::runtime::Runtime;

let rt = Runtime::new()
    .unwrap();

// Use the runtime...
```

# Errors

Returns an error if the I/O driver or other OS resources required by the
runtime cannot be initialized.

[mod]: index.html
[main]: ../attr.main.html
[threaded scheduler]: index.html#threaded-scheduler
[runtime builder]: crate::runtime::Builder

```rust
fn new() -> std::io::Result<Runtime>
```



## tokio::runtime::runtime::Runtime::shutdown_background

*Function*

Shuts down the runtime, without waiting for any spawned work to stop.

This can be useful if you want to drop a runtime from within another runtime.
Normally, dropping a runtime will block indefinitely for spawned blocking tasks
to complete, which would normally not be permitted within an asynchronous context.
By calling `shutdown_background()`, you can drop the runtime from such a context.

Note however, that because we do not wait for any blocking tasks to complete, this
may result in a resource leak (in that any blocking tasks are still running until they
return.

See the [struct level documentation](Runtime#shutdown) for more details.

This function is equivalent to calling `shutdown_timeout(Duration::from_nanos(0))`.

```
# #[cfg(not(target_family = "wasm"))]
# {
use tokio::runtime::Runtime;

fn main() {
   let runtime = Runtime::new().unwrap();

   runtime.block_on(async move {
       let inner_runtime = Runtime::new().unwrap();
       // ...
       inner_runtime.shutdown_background();
   });
}
# }
```

```rust
fn shutdown_background(self: Self)
```



## tokio::runtime::runtime::Runtime::shutdown_timeout

*Function*

Shuts down the runtime, waiting for at most `duration` for all spawned
work to stop.

See the [struct level documentation](Runtime#shutdown) for more details.

# Examples

```
# #[cfg(not(target_family = "wasm"))]
# {
use tokio::runtime::Runtime;
use tokio::task;

use std::thread;
use std::time::Duration;

fn main() {
   let runtime = Runtime::new().unwrap();

   runtime.block_on(async move {
       task::spawn_blocking(move || {
           thread::sleep(Duration::from_secs(10_000));
       });
   });

   runtime.shutdown_timeout(Duration::from_millis(100));
}
# }
```

```rust
fn shutdown_timeout(self: Self, duration: Duration)
```



## tokio::runtime::runtime::Runtime::spawn

*Function*

Spawns a future onto the Tokio runtime.

This spawns the given future onto the runtime's executor, usually a
thread pool. The thread pool is then responsible for polling the future
until it completes.

The provided future will start running in the background immediately
when `spawn` is called, even if you don't await the returned
`JoinHandle` (assuming that the runtime [is running][running-runtime]).

See [module level][mod] documentation for more details.

[mod]: index.html
[running-runtime]: index.html#driving-the-runtime

# Examples

```
# #[cfg(not(target_family = "wasm"))]
# {
use tokio::runtime::Runtime;

# fn dox() {
// Create the runtime
let rt = Runtime::new().unwrap();

// Spawn a future onto the runtime
rt.spawn(async {
    println!("now running on a worker thread");
});
# }
# }
```

```rust
fn spawn<F>(self: &Self, future: F) -> JoinHandle<<F as >::Output>
```



## tokio::runtime::runtime::Runtime::spawn_blocking

*Function*

Runs the provided function on an executor dedicated to blocking operations.

# Examples

```
# #[cfg(not(target_family = "wasm"))]
# {
use tokio::runtime::Runtime;

# fn dox() {
// Create the runtime
let rt = Runtime::new().unwrap();

// Spawn a blocking function onto the runtime
rt.spawn_blocking(|| {
    println!("now running on a worker thread");
});
# }
# }
```

```rust
fn spawn_blocking<F, R>(self: &Self, func: F) -> JoinHandle<R>
```



