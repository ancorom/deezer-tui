**tokio > runtime > builder**

# Module: runtime::builder

## Contents

**Structs**

- [`Builder`](#builder) - Builds Tokio Runtime with custom configuration values.

---

## tokio::runtime::builder::Builder

*Struct*

Builds Tokio Runtime with custom configuration values.

Methods can be chained in order to set the configuration values. The
Runtime is constructed by calling [`build`].

New instances of `Builder` are obtained via [`Builder::new_multi_thread`]
or [`Builder::new_current_thread`].

See function level documentation for details on the various configuration
settings.

[`build`]: method@Self::build
[`Builder::new_multi_thread`]: method@Self::new_multi_thread
[`Builder::new_current_thread`]: method@Self::new_current_thread

# Examples

```
# #[cfg(not(target_family = "wasm"))]
# {
use tokio::runtime::Builder;

fn main() {
    // build runtime
    let runtime = Builder::new_multi_thread()
        .worker_threads(4)
        .thread_name("my-custom-name")
        .thread_stack_size(3 * 1024 * 1024)
        .build()
        .unwrap();

    // use runtime ...
}
# }
```

**Methods:**

- `fn new_current_thread() -> Builder` - Returns a new builder with the current thread scheduler selected.
- `fn new_multi_thread() -> Builder` - Returns a new builder with the multi thread scheduler selected.
- `fn enable_all(self: & mut Self) -> & mut Self` - Enables both I/O and time drivers.
- `fn worker_threads(self: & mut Self, val: usize) -> & mut Self` - Sets the number of worker threads the `Runtime` will use.
- `fn max_blocking_threads(self: & mut Self, val: usize) -> & mut Self` - Specifies the limit for additional threads spawned by the Runtime.
- `fn thread_name<impl Into<String>>(self: & mut Self, val: impl Trait) -> & mut Self` - Sets name of threads spawned by the `Runtime`'s thread pool.
- `fn name<impl Into<String>>(self: & mut Self, val: impl Trait) -> & mut Self` - Sets the name of the runtime.
- `fn thread_name_fn<F>(self: & mut Self, f: F) -> & mut Self` - Sets a function used to generate the name of threads spawned by the `Runtime`'s thread pool.
- `fn thread_stack_size(self: & mut Self, val: usize) -> & mut Self` - Sets the stack size (in bytes) for worker threads.
- `fn on_thread_start<F>(self: & mut Self, f: F) -> & mut Self` - Executes function `f` after each thread is started but before it starts
- `fn on_thread_stop<F>(self: & mut Self, f: F) -> & mut Self` - Executes function `f` before each thread stops.
- `fn on_thread_park<F>(self: & mut Self, f: F) -> & mut Self` - Executes function `f` just before a thread is parked (goes idle).
- `fn on_thread_unpark<F>(self: & mut Self, f: F) -> & mut Self` - Executes function `f` just after a thread unparks (starts executing tasks).
- `fn build(self: & mut Self) -> io::Result<Runtime>` - Creates the configured `Runtime`.
- `fn build_local(self: & mut Self, _options: LocalOptions) -> io::Result<LocalRuntime>` - Creates the configured [`LocalRuntime`].
- `fn thread_keep_alive(self: & mut Self, duration: Duration) -> & mut Self` - Sets a custom timeout for a thread in the blocking pool.
- `fn global_queue_interval(self: & mut Self, val: u32) -> & mut Self` - Sets the number of scheduler ticks after which the scheduler will poll the global
- `fn event_interval(self: & mut Self, val: u32) -> & mut Self` - Sets the number of scheduler ticks after which the scheduler will poll for
- `fn enable_io(self: & mut Self) -> & mut Self` - Enables the I/O driver.
- `fn max_io_events_per_tick(self: & mut Self, capacity: usize) -> & mut Self` - Sets the max number of I/O events processed per tick.
- `fn max_io_events_per_busy_tick(self: & mut Self, capacity: usize) -> & mut Self` - Sets the max number of I/O events a worker processes when it polls
- `fn enable_time(self: & mut Self) -> & mut Self` - Enables the time driver.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`



