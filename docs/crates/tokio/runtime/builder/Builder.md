**tokio > runtime > builder > Builder**

# Module: runtime::builder::Builder

## Contents

**Functions**

- [`build`](#build) - Creates the configured `Runtime`.
- [`build_local`](#build_local) - Creates the configured [`LocalRuntime`].
- [`enable_all`](#enable_all) - Enables both I/O and time drivers.
- [`enable_io`](#enable_io) - Enables the I/O driver.
- [`enable_time`](#enable_time) - Enables the time driver.
- [`event_interval`](#event_interval) - Sets the number of scheduler ticks after which the scheduler will poll for
- [`global_queue_interval`](#global_queue_interval) - Sets the number of scheduler ticks after which the scheduler will poll the global
- [`max_io_events_per_busy_tick`](#max_io_events_per_busy_tick) - Sets the max number of I/O events a worker processes when it polls
- [`max_io_events_per_tick`](#max_io_events_per_tick) - Sets the max number of I/O events processed per tick.
- [`new_current_thread`](#new_current_thread) - Returns a new builder with the current thread scheduler selected.
- [`new_multi_thread`](#new_multi_thread) - Returns a new builder with the multi thread scheduler selected.
- [`thread_keep_alive`](#thread_keep_alive) - Sets a custom timeout for a thread in the blocking pool.
- [`worker_threads`](#worker_threads) - Sets the number of worker threads the `Runtime` will use.

---

## tokio::runtime::builder::Builder::build

*Function*

Creates the configured `Runtime`.

The returned `Runtime` instance is ready to spawn tasks.

# Examples

```
# #[cfg(not(target_family = "wasm"))]
# {
use tokio::runtime::Builder;

let rt  = Builder::new_multi_thread().build().unwrap();

rt.block_on(async {
    println!("Hello from the Tokio runtime");
});
# }
```

# Errors

Returns an error if the I/O driver or other OS resources required by the
runtime cannot be initialized.

```rust
fn build(self: & mut Self) -> io::Result<Runtime>
```



## tokio::runtime::builder::Builder::build_local

*Function*

Creates the configured [`LocalRuntime`].

The returned [`LocalRuntime`] instance is ready to spawn tasks.

# Panics

This will panic if the runtime is configured with [`new_multi_thread()`].

[`new_multi_thread()`]: Builder::new_multi_thread

# Examples

```
use tokio::runtime::{Builder, LocalOptions};

let rt = Builder::new_current_thread()
    .build_local(LocalOptions::default())
    .unwrap();

rt.spawn_local(async {
    println!("Hello from the Tokio runtime");
});
```

# Errors

Returns an error if the I/O driver or other OS resources required by the
runtime cannot be initialized.

```rust
fn build_local(self: & mut Self, _options: LocalOptions) -> io::Result<LocalRuntime>
```



## tokio::runtime::builder::Builder::enable_all

*Function*

Enables both I/O and time drivers.

Doing this is a shorthand for calling `enable_io` and `enable_time`
individually. If additional components are added to Tokio in the future,
`enable_all` will include these future components.

# Examples

```
# #[cfg(not(target_family = "wasm"))]
# {
use tokio::runtime;

let rt = runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap();
# }
```

```rust
fn enable_all(self: & mut Self) -> & mut Self
```



## tokio::runtime::builder::Builder::enable_io

*Function*

Enables the I/O driver.

Doing this enables using net, process, signal, and some I/O types on
the runtime.

# Examples

```
use tokio::runtime;

let rt = runtime::Builder::new_current_thread()
    .enable_io()
    .build()
    .unwrap();
```

```rust
fn enable_io(self: & mut Self) -> & mut Self
```



## tokio::runtime::builder::Builder::enable_time

*Function*

Enables the time driver.

Doing this enables using `tokio::time` on the runtime.

# Examples

```
# #[cfg(not(target_family = "wasm"))]
# {
use tokio::runtime;

let rt = runtime::Builder::new_multi_thread()
    .enable_time()
    .build()
    .unwrap();
# }
```

```rust
fn enable_time(self: & mut Self) -> & mut Self
```



## tokio::runtime::builder::Builder::event_interval

*Function*

Sets the number of scheduler ticks after which the scheduler will poll for
external events (timers, I/O, and so on).

A scheduler "tick" roughly corresponds to one `poll` invocation on a task.

By default, the event interval is `61` for all scheduler types.

Setting the event interval determines the effective "priority" of delivering
these external events (which may wake up additional tasks), compared to
executing tasks that are currently ready to run. A smaller value is useful
when tasks frequently spend a long time in polling, or infrequently yield,
which can result in overly long delays picking up I/O events. Conversely,
picking up new events requires extra synchronization and syscall overhead,
so if tasks generally complete their polling quickly, a higher event interval
will minimize that overhead while still keeping the scheduler responsive to
events.

# Panics

This function will panic if 0 is passed as an argument.

# Examples

```
# #[cfg(not(target_family = "wasm"))]
# {
# use tokio::runtime;
# pub fn main() {
let rt = runtime::Builder::new_multi_thread()
    .event_interval(31)
    .build();
# }
# }
```

```rust
fn event_interval(self: & mut Self, val: u32) -> & mut Self
```



## tokio::runtime::builder::Builder::global_queue_interval

*Function*

Sets the number of scheduler ticks after which the scheduler will poll the global
task queue.

A scheduler "tick" roughly corresponds to one `poll` invocation on a task.

By default the global queue interval is 31 for the current-thread scheduler. Please see
[the module documentation] for the default behavior of the multi-thread scheduler.

Schedulers have a local queue of already-claimed tasks, and a global queue of incoming
tasks. Setting the interval to a smaller value increases the fairness of the scheduler,
at the cost of more synchronization overhead. That can be beneficial for prioritizing
getting started on new work, especially if tasks frequently yield rather than complete
or await on further I/O. Setting the interval to `1` will prioritize the global queue and
tasks from the local queue will be executed only if the global queue is empty.
Conversely, a higher value prioritizes existing work, and is a good choice when most
tasks quickly complete polling.

[the module documentation]: crate::runtime#multi-threaded-runtime-behavior-at-the-time-of-writing

# Panics

This function will panic if 0 is passed as an argument.

# Examples

```
# #[cfg(not(target_family = "wasm"))]
# {
# use tokio::runtime;
# pub fn main() {
let rt = runtime::Builder::new_multi_thread()
    .global_queue_interval(31)
    .build();
# }
# }
```

```rust
fn global_queue_interval(self: & mut Self, val: u32) -> & mut Self
```



## tokio::runtime::builder::Builder::max_io_events_per_busy_tick

*Function*

Sets the max number of I/O events a worker processes when it polls
the driver while it still has tasks to run.

A busy worker polls the driver every [`event_interval`] tasks, and
every task that poll wakes goes to its local queue. A large batch
overflows that queue, and under sustained overload the overflow
grows until requests time out. A small busy batch leaves the rest
in the kernel. An idle worker still takes up to
[`max_io_events_per_tick`] events.

The runtime treats any poll that does not wait as busy. That
includes a park with a timer that has already expired, because the
worker runs that timer's task next.

A multi-thread worker's local queue holds 256 tasks, so set
`max_io_events_per_tick` to at most 256 as well, with room for the
tasks those tasks wake.

The default is to use the same value as [`max_io_events_per_tick`].

[`event_interval`]: Builder::event_interval
[`max_io_events_per_tick`]: Builder::max_io_events_per_tick

# Panics

Panics if `capacity` is zero.

# Examples

```
use tokio::runtime;

let rt = runtime::Builder::new_current_thread()
    .enable_io()
    .max_io_events_per_tick(128)
    .max_io_events_per_busy_tick(8)
    .build()
    .unwrap();
```

```rust
fn max_io_events_per_busy_tick(self: & mut Self, capacity: usize) -> & mut Self
```



## tokio::runtime::builder::Builder::max_io_events_per_tick

*Function*

Sets the max number of I/O events processed per tick.

To take a smaller batch on polls that do not wait, see
[`max_io_events_per_busy_tick`].

[`max_io_events_per_busy_tick`]: Builder::max_io_events_per_busy_tick

# Panics

Panics if `capacity` is zero.

# Examples

```
use tokio::runtime;

let rt = runtime::Builder::new_current_thread()
    .enable_io()
    .max_io_events_per_tick(1024)
    .build()
    .unwrap();
```

```rust
fn max_io_events_per_tick(self: & mut Self, capacity: usize) -> & mut Self
```



## tokio::runtime::builder::Builder::new_current_thread

*Function*

Returns a new builder with the current thread scheduler selected.

Configuration methods can be chained on the return value.

To spawn non-`Send` tasks on the resulting runtime, combine it with a
[`LocalSet`], or call [`build_local`] to create a [`LocalRuntime`].

[`LocalSet`]: crate::task::LocalSet
[`LocalRuntime`]: crate::runtime::LocalRuntime
[`build_local`]: crate::runtime::Builder::build_local

```rust
fn new_current_thread() -> Builder
```



## tokio::runtime::builder::Builder::new_multi_thread

*Function*

Returns a new builder with the multi thread scheduler selected.

Configuration methods can be chained on the return value.

```rust
fn new_multi_thread() -> Builder
```



## tokio::runtime::builder::Builder::thread_keep_alive

*Function*

Sets a custom timeout for a thread in the blocking pool.

By default, the timeout for a thread is set to 10 seconds. This can
be overridden using `.thread_keep_alive()`.

# Example

```
# #[cfg(not(target_family = "wasm"))]
# {
# use tokio::runtime;
# use std::time::Duration;
# pub fn main() {
let rt = runtime::Builder::new_multi_thread()
    .thread_keep_alive(Duration::from_millis(100))
    .build();
# }
# }
```

```rust
fn thread_keep_alive(self: & mut Self, duration: Duration) -> & mut Self
```



## tokio::runtime::builder::Builder::worker_threads

*Function*

Sets the number of worker threads the `Runtime` will use.

This can be any number above 0 though it is advised to keep this value
on the smaller side.

This will override the value read from environment variable `TOKIO_WORKER_THREADS`.

# Default

The default value is the number of cores available to the system.

Currently, this is determined by calling
[`std::thread::available_parallelism`]. Please see its documentation
for caveats.

When using the `current_thread` runtime this method has no effect.

# Examples

## Multi threaded runtime with 4 threads

```
# #[cfg(not(target_family = "wasm"))]
# {
use tokio::runtime;

// This will spawn a work-stealing runtime with 4 worker threads.
let rt = runtime::Builder::new_multi_thread()
    .worker_threads(4)
    .build()
    .unwrap();

rt.spawn(async move {});
# }
```

## Current thread runtime (will only run on the current thread via `Runtime::block_on`)

```
use tokio::runtime;

// Create a runtime that _must_ be driven from a call
// to `Runtime::block_on`.
let rt = runtime::Builder::new_current_thread()
    .build()
    .unwrap();

// This will run the runtime and future on the current thread
rt.block_on(async move {});
```

# Panics

This will panic if `val` is not larger than `0`.

```rust
fn worker_threads(self: & mut Self, val: usize) -> & mut Self
```



