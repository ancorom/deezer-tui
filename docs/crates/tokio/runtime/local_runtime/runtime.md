**tokio > runtime > local_runtime > runtime**

# Module: runtime::local_runtime::runtime

## Contents

**Structs**

- [`LocalRuntime`](#localruntime) - A local Tokio runtime.

---

## tokio::runtime::local_runtime::runtime::LocalRuntime

*Struct*

A local Tokio runtime.

This runtime is capable of driving tasks which are not `Send + Sync` without the use of a
`LocalSet`, and thus supports `spawn_local` without the need for a `LocalSet` context.

This runtime cannot be moved between threads or driven from different threads.

This runtime is incompatible with `LocalSet`. You should not attempt to drive a `LocalSet` within a
`LocalRuntime`.

Currently, this runtime supports one flavor, which is internally identical to `current_thread`,
save for the aforementioned differences related to `spawn_local`.

For more general information on how to use runtimes, see the [module] docs.

[runtime]: crate::runtime::Runtime
[module]: crate::runtime

**Methods:**

- `fn new() -> std::io::Result<LocalRuntime>` - Creates a new local runtime instance with default configuration values.
- `fn handle(self: &Self) -> &Handle` - Returns a handle to the runtime's spawner.
- `fn spawn_local<F>(self: &Self, future: F) -> JoinHandle<<F as >::Output>` - Spawns a task on the runtime.
- `fn spawn_blocking<F, R>(self: &Self, func: F) -> JoinHandle<R>` - Runs the provided function on a thread from a dedicated blocking thread pool.
- `fn block_on<F>(self: &Self, future: F) -> <F as >::Output` - Runs a future to completion on the Tokio runtime. This is the
- `fn enter(self: &Self) -> EnterGuard` - Enters the runtime context.
- `fn shutdown_timeout(self: Self, duration: Duration)` - Shuts down the runtime, waiting for at most `duration` for all spawned
- `fn shutdown_background(self: Self)` - Shuts down the runtime, without waiting for any spawned work to stop.
- `fn metrics(self: &Self) -> crate::runtime::RuntimeMetrics` - Returns a view that lets you get information about how the runtime

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



